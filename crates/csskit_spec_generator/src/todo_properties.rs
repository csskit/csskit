use std::collections::{HashMap, HashSet};

/// Properties that should be commented out because their parsing rules are tricky.
/// Once we figure out how to parse them in csskit_proc_macro/src/def.rs, we can
/// remove them from this map and the generator will uncomment them.
pub fn get_todo_properties() -> HashMap<&'static str, HashSet<&'static str>> {
	let mut map = HashMap::new();

	map.insert(
		"align",
		HashSet::from([
			"justify-content",
			"justify-self",
			"place-self",
			"justify-items",
			"align-items",
			"place-content",
			"place-items",
		]),
	);

	map.insert("anchor-position", HashSet::from(["position-visibility", "position-try-fallbacks", "position-try"]));

	map.insert(
		"animations",
		HashSet::from([
			"animation",
			"animation-trigger-exit-range",
			"animation-trigger-exit-range-end",
			"animation-trigger-exit-range-start",
			"animation-trigger-range",
			"animation-trigger-range-end",
			"animation-trigger-range-start",
			"timeline-trigger",
			"timeline-trigger-range",
			"timeline-trigger-range-end",
			"timeline-trigger-range-start",
			"timeline-trigger-exit-range",
			"timeline-trigger-exit-range-end",
			"timeline-trigger-exit-range-start",
		]),
	);

	map.insert(
		"backgrounds",
		HashSet::from([
			"background",
			"background-position",
			"background-position-block",
			"background-position-inline",
			"background-position-x",
			"background-position-y",
		]),
	);

	map.insert(
		"borders",
		HashSet::from([
			"border-block-end-radius",
			"border-block-start-radius",
			"border-bottom-radius",
			"border-color",
			"border-inline-end-radius",
			"border-inline-start-radius",
			"border-left-radius",
			"border-limit",
			"border-radius",
			"border-right-radius",
			"border-shape",
			"border-top-radius",
			"box-shadow-offset",
			"corner-top-left",
			"corner-top-right",
			"corner-bottom-left",
			"corner-bottom-right",
			"corner-start-start",
			"corner-start-end",
			"corner-end-start",
			"corner-end-end",
			"corner-top",
			"corner-left",
			"corner-right",
			"corner-bottom",
			"corner-block-start",
			"corner-block-end",
			"corner-inline-start",
			"corner-inline-end",
			"corner",
			"border-image",
			"border-image-slice",
			"border-image-width",
		]),
	);

	map.insert("box", HashSet::from(["margin-trim"]));

	map.insert("color-adjust", HashSet::from(["color-scheme"]));

	map.insert("conditional", HashSet::from(["container", "container-type"]));

	map.insert("contain", HashSet::from(["contain"]));

	map.insert("content", HashSet::from(["content", "quotes"]));

	map.insert("display", HashSet::from(["display"]));

	map.insert("flexbox", HashSet::from(["flex"]));

	map.insert(
		"fonts",
		HashSet::from([
			"font",
			"font-feature-settings",
			"font-palette",
			"font-size-adjust",
			"font-synthesis",
			"font-variant",
			"font-variant-alternates",
			"font-variant-east-asian",
			"font-variant-ligatures",
			"font-variant-numeric",
			"font-variation-settings",
		]),
	);

	map.insert("gcpm", HashSet::from(["string-set", "copy-into"]));

	map.insert(
		"grid",
		HashSet::from([
			"grid",
			"grid-area",
			"grid-auto-flow",
			"grid-column",
			"grid-row",
			"grid-template",
			"grid-template-columns",
			"grid-template-rows",
			"item-cross",
			"item-flow",
			"item-pack",
			"item-wrap",
		]),
	);

	map.insert("images", HashSet::from(["image-orientation", "image-resolution", "object-fit"]));

	map.insert("inline", HashSet::from(["initial-letter", "initial-letter-align", "text-box", "vertical-align"]));

	map.insert("multicol", HashSet::from(["columns"]));

	map.insert("lists", HashSet::from(["counter-increment", "counter-reset", "counter-set", "list-style"]));

	map.insert("overflow", HashSet::from(["line-clamp", "scrollbar-gutter", "text-overflow"]));

	map.insert("regions", HashSet::from(["flow-into"]));

	map.insert("ruby", HashSet::from(["ruby-position"]));

	map.insert("scroll-snap", HashSet::from(["scroll-snap-type"]));

	map.insert("shapes", HashSet::from(["shape-inside", "shape-outside"]));

	map.insert(
		"sizing",
		HashSet::from([
			"contain-intrinsic-block-size",
			"contain-intrinsic-height",
			"contain-intrinsic-inline-size",
			"contain-intrinsic-size",
			"contain-intrinsic-width",
			"min-intrinsic-sizing",
		]),
	);

	map.insert(
		"speech",
		HashSet::from(["speak-as", "voice-family", "voice-pitch", "voice-range", "voice-rate", "voice-volume"]),
	);

	map.insert(
		"text",
		HashSet::from([
			"hanging-punctuation",
			"hyphenate-limit-chars",
			"text-indent",
			"text-justify",
			"text-spacing",
			"text-transform",
			"white-space",
			"white-space-trim",
			"word-space-transform",
		]),
	);

	map.insert(
		"text-decor",
		HashSet::from([
			"text-decoration",
			"text-decoration-line",
			"text-decoration-skip-self",
			"text-decoration-skip-spaces",
			"text-emphasis",
			"text-emphasis-position",
			"text-emphasis-style",
			"text-underline-position",
		]),
	);

	map.insert("transforms", HashSet::from(["rotate", "transform-origin", "translate"]));

	map.insert("ui", HashSet::from(["nav-down", "nav-left", "nav-right", "nav-up"]));

	map.insert("variables", HashSet::from(["--*"]));

	map.insert("writing-modes", HashSet::from(["glyph-orientation-vertical"]));

	map.insert("counter-styles", HashSet::from(["range", "additive-symbols"]));

	// Font properties with complex grammars that cause panics or use unsupported operators like /
	map.insert(
		"fonts",
		HashSet::from([
			"superscript-size-override",
			"superscript-position-override",
			"ascent-override",
			"line-gap-override",
			"subscript-size-override",
			"descent-override",
			"subscript-position-override",
			"font-variant",
			"font-variant-east-asian",
			"font-size-adjust",
			"font-variant-numeric",
			"font-variant-ligatures",
			"font-variant-alternates",
			"font",                    // Uses / operator for font-size / line-height
			"unicode-range",           // Requires UnicodeRangeToken type
			"src",                     // Requires FontSrcList type
			"font-palette",            // Requires PaletteMixFunction function
			"font-feature-settings",   // Requires FeatureTagValue type
			"font-variation-settings", // Requires OpentypeTag type
		]),
	);

	// Page properties with complex grammars
	map.insert("page", HashSet::from(["size", "marks"]));

	// Mixins properties - declaration-value is a trait not a type
	map.insert("mixins", HashSet::from(["result"]));

	map
}
