// The WebAssembly build of csskit: lex, minify, format and parse-error reports.
//
// This entry needs no native addon, thus it runs where wasm runs. It has no object model: `parse`
// gives a serialised AST and diagnostics. For the object model use the `csskit` entry.

import { parse_error_report } from './bundle/csskit_wasm_node.js';

export { format, lex, minify, parse } from './bundle/csskit_wasm_node.js';

/** Render a human-readable parse-error report for `source`. */
export function parseErrorReport(source) {
	return parse_error_report(source);
}
