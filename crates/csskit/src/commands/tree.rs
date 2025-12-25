use std::io::Read;

use bumpalo::Bump;
use clap::Args;
use css_ast::visit::{QueryableNode, Visit, Visitable};
use css_ast::{CssAtomSet, PROPERTY_KIND_VARIANTS, PropertyKind, StyleSheet};
use css_lexer::Lexer;
use css_parse::{Cursor, Parser, ToSpan};
use csskit_ast::{QueryFunctionalPseudoClass, QueryPseudoClass};

use crate::{CliError, CliResult, GlobalConfig, InputArgs};

/// Display CSS AST as a tree structure
///
/// Shows the hierarchical structure of CSS with QuerySelector-like annotations.
/// Useful for understanding how to select nodes with csskit find.
///
/// Examples:
///   csskit tree style.css
///   csskit tree -c '.button { color: red; }'
#[derive(Debug, Args)]
pub struct Tree {
	#[command(flatten)]
	input: InputArgs,
}

#[derive(Clone, Debug)]
struct NodeInfo {
	/// Pre-formatted display string: `type[attrs]:pseudos`
	display: String,
	depth: usize,
	child_start_idx: usize,
}

struct CollectionVisitor<'a> {
	source: &'a str,
	nodes: Vec<NodeInfo>,
	depth: usize,
}

impl<'a> CollectionVisitor<'a> {
	fn new(source: &'a str) -> Self {
		Self { source, nodes: Vec::new(), depth: 0 }
	}

	fn build_display<T: QueryableNode>(&self, node: &T) -> String {
		let meta = node.self_metadata();
		let mut display = node.node_id().tag_name().to_string();

		// Attributes
		for &kind in PROPERTY_KIND_VARIANTS {
			if !meta.property_kinds.contains(kind) {
				continue;
			}
			if let Some(cursor) = node.get_property(kind) {
				let value = self.cursor_to_str(cursor);
				let attr_name = match kind {
					PropertyKind::Name => "name",
					_ => continue,
				};
				display.push_str(&format!("[{}={}]", attr_name, value));
			}
		}

		// Pseudos
		for name in QueryPseudoClass::matching_metadata_pseudos(&meta) {
			display.push(':');
			display.push_str(name);
		}
		if let Some(size) = QueryFunctionalPseudoClass::matching_size(&meta) {
			display.push_str(&format!(":size({})", size));
		}

		display
	}

	fn cursor_to_str(&self, cursor: Cursor) -> &str {
		let span = cursor.to_span();
		&self.source[span.start().0 as usize..span.end().0 as usize]
	}

	/// Find the index of the ancestor at the given depth, searching backwards from `start_idx`.
	fn find_ancestor_at_depth(&self, start_idx: usize, target_depth: usize) -> Option<usize> {
		(0..start_idx).rev().find(|&j| self.nodes[j].depth == target_depth)
	}

	/// Check if `node_idx` is the last direct child of its parent.
	fn is_last_child(&self, node_idx: usize) -> bool {
		let node_depth = self.nodes[node_idx].depth;
		if node_depth == 0 {
			return false;
		}

		let Some(parent_idx) = self.find_ancestor_at_depth(node_idx, node_depth - 1) else {
			return false;
		};

		let parent = &self.nodes[parent_idx];
		let parent_depth = parent.depth;

		// Find the last direct child of the parent
		let mut last_child_idx = None;
		for j in parent.child_start_idx..self.nodes.len() {
			if self.nodes[j].depth == parent_depth + 1 {
				last_child_idx = Some(j);
			} else if self.nodes[j].depth <= parent_depth {
				break;
			}
		}

		last_child_idx == Some(node_idx)
	}
}

impl<'a> Visit for CollectionVisitor<'a> {
	fn visit_queryable_node<T: QueryableNode>(&mut self, node: &T) {
		let display = self.build_display(node);
		let node_info = NodeInfo { display, depth: self.depth, child_start_idx: self.nodes.len() + 1 };
		self.nodes.push(node_info);
		self.depth += 1;
	}

	fn exit_queryable_node<T: QueryableNode>(&mut self, _node: &T) {
		self.depth -= 1;
	}
}

impl Tree {
	pub fn run(&self, _config: GlobalConfig) -> CliResult {
		let bump = Bump::default();

		for (filename, mut source) in self.input.sources()? {
			let mut src = String::new();
			source.read_to_string(&mut src)?;

			let lexer = Lexer::new(&CssAtomSet::ATOMS, &src);
			let mut parser = Parser::new(&bump, &src, lexer);
			let result = parser.parse_entirely::<StyleSheet>();

			let Some(stylesheet) = result.output.as_ref() else {
				for err in result.errors {
					eprintln!("{}", crate::commands::format_diagnostic_error(&err, &src, filename));
				}
				return Err(CliError::ParseFailed);
			};

			// Collect all nodes
			let mut visitor = CollectionVisitor::new(&src);
			stylesheet.accept(&mut visitor);

			// Print root
			if let Some(root) = visitor.nodes.first() {
				println!("{}", root.display);
			}

			// Print tree (skip the first node which is the root we just printed)
			for (i, node) in visitor.nodes.iter().enumerate().skip(1) {
				let prefix = self.build_prefix(&visitor, i, node.depth);
				println!("{}{}", prefix, node.display);
			}
		}

		Ok(())
	}

	/// Build the tree prefix (vertical bars and connectors) for a node.
	fn build_prefix(&self, visitor: &CollectionVisitor, node_idx: usize, node_depth: usize) -> String {
		let mut prefix = String::new();

		// For each ancestor level, determine if we need a vertical bar
		for d in 0..node_depth.saturating_sub(1) {
			let ancestor_idx = visitor.find_ancestor_at_depth(node_idx, d + 1);
			let show_bar = ancestor_idx.is_none_or(|idx| !visitor.is_last_child(idx));
			prefix.push_str(if show_bar { "│  " } else { "   " });
		}

		// Add the connector for this node
		let connector = if visitor.is_last_child(node_idx) { "╰─ " } else { "├─ " };
		prefix.push_str(connector);

		prefix
	}
}
