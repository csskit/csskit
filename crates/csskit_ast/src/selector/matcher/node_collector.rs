use super::NodeData;
use css_ast::visit::{NodeId, QueryableNode, Visit};
use smallvec::SmallVec;
use std::collections::HashMap;

/// Sibling position data, computed lazily only when needed.
#[derive(Clone, Copy, Default)]
pub(crate) struct SiblingData {
	/// 1-based position among siblings.
	pub sibling_index: i32,
	/// Total number of siblings (including self).
	pub total_siblings: usize,
	/// Number of same-type siblings to the left.
	pub type_left: usize,
	/// Number of same-type siblings to the right.
	pub type_right: usize,
}

/// A node in the collected tree.
#[derive(Clone)]
pub(crate) struct TreeNode {
	pub data: NodeData,
	/// Index of parent node in the nodes array, None for root.
	pub parent_idx: Option<usize>,
	/// Sibling position data (computed lazily).
	pub sibling: SiblingData,
	/// Whether this node or any ancestor is a StyleRule.
	pub is_nested: bool,
	/// Indices of child nodes (only populated for nodes that could be ancestors).
	pub children: SmallVec<[usize; 4]>,
}

/// Collector visitor for building the node tree.
pub(crate) struct NodeCollector {
	/// All collected nodes.
	nodes: Vec<TreeNode>,
	/// Stack of (node_index, children_nested) for tracking parent during traversal.
	stack: Vec<(usize, bool)>,
}

impl NodeCollector {
	pub fn new() -> Self {
		Self { nodes: Vec::new(), stack: Vec::new() }
	}

	pub fn finalize(mut self, needs_type_tracking: bool) -> Vec<TreeNode> {
		let roots: SmallVec<[usize; 4]> =
			self.nodes.iter().enumerate().filter(|(_, n)| n.parent_idx.is_none()).map(|(i, _)| i).collect();

		self.compute_sibling_info(&roots, needs_type_tracking);

		for idx in 0..self.nodes.len() {
			let children = self.nodes[idx].children.clone();
			if !children.is_empty() {
				self.compute_sibling_info(&children, needs_type_tracking);
			}
		}

		self.nodes
	}

	fn compute_sibling_info(&mut self, children: &[usize], needs_type_tracking: bool) {
		let total = children.len();
		if total == 0 {
			return;
		}

		for (i, &child_idx) in children.iter().enumerate() {
			self.nodes[child_idx].sibling.sibling_index = (i + 1) as i32;
			self.nodes[child_idx].sibling.total_siblings = total;
		}

		if needs_type_tracking {
			let mut type_positions: HashMap<NodeId, Vec<usize>> = HashMap::new();

			// Two pass iteraiton to gather sibling counts
			for (i, &child_idx) in children.iter().enumerate() {
				let node_id = self.nodes[child_idx].data.node_id;
				type_positions.entry(node_id).or_default().push(i);
			}

			for (_node_id, positions) in type_positions {
				for (idx_in_type, &pos) in positions.iter().enumerate() {
					let child_idx = children[pos];
					self.nodes[child_idx].sibling.type_left = idx_in_type;
					self.nodes[child_idx].sibling.type_right = positions.len() - idx_in_type - 1;
				}
			}
		}
	}
}

impl Visit for NodeCollector {
	fn visit_queryable_node<T: QueryableNode>(&mut self, node: &T) {
		let node_id = node.node_id();

		let (parent_idx, parent_children_nested) =
			self.stack.last().map(|&(idx, nested)| (Some(idx), nested)).unwrap_or((None, false));

		let node_data = NodeData::from_node(node);
		let node_idx = self.nodes.len();

		let is_nested = parent_children_nested;

		self.nodes.push(TreeNode {
			data: node_data,
			parent_idx,
			sibling: SiblingData::default(),
			is_nested,
			children: SmallVec::new(),
		});

		if let Some(parent_idx) = parent_idx {
			self.nodes[parent_idx].children.push(node_idx);
		}

		let is_style_rule = node_id == NodeId::StyleRule;
		let children_nested = is_nested || is_style_rule;
		self.stack.push((node_idx, children_nested));
	}

	fn exit_queryable_node<T: QueryableNode>(&mut self, _: &T) {
		self.stack.pop();
	}
}
