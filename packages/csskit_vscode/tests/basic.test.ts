const assert = require("assert");

suite("Extension Test Suite", () => {
	test("Sample test", () => {
		assert.strictEqual([1, 2, 3].indexOf(5), -1);
		assert.strictEqual([1, 2, 3].indexOf(0), -1);
	});
});
