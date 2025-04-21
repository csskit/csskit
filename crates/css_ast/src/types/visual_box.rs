use css_parse::keyword_set;

// https://drafts.csswg.org/css-box-4/#typedef-visual-box
// <visual-box> = content-box | padding-box | border-box
keyword_set!(VisualBox { ContentBox: "content-box", PaddingBox: "padding-box", BorderBox: "border-box" });
