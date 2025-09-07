import { DOMParser } from "jsr:@b-fuze/deno-dom";

// TODO!!!
// These properties have yet to be implemented because their parsing rules can be a little tricky.
// We should figure out how to parse them in ../../crates/csskit_proc_macro/src/def.rs then we can
// remove them from this Map and the generator will uncomment them!
const todoPropertiesThatWillBeCommentedOut = new Map([
	[
		"align",
		new Set([
			// normal | <content-distribution> | <overflow-position>? [ <content-position> | left | right ]
			"justify-content",

			// auto | normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ]
			"justify-self",

			// <'align-self'> <'justify-self'>?
			"place-self",

			// normal | stretch | <baseline-position> | <overflow-position>? [ <self-position> | left | right ] | legacy | legacy && [ left | right | center ]
			"justify-items",

			// normal | stretch | <baseline-position> | [ <overflow-position>? <self-position> ]
			"align-items",

			// <'align-content'> <'justify-content'>?
			"place-content",

			// <'align-content'> <'justify-content'>?
			"place-items",
		]),
	],
	[
		"anchor-position",
		new Set([
			// always | [ anchors-valid || anchors-visible || no-overflow ]
			"position-visibility",

			// none | [ [<dashed-ident> || <try-tactic>] | <'position-area'> ]#
			"position-try-fallbacks",

			// <'position-try-order'>? <'position-try-fallbacks'>
			"position-try",
		]),
	],
	[
		"animations",
		new Set([
			// <single-animation>#
			"animation",

			// [ <'animation-trigger-exit-range-start'> <'animation-trigger-exit-range-end'>? ]#
			"animation-trigger-exit-range",

			// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
			"animation-trigger-exit-range-end",

			// [ auto | normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
			"animation-trigger-exit-range-start",

			// [ <'animation-trigger-range-start'> <'animation-trigger-range-end'>? ]#
			"animation-trigger-range",

			// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
			"animation-trigger-range-end",

			// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]#
			"animation-trigger-range-start",
		]),
	],
	[
		"backgrounds",
		new Set([
			// <bg-layer>#? , <final-bg-layer>
			"background",

			// <bg-position>#
			"background-position",

			// [ center | [ [ start | end ]? <length-percentage>? ]! ]#
			"background-position-block",

			// [ center | [ [ start | end ]? <length-percentage>? ]! ]#
			"background-position-inline",

			// [ center | [ [ left | right | x-start | x-end ]? <length-percentage>? ]! ]#
			"background-position-x",

			// [ center | [ [ top | bottom | y-start | y-end ]? <length-percentage>? ]! ]#
			"background-position-y",

			// <'border-image-source'> || <'border-image-slice'> [ / <'border-image-width'> | / <'border-image-width'>? / <'border-image-outset'> ]? || <'border-image-repeat'>
			"border-image",

			// [<number [0,∞]> | <percentage [0,∞]>]{1,4} && fill?
			"border-image-slice",

			// [ <length-percentage [0,∞]> | <number [0,∞]> | auto ]{1,4}
			"border-image-width",
		]),
	],
	[
		"borders",
		new Set([
			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-block-end-radius",

			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-block-start-radius",

			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-bottom-radius",

			// [ <color> | <image-1D> ]{1,4}
			"border-color",

			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-inline-end-radius",

			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-inline-start-radius",

			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-left-radius",

			// all | [ sides | corners ] <length-percentage [0,∞]>? | [ top | right | bottom | left ] <length-percentage [0,∞]>
			"border-limit",

			// <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]?
			"border-radius",

			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-right-radius",

			// none | [ <basic-shape> <geometry-box>?]{1,2}
			"border-shape",

			// <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]?
			"border-top-radius",

			// [ none | <length>{2} ]#
			"box-shadow-offset",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-top-left",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-top-right",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-bottom-left",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-bottom-right",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-start-start",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-start-end",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-end-start",

			// <length-percentage [0,∞]>{1,2} || <corner-shape-value>
			"corner-end-end",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-top",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-left",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-right",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-bottom",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-block-start",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-block-end",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-inline-start",

			// [ <length-percentage [0,∞]>{1,2} [ / <length-percentage [0,∞]>{1,2} ]? ] || <corner-shape-value>{1,2}
			"corner-inline-end",

			// [ <length-percentage [0,∞]>{1,4} [ / <length-percentage [0,∞]>{1,4} ]? ] || <corner-shape-value>{1,4}
			"corner",
		]),
	],
	[
		"box",
		new Set([
			// none | [ block || inline ] | [ block-start || inline-start || block-end || inline-end ]
			"margin-trim",
		]),
	],
	[
		"color-adjust",
		new Set([
			// normal | [ light | dark | <custom-ident> ]+ && only?
			"color-scheme",
		]),
	],
	[
		"conditional",
		new Set([
			// <'container-name'> [ / <'container-type'> ]?
			"container",
			// normal | [ [ size | inline-size ] || scroll-state ]
			"container-type",
		]),
	],
	[
		"contain",
		new Set([
			// none | strict | content | [ [size | inline-size] || layout || style || paint ]
			"contain",
		]),
	],
	[
		"content",
		new Set([
			// normal | none | [ <content-replacement> | <content-list> ] [/ [ <string> | <counter> | <attr()> ]+ ]?
			"content",

			// auto | none | match-parent | [ <string> <string> ]+
			"quotes",
		]),
	],
	[
		"display",
		new Set([
			// [ <display-outside> || <display-inside> ] | <display-listitem> | <display-internal> | <display-box> | <display-legacy>
			"display",
		]),
	],
	[
		"flexbox",
		new Set([
			// none | [ <'flex-grow'> <'flex-shrink'>? || <'flex-basis'> ]
			"flex",
		]),
	],
	[
		"fonts",
		new Set([
			// [ [ <'font-style'> || <font-variant-css2> || <'font-weight'> || <font-width-css3> ]? <'font-size'> [ / <'line-height'> ]? <'font-family'># ] | <system-family-name>
			"font",

			// normal | <feature-tag-value>#
			"font-feature-settings",

			// normal | light | dark | <palette-identifier> | <palette-mix()>
			"font-palette",

			// none | [ ex-height | cap-height | ch-width | ic-width | ic-height ]? [ from-font | <number [0,∞]> ]
			"font-size-adjust",

			// none | [ weight || style || small-caps || position]
			"font-synthesis",

			// normal | none | [ [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ] || [ small-caps | all-small-caps | petite-caps | all-petite-caps | unicase | titling-caps ] || [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ] || [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ] || [ <east-asian-variant-values> || <east-asian-width-values> || ruby ] || [ sub | super ] || [ text | emoji | unicode ] ]
			"font-variant",

			// normal | [ stylistic(<feature-value-name>) || historical-forms || styleset(<feature-value-name>#) || character-variant(<feature-value-name>#) || swash(<feature-value-name>) || ornaments(<feature-value-name>) || annotation(<feature-value-name>) ]
			"font-variant-alternates",

			// normal | [ <east-asian-variant-values> || <east-asian-width-values> || ruby ]
			"font-variant-east-asian",

			// normal | none | [ <common-lig-values> || <discretionary-lig-values> || <historical-lig-values> || <contextual-alt-values> ]
			"font-variant-ligatures",

			// normal | [ <numeric-figure-values> || <numeric-spacing-values> || <numeric-fraction-values> || ordinal || slashed-zero ]
			"font-variant-numeric",

			// normal | [ <opentype-tag> <number> ]#
			"font-variation-settings",
		]),
	],
	[
		"gcpm",
		new Set([
			// [ <custom-ident> <content-list> ]# | none
			"string-set",
			// none |  [ [ <custom-ident>  <content-level>] [,  <custom-ident>  <content-level>]*  ]?
			"copy-into",
		]),
	],
	[
		"grid",
		new Set([
			// <'grid-template'> | <'grid-template-rows'> / [ auto-flow && dense? ] <'grid-auto-columns'>? | [ auto-flow && dense? ] <'grid-auto-rows'>? / <'grid-template-columns'>
			"grid",

			// <grid-line> [ / <grid-line> ]{0,3}
			"grid-area",

			// [ row | column ] || dense
			"grid-auto-flow",

			// <grid-line> [ / <grid-line> ]?
			"grid-column",

			// <grid-line> [ / <grid-line> ]?
			"grid-row",

			// none | [ <'grid-template-rows'> / <'grid-template-columns'> ] | [ <line-names>? <string> <track-size>? <line-names>? ]+ [ / <explicit-track-list> ]?
			"grid-template",

			// none | <track-list> | <auto-track-list> | subgrid <line-name-list>?
			"grid-template-columns",

			// none | <track-list> | <auto-track-list> | subgrid <line-name-list>?
			"grid-template-rows",

			// [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse
			"item-cross",

			// <'item-direction'> || <'item-wrap'> || <'item-pack'> || <'item-slack'>
			"item-flow",

			// normal | dense || balance
			"item-pack",

			// [ auto | nowrap | wrap ] || [ normal | reverse ] | wrap-reverse
			"item-wrap",
		]),
	],
	[
		"images",
		new Set([
			// from-image | none | [ <angle> || flip ]
			"image-orientation",

			// [ from-image || <resolution> ] && snap?
			"image-resolution",

			// fill | none | [contain | cover] || scale-down
			"object-fit",
		]),
	],
	[
		"inline",
		new Set([
			// normal | <number [1,∞]> <integer [1,∞]> | <number [1,∞]> && [ drop | raise ]?
			"initial-letter",

			// [ border-box? [ alphabetic | ideographic | hanging | leading ]? ]!
			"initial-letter-align",

			// normal | <'text-box-trim'> || <'text-box-edge'>
			"text-box",

			// [ first | last] || <'alignment-baseline'> || <'baseline-shift'>
			"vertical-align",
		]),
	],
	[
		"multicol",
		new Set([
			// <'column-width'> || <'column-count'> [ / <'column-height'> ]?
			"columns",
		]),
	],
	[
		"lists",
		new Set([
			// [ <counter-name> <integer>? ]+ | none
			"counter-increment",

			// [ <counter-name> <integer>? | <reversed-counter-name> <integer>? ]+ | none
			"counter-reset",

			// [ <counter-name> <integer>? ]+ | none
			"counter-set",

			// <'list-style-position'> || <'list-style-image'> || <'list-style-type'>
			"list-style",
		]),
	],
	[
		"overflow",
		new Set([
			// none | [<integer [1,∞]> || <'block-ellipsis'>] -webkit-legacy?
			"line-clamp",

			// auto | stable && both-edges?
			"scrollbar-gutter",

			// [ clip | ellipsis | <string> | fade | <fade()> ]{1,2}
			"text-overflow",
		]),
	],
	[
		"regions",
		new Set([
			// none | <custom-ident> [element | content]?
			"flow-into",
		]),
	],
	[
		"ruby",
		new Set([
			// [ alternate || [ over | under ] ] | inter-character
			"ruby-position",
		]),
	],
	[
		"scroll-snap",
		new Set([
			// none | [ x | y | block | inline | both ] [ mandatory | proximity ]?
			"scroll-snap-type",
		]),
	],
	[
		"shapes",
		new Set([
			// auto | outside-shape | [ <basic-shape> || shape-box ] | <image> | display
			"shape-inside",

			// none | [ <basic-shape> || <shape-box> ] | <image>
			"shape-outside",
		]),
	],
	[
		"sizing",
		new Set([
			// auto? [ none | <length [0,∞]> ]
			"contain-intrinsic-block-size",

			// auto? [ none | <length [0,∞]> ]
			"contain-intrinsic-height",

			// auto? [ none | <length [0,∞]> ]
			"contain-intrinsic-inline-size",

			// [ auto? [ none | <length> ] ]{1,2}
			"contain-intrinsic-size",

			// auto? [ none | <length [0,∞]> ]
			"contain-intrinsic-width",

			// legacy | zero-if-scroll || zero-if-extrinsic
			"min-intrinsic-sizing",
		]),
	],
	[
		"speech",
		new Set([
			// normal | spell-out || digits || [ literal-punctuation | no-punctuation ]
			"speak-as",

			// [[<family-name> | <generic-voice>],]* [<family-name> | <generic-voice>] | preserve
			"voice-family",

			// <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]]
			"voice-pitch",

			// <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]]
			"voice-range",

			// [normal | x-slow | slow | medium | fast | x-fast] || <percentage [0,∞]>
			"voice-rate",

			// silent | [[x-soft | soft | medium | loud | x-loud] || <decibel>]
			"voice-volume",
		]),
	],
	[
		"text",
		new Set([
			// none | [ first || [ force-end | allow-end ] || last ]
			"hanging-punctuation",

			// [ auto | <integer> ]{1,3}
			"hyphenate-limit-chars",

			// [ <length-percentage> ] && hanging? && each-line?
			"text-indent",

			// [ auto | none | inter-word | inter-character | ruby ] || no-compress
			"text-justify",

			// none | auto | <spacing-trim> || <autospace>
			"text-spacing",

			// none | [capitalize | uppercase | lowercase ] || full-width || full-size-kana | math-auto
			"text-transform",

			// normal | pre | pre-wrap | pre-line | <'white-space-collapse'> || <'text-wrap-mode'> || <'white-space-trim'>
			"white-space",

			// none | discard-before || discard-after || discard-inner
			"white-space-trim",

			// none | [ space | ideographic-space ] && auto-phrase?
			"word-space-transform",
		]),
	],
	[
		"text-decor",
		new Set([
			"text-decoration",

			// none | [ underline || overline || line-through || blink ] | spelling-error | grammar-error
			"text-decoration-line",

			// auto | skip-all | [ skip-underline || skip-overline || skip-line-through ] | no-skip
			"text-decoration-skip-self",

			// none | all | [ start || end ]
			"text-decoration-skip-spaces",

			// <'text-emphasis-style'> || <'text-emphasis-color'>
			"text-emphasis",

			// [ over | under ] && [ right | left ]?
			"text-emphasis-position",

			// none | [ [ filled | open ] || [ dot | circle | double-circle | triangle | sesame ] ] | <string>
			"text-emphasis-style",

			// auto | [ from-font | under ] || [ left | right ]
			"text-underline-position",
		]),
	],
	[
		"transforms",
		new Set([
			// none | <angle> | [ x | y | z | <number>{3} ] && <angle>
			"rotate",

			// [ left | center | right | top | bottom | <length-percentage> ] |   [ left | center | right | <length-percentage> ]  [ top | center | bottom | <length-percentage> ] <length>? |  [ [ center | left | right ] && [ center | top | bottom ] ] <length>?
			"transform-origin",

			// none | <length-percentage> [ <length-percentage> <length>? ]?
			"translate",
		]),
	],
	[
		"ui",
		new Set([
			// auto | <id> [ current | root | <target-name> ]?
			"nav-down",

			// auto | <id> [ current | root | <target-name> ]?
			"nav-left",

			// auto | <id> [ current | root | <target-name> ]?
			"nav-right",

			// auto | <id> [ current | root | <target-name> ]?
			"nav-up",
		]),
	],
	["variables", new Set(["--*"])],
]);

const ucfirst = (name: string) => name[0].toUpperCase() + name.slice(1);
const camel = (name: string) => name.replace(/([_-\s]\w)/g, (n) => n.slice(1).toUpperCase());
const pascal = (name: string) => ucfirst(camel(name));
const snake = (name: string) => name.replace(/([_-\s]\w)/g, (n) => `_${n.slice(1)}`).toLowerCase();

// Some properties should have lifetime annotations. It's a little tricky to detect which ones
// so it's easier just to hardcode these as a list...
const requiresAllocatorLifetime: Map<string, Set<string>> = new Map([
	["ui", new Set(["outline"])],
	["borders", new Set(["border-inline-color", "border-block-color"])],
	["conditional", new Set(["container-name"])],
	["view-transitions", new Set(["view-transition-class"])],
	["grid", new Set(["grid-template-areas", "grid-auto-columns", "grid-auto-rows"])],
]);

// Some properties should be enums but they have complex grammars that aren't worth attempting to
// parse so let's just hardcode a list...
const enumOverrides: Map<string, Set<string>> = new Map([]);
const structOverrides: Map<string, Set<string>> = new Map([
	["speech", new Set(["cue-before", "cue-after"])],
	["text-decor", new Set(["text-decoration-trim"])],
	["transforms", new Set(["scale"])],
]);

// Some properties can be awkward to type, and have rules which make them somewhat unique, so it's not worth
// spending the time to make the Parse generate call work for these, when a manual implementation would suffice.
// For these types we should simply not `derive(Parse)` and can hand write an impl.
const manualParseImpl = new Map([["writing-modes", new Set(["glyph-orientation-vertical"])]]);

// Some properties' values are defined across multiple specs, so we need to accomodate for that...
// parse so let's just hardcode a list...
const valueExtensions = new Map([
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	[
		"sizing",
		{
			width: " | stretch | fit-content | contain",
			"max-width": " | stretch | fit-content | contain",
			"min-width": " | stretch | fit-content | contain",
			height: " | stretch | fit-content | contain",
			"max-height": " | stretch | fit-content | contain",
			"min-height": " | stretch | fit-content | contain",
		},
	],
]);

// Ignore properties from some specs as they've moved around or are very rough
const ignore = new Map([
	// https://drafts.csswg.org/css-ui-4/#changes-22-12-2017
	// Moved the box-sizing and text-overflow properties to [CSS-SIZING-3] and [CSS-OVERFLOW-4] respectively.
	["ui", new Set(["box-sizing", "text-overflow"])],
	// CSS Shapes [CSS-SHAPES-2] define the shape-inside property that aligns contents along the edge of a possibly non-rectangular wrapping area.
	// (Round-Display just extends to add the `display` keyword which is specified in shapes-2 anyway)
	["round-display", new Set(["shape-inside"])],
	[
		"backgrounds",
		new Set([
			// https://drafts.csswg.org/css-backgrounds-4/#background-layers
			// The name of this property is discussed in issue https://github.com/w3c/csswg-drafts/issues/9083.
			"background-tbd",
			// https://drafts.csswg.org/css-borders-4/#intro
			//  This module is currently maintained as a diff against the parts related to borders and box
			//  decorations of CSS Backgrounds and Borders Module Level 3 [CSS3BG]. We will fold in the text
			//  once it’s all formatted up and in CR again, as this will reduce the effort of keeping them in
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
			"box-shadow",
		]),
	],
	// While GCPM is effectively "deprecated" (see https://github.com/w3c/csswg-drafts/issues/6435) the "string-set"
	// property definition inside of css-content is incomplete, as it is missing <content-list> (see
	// https://github.com/w3c/csswg-drafts/issues/1829). This is properly defined in GCPM. So we should exclude the
	// definition from css-content and use the more complete one from gcpm until both issues are properly addressed.
	["content", new Set(["string-set"])],
	[
		"logical",
		new Set([
			// https://drafts.csswg.org/css-logical-1/
			// These are extented definitions which are already defined in their respective specifications
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
	],
	// https://drafts.csswg.org/css-multicol/
	// Moved the column-rule-* properties to [CSS-GAP-1].
	["multicol", new Set(["column-rule", "column-rule-width", "column-rule-color", "column-rule-style"])],
	// https://drafts.csswg.org/css-flexbox-1/
	// Moved some align properties to [CSS-ALIGN-3].
	["flexbox", new Set(["align-content", "align-items", "align-self", "justify-content"])],
]);

const runtimeCache = new Map();
async function fetchCached(url: string, key: string) {
	let text = runtimeCache.get(key);
	if (!text) {
		try {
			text = await Deno.readTextFile(`./.caches/${key}`);
		} catch {
			console.log(`Fetching ${url}...`);
			const res = await fetch(url);
			text = await res.text();
		}
	}
	await Deno.writeTextFile(`./.caches/${key}`, text);
	if (key.endsWith(".json")) {
		return JSON.parse(text);
	}
	return text;
}

async function getIndex() {
	const json = await fetchCached("https://api.github.com/repos/w3c/csswg-drafts/git/trees/main", "index.json");
	return json.tree.reduce((acc: Record<string, number[]>, { path, type }) => {
		if (type == "tree" && path.startsWith("css-")) {
			let parts = path.split(/-/g).slice(1);
			let i = Number(parts.pop());
			const index = parts.join("-");
			acc[index] ||= [];
			acc[index].push(i);
		}
		return acc;
	}, {});
}

async function fetchSpec(name: string, ver: number) {
	return fetchCached(`https://drafts.csswg.org/css-${name}-${ver}/`, `${name}-${ver}.txt`);
}

async function getSpec(name: string, index: Record<string, number[]>, descriptions: Map<string, string>) {
	const types = new Map();
	const spec_descriptions = new Map();
	let url = "";
	let spec = "";
	let title = "";
	let ignored: Set<string> = new Set();
	for (const i of index[name]) {
		spec = `css-${name}-${i}`;
		url = `https://drafts.csswg.org/${spec}/`;
		const document = new DOMParser().parseFromString(await fetchSpec(name, i), "text/html");
		const propertyIndexHeader = document.querySelectorAll("#property-index");
		if (!propertyIndexHeader) {
			console.error(`${name}-${i} has no properties`);
			continue;
		}
		const index = document.querySelectorAll("#property-index + .big-element-wrapper table.index");
		if (index.length != 1) {
			console.error(`saw ${index.length} index tables in ${name}-${i}. Refusing to go further`);
			continue;
		}
		title = document.querySelector("h1")?.textContent || "";
		const propTables = [...document.querySelectorAll("table.propdef")]
			.flatMap((table) => {
				const newTable = Object.fromEntries(
					[...table.querySelectorAll("tr")].map((e) => [
						snake(e.querySelector("th").textContent.trim().slice(0, -1)),
						e.querySelector("td").textContent.trim(),
					]),
				);
				const names = newTable.name.split(/\s*,\s*/g);
				return names.map((name) => ({ ...newTable, name }));
			})
			.filter((e) => !e.new_values);
		for (const table of propTables) {
			if (ignore.get(name)?.has(table.name)) {
				ignored.add(table.name);
			} else {
				types.set(table.name, table);
			}
		}
	}

	for (const prop of ignored) {
		ignore.get(name)?.delete(prop);
	}

	const typeDefs = [...types.values()].map((table) => {
		const enums: Map = enumOverrides.get(name);
		const structs = structOverrides.get(name);
		const manualParse = manualParseImpl.get(name);
		const valueExts = valueExtensions.get(name);
		const justTopLevels = table.value
			.replace(/<[^>]+>/g, "")
			.replace(/\[[^\[\]]*\]/g, "")
			.trim();
		const isCombinedType =
			/^<(length|time|number|percentage)(?:[^\|]+) \| <(length|time|number|percentage)(?:[^\|]+)>$/.test(table.value);
		const isTypeOrAuto =
			/^(?:auto|none) \| <(?:[^\|]+)$|^<(?:[^\|]+)> \| (?:auto|none)$/.test(table.value) ||
			/^(?:auto|none) \| (?:auto|none) \| <(?:[^\|]+)$|^<(?:[^\|]+)> \| (?:auto|none) \| (?:auto|none)$/.test(
				table.value,
			);
		const hasTopLevelAlternative = /(?<!\|)\|(?!\|)/.test(justTopLevels) && !isCombinedType && !isTypeOrAuto;
		if (enums?.has(table.name) && structs?.has(table.name)) {
			throw new Error(
				`${table.name} was in both the enumOverrides table and the structOverrides table. It should not be in both.`,
			);
		}
		if (enums?.has(table.name) && hasTopLevelAlternative) {
			throw new Error(
				`${table.name} was inferred to be an enum from the grammar, but it is also in the enumOverrides table. It should be removed from that table to keep thigns clean.`,
			);
		}
		if (structs?.has(table.name) && !hasTopLevelAlternative) {
			throw new Error(
				`${table.name} was inferred to be an struct from the grammar, but it is also in the structOverrides table. It should be removed from that table to keep thigns clean.`,
			);
		}
		const dataType =
			(hasTopLevelAlternative || enums?.has(table.name)) && !structs?.has(table.name) ? "enum" : "struct";
		const trail = dataType == "enum" ? " {}" : ";";
		console.log("  //", table.value, "\n ", dataType, `${pascal(table.name)}${trail}`);
		let generics = "";
		const lifetimes = requiresAllocatorLifetime.get(name);
		const mustRequireLifetime =
			table.value.includes("<content-list>") ||
			table.value.includes("<counter-style>") ||
			table.value.includes("<dynamic-range-limit-mix()>") ||
			table.value.includes("<image-1D>") ||
			table.value.includes("<image>") ||
			table.value.includes("<line-width-list>") ||
			table.value.includes("<param()>") ||
			table.value.includes("<'column-rule-width'>") ||
			table.value.includes("<transform-list>") ||
			table.value.includes("]+") ||
			table.value.includes("]#") ||
			/#(:?$|[^\{])/.test(table.value);
		if (lifetimes?.has(table.name) && mustRequireLifetime) {
			throw new Error(
				`${table.name} was inferred to require lifetime, but it is also in the requiresAllocatorLifetime table. It should be removed from that set to keep thigns clean.`,
			);
		}
		if (lifetimes?.has(table.name) || mustRequireLifetime) {
			generics = "<'a>";
		}
		let l = "";
		if (todoPropertiesThatWillBeCommentedOut.get(name)?.has(table.name)) {
			todoPropertiesThatWillBeCommentedOut.get(name)?.delete(table.name);
			l = "// ";
		}

		const grammar = `${table.value.replace(/\n/g, " ")}${valueExts?.[table.name] || ""}`;

		let featureName = `css.properties.${table.name}`;
		let description = descriptions.get(featureName) || "";
		if (description) {
			description = `\n${l}/// ${description}\n${l}///`;
		}

		let value = `" ${grammar} "`;
		if (value.length > 110) {
			value = `\n${l}\t${value}\n${l}`;
		}

		let derives = [
			"Parse",
			"Peek",
			"ToSpan",
			"ToCursors",
			"StyleValue",
			"Visitable",
			"Debug",
			"Clone",
			"PartialEq",
			"Eq",
			"PartialOrd",
			"Ord",
			"Hash",
		];

		if (manualParse?.has(table.name)) {
			derives.shift();
		}

		return `
${l}/// Represents the style value for \`${table.name}\` as defined in [${spec}](${url}#${table.name == "--*" ? "defining-variables" : table.name}).
${l}///${description}
${l}/// The grammar is defined as:
${l}///
${l}/// \`\`\`text,ignore
${l}/// ${grammar}
${l}/// \`\`\`
${l}///
${l}// ${url}#${table.name == "--*" ? "defining-variables" : table.name}
${l}#[syntax(${value})]
${l}#[derive(${derives.join(", ")})]
${l}#[style_value(
${l}	initial = "${table.initial}",
${l}  applies_to = "${table.applies_to.replace(/\n/g, " ")}",
${l}	inherited = "${table.inherited.replace(/\n/g, " ").toLowerCase()}",
${l}	percentages = "${table.percentages.replace(/\n/g, " ").toLowerCase()}",
${l}	canonical_order = "${table.canonical_order.replace(/\n/g, " ").toLowerCase()}",
${l}	animation_type = "${table.animation_type?.replace(/\n/g, " ").toLowerCase() ?? "not animatable"}",
${l})]
${l}#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
${l}#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature("${featureName}"))]
${l}#[visit]
${l}pub ${dataType} ${table.name == "--*" ? "Custom" : pascal(table.name)}StyleValue${generics}${trail}`;
	});

	if (typeDefs.length == 0) return "";
	return `#![allow(warnings)]
//! ${title}
//! ${url}

mod impls;
use impls::*;
${typeDefs.join("\n")}
`;
}

(async (name) => {
	try {
		await Deno.mkdir(`.caches/`);
	} catch {}
	const webFeaturesData = await fetchCached(
		"https://github.com/web-platform-dx/web-features/releases/latest/download/data.extended.json",
		"web-features-data.extended.json",
	);

	if (name == "css-feature-data") {
		const getPopularity = async (propertyName: string) => {
			const popularity = await fetchCached("https://chromestatus.com/data/csspopularity", "popularity.json");
			return (popularity.find(({ property_name }) => propertyName == property_name)?.day_percentage ?? 0) * 100;
		};

		const allGroups = new Map();
		const allSpecs = new Map();

		// Helper function to escape strings for Rust
		const escapeRustString = (str: string) => {
			if (typeof str !== "string") return "";
			return str
				.replace(/\\/g, "\\\\")
				.replace(/"/g, '\\"')
				.replace(/\n/g, "\\n")
				.replace(/\r/g, "\\r")
				.replace(/\t/g, "\\t");
		};

		const dateStrToOptNaiveDate = (date: string) => {
			let parts = String(date || "").split("-");
			if (parts.length != 3) {
				return "None";
			}
			let [year, month, day] = parts;
			// basline sometimes has dates with ≤, e.g. 2019-01-01
			if (year.length > 4) {
				year = year.substring(year.length - 4);
			}
			return `NaiveDate::from_ymd_opt(${parseInt(year)},${parseInt(month)},${parseInt(day)})`;
		};

		const getBaselineStatus = (baseline: string | boolean, high_date: string, low_date: string) => {
			if (baseline === "high") {
				return `BaselineStatus::High { since: ${dateStrToOptNaiveDate(high_date)}.unwrap(), low_since: ${dateStrToOptNaiveDate(low_date)}.unwrap() }`;
			}
			if (baseline === "low") return `BaselineStatus::Low(${dateStrToOptNaiveDate(low_date)}.unwrap())`;
			if (baseline === false) return "BaselineStatus::False";
			return "BaselineStatus::Unknown";
		};

		const splitVersion = (version: string) =>
			version.includes(".") ? `${version.split(".").join(",")}` : `${version},0`;
		const browserVersion = (version: string) => {
			if (version && /^\d+(\.\d+)?$/.test(version)) {
				return `BrowserVersion(${splitVersion(version)})`;
			}
			return "BrowserVersion(0, 0)";
		};

		const getBrowserSupport = (status: any) => {
			const support = status?.support || {};
			return `BrowserSupport {
						chrome: ${browserVersion(support.chrome)},
						chrome_android: ${browserVersion(support.chrome_android)},
						edge: ${browserVersion(support.edge)},
						firefox: ${browserVersion(support.firefox)},
						firefox_android: ${browserVersion(support.firefox_android)},
						safari: ${browserVersion(support.safari)},
						safari_ios: ${browserVersion(support.safari_ios)},
				}`;
		};

		const dataFile: string[] = [];
		dataFile.push("//! Auto-generated CSS features data");
		dataFile.push("//! Generated on: " + new Date().toISOString());
		dataFile.push("");
		dataFile.push("use crate::*;");
		dataFile.push("use phf::{phf_map, Map};");
		dataFile.push("use chrono::NaiveDate;");
		dataFile.push("");
		dataFile.push("pub static CSS_FEATURES: Map<&'static str, CSSFeature> = phf_map! {");
		for (const [featureId, feature] of Object.entries(webFeaturesData.features) as any) {
			const compatFeatures = feature?.status?.by_compat_key ?? {};
			for (const [id, subFeature] of Object.entries(compatFeatures) as any) {
				if (id.startsWith("css.")) {
					const name = escapeRustString(feature.name || "");
					const description = escapeRustString(feature.description || "");
					const spec = `"${escapeRustString(feature.spec || "")}"`;
					if (feature.spec) {
						if (!allSpecs.has(feature.spec)) {
							allSpecs.set(feature.spec, new Set());
						}
						allSpecs.get(feature.spec).add(id);
					}
					let groups = (Array.isArray(feature.group) ? feature.group : [feature.group])
						.filter((g: string) => g && g != "css")
						.map((g) => `"${escapeRustString(g)}"`);
					const popularity = (await getPopularity(featureId)).toFixed(4) || "f32::NAN";
					if (feature.group) {
						for (const group of Array.isArray(feature.group) ? feature.group : [feature.group]) {
							if (!allGroups.has(group)) {
								allGroups.set(group, new Set());
							}
							allGroups.get(group).add(id);
						}
					}
					const baselineStatus = getBaselineStatus(
						subFeature.baseline || feature.status.baseline,
						subFeature.baseline_high_date || feature.status?.baseline_high_date,
						subFeature.baseline_low_date || feature.status?.baseline_low_date,
					);
					let caniuse = subFeature.caniuse || feature.caniuse;
					if (!Array.isArray(caniuse)) caniuse = [caniuse];
					caniuse = caniuse.filter(Boolean).map((key) => `"https://caniuse.com/${key}"`);
					const browserSupport = getBrowserSupport(subFeature.status || feature.status);
					dataFile.push(`	"${escapeRustString(id)}" => CSSFeature {`);
					dataFile.push(`		id: "${escapeRustString(id)}",`);
					dataFile.push(`		name: "${name}",`);
					dataFile.push(`		description: "${description}",`);
					dataFile.push(`		spec: ${spec},`);
					dataFile.push(`		groups: &[${groups.join(", ")}],`);
					dataFile.push(`		baseline_status: ${baselineStatus},`);
					dataFile.push(`		browser_support: ${browserSupport},`);
					dataFile.push(`		caniuse: &[${caniuse.join(", ")}],`);
					dataFile.push(`		popularity: ${popularity},`);
					dataFile.push("	},");
				}
			}
		}
		dataFile.push("};");
		dataFile.push("");
		dataFile.push("pub static GROUPS: Map<&'static str, &'static [&'static str]> = phf_map! {");
		for (const [group, members] of allGroups.entries()) {
			if (!escapeRustString(group)) continue;
			dataFile.push(`	"${escapeRustString(group)}" => &[`);
			for (const member of members) {
				dataFile.push(`		"${escapeRustString(member)}",`);
			}
			dataFile.push(`	],`);
		}
		dataFile.push("};");
		dataFile.push("");
		dataFile.push("pub static SPECS: Map<&'static str, &'static [&'static str]> = phf_map! {");
		for (const [spec, members] of allSpecs.entries()) {
			if (!escapeRustString(spec)) continue;
			dataFile.push(`	"${escapeRustString(spec)}" => &[`);
			for (const member of members) {
				dataFile.push(`		"${escapeRustString(member)}",`);
			}
			dataFile.push(`	],`);
		}
		dataFile.push("};");
		await Deno.writeTextFile("../../crates/css_feature_data/src/data.rs", dataFile.join("\n"));
		console.log("Generated ../../crates/css_feature_data/src/data.rs");
		return;
	}
	const index = await getIndex();
	if (!name) {
		throw new Error("Supply a working draft name");
	}
	if (!index[name]) {
		throw new Error(`Supplied name ${name} doesn't seem to be a valid working draft`);
	}
	const descriptions: Map<string, string> = new Map();
	for (const feature of Object.values(webFeaturesData.features) as any) {
		const compatFeatures = feature?.status?.by_compat_key ?? {};
		for (const [id, subFeature] of Object.entries(compatFeatures) as any) {
			if (id.startsWith("css.")) {
				descriptions.set(id, subFeature.description || feature.description);
			}
		}
	}
	const rs = await getSpec(name, index, descriptions);
	if (!rs) {
		try {
			await Deno.remove(`../../crates/css_ast/src/values/${snake(name)}/`, { recursive: true });
		} catch {}
	} else {
		await Deno.mkdir(`../../crates/css_ast/src/values/${snake(name)}/`, { recursive: true });
		await Deno.writeTextFile(`../../crates/css_ast/src/values/${snake(name)}/mod.rs`, rs);
		if (ignore.get(name)?.size > 0) {
			const props = [...ignore.get(name)].join(",");
			throw new Error(
				`Spec ${name} wanted to ignore out the following rules but they are not present in this spec: ${props}`,
			);
		}
		if (todoPropertiesThatWillBeCommentedOut.get(name)?.size > 0) {
			const props = [...todoPropertiesThatWillBeCommentedOut.get(name)].join(",");
			throw new Error(
				`Spec ${name} wanted to comment out the following rules but they are not present in this spec: ${props}`,
			);
		}
	}
})(...Deno.args);
