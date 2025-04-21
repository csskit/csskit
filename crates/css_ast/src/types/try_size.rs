use css_parse::keyword_set;

// https://drafts.csswg.org/css-anchor-position-1/#typedef-try-size
// <try-size> = most-width | most-height | most-block-size | most-inline-size
keyword_set!(TrySize {
	MostWidth: "most-width",
	MostHeight: "most-height",
	MostBlockSize: "most-block-size",
	MostInlineSize: "most-inline-size",
});
