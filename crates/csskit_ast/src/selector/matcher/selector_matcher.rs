use super::{MatchContext, MatchOutput, Matcher, Matches, NodeCollector, SelectorBuckets, TreeNode};
use crate::{
	QueryCombinator, QueryCompoundSelector, QueryFunctionalPseudoClass, QuerySelectorComponent, QuerySelectorList,
	SelectorRequirements, SelectorSegment,
};
use css_ast::CssMetadata;
use css_ast::visit::Visitable;
use css_parse::NodeWithMetadata;

/// Matches CSS-like selectors against a tree of nodes.
///
/// The matcher operates in three phases:
/// 1. Bucket selectors by their rightmost component (type, attribute, pseudo-class)
/// 2. Collect all nodes into a tree structure with parent/child/sibling relationships
/// 3. Match each node against relevant selectors using right-to-left traversal
pub struct SelectorMatcher<'a, 'b> {
	query_str: &'b str,
	selectors: Vec<&'a QueryCompoundSelector<'b>>,
	source: &'a str,
	matches: Matches,
	needs_type_tracking: bool,
}

impl<'a, 'b> SelectorMatcher<'a, 'b> {
	pub fn new(selectors: &'a QuerySelectorList<'b>, query_str: &'b str, source: &'a str) -> Self {
		Self {
			query_str,
			selectors: selectors.selectors().collect(),
			source,
			matches: Matches::default(),
			needs_type_tracking: false,
		}
	}

	pub fn run<T: Visitable + NodeWithMetadata<CssMetadata>>(mut self, root: &T) -> impl Iterator<Item = MatchOutput> {
		let css_meta = root.metadata();
		self.selectors.retain(|s| {
			let m = s.metadata();
			m.can_match(&css_meta) || m.requirements.contains(SelectorRequirements::Prefixed)
		});
		if self.selectors.is_empty() {
			return self.matches.into_iter();
		}

		// Check if any selector needs type-based sibling tracking
		self.needs_type_tracking = self.selectors.iter().any(|s| {
			let m = s.metadata();
			m.deferred && m.needs_type_tracking
		});

		let buckets = SelectorBuckets::new(&self.selectors);
		let nodes = self.collect_nodes(root);
		for (idx, node) in nodes.iter().enumerate() {
			for selector in buckets.selectors_for_node(&node.data) {
				if self.matches_selector(selector, idx, &nodes) {
					self.matches.insert(MatchOutput { span: node.data.span, node_id: node.data.node_id });
				}
			}
		}

		self.matches.into_iter()
	}

	fn collect_nodes<T: Visitable + NodeWithMetadata<CssMetadata>>(&self, root: &T) -> Vec<TreeNode> {
		let mut collector = NodeCollector::new();
		root.accept(&mut collector);
		collector.finalize(self.needs_type_tracking)
	}

	fn matches_selector(&self, selector: &QueryCompoundSelector<'b>, node_idx: usize, nodes: &[TreeNode]) -> bool {
		let node = &nodes[node_idx];
		let meta = selector.metadata();

		// If the type is a _definite_ mismatch or a definite singular match we can return early.
		if meta.rejects_type(node.data.node_id) {
			return false;
		}
		if meta.is_type_only {
			return self.match_ancestors(selector.ancestor_segments(), selector.parts(), node_idx, nodes);
		}

		let ctx = self.make_context(node);

		if !meta.property_groups.is_none() && !ctx.node.metadata.property_groups.contains(meta.property_groups) {
			return false;
		}

		if !meta.vendor_filter.is_none()
			&& !ctx.node.metadata.vendor_prefixes.contains(meta.vendor_filter)
			&& ctx.node.properties.name.is_none()
		{
			return false;
		}

		if !self.matches_parts(selector.rightmost(), &ctx, node_idx, nodes) {
			return false;
		}

		self.match_ancestors(selector.ancestor_segments(), selector.parts(), node_idx, nodes)
	}

	fn match_ancestors(
		&self,
		segments: &[SelectorSegment],
		parts: &[QuerySelectorComponent<'b>],
		start_idx: usize,
		nodes: &[TreeNode],
	) -> bool {
		if segments.is_empty() {
			return true;
		}

		let segment = &segments[0];
		let remaining = &segments[1..];
		let simple_parts = segment.parts(parts);

		match segment.combinator.as_ref() {
			Some(QueryCombinator::Child(_)) => {
				let Some(parent_idx) = nodes[start_idx].parent_idx else {
					return false;
				};
				let parent = &nodes[parent_idx];
				let ctx = self.make_context(parent);
				if !self.matches_parts(simple_parts, &ctx, parent_idx, nodes) {
					return false;
				}
				self.match_ancestors(remaining, parts, parent_idx, nodes)
			}
			Some(QueryCombinator::Descendant(_)) | None => {
				// Try each matching ancestor; continue searching if remaining segments fail
				let mut ancestor_idx = nodes[start_idx].parent_idx;
				while let Some(idx) = ancestor_idx {
					let ancestor = &nodes[idx];
					let ctx = self.make_context(ancestor);
					if self.matches_parts(simple_parts, &ctx, idx, nodes)
						&& self.match_ancestors(remaining, parts, idx, nodes)
					{
						return true;
					}
					ancestor_idx = ancestor.parent_idx;
				}
				false
			}
			Some(QueryCombinator::NextSibling(_)) => {
				// Must match immediate previous sibling
				let Some(parent_idx) = nodes[start_idx].parent_idx else {
					return false;
				};
				let parent = &nodes[parent_idx];
				// sibling_index is 1-based, so convert to 0-based for array access
				let current_pos = (nodes[start_idx].sibling.sibling_index - 1) as usize;
				let Some(&prev_idx) = current_pos.checked_sub(1).and_then(|pos| parent.children.get(pos)) else {
					return false; // No previous sibling
				};
				let prev_sibling = &nodes[prev_idx];
				let ctx = self.make_context(prev_sibling);
				if !self.matches_parts(simple_parts, &ctx, prev_idx, nodes) {
					return false;
				}
				self.match_ancestors(remaining, parts, prev_idx, nodes)
			}
			Some(QueryCombinator::SubsequentSibling(_)) => {
				// Try each matching previous sibling; continue searching if remaining segments fail
				let Some(parent_idx) = nodes[start_idx].parent_idx else {
					return false;
				};
				let parent = &nodes[parent_idx];
				let current_pos = (nodes[start_idx].sibling.sibling_index - 1) as usize;
				for i in (0..current_pos).rev() {
					let sibling_idx = parent.children[i];
					let sibling = &nodes[sibling_idx];
					let ctx = self.make_context(sibling);
					if self.matches_parts(simple_parts, &ctx, sibling_idx, nodes)
						&& self.match_ancestors(remaining, parts, sibling_idx, nodes)
					{
						return true;
					}
				}
				false
			}
		}
	}

	fn matches_parts(
		&self,
		parts: &[QuerySelectorComponent<'b>],
		ctx: &MatchContext<'a, 'b>,
		node_idx: usize,
		nodes: &[TreeNode],
	) -> bool {
		for part in parts {
			// :not() requires recursive selector matching with full tree context
			if let QuerySelectorComponent::FunctionalPseudoClass(QueryFunctionalPseudoClass::Not(not)) = part {
				if self.matches_selector(&not.selector, node_idx, nodes) {
					return false;
				}
			} else if !part.matches(ctx) {
				return false;
			}
		}
		true
	}

	#[inline]
	fn make_context(&self, node: &TreeNode) -> MatchContext<'a, 'b> {
		MatchContext {
			node: node.data,
			sibling_index: node.sibling.sibling_index,
			type_left: node.sibling.type_left,
			total: node.sibling.total_siblings,
			type_right: node.sibling.type_right,
			is_root: node.parent_idx.is_none(),
			is_nested: node.is_nested,
			source: self.source,
			query_str: self.query_str,
		}
	}
}
