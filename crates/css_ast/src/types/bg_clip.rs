use css_parse::keyword_set;

// https://drafts.csswg.org/css-backgrounds-4/#typedef-bg-clip
// <bg-clip> = <visual-box> | border-area| text
// https://drafts.csswg.org/css-box-4/#typedef-visual-box
// <visual-box> = <visual-box> | margin-box
keyword_set!(BgClip {
	ContentBox: "content-box",
	LayoutBox: "padding-box",
	BorderBox: "border-box",
	BorderArea: "border-area",
	Text: "text",
});
