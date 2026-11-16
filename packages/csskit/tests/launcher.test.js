import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { chmodSync, copyFileSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";

const launcher = fileURLToPath(new URL("../bin/csskit", import.meta.url));

test(
	"launcher finds the platform binary beside the physical package of a symlinked install",
	{ skip: process.platform !== "linux" && process.platform !== "darwin" },
	(t) => {
		const root = mkdtempSync(join(tmpdir(), "csskit-launcher-"));
		t.after(() => rmSync(root, { recursive: true, force: true }));

		const store = join(root, "store", "csskit@0.0.0", "node_modules");
		mkdirSync(join(store, "csskit", "bin"), { recursive: true });
		copyFileSync(launcher, join(store, "csskit", "bin", "csskit"));

		const native = join(store, `csskit-${process.platform}-${process.arch}`, "bin", "csskit");
		mkdirSync(join(native, ".."), { recursive: true });
		writeFileSync(native, '#!/bin/sh\necho "native $*"\n');
		chmodSync(native, 0o755);

		const global = join(root, "global", "node_modules");
		mkdirSync(global, { recursive: true });
		symlinkSync(join(store, "csskit"), join(global, "csskit"), "dir");

		const out = execFileSync("bash", [join(global, "csskit", "bin", "csskit"), "--version"], { encoding: "utf8" });
		assert.equal(out, "native --version\n");
	},
);
