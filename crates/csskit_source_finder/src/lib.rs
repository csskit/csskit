#![deny(warnings)]
use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use std::str::from_utf8;

use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::{RegexMatcher, RegexMatcherBuilder};
use grep_searcher::{Searcher, SearcherBuilder, Sink, SinkError, SinkMatch};
use syn::{DeriveInput, parse_str};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisitMode {
	/// `#[visit]` or `#[visit(self)]`
	Self_,
	/// `#[visit(all)]`
	All,
	/// `#[visit(skip)]`
	Skip,
	/// `#[visit(children)]`
	Children,
	/// Manual impl VisitableTrait
	Manual,
}

impl VisitMode {
	/// Returns true if this mode makes the node queryable (has a visit_self call)
	pub fn is_queryable(&self) -> bool {
		matches!(self, VisitMode::Self_ | VisitMode::All | VisitMode::Manual)
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

pub struct NodeMatcher<'a> {
	matcher: &'a RegexMatcher,
	matches: &'a mut HashSet<VisitableNode>,
}

impl Sink for NodeMatcher<'_> {
	type Error = io::Error;

	fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error> {
		let mut captures = self.matcher.new_captures()?;
		let line = match from_utf8(mat.bytes()) {
			Ok(matched) => matched,
			Err(err) => return Err(io::Error::error_message(err)),
		};
		self.matcher.captures_iter(mat.bytes(), &mut captures, |captures| -> bool {
			// Group 1 contains everything between derive and pub struct/enum
			let attrs_section = &line[captures.get(1).unwrap()];

			// Search for visit attribute in the captured section
			// The default (no visit attr) is Children mode per derive macro semantics
			let visit_mode = if attrs_section.contains("visit(skip)") {
				VisitMode::Skip
			} else if attrs_section.contains("visit(children)") {
				VisitMode::Children
			} else if attrs_section.contains("visit(all)") {
				VisitMode::All
			} else if attrs_section.contains("visit(self)") {
				VisitMode::Self_
			} else if attrs_section.contains("visit") {
				// Just `visit` (or `visit()`) means visit self AND children
				VisitMode::All
			} else {
				// No visit attribute found - default is children only (not queryable)
				VisitMode::Children
			};

			let capture = format!("{} {} {{}}", &line[captures.get(2).unwrap()], &line[captures.get(5).unwrap()]);
			match parse_str::<DeriveInput>(&capture) {
				Ok(input) => {
					self.matches.insert(VisitableNode { input, visit_mode });
				}
				Err(err) => {
					panic!("#[visit] or unknown: {capture} {err}");
				}
			}
			true
		})?;
		Ok(true)
	}
}

fn build_visit_attr_matcher() -> RegexMatcher {
	RegexMatcherBuilder::new()
		.multi_line(true)
		.dot_matches_new_line(true)
		.ignore_whitespace(true)
		.build(
			r#"
			^\s*\#\[
			# Match any type with derive(Visitable)
			cfg_attr\([^,]+,\s*derive\((?:csskit_derives::)?Visitable\)
			# Capture everything from here until the type declaration to search for visit attr
			# This captures the visit attr whether it's on same line or separate line
			([^\{\}]*?)
			# Match the type declaration
			(pub\s*(?:struct|enum)\s*)
			# munch any comments/attributes between this and our name (for macros)
			(:?\n?\s*(:?\/\/|\#)[^\n]*)*
			# finally grab the word (plus any generics)
			\s*(\w*(:?<[^>]+>)?)"#,
		)
		.unwrap()
}

fn build_manual_impl_matcher() -> RegexMatcher {
	RegexMatcherBuilder::new()
		.multi_line(true)
		.ignore_whitespace(true)
		.build(
			r#"
			# Match manual impl VisitableTrait for Type
			impl\s*(?:<[^>]+>\s*)?
			VisitableTrait\s+for\s+
			# Capture the type name with optional generics
			(\w+)(?:<[^>]+>)?"#,
		)
		.unwrap()
}

pub struct ManualImplMatcher<'a> {
	matcher: &'a RegexMatcher,
	matches: &'a mut HashSet<VisitableNode>,
}

impl Sink for ManualImplMatcher<'_> {
	type Error = io::Error;

	fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error> {
		let mut captures = self.matcher.new_captures()?;
		let line = match from_utf8(mat.bytes()) {
			Ok(matched) => matched,
			Err(err) => return Err(io::Error::error_message(err)),
		};
		self.matcher.captures_iter(mat.bytes(), &mut captures, |captures| -> bool {
			let type_name = &line[captures.get(1).unwrap()];
			// Skip if already found by attr matcher
			if self.matches.iter().any(|n| n.input.ident == type_name) {
				return true;
			}
			let capture = format!("pub struct {} {{}}", type_name);
			match parse_str::<DeriveInput>(&capture) {
				Ok(input) => {
					self.matches.insert(VisitableNode { input, visit_mode: VisitMode::Manual });
				}
				Err(err) => {
					panic!("manual impl VisitableTrait: {capture} {err}");
				}
			}
			true
		})?;
		Ok(true)
	}
}

/// Find all types with `#[visit]` attribute or manual VisitableTrait impl
pub fn find_visitable_nodes(dir: &str, matches: &mut HashSet<VisitableNode>, path_callback: impl Fn(&PathBuf) + Copy) {
	let attr_matcher = build_visit_attr_matcher();
	let manual_matcher = build_manual_impl_matcher();
	let mut searcher = SearcherBuilder::new().line_number(false).multi_line(true).build();
	let entries: Vec<_> = glob(dir).unwrap().filter_map(|p| p.ok()).collect();
	// First pass: find types with derive(Visitable)
	for entry in &entries {
		path_callback(entry);
		let context = NodeMatcher { matcher: &attr_matcher, matches };
		searcher.search_path(&attr_matcher, entry, context).unwrap();
	}
	// Second pass: find types with manual impl VisitableTrait
	for entry in &entries {
		let context = ManualImplMatcher { matcher: &manual_matcher, matches };
		searcher.search_path(&manual_matcher, entry, context).unwrap();
	}
}

/// Find types that are queryable (have `#[visit]`, `#[visit(self)]`, or `#[visit(all)]` - not skip/children)
///
/// Queryable nodes are those that get a NodeId and can be matched by selectors.
pub fn find_queryable_nodes(dir: &str, matches: &mut HashSet<VisitableNode>, path_callback: impl Fn(&PathBuf) + Copy) {
	let mut all_visitable = HashSet::new();
	find_visitable_nodes(dir, &mut all_visitable, path_callback);
	// Filter to only queryable modes
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
