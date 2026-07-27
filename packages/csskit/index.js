export * from "./nodes.js";
export * from "./om.js";
import { StyleSheet } from "./nodes.js";
/**
 * Parses `source` as `context`, which defaults to `StyleSheet`.
 *
 * `context` is a node class, not a name: `parse(src, { context: Color })` and `Color.parse(src)` are
 * the same.
 */
export function parse(source, options) {
	return (options?.context ?? StyleSheet).parse(source);
}
