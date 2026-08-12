#![deny(warnings)]
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;

use glob::glob;
use syn::{DeriveInput, parse_str};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisitMode {
	/// `#[visit]` or `#[visit(self)]`
	Self_,
	/// `#[visit(all)]`
	All,
	/// `#[visit(skip)]`
	Skip,
	/// `#[visit(children)]`
	Children,
}

impl VisitMode {
	/// Returns true if this mode makes the node queryable (has a visit_self call)
	pub fn is_queryable(&self) -> bool {
		matches!(self, VisitMode::Self_ | VisitMode::All)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VisitableNode {
	pub input: DeriveInput,
	pub visit_mode: VisitMode,
}

impl VisitableNode {
	pub fn ident(&self) -> &syn::Ident {
		&self.input.ident
	}

	pub fn generics(&self) -> &syn::Generics {
		&self.input.generics
	}
}

/// Parses a `pub struct`/`pub enum` declaration line into its kind keyword, name,
/// and (optional) generics header. Everything from the name onwards is truncated
/// at the body/where-clause so the reconstructed type carries only name + generics.
fn parse_decl(line: &str) -> Option<(&'static str, String, String)> {
	let mut rest = line.trim_start();
	rest = rest.strip_prefix("pub")?;
	// Optional visibility restriction, e.g. `pub(crate)`.
	rest = rest.trim_start();
	if let Some(after) = rest.strip_prefix('(') {
		let close = after.find(')')?;
		rest = after[close + 1..].trim_start();
	}
	let kind = if let Some(r) = rest.strip_prefix("enum") {
		rest = r;
		"enum"
	} else {
		let r = rest.strip_prefix("struct")?;
		rest = r;
		"struct"
	};
	// A keyword boundary must follow (whitespace before the name).
	if !rest.starts_with(char::is_whitespace) {
		return None;
	}
	rest = rest.trim_start();
	let name_end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_')).unwrap_or(rest.len());
	let name = &rest[..name_end];
	if name.is_empty() {
		return None;
	}
	let after_name = &rest[name_end..];
	// Capture a single-level generics header `<...>` if present, mirroring the
	// legacy finder (no nested `>`), stopping before the body/where-clause.
	let generics = if after_name.trim_start().starts_with('<') {
		let start = after_name.find('<').unwrap();
		match after_name[start..].find('>') {
			Some(end) => after_name[start..=start + end].to_string(),
			None => String::new(),
		}
	} else {
		String::new()
	};
	Some((kind, name.to_string(), generics))
}

/// Reads the `visit(...)` mode off a type's attribute block. `visit(skip)` beats
/// `visit(children)` beats `visit(all)` beats `visit(self)`/bare `visit`; a
/// visitable type with no `visit` at all defaults to children-only.
///
/// `feature = "visitable"` also contains the substring `visit`, so bare `visit`
/// is detected as `visit)` (the token that closes its `cfg_attr`), never a raw
/// `visit` substring.
fn visit_mode(block: &str) -> VisitMode {
	if block.contains("visit(skip)") {
		VisitMode::Skip
	} else if block.contains("visit(children)") {
		VisitMode::Children
	} else if block.contains("visit(all)") {
		VisitMode::All
	} else if ["visit(self)", "visit)", "visit,", "visit]"].iter().any(|p| block.contains(p)) {
		VisitMode::Self_
	} else {
		VisitMode::Children
	}
}

fn brackets(line: &str) -> i32 {
	line.bytes().fold(0i32, |acc, b| match b {
		b'(' | b'[' => acc + 1,
		b')' | b']' => acc - 1,
		_ => acc,
	})
}

/// Scans a source file for every `#[node]`/`#[syntax]`-tagged, `Visitable` node
/// type, recording its name, generics and visit mode. Line based so it sees
/// types written directly, inside `macro_rules!` templates, and inside
/// `ranged_feature!`/`discrete_feature!`/`boolean_feature!` invocations alike.
fn collect(content: &str, matches: &mut HashSet<VisitableNode>) {
	let mut block: Vec<&str> = Vec::new();
	let mut depth = 0i32;
	for line in content.lines() {
		if depth > 0 {
			block.push(line);
			depth = (depth + brackets(line)).max(0);
			continue;
		}
		let t = line.trim_start();
		if let Some((kind, name, generics)) = parse_decl(line) {
			let has_marker = block.iter().any(|l| {
				let l = l.trim_start();
				l.starts_with("#[node]") || l.starts_with("#[syntax")
			});
			let joined =
				block.iter().filter(|l| !l.trim_start().starts_with("//")).copied().collect::<Vec<_>>().join("\n");
			if has_marker && joined.contains("Visitable") {
				let src = format!("pub {kind} {name}{generics} {{}}");
				match parse_str::<DeriveInput>(&src) {
					Ok(input) => {
						matches.insert(VisitableNode { input, visit_mode: visit_mode(&joined) });
					}
					Err(err) => panic!("could not reconstruct node type: {src} {err}"),
				}
			}
			block.clear();
		} else if t.is_empty() {
			block.clear();
		} else if t.starts_with("#[") {
			block.push(line);
			depth = (depth + brackets(line)).max(0);
		} else if t.starts_with("//") {
			block.push(line);
		} else {
			block.clear();
		}
	}
}

/// Find all visitable node types (`#[node]`/`#[syntax]` types carrying a
/// `Visitable` derive), excluding `visit(children)`-only types.
pub fn find_visitable_nodes(dir: &str, matches: &mut HashSet<VisitableNode>, path_callback: impl Fn(&PathBuf) + Copy) {
	let mut all: HashSet<VisitableNode> = HashSet::new();
	for entry in glob(dir).unwrap().filter_map(|p| p.ok()) {
		path_callback(&entry);
		collect(&read_to_string(&entry).unwrap(), &mut all);
	}
	matches.extend(all.into_iter().filter(|node| !matches!(node.visit_mode, VisitMode::Children)));
}

/// Extracts the name of a `struct`/`enum`/`type` declaration on `line` along with whether its
/// generics include a lifetime parameter (`Foo<'a, ...>`). Text-based so it also sees declarations
/// nested inside macro invocations, which a `syn` item walk would miss.
fn decl_name(line: &str) -> Option<(&str, bool)> {
	let line = line.trim_start();
	let line = if let Some(rest) = line.strip_prefix("pub(") {
		match rest.find(')') {
			Some(i) => rest[i + 1..].trim_start(),
			None => rest,
		}
	} else if let Some(rest) = line.strip_prefix("pub ") {
		rest.trim_start()
	} else {
		line
	};
	let rest = ["struct ", "enum ", "type "].iter().find_map(|kw| line.strip_prefix(kw))?;
	let rest = rest.trim_start();
	let name_end = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))?;
	let name = &rest[..name_end];
	if name.is_empty() {
		return None;
	}
	// A lifetime parameter, if present, is the first generic argument, so it lands within the
	// first `<...>` group after the name.
	let has_lifetime = rest[name_end..]
		.trim_start()
		.strip_prefix('<')
		.and_then(|generics| generics.find('>').map(|end| generics[..end].contains('\'')))
		.unwrap_or(false);
	Some((name, has_lifetime))
}

/// Find the names of every `struct`/`enum`/`type` declaration that carries no lifetime parameter in
/// *any* of its declarations across all `.rs` files matching `dir` (a glob), including declarations
/// produced inside macro invocations.
///
/// A `<type>` grammar reference resolves to an AST type; the overwhelming majority carry their own
/// `'a` (and so must be emitted as `Name<'a>` inside a value slot). The sized minority is the
/// exception, so we enumerate it directly and treat everything else as unsized by default. When a
/// listed type later gains a lifetime it simply drops out of this set and is treated as unsized
/// automatically. A name declared both with and without a lifetime (e.g. a real `Foo<'a>` type plus
/// a lifetime-free macro-template stub) counts as unsized.
pub fn find_sized_types(dir: &str, path_callback: impl Fn(&PathBuf) + Copy) -> std::collections::BTreeSet<String> {
	let mut sized = std::collections::BTreeSet::new();
	let mut unsized_names = std::collections::BTreeSet::new();
	for entry in glob(dir).unwrap().filter_map(|p| p.ok()) {
		path_callback(&entry);
		let Ok(source) = std::fs::read_to_string(&entry) else { continue };
		let mut depth = 0i32;
		let mut test_depth = None;
		let mut pending_test_attr = false;
		for line in source.lines() {
			let trimmed = line.trim();
			if test_depth.is_none() {
				if pending_test_attr && trimmed.starts_with("mod ") {
					test_depth = Some(depth);
				} else if trimmed.starts_with("#[cfg(") && trimmed.contains("test") {
					pending_test_attr = true;
				} else if !trimmed.is_empty() && !trimmed.starts_with("#[") {
					pending_test_attr = false;
				}
			}
			if test_depth.is_none()
				&& let Some((name, has_lifetime)) = decl_name(line)
			{
				if has_lifetime {
					unsized_names.insert(name.to_string());
				} else {
					sized.insert(name.to_string());
				}
			}
			depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
			if test_depth.is_some_and(|d| depth <= d) {
				test_depth = None;
			}
		}
	}
	sized.retain(|name| !unsized_names.contains(name));
	sized
}

#[test]
fn test_find_visitable_nodes() {
	use itertools::Itertools;
	use quote::ToTokens;
	let mut matches = HashSet::new();
	find_visitable_nodes("../css_ast/src/**/*.rs", &mut matches, |_| {});
	::insta::assert_ron_snapshot!(
		"all_visitable_nodes",
		matches.iter().map(|node| node.input.to_token_stream().to_string()).sorted().collect::<Vec<_>>()
	);
}

#[test]
fn test_find_sized_types() {
	let types = find_sized_types("../css_ast/src/**/*.rs", |_| {});
	// Sized numeric primitives and keyword enums must be present.
	assert!(types.contains("Length"), "expected sized Length to be found");
	// Lifetime-carrying types must be absent.
	assert!(!types.contains("Color"), "Color carries a lifetime and must not appear");
	// A macro-defined lifetime-carrying declaration must also be absent.
	assert!(!types.contains("Rule"), "macro-defined Rule carries a lifetime and must not appear");
}
