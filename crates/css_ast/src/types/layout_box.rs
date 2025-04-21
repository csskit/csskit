use css_parse::keyword_set;

// https://drafts.csswg.org/css-box-4/#typedef-layout-box
// <layout-box> = <visual-box> | margin-box
keyword_set!(LayoutBox {
	ContentBox: "content-box",
	LayoutBox: "padding-box",
	BorderBox: "border-box",
	MarginBox: "margin-box",
});
