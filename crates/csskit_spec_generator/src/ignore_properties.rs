use std::collections::{HashMap, HashSet};

/// Properties to ignore from certain specs because they've moved around or are defined
/// in multiple specifications. These properties should not be generated from these specs.
pub fn get_ignore_properties() -> HashMap<&'static str, HashSet<&'static str>> {
	let mut map = HashMap::new();

	// https://drafts.csswg.org/css-ui-4/#changes-22-12-2017
	// Moved the box-sizing and text-overflow properties to [CSS-SIZING-3] and [CSS-OVERFLOW-4] respectively.
	map.insert("ui", HashSet::from(["box-sizing", "text-overflow"]));

	// CSS Shapes [CSS-SHAPES-2] define the shape-inside property that aligns contents along the edge of a possibly non-rectangular wrapping area.
	// (Round-Display just extends to add the `display` keyword which is specified in shapes-2 anyway)
	map.insert("round-display", HashSet::from(["shape-inside"]));

	map.insert(
		"backgrounds",
		HashSet::from([
			// https://drafts.csswg.org/css-backgrounds-4/#background-layers
			// The name of this property is discussed in issue https://github.com/w3c/csswg-drafts/issues/9083.
			"background-tbd",
			// https://drafts.csswg.org/css-borders-4/#intro
			//  This module is currently maintained as a diff against the parts related to borders and box
			//  decorations of CSS Backgrounds and Borders Module Level 3 [CSS3BG]. We will fold in the text
			//  once it's all formatted up and in CR again, as this will reduce the effort of keeping them in
			//  sync (source diffs will be accurate in reflecting the differences).
			// (IOW these are all defined in CSS Borders 4)
			"border-color",
			"border-top-color",
			"border-right-color",
			"border-bottom-color",
			"border-left-color",
			"border-style",
			"border-top-style",
			"border-right-style",
			"border-bottom-style",
			"border-left-style",
			"border-width",
			"border-top-width",
			"border-right-width",
			"border-bottom-width",
			"border-left-width",
			"border",
			"border-top",
			"border-right",
			"border-bottom",
			"border-left",
			"border-radius",
			"border-top-left-radius",
			"border-top-right-radius",
			"border-bottom-left-radius",
			"border-bottom-right-radius",
			"border-image",
			"border-image-slice",
			"border-image-source",
			"border-image-outset",
			"border-image-repeat",
			"border-image-width",
			"box-shadow",
		]),
	);

	// While GCPM is effectively "deprecated" (see https://github.com/w3c/csswg-drafts/issues/6435) the "string-set"
	// property definition inside of css-content is incomplete, as it is missing <content-list> (see
	// https://github.com/w3c/csswg-drafts/issues/1829). This is properly defined in GCPM. So we should exclude the
	// definition from css-content and use the more complete one from gcpm until both issues are properly addressed.
	map.insert("content", HashSet::from(["string-set"]));

	map.insert(
		"logical",
		HashSet::from([
			// https://drafts.csswg.org/css-logical-1/
			// These are extended definitions which are already defined in their respective specifications
			//
			// https://drafts.csswg.org/css-borders-4/
			"border-block",
			"border-block-color",
			"border-block-end",
			"border-block-end-color",
			"border-block-end-style",
			"border-block-end-width",
			"border-block-start",
			"border-block-start-color",
			"border-block-start-style",
			"border-block-start-width",
			"border-block-style",
			"border-block-width",
			"border-end-end-radius",
			"border-end-start-radius",
			"border-inline",
			"border-inline-color",
			"border-inline-end",
			"border-inline-end-color",
			"border-inline-end-style",
			"border-inline-end-width",
			"border-inline-start",
			"border-inline-start-color",
			"border-inline-start-style",
			"border-inline-start-width",
			"border-inline-style",
			"border-inline-width",
			"border-start-end-radius",
			"border-start-start-radius",
			// https://drafts.csswg.org/css-position-4/
			"inset",
			"inset-block",
			"inset-block-start",
			"inset-block-end",
			"inset-inline",
			"inset-inline-end",
			"inset-inline-start",
		]),
	);

	// https://drafts.csswg.org/css-multicol/
	// Moved the column-rule-* properties to [CSS-GAP-1].
	map.insert(
		"multicol",
		HashSet::from(["column-rule", "column-rule-width", "column-rule-color", "column-rule-style"]),
	);

	// https://drafts.csswg.org/css-flexbox-1/
	// Moved some align properties to [CSS-ALIGN-3].
	map.insert("flexbox", HashSet::from(["align-content", "align-items", "align-self", "justify-content"]));

	// The --* (custom properties) are handled specially in properties/mod.rs with a hardcoded Custom variant
	map.insert("variables", HashSet::from(["--*"]));

	// aspect-ratio, width, height, inline-size, block-size are defined in both conditional and sizing specs. Sizing is the canonical location.
	map.insert("conditional", HashSet::from(["aspect-ratio", "inline-size", "block-size", "width", "height"]));

	// fallback is defined in both anchor-position and counter-styles specs. Counter-styles is the canonical location.
	map.insert("anchor-position", HashSet::from(["fallback"]));

	// src is defined in both color and fonts specs. Fonts is the canonical location (@font-face).
	map.insert("color", HashSet::from(["src"]));

	// speak-as is defined in both counter-styles and speech specs. Speech is the canonical location.
	map.insert("counter-styles", HashSet::from(["speak-as"]));

	map
}
