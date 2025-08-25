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
const requiresAllocatorLifetime = new Map([
	["anchor-position", new Set([])],
	["ui", new Set(["outline"])],
	["borders", new Set(["border-inline-color", "border-block-color"])],
	["conditional", new Set(["container-name"])],
	["view-transitions", new Set(["view-transition-class"])],
	["grid", new Set(["grid-template-areas", "grid-auto-columns", "grid-auto-rows"])],
]);

// Some properties should be enums but they have complex grammars that aren't worth attempting to
// parse so let's just hardcode a list...
const enumOverrides = new Map([]);
const structOverrides = new Map([]);

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
async function fetchCached(url, key) {
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

async function getPopularity(propertyName) {
	const popularity = await fetchCached("https://chromestatus.com/data/csspopularity", "popularity.json");
	return (popularity.find(({ property_name }) => propertyName == property_name)?.day_percentage ?? 0) * 100;
}

async function getIndex() {
	const json = await fetchCached("https://api.github.com/repos/w3c/csswg-drafts/git/trees/main", "index.json");
	return json.tree.reduce((acc: Record<string, number>, { path, type }) => {
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

async function getSpec(name: string, index: Record<string, number[]>) {
	const types = new Map();
	const compats = new Map();
	const metas = new Map();
	const descriptions = new Map();
	const popularities = new Map();
	let url = "";
	let spec = "";
	let title = "";
	let ignored = new Set();
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
		let elements = [];
		let previous = "";
		for (const el of document.querySelectorAll("main > *")) {
			if (el.tagName == "H3") {
				const description = elements;
				const html = description.map((e) => e.outerHTML).join("");
				descriptions.set(previous, html);
				elements = [];
			}
			let id = (el.id ?? "").replace(/-property$/, "");
			if (types.has(id)) {
				previous = id;
			}
			if (el.tagName != "H3" && el.tagName != "TABLE") {
				elements.push(el);
			}
		}
	}

	for (const prop of ignored) {
		ignore.get(name).delete(prop);
	}

	for (const { name } of types.values()) {
		compats.set(name, await fetchCached(`https://api.webstatus.dev/v1/features/${name}`, `${name}-compat.json`));
		popularities.set(name, await getPopularity(name));
		metas.set(
			name,
			await fetchCached(
				`https://api.webstatus.dev/v1/features/${name}/feature-metadata`,
				`${name}-feature-metadata.json`,
			),
		);
	}

	const typeDefs = [...types.values()].map((table) => {
		const enums = enumOverrides.get(name);
		const structs = structOverrides.get(name);
		const valueExts = valueExtensions.get(name);
		const compat = compats.get(table.name) ?? {};
		const meta = metas.get(table.name) ?? {};
		let caniuse = meta.can_i_use?.items?.[0]?.id;
		if (caniuse) {
			caniuse = `"https://caniuse.com/${caniuse}"`;
		}
		let baseline = compat.baseline?.status ?? "Unknown";
		let versions = [];
		if (compat.browser_implementations) {
			for (const [browser, { version }] of Object.entries(compat.browser_implementations)) {
				versions.push(`${browser}:${version}`);
			}
		}
		let popularity = popularities.get(name);
		popularity = popularity ? popularity.toFixed(3) : "Unknown";
		const justTopLevels = table.value
			.replace(/<[^>]+>/g, "")
			.replace(/\[[^\[\]]*\]/g, "")
			.trim();
		const isCombinedType =
			/^<(length|time|number|percentage)(?:[^\|]+) \| <(length|time|number|percentage)(?:[^\|]+)>$/.test(table.value);
		console.log(table.value, isCombinedType);
		const isTypeOrAuto = /^auto \| <(length|time|number)(?:[^\|]+)$|^<(length|time|number)(?:[^\|]+)> \| auto$/.test(
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
		let generics = "";
		const lifetimes = requiresAllocatorLifetime.get(name);
		const mustRequireLifetime =
			table.value.includes("<image>") ||
			table.value.includes("<counter-style>") ||
			table.value.includes("<content-list>") ||
			table.value.includes("<image-1D>") ||
			table.value.includes("<transform-list>") ||
			table.value.includes("<dynamic-range-limit-mix()>") ||
			table.value.includes("<param()>") ||
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

		let description = "";
		if (meta.description) {
			description = `\n${l}/// ${meta.description}`;
		}

		let applies_to = `"${table.applies_to.replace(/\n/g, " ")}"`;
		if (applies_to.length > 105) {
			applies_to = `\n${l}\t${applies_to}\n${l}`;
		}

		let value = `" ${grammar} "`;
		if (value.length > 110) {
			value = `\n${l}\t${value}\n${l}`;
		}

		/// ${JSON.stringify(compat)}
		///
		/// ${descriptions.get(table.name) ?? ""}
		///
		return `
${l}/// Represents the style value for \`${table.name}\` as defined in [${spec}](${url}#${table.name == "--*" ? "defining-variables" : table.name}).
${l}///${description}
${l}///
${l}/// The grammar is defined as:
${l}///
${l}/// \`\`\`text,ignore
${l}/// ${grammar}
${l}/// \`\`\`
${l}///
${l}// ${url}#${table.name == "--*" ? "defining-variables" : table.name}
${l}#[value(${value})]
${l}#[initial("${table.initial}")]
${l}#[applies_to(${applies_to})]
${l}#[inherited("${table.inherited.replace(/\n/g, " ").toLowerCase()}")]
${l}#[percentages("${table.percentages.replace(/\n/g, " ").toLowerCase()}")]
${l}#[canonical_order("${table.canonical_order.replace(/\n/g, " ").toLowerCase()}")]
${l}#[animation_type("${table.animation_type?.replace(/\n/g, " ").toLowerCase() ?? "not animatable"}")]
${l}#[popularity(${popularity})]
${l}#[caniuse(${caniuse ?? "Unknown"})]
${l}#[baseline(${baseline})]
${l}#[versions(${versions.join(",") || "Unknown"})]
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
	const index = await getIndex();
	if (!name) {
		throw new Error("Supply a working draft name");
	}
	if (!index[name]) {
		throw new Error(`Supplied name ${name} doesn't seem to be a valid working draft`);
	}
	const rs = await getSpec(name, index);
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
