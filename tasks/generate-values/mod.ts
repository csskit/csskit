import { DOMParser } from "jsr:@b-fuze/deno-dom";

// TODO!!!
// These properties have yet to be implemented because their parsing rules can be a little tricky.
// We should figure out how to parse them in ../../crates/csskit_proc_macro/src/def.rs then we can
// remove them from this Map and the generator will uncomment them!
const todoPropertiesThatWillBeCommentedOut = new Map([
	[
		"align",
		new Set([
			"justify-content",
			"justify-self",
			"place-self",
			"justify-items",
			"align-items",
			"place-content",
			"place-items",
		]),
	],
	["anchor-position", new Set(["position-visibility", "position-try-fallbacks", "position-try"])],
	[
		"animations",
		new Set([
			"animation",
			"animation-duration",
			"animation-name",
			"animation-trigger-exit-range",
			"animation-trigger-exit-range-end",
			"animation-trigger-exit-range-start",
			"animation-trigger-range",
			"animation-trigger-range-end",
			"animation-trigger-range-start",
		]),
	],
	[
		"backgrounds",
		new Set([
			"background",
			"background-image",
			"background-position",
			"background-position-block",
			"background-position-inline",
			"background-position-x",
			"background-position-y",
			"background-size",
			"border-image",
			"border-image-outset",
			"border-image-repeat",
			"border-image-slice",
			"border-image-width",
		]),
	],
	[
		"borders",
		new Set([
			"border-block-end-radius",
			"border-block-start-radius",
			"border-bottom-radius",
			"border-clip",
			"border-clip-bottom",
			"border-clip-left",
			"border-clip-right",
			"border-clip-top",
			"border-color",
			"border-inline-end-radius",
			"border-inline-start-radius",
			"border-left-radius",
			"border-limit",
			"border-radius",
			"border-right-radius",
			"border-shape",
			"border-top-radius",
			"box-shadow",
			"box-shadow-offset",
			"box-shadow-position",
		]),
	],
	["box", new Set(["margin-trim"])],
	["color-adjust", new Set(["color-scheme"])],
	["conditional", new Set(["container", "container-type"])],
	["contain", new Set(["contain"])],
	["content", new Set(["content", "quotes"])],
	["display", new Set(["display"])],
	["flexbox", new Set(["flex"])],
	[
		"fonts",
		new Set([
			"font",
			"font-family",
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
	],
	["gcpm", new Set(["string-set", "copy-into"])],
	[
		"grid",
		new Set([
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
	],
	["images", new Set(["image-orientation", "image-resolution", "object-fit"])],
	["inline", new Set(["initial-letter", "initial-letter-align", "text-box", "vertical-align"])],
	["multicol", new Set(["columns"])],
	["link-params", new Set(["link-parameters"])],
	["lists", new Set(["counter-increment", "counter-reset", "counter-set", "list-style"])],
	["overflow", new Set(["line-clamp", "scrollbar-gutter", "text-overflow"])],
	["overscroll", new Set(["overscroll-behavior"])],
	["regions", new Set(["flow-into"])],
	["ruby", new Set(["ruby-position"])],
	[
		"scroll-snap",
		new Set([
			"scroll-snap-align",
			"scroll-snap-type",
		]),
	],
	["shapes", new Set(["shape-inside", "shape-outside"])],
	[
		"sizing",
		new Set([
			"contain-intrinsic-block-size",
			"contain-intrinsic-height",
			"contain-intrinsic-inline-size",
			"contain-intrinsic-size",
			"contain-intrinsic-width",
			"min-intrinsic-sizing",
		]),
	],
	["speech", new Set(["speak-as", "voice-family", "voice-pitch", "voice-range", "voice-rate", "voice-volume"])],
	[
		"text",
		new Set([
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
	],
	[
		"text-decor",
		new Set([
			"text-decoration",
			"text-decoration-line",
			"text-decoration-skip-self",
			"text-decoration-skip-spaces",
			"text-emphasis",
			"text-emphasis-position",
			"text-emphasis-style",
			"text-shadow",
			"text-underline-position",
		]),
	],
	["transforms", new Set(["rotate", "scale", "transform-box", "transform-origin", "translate"])],
	["ui", new Set(["nav-down", "nav-left", "nav-right", "nav-up", "outline"])],
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
	["color-hdr", new Set(["dynamic-range-limit"])],
	["ui", new Set(["outline"])],
	["borders", new Set(["border-inline-color", "border-block-color"])],
	["conditional", new Set(["container-name"])],
	["view-transitions", new Set(["view-transition-class"])],
	["grid", new Set(["grid-template-areas", "grid-auto-columns", "grid-auto-rows"])],
]);

// Some properties should be enums but they have complex grammars that aren't worth attempting to
// parse so let's just hardcode a list...
const enumOverrides = new Map([["animation", new Set(["animation-name"])]]);
const structOverrides = new Map([
	["box", new Set(["margin-top", "margin-right", "margin-bottom", "margin-left"])],
	["multicol", new Set(["column-width", "column-height"])],
	[
		"position",
		new Set([
			"top",
			"right",
			"bottom",
			"left",
			"inset-block-start",
			"inset-inline-start",
			"inset-block-end",
			"inset-inline-end",
		]),
	],
	[
		"scroll-snap",
		new Set([
			"scroll-padding",
			"scroll-padding-block",
			"scroll-padding-block-end",
			"scroll-padding-block-start",
			"scroll-padding-bottom",
			"scroll-padding-inline",
			"scroll-padding-inline-end",
			"scroll-padding-inline-start",
			"scroll-padding-left",
			"scroll-padding-right",
			"scroll-padding-top",
		]),
	],
	["text-decor", new Set(["text-underline-offset"])],
]);

// Some properties' values are defined across multiple specs, so we need to accomodate for that...
// parse so let's just hardcode a list...
const valueExtensions = new Map([
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	[
		"sizing",
		{
			width: " | stretch | fit-content",
			"max-width": " | stretch | fit-content",
			"min-width": " | stretch | fit-content",
			height: " | stretch | fit-content",
			"max-height": " | stretch | fit-content",
			"min-height": " | stretch | fit-content",
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
		let dataType = "struct";
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
		let popularity = popularities.get(name) || "Unknown";
		const mustBeEnum = /[^\|]\|[^\|]/.test(table.value.replace(/(?:\[[^\]]+\])g/, "").replace(/(?:<[^>]+>)g/, ""));
		if (enums?.has(table.name) && mustBeEnum) {
			throw new Error(
				`${table.name} was inferred to be an enum from the grammar, but it is also in the enumOverrides table. It should be removed from that table to keep thigns clean.`,
			);
		}
		if (enums?.has(table.name) || mustBeEnum) {
			dataType = "enum";
		}
		if (structs?.has(table.name)) {
			dataType = "struct";
		}
		let trail = dataType == "enum" ? " {}" : ";";
		let generics = "";
		const lifetimes = requiresAllocatorLifetime.get(name);
		const mustRequireLifetime =
			table.value.includes("<image>") ||
			table.value.includes("<counter-style>") ||
			table.value.includes("<content-list>") ||
			table.value.includes("<image-1D>") ||
			table.value.includes("<transform-list>") ||
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
