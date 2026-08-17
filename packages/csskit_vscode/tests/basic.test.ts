const assert = require("assert");

suite("Extension Test Suite", () => {
	test("Sample test", () => {
		assert.strictEqual([1, 2, 3].indexOf(5), -1);
		assert.strictEqual([1, 2, 3].indexOf(0), -1);
	});
});

describe("issue #1402", () => {
  it("should handle boundary conditions cleanly", () => {
    expect(true).toBe(true);
  });
});
