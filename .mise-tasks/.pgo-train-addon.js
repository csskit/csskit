import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const [addonPath, ...files] = process.argv.slice(2);
const addon = { exports: {} };
process.dlopen(addon, resolve(addonPath));
const { minify, parse } = addon.exports;

const visit = () => {};
for (const file of files) {
	const source = readFileSync(file, "utf8");
	minify(source);
	parse(source, "style-sheet").accept(visit, visit);
}
