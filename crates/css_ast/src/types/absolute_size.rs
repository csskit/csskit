use css_parse::keyword_set;

// https://drafts.csswg.org/css2/#value-def-absolute-size
// <absolute-size> = [ xx-small | x-small | small | medium | large | x-large | xx-large ]
keyword_set!(AbsoluteSize {
	XxSmall: "xx-small",
	XSmall: "x-small",
	Small: "small",
	Medium: "medium",
	Large: "large",
	XLarge: "x-large",
	XxLarge: "xx-large",
});
