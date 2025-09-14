use crate::{
	Cursor, CursorSink, CursorToSourceCursorSink, ParserReturn, SourceCursor, SourceCursorSink, SourceOffset, Span,
	ToCursors, ToSpan,
};
use bumpalo::{Bump, collections::Vec};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct CursorOverlaySet<'a> {
	bump: &'a Bump,
	overlays: BTreeMap<SourceOffset, (SourceOffset, Vec<'a, SourceCursor<'a>>)>,
}

impl<'a> CursorOverlaySet<'a> {
	pub fn new(bump: &'a Bump) -> Self {
		Self { bump, overlays: BTreeMap::new() }
	}

	pub fn insert<T: ToCursors>(&mut self, span: Span, parser_return: ParserReturn<'a, T>) {
		let start = span.start();
		let end = span.end();
		let mut cursors = Vec::new_in(self.bump);
		let mut sink = CursorToSourceCursorSink::new(parser_return.source_text, &mut cursors);
		parser_return.to_cursors(&mut sink);
		self.overlays.insert(start, (end, cursors));
	}

	pub(crate) fn find(&self, end: SourceOffset) -> Option<&(SourceOffset, Vec<'a, SourceCursor<'a>>)> {
		self.overlays.range(..=end).rev().find(|&(_, &(overlay_end, _))| end < overlay_end).map(|(_, ret)| ret)
	}
}

/// This is a [CursorSink] that wraps a [SourceCursorSink], while also taking a [CursorOverlaySet]. As [Cursor]s get
/// appended into this sink, it will replay those to the underlying [SourceCursorSink] _unless_ a [CursorOverlaySet]
/// overlaps the [Cursor]'s span, at which point the overlay wil be replayed to the underlying [SourceCursorSink].
/// This Sink is useful for collecting new Cursors (say from an AST) to overlap (or, say, transform) the underlying base
/// Cursors (read: AST). In other words, writing over the top of the source.
///
/// Other than replaying overlays in place of the underyling cursors, no other modifications are made to the Cursors,
/// that is up to the base SourceCursorSink, which can apply additional formatting or logic.
#[derive(Debug)]
pub struct CursorOverlaySink<'a, T: SourceCursorSink<'a>> {
	source_text: &'a str,
	overlays: &'a CursorOverlaySet<'a>,
	sink: T,
	processed_overlay_ranges: BTreeMap<SourceOffset, SourceOffset>,
}

impl<'a, T: SourceCursorSink<'a>> CursorOverlaySink<'a, T> {
	pub fn new(source_text: &'a str, overlays: &'a CursorOverlaySet<'a>, sink: T) -> Self {
		Self { source_text, overlays, sink, processed_overlay_ranges: BTreeMap::new() }
	}
}

impl<'a, T: SourceCursorSink<'a>> SourceCursorSink<'a> for CursorOverlaySink<'a, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		let cursor_start = c.to_span().start();
		let cursor_end = c.to_span().end();

		// Check if this entire cursor falls within an already-processed overlay range
		// Look for any processed range that starts at or before cursor_start
		if let Some((&range_start, &range_end)) = self.processed_overlay_ranges.range(..=cursor_start).next_back() {
			if cursor_start >= range_start && cursor_end <= range_end {
				// This cursor is entirely within a processed overlay, skip it
				return;
			}
		}

		let mut pos = cursor_start;
		while pos < cursor_end {
			// dbg!(pos, self.overlays.find(pos));
			if let Some((overlay_end, overlay)) = self.overlays.find(pos) {
				for c in overlay {
					self.sink.append(*c);
				}
				self.processed_overlay_ranges.insert(pos, *overlay_end);
				pos = *overlay_end;
			} else {
				self.sink.append(c);
				pos = c.to_span().end();
			}
		}
	}
}

impl<'a, T: SourceCursorSink<'a>> CursorSink for CursorOverlaySink<'a, T> {
	fn append(&mut self, c: Cursor) {
		SourceCursorSink::append(self, SourceCursor::from(c, c.str_slice(self.source_text)))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{ComponentValue, CursorPrettyWriteSink, CursorWriteSink, Parser, QuoteStyle, T, ToCursors, ToSpan};
	use bumpalo::{Bump, collections::Vec};

	#[test]
	fn test_basic() {
		// Parse the original AST
		let source_text = "black white";
		let bump = Bump::default();
		let mut parser = Parser::new(&bump, source_text);
		let output = parser.parse_entirely::<(T![Ident], T![Ident])>().output.unwrap();

		// Build an overlay AST
		let overlay_text = "green";
		let mut parser2 = Parser::new(&bump, overlay_text);
		let overlay = parser2.parse_entirely::<T![Ident]>();
		let mut overlays = CursorOverlaySet::new(&bump);
		overlays.insert(output.1.to_span(), overlay);

		// Smoosh
		let mut str = String::new();
		let mut stream = CursorOverlaySink::new(source_text, &overlays, CursorWriteSink::new(source_text, &mut str));
		output.to_cursors(&mut stream);

		// str should include overlays
		assert_eq!(str, "black green");
	}

	#[test]
	fn test_with_pretty_writer() {
		// Parse the original AST
		let source_text = "foo{use:other;}";
		let bump = Bump::default();
		let mut parser = Parser::new(&bump, source_text);
		let output = parser.parse_entirely::<Vec<'_, ComponentValue>>().output.unwrap();
		let ComponentValue::SimpleBlock(ref block) = output[1] else { panic!("output[1] was not a block") };
		dbg!(block.to_span(), block.values.to_span());

		// Build an overlay AST
		let overlay_text = "inner{foo: bar;}";
		let mut parser2 = Parser::new(&bump, overlay_text);
		let overlay = parser2.parse_entirely::<Vec<'_, ComponentValue>>();
		let mut overlays = CursorOverlaySet::new(&bump);
		overlays.insert(dbg!(block.values.to_span()), overlay);

		// Smoosh
		let mut str = String::new();
		let mut stream = CursorOverlaySink::new(
			source_text,
			&overlays,
			CursorPrettyWriteSink::new(source_text, &mut str, None, QuoteStyle::Double),
		);
		output.to_cursors(&mut stream);

		// str should include overlays
		assert_eq!(
			str,
			r#"
foo {
	inner {
		foo: bar;
	}
}
			"#
			.trim()
		);
	}
}
