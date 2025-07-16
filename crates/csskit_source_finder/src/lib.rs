use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use std::str::from_utf8;

use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::{RegexMatcher, RegexMatcherBuilder};
use grep_searcher::{Searcher, SearcherBuilder, Sink, SinkError, SinkMatch};
use syn::{Type, parse_str};

pub struct NodeMatcher<'a> {
	matcher: &'a RegexMatcher,
	matches: &'a mut HashSet<Type>,
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
			dbg!(&line, &captures, captures.get(2).map(|r| &line[r]), captures.get(5).map(|r| &line[r]));
			let capture = &line[captures.get(5).unwrap()];
			if !capture.is_empty() {
				if let Ok(ty) = parse_str(capture) {
					self.matches.insert(ty);
				}
			} else {
				dbg!(&line);
				panic!("#[visit] or #[value] on unknown");
			}
			true
		})?;
		Ok(true)
	}
}

pub fn find_visitable_nodes(dir: &str, matches: &mut HashSet<Type>, path_callback: impl Fn(&PathBuf)) {
	let matcher = RegexMatcherBuilder::new()
		.multi_line(true)
		.dot_matches_new_line(true)
		.ignore_whitespace(true)
		.build(
			r#"
			# match the #[value] or #[visit] attribute
			^\s*\#\[(value|visit)
			# munch the data between the attribute and the definition
			.*?
			(
				# Is this a public definition?
				pub\s*(?:struct|enum)\s*
			)
			# munch any comments/attributes between this and our name (for macros)
			(:?\n?\s*(:?\/\/|\#)[^\n]*)*
			# finally grab the word (plus any lifetime definition)
			\s*(\w*(:?<'a>)?)"#,
		)
		.unwrap();
	let mut searcher = SearcherBuilder::new().line_number(false).multi_line(true).build();
	let entries = glob(dir).unwrap();
	for entry in entries.filter_map(|p| p.ok()) {
		path_callback(&entry);
		let context = NodeMatcher { matcher: &matcher, matches };
		searcher.search_path(&matcher, entry, context).unwrap();
	}
}
