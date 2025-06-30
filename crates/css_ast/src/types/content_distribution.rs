use css_lexer::Cursor;
use css_parse::{diagnostics, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use css_parse::keyword_set;

// https://drafts.csswg.org/css-align-3/#typedef-content-distribution
// <content-distribution> = space-between | space-around | space-evenly | stretch
keyword_set!(ContentDistribution {
	SpaceBetween: "space-between",
	SpaceAround: "space-around",
	SpaceEvenly: "space-evenly",
	Stretch: "stretch",
});
