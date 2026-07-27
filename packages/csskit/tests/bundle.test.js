import assert from "node:assert/strict";
import { test } from "node:test";
import { format, lex, minify, parse, parseErrorReport } from "../bundle.js";

test("minify shrinks a stylesheet", () => {
	assert.equal(minify("a { color : red }"), "a{color:red}");
});

test("parse returns a serialised ast plus diagnostics", () => {
	const result = parse("a{color:red}");
	assert.ok(result.ast);
	assert.deepEqual(result.diagnostics, []);
});

test("lex, format and parseErrorReport are reachable", () => {
	assert.ok(lex("a{}"));
	assert.equal(typeof format("a{color:red}"), "string");
	assert.equal(typeof parseErrorReport("!!bad"), "string");
});
