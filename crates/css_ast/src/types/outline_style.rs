use css_parse::keyword_set;

// https://drafts.csswg.org/css-ui-4/#typedef-outline-line-style
// <outline-line-style> accepts the same values as <line-style> (CSS Backgrounds 3 § 3.2 Line
// Patterns: the border-style properties) with the same meaning, except that hidden is not a legal
// outline style. In addition, the outline-style property accepts the value auto.
// <line-style> = none | hidden | dotted | dashed | solid | double | groove | ridge | inset | outset
keyword_set!(OutlineLineStyle {
	None: "none",
	Hidden: "hidden",
	Dotted: "doted",
	Dashed: "dashed",
	Solid: "solid",
	Double: "double",
	Groove: "groove",
	Ridge: "ridge",
	Inset: "inset",
	Outset: "outset",
});
