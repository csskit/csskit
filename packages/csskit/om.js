import addon from './addon.js';
import * as nodes from './nodes.js';

// Rust returns instances of one native class. Each node carries a `kindId`, an index into the table
// the addon reports at load. `adopt` uses that index to set the concrete class.
let byId = null;

function kindTable() {
	if (byId === null) {
		const byTag = new Map(Object.values(nodes).map((NodeClass) => [NodeClass.tag, NodeClass]));
		byId = addon.nodeKinds().map((tag) => {
			const NodeClass = byTag.get(tag);
			if (!NodeClass) {
				throw new Error(`csskit: no node class for '${tag}'; run \`mise run generate-node-classes\``);
			}
			return NodeClass;
		});
	}
	return byId;
}

function adopt(node) {
	return Object.setPrototypeOf(node, kindTable()[node.kindId].prototype);
}

/** The base class of every AST node class. */
export class Node extends addon.Node {
	/**
	 * Parses `source` as this class, as Rust's `parse_entirely::<T>()` does. A kind whose grammar
	 * exists only in context refuses to parse.
	 */
	static parse(source) {
		return Object.setPrototypeOf(addon.parse(source, this.tag), this.prototype);
	}

	/** The class name of this node, for example `'StyleRule'`. */
	get kind() {
		return this.constructor.name;
	}

	/** The kebab-case tag of this node, for example `'style-rule'`. */
	get tag() {
		return this.constructor.tag;
	}

	/** The first descendant that matches `selector`, or `null`. */
	querySelector(selector) {
		const found = super.querySelector(selector);
		return found === null ? null : adopt(found);
	}

	/** All descendants that match `selector`, for example `'style-rule *[name=color]'`. */
	querySelectorAll(selector) {
		const found = super.querySelectorAll(selector);
		for (let i = 0; i < found.length; i++) adopt(found[i]);
		return found;
	}

	/** Runs the Rust visitor over this subtree. Calls `enter` and `exit` for each node. */
	accept(enter, exit) {
		return super.accept(
			enter && ((node) => enter(adopt(node))),
			exit && ((node) => exit(adopt(node))),
		);
	}
}
