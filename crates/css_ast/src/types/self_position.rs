use css_parse::keyword_set;

// https://drafts.csswg.org/css-align-3/#typedef-self-position
keyword_set!(SelfPosition {
	Center: "center",
	Start: "start",
	End: "end",
	SelfStart: "self-start",
	SelfEnd: "self-end",
	FlexStart: "flex-start",
	FlexEnd: "flex-end",
});
