use css_parse::keyword_set;

// https://drafts.csswg.org/css-text-4/#typedef-spacing-trim
// <spacing-trim> = space-all | normal | space-first | trim-start | trim-both | trim-all
keyword_set!(SpacingTrim {
	SpaceAll: "space-all",
	Normal: "normal",
	SpaceFirst: "space-first",
	TrimStart: "trim-start",
	TrimBoth: "trim-both",
	TrimAll: "trim-all",
});
