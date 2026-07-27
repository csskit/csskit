// Type definitions for the csskit object model (`csskit`).

import * as nodeClasses from './nodes.js';

/** Every node class, one per AST node kind; the same module as `csskit/nodes`. */
export declare const nodes: typeof nodeClasses;

/** A parse diagnostic. */
export interface Diagnostic {
	from: number;
	to: number;
	severity: string;
	code: string;
	message: string;
	help: string;
}

/**
 * A visitor callback. Return `'skip'` or `'skip-children'` to prune the subtree, `'stop'` to end the
 * traversal. All other values descend.
 */
export type VisitorCallback = (node: Node) => 'skip' | 'skip-children' | 'stop' | void | unknown;

/**
 * A node in a parsed AST.
 *
 * Each node kind has its own subclass, see `csskit/nodes`. You cannot construct a node from JS: a
 * node comes from `parse` or from a query. The root node owns the parse. All other nodes keep it
 * alive.
 */
export declare class Node {
	/** The kebab-case tag of this kind, for example `'style-rule'`. */
	static readonly tag: string;
	/** Parses `source` as this class. A kind with no standalone grammar throws. */
	static parse<T extends typeof Node>(this: T, source: string): InstanceType<T>;

	/** The class name of this node, for example `'StyleRule'`. */
	readonly kind: string;
	/** The kebab-case tag of this kind. */
	readonly tag: string;
	/** The kind index in the addon table. Stable within one process only. */
	readonly kindId: number;
	/** The byte offset of the start of this node. */
	readonly start: number;
	/** The byte offset of the end of this node. */
	readonly end: number;
	/** The full source text given to `parse`. */
	readonly source: string;
	/** The source text of this node. */
	readonly text: string;
	/** The diagnostics of the parse. Includes diagnostics from outside this subtree. */
	readonly diagnostics: Diagnostic[];

	/** True if this node matches `selector`. */
	matches(selector: string): boolean;
	/** Every descendant that matches `selector`, for example `'style-rule *[name=color]'`. */
	querySelectorAll(selector: string): Node[];
	/** The first descendant that matches `selector`, or `null`. */
	querySelector(selector: string): Node | null;
	/** Runs the Rust visitor over this subtree. Calls `enter` and `exit` for each node. */
	accept(enter?: VisitorCallback, exit?: VisitorCallback): void;
}

/**
 * Parses `source` as `context`, which defaults to `StyleSheet`.
 *
 * `context` is a node class, not a name: `parse(src, { context: Color })` and `Color.parse(src)` are
 * the same.
 */
export declare function parse<T extends Node>(source: string, options: { context: { parse(source: string): T } }): T;
export declare function parse(source: string, options?: { context?: undefined }): nodeClasses.StyleSheet;

export declare const Color: typeof nodeClasses.Color;
export declare const ComponentValues: typeof nodeClasses.ComponentValues;
export declare const SelectorList: typeof nodeClasses.SelectorList;
export declare const StyleRule: typeof nodeClasses.StyleRule;
export declare const StyleSheet: typeof nodeClasses.StyleSheet;
