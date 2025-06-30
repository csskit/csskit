use css_lexer::Cursor;
use css_parse::{diagnostics, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use css_parse::keyword_set;

// https://drafts.csswg.org/css-align-3/#typedef-content-position
// <content-position> = center | start | end | flex-start | flex-end
keyword_set!(ContentPosition {
	Center: "center",
	Start: "start",
	End: "end",
	FlexStart: "flex-start",
	FlexEnd: "flex-end",
});
