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

/// Find types that are queryable (`#[visit]`, `#[visit(self)]`, or `#[visit(all)]`
/// - not skip/children).
///
/// Queryable nodes are those that get a NodeId and can be matched by selectors.
pub fn find_queryable_nodes(dir: &str, matches: &mut HashSet<VisitableNode>, path_callback: impl Fn(&PathBuf) + Copy) {
	let mut all_visitable = HashSet::new();
	find_visitable_nodes(dir, &mut all_visitable, path_callback);
	matches.extend(all_visitable.into_iter().filter(|node| node.visit_mode.is_queryable()));
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
