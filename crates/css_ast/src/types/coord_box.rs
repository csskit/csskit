use css_parse::keyword_set;

// https://drafts.csswg.org/css-box-4/#typedef-coord-box
// <coord-box> = <paint-box> | view-box
keyword_set!(CoordBox {
	ContentBox: "content-box",
	PaddingBox: "padding-box",
	BorderBox: "border-box",
	FillBox: "fill-box",
	StrokeBox: "stroke-box",
	ViewBox: "view-box",
});
