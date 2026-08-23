import assert from "node:assert/strict";
import { test } from "node:test";
import {
	Angle,
	Color,
	ComponentValues,
	CompoundSelector,
	FontFeatureValue,
	MediaRule,
	Node,
	ScopeRule,
	SelectorList,
	StyleRule,
	StyleSheet,
	StyleValue,
	WidthStyleValue,
	parse,
} from "../index.js";

test("parse returns the root as an instance of its own class", () => {
	const sheet = parse("a{color:red}");
	assert.ok(sheet instanceof StyleSheet);
	assert.ok(sheet instanceof Node);
	assert.equal(sheet.constructor.name, "StyleSheet");
	assert.equal(sheet.kind, "StyleSheet");
	assert.equal(sheet.tag, "style-sheet");
	assert.equal(sheet.diagnostics.length, 0);
	assert.equal(sheet.text, "a{color:red}");
});

test("a node class is chosen by class, not by name string", () => {
	assert.ok(parse("#ff0000", { context: Color }) instanceof Color);
	assert.ok(parse("a > b, .c", { context: SelectorList }) instanceof SelectorList);
	// The static form is the same thing.
	assert.equal(Color.parse("#ff0000").text, "#ff0000");
	assert.equal(SelectorList.parse("a > b").constructor, SelectorList);
});

test("every node kind has a class, including property values and at-rules", () => {
	assert.equal(WidthStyleValue.parse("100px").constructor, WidthStyleValue);
	assert.equal(MediaRule.parse("@media print{a{color:red}}").tag, "media-rule");
	assert.equal(ComponentValues.parse("1px solid red").text, "1px solid red");
	// Kinds no curated list ever named parse too.
	assert.equal(ScopeRule.parse("@scope (.a){b{color:red}}").tag, "scope-rule");
	assert.equal(Angle.parse("45deg").text, "45deg");
});

test("a node kind with no standalone grammar is rejected as a context", () => {
	// A style value only parses in the context of the property that owns it.
	assert.throws(() => parse("red", { context: StyleValue }), /style-value cannot be parsed/);
	assert.throws(() => FontFeatureValue.parse("1"), /font-feature-value cannot be parsed/);
	assert.throws(() => parse("a{}", { context: Node }));
});

test("diagnostics are reported with spans and messages", () => {
	const sheet = parse("!!bad");
	assert.ok(sheet.diagnostics.length >= 1);
	const d = sheet.diagnostics[0];
	assert.equal(typeof d.message, "string");
	assert.equal(typeof d.code, "string");
	assert.ok(d.to >= d.from);
});

test("querySelectorAll returns concrete node classes", () => {
	const sheet = parse("a{color:red}b{color:blue}");
	const rules = sheet.querySelectorAll("style-rule");
	assert.equal(rules.length, 2);
	assert.ok(rules[0] instanceof StyleRule);
	assert.equal(rules[0].text, "a{color:red}");
	const colors = sheet.querySelectorAll("color");
	assert.equal(colors.length, 2);
	assert.ok(colors.every((c) => c instanceof Color));
});

test("querySelector returns the first match or null", () => {
	const sheet = parse("a{color:red}b{}");
	assert.equal(sheet.querySelector("style-rule").text, "a{color:red}");
	assert.equal(sheet.querySelector("media-rule"), null);
});

test("attribute selectors match node properties", () => {
	const sheet = parse("a{color:red;width:2px}");
	const named = sheet.querySelectorAll("*[name=color]");
	assert.equal(named.length, 1);
	assert.equal(named[0].text, "color:red;");
});

test("node.matches respects tree position", () => {
	const rule = parse("a{color:red}").querySelector("style-rule");
	assert.equal(rule.matches("style-rule"), true);
	assert.equal(rule.matches("color"), false);
});

test("node.querySelectorAll is scoped to the subtree", () => {
	const sheet = parse("a{color:red}b{color:blue;width:2px}");
	const second = sheet.querySelectorAll("style-rule")[1];
	assert.equal(second.text, "b{color:blue;width:2px}");
	assert.equal(second.querySelectorAll("color").length, 1, "only the second rule");
	assert.equal(sheet.querySelectorAll("color").length, 2, "both rules from root");
});

test("scoped queries exclude same-span ancestors", () => {
	const rule = parse("a{color:red}").querySelector("style-rule");
	assert.equal(rule.querySelector("style-sheet"), null);
	assert.ok(!rule.querySelectorAll("*").some((node) => node instanceof StyleSheet));
});

test("a root does not match itself in its own query results", () => {
	const sheet = parse("a{color:red}");
	assert.equal(sheet.querySelectorAll("style-sheet").length, 0);
	assert.equal(sheet.matches("style-sheet"), true);
});

test("accept drives the visitor with real nodes", () => {
	const sheet = parse("a{color:red}");
	const entered = [];
	const exited = [];
	sheet.accept(
		(node) => {
			entered.push(node.constructor);
		},
		(node) => {
			exited.push(node.constructor);
		},
	);
	assert.ok(entered.includes(StyleSheet));
	assert.ok(entered.includes(StyleRule));
	assert.ok(entered.includes(Color));
	assert.equal(entered.length, exited.length, "every entered node is exited");
});

test("accept on a descendant starts at the scoped node", () => {
	const rule = parse("a{color:red}").querySelector("style-rule");
	const entered = [];
	rule.accept((node) => entered.push(node.constructor));
	assert.equal(entered[0], StyleRule);
	assert.ok(!entered.includes(StyleSheet));
});

test('visitor "skip" prunes a subtree, "stop" halts traversal', () => {
	const sheet = parse("a{color:red}");

	const withSkip = [];
	sheet.accept((node) => {
		withSkip.push(node.constructor);
		if (node instanceof SelectorList) return "skip";
	});
	assert.ok(withSkip.includes(SelectorList));
	assert.ok(!withSkip.includes(CompoundSelector), "subtree pruned by skip");

	const withStop = [];
	sheet.accept((node) => {
		withStop.push(node.constructor);
		if (node instanceof StyleRule) return "stop";
	});
	assert.deepEqual(withStop, [StyleSheet, StyleRule]);
});

test("a thrown error inside a visitor propagates", () => {
	const sheet = parse("a{color:red}");
	assert.throws(
		() =>
			sheet.accept(() => {
				throw new Error("boom");
			}),
		/boom/,
	);
});

test("invalid query selector throws", () => {
	assert.throws(() => parse("a{}").querySelectorAll("!!!nonsense!!!"));
});

test("nodes cannot be constructed from JS", () => {
	assert.throws(() => new StyleSheet());
});
