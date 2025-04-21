use css_parse::keyword_set;

// https://drafts.csswg.org/css-box-4/#typedef-paint-box
// <paint-box> = <visual-box> | fill-box | stroke-box
keyword_set!(PaintBox {
	ContentBox: "content-box",
	PaddingBox: "padding-box",
	BorderBox: "border-box",
	FillBox: "fill-box",
	StrokeBox: "stroke-box",
});
