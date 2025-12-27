use super::{MatchContext, MatchOutput, Matcher, Matches, NodeCollector, NodeData, SelectorBuckets, TreeNode};
use crate::{
	QueryCombinator, QueryCompoundSelector, QueryFunctionalPseudoClass, QuerySelectorComponent, QuerySelectorList,
	SelectorRequirements, SelectorSegment,
};
use css_ast::visit::{NodeId, Visitable};
use css_ast::{CssMetadata, PropertyKind};
use css_parse::NodeWithMetadata;

/// Prefilter for :has() candidate nodes based on the first-matched segment's metadata.
/// Allows skipping nodes that definitely can't match before doing full matching.
struct HasPrefilter {
	/// If set, candidate must have this node type
	type_id: Option<NodeId>,
	/// If set, candidate must have these requirements
	requirements: SelectorRequirements,
	/// If set, candidate must have this attribute
	attribute_filter: PropertyKind,
}

impl HasPrefilter {
	/// Extract prefilter from a segment's parts.
	fn from_segment<'b>(segment: &SelectorSegment, all_parts: &[QuerySelectorComponent<'b>], source: &str) -> Self {
		let mut prefilter =
			Self { type_id: None, requirements: SelectorRequirements::none(), attribute_filter: PropertyKind::none() };
		for part in segment.parts(all_parts) {
			match part {
				QuerySelectorComponent::Type(t) => {
					if prefilter.type_id.is_none() {
						prefilter.type_id = t.node_id(source);
					}
				}
				QuerySelectorComponent::PseudoClass(p) => {
					let meta = <_ as NodeWithMetadata<crate::QuerySelectorMetadata>>::self_metadata(p);
					prefilter.requirements |= meta.self_requirements;
				}
				QuerySelectorComponent::Attribute(a) => {
					let meta = <_ as NodeWithMetadata<crate::QuerySelectorMetadata>>::self_metadata(a);
					prefilter.attribute_filter |= meta.self_attribute_filter;
				}
				QuerySelectorComponent::FunctionalPseudoClass(f) => {
					let meta = <_ as NodeWithMetadata<crate::QuerySelectorMetadata>>::self_metadata(f);
					prefilter.requirements |= meta.self_requirements;
				}
				_ => {}
			}
		}
		prefilter
	}

	/// Returns true if the node might match based on prefilter criteria.
	#[inline]
	fn might_match(&self, node: &NodeData) -> bool {
		// Type check
		if let Some(type_id) = self.type_id
			&& node.node_id != type_id
		{
			return false;
		}
		if !self.requirements.is_none() && !self.requirements.can_match(&node.metadata) {
			return false;
		}
		if !self.attribute_filter.is_none() && !node.metadata.property_kinds.contains(self.attribute_filter) {
			return false;
		}
		true
	}
}

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
			!m.is_invalid && (m.can_match(&css_meta) || m.requirements.contains(SelectorRequirements::Prefixed))
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
			return self.match_ancestors_iter(selector.segments(), selector.parts(), node_idx, nodes, None);
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

		self.match_ancestors_iter(selector.segments(), selector.parts(), node_idx, nodes, None)
	}

	/// Match ancestor segments. Segments are in forward order (leftmost first).
	/// We iterate from the second-to-last segment backwards (towards ancestors).
	/// `boundary` optionally restricts ancestor search to within a subtree (for :has()).
	fn match_ancestors_iter(
		&self,
		segments: &[SelectorSegment],
		parts: &[QuerySelectorComponent<'b>],
		start_idx: usize,
		nodes: &[TreeNode],
		boundary: Option<usize>,
	) -> bool {
		if segments.len() <= 1 {
			return true; // No ancestor segments
		}
		// Start from second-to-last, use combinator from the segment to the right
		self.match_ancestor_at(segments, parts, segments.len() - 2, start_idx, nodes, boundary)
	}

	/// Returns true if the index crosses the boundary (exclusive).
	/// Used for :has() to prevent ancestor matching from escaping the anchor's subtree.
	#[inline]
	fn crosses_boundary(idx: usize, boundary: Option<usize>) -> bool {
		boundary.is_some_and(|b| idx == b)
	}

	/// Match ancestor at a specific segment index. Uses combinator from segment[idx].
	/// `boundary` optionally restricts ancestor search (exclusive) - used for :has() inner matching.
	fn match_ancestor_at(
		&self,
		segments: &[SelectorSegment],
		parts: &[QuerySelectorComponent<'b>],
		idx: usize,
		start_idx: usize,
		nodes: &[TreeNode],
		boundary: Option<usize>,
	) -> bool {
		let segment = &segments[idx];
		let simple_parts = segment.parts(parts);
		let combinator = segment.combinator.as_ref();

		// Recursively match remaining ancestor segments (idx-1, idx-2, ..., 0).
		// Base case: idx == 0 means all ancestor segments matched.
		let match_remaining =
			|matched_idx| idx == 0 || self.match_ancestor_at(segments, parts, idx - 1, matched_idx, nodes, boundary);

		match combinator {
			Some(QueryCombinator::Child(_)) => {
				let Some(parent_idx) = nodes[start_idx].parent_idx else {
					return false;
				};
				if Self::crosses_boundary(parent_idx, boundary) {
					return false;
				}
				let parent = &nodes[parent_idx];
				let ctx = self.make_context(parent);
				if !self.matches_parts(simple_parts, &ctx, parent_idx, nodes) {
					return false;
				}
				match_remaining(parent_idx)
			}
			Some(QueryCombinator::Descendant(_)) | None => {
				let mut current_ancestor_idx = nodes[start_idx].parent_idx;
				while let Some(ancestor_idx) = current_ancestor_idx {
					if Self::crosses_boundary(ancestor_idx, boundary) {
						return false;
					}
					let ancestor = &nodes[ancestor_idx];
					let ctx = self.make_context(ancestor);
					if self.matches_parts(simple_parts, &ctx, ancestor_idx, nodes) && match_remaining(ancestor_idx) {
						return true;
					}
					current_ancestor_idx = ancestor.parent_idx;
				}
				false
			}
			Some(QueryCombinator::NextSibling(_)) => {
				let node = &nodes[start_idx];
				let Some(parent_idx) = node.parent_idx else {
					return false;
				};
				let parent = &nodes[parent_idx];
				// sibling_index is 1-based; convert to 0-based array position
				let current_sibling_pos = (node.sibling.sibling_index - 1) as usize;
				let Some(&prev_sibling_idx) =
					current_sibling_pos.checked_sub(1).and_then(|pos| parent.children.get(pos))
				else {
					return false;
				};
				let prev_sibling = &nodes[prev_sibling_idx];
				let ctx = self.make_context(prev_sibling);
				if !self.matches_parts(simple_parts, &ctx, prev_sibling_idx, nodes) {
					return false;
				}
				match_remaining(prev_sibling_idx)
			}
			Some(QueryCombinator::SubsequentSibling(_)) => {
				let node = &nodes[start_idx];
				let Some(parent_idx) = node.parent_idx else {
					return false;
				};
				let parent = &nodes[parent_idx];
				// sibling_index is 1-based; convert to 0-based array position
				let current_sibling_pos = (node.sibling.sibling_index - 1) as usize;
				for pos in (0..current_sibling_pos).rev() {
					let prev_sibling_idx = parent.children[pos];
					let prev_sibling = &nodes[prev_sibling_idx];
					let ctx = self.make_context(prev_sibling);
					if self.matches_parts(simple_parts, &ctx, prev_sibling_idx, nodes)
						&& match_remaining(prev_sibling_idx)
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
			match part {
				// :not() requires recursive selector matching with full tree context
				QuerySelectorComponent::FunctionalPseudoClass(QueryFunctionalPseudoClass::Not(not)) => {
					if self.matches_selector(&not.selector, node_idx, nodes) {
						return false;
					}
				}
				// :has() requires searching descendants/siblings
				QuerySelectorComponent::FunctionalPseudoClass(QueryFunctionalPseudoClass::Has(has)) => {
					if !self.matches_has(&has.selector, node_idx, nodes) {
						return false;
					}
				}
				_ => {
					if !part.matches(ctx) {
						return false;
					}
				}
			}
		}
		true
	}

	/// Check if any descendant/sibling matches the :has() inner selector.
	/// The inner selector can have a leading combinator (e.g., `:has(> child)`, `:has(+ sibling)`).
	fn matches_has(&self, inner: &QueryCompoundSelector<'b>, anchor_idx: usize, nodes: &[TreeNode]) -> bool {
		let parts = inner.parts();
		if parts.is_empty() {
			return false;
		}

		let segments = inner.segments();
		let segment_count = segments.len();
		if segment_count == 0 {
			return false;
		}

		let leading_combinator = inner.leading_combinator();
		let first_match_segment = if leading_combinator.is_some() { 0 } else { segment_count - 1 };
		let prefilter = HasPrefilter::from_segment(&segments[first_match_segment], parts, self.query_str);
		self.walk_relation(leading_combinator.as_ref(), anchor_idx, nodes, Some(&prefilter), &|candidate_idx| {
			self.matches_has_inner(inner, candidate_idx, anchor_idx, nodes)
		})
	}

	/// Check if a candidate node matches the :has() inner selector.
	/// Dispatches to forward or backward matching based on leading combinator presence.
	fn matches_has_inner(
		&self,
		inner: &QueryCompoundSelector<'b>,
		candidate_idx: usize,
		anchor_idx: usize,
		nodes: &[TreeNode],
	) -> bool {
		if inner.leading_combinator().is_some() {
			self.matches_has_forward(inner, candidate_idx, nodes)
		} else {
			self.matches_has_backward(inner, candidate_idx, anchor_idx, nodes)
		}
	}

	/// Match :has() with leading combinator (e.g., :has(> A > B)).
	/// Candidate matches FIRST segment, then we walk forward through remaining segments.
	fn matches_has_forward(&self, inner: &QueryCompoundSelector<'b>, candidate_idx: usize, nodes: &[TreeNode]) -> bool {
		let parts = inner.parts();
		let segments = inner.segments();
		let first_selector_parts = segments[0].parts(parts);
		let candidate = &nodes[candidate_idx];
		let ctx = self.make_context(candidate);

		if !self.matches_parts(first_selector_parts, &ctx, candidate_idx, nodes) {
			return false;
		}
		segments.len() <= 1 || self.walk_has_forward(inner, 1, candidate_idx, nodes)
	}

	/// Match :has() without leading combinator (e.g., :has(A B)).
	/// Candidate matches RIGHTMOST segment, then we match ancestors backward.
	/// `anchor_idx` bounds ancestor search to within the :has() anchor's subtree.
	fn matches_has_backward(
		&self,
		inner: &QueryCompoundSelector<'b>,
		candidate_idx: usize,
		anchor_idx: usize,
		nodes: &[TreeNode],
	) -> bool {
		let parts = inner.parts();
		let segments = inner.segments();
		let segment_count = segments.len();
		let rightmost_parts = segments[segment_count - 1].parts(parts);
		let candidate = &nodes[candidate_idx];
		let ctx = self.make_context(candidate);

		if !self.matches_parts(rightmost_parts, &ctx, candidate_idx, nodes) {
			return false;
		}
		segment_count <= 1 || self.match_ancestors_iter(segments, parts, candidate_idx, nodes, Some(anchor_idx))
	}

	/// Unified relation walker: iterates candidates based on combinator type and calls matcher.
	/// Optional prefilter allows early skipping of non-matching nodes.
	fn walk_relation<F>(
		&self,
		combinator: Option<&QueryCombinator>,
		from_idx: usize,
		nodes: &[TreeNode],
		prefilter: Option<&HasPrefilter>,
		matcher: &F,
	) -> bool
	where
		F: Fn(usize) -> bool,
	{
		let check = |candidate_idx: usize| {
			prefilter.is_none_or(|p| p.might_match(&nodes[candidate_idx].data)) && matcher(candidate_idx)
		};

		match combinator {
			Some(QueryCombinator::Child(_)) => {
				let node = &nodes[from_idx];
				node.children.iter().any(|&child_idx| check(child_idx))
			}
			Some(QueryCombinator::NextSibling(_)) => {
				let node = &nodes[from_idx];
				let Some(parent_idx) = node.parent_idx else { return false };
				let parent = &nodes[parent_idx];
				// sibling_index is 1-based, so it directly gives the next sibling's 0-based position
				let next_sibling_pos = node.sibling.sibling_index as usize;
				parent.children.get(next_sibling_pos).is_some_and(|&next_sibling_idx| check(next_sibling_idx))
			}
			Some(QueryCombinator::SubsequentSibling(_)) => {
				let node = &nodes[from_idx];
				let Some(parent_idx) = node.parent_idx else { return false };
				let parent = &nodes[parent_idx];
				// sibling_index is 1-based, so it directly gives the first subsequent sibling's 0-based position
				let next_sibling_pos = node.sibling.sibling_index as usize;
				parent.children.iter().skip(next_sibling_pos).any(|&sibling_idx| check(sibling_idx))
			}
			Some(QueryCombinator::Descendant(_)) | None => self.walk_descendants(from_idx, nodes, prefilter, matcher),
		}
	}

	/// Walk through descendants. Returns true on first match.
	fn walk_descendants<F>(
		&self,
		from_idx: usize,
		nodes: &[TreeNode],
		prefilter: Option<&HasPrefilter>,
		matcher: &F,
	) -> bool
	where
		F: Fn(usize) -> bool,
	{
		let mut queue: Vec<usize> = Vec::with_capacity(32);
		queue.push(from_idx);
		let mut head = 0;

		while head < queue.len() {
			let node = &nodes[queue[head]];
			head += 1;

			for &child_idx in &node.children {
				if prefilter.is_none_or(|p| p.might_match(&nodes[child_idx].data)) && matcher(child_idx) {
					return true;
				}
				queue.push(child_idx);
			}
		}
		false
	}

	/// Walk forward through :has() segments.
	/// For :has(> A > B), after matching A (at index 0), walks through B (at index 1).
	/// `segment_idx` is the next segment to match.
	fn walk_has_forward(
		&self,
		inner: &QueryCompoundSelector<'b>,
		segment_idx: usize,
		start_idx: usize,
		nodes: &[TreeNode],
	) -> bool {
		let segments = inner.segments();
		let parts = inner.parts();

		if segment_idx >= segments.len() {
			return true; // No more segments to match
		}

		let target_parts = segments[segment_idx].parts(parts);
		let combinator = segments[segment_idx - 1].combinator.as_ref();
		self.walk_relation(combinator, start_idx, nodes, None, &|candidate_idx| {
			let candidate = &nodes[candidate_idx];
			let ctx = self.make_context(candidate);
			self.matches_parts(target_parts, &ctx, candidate_idx, nodes)
				&& self.walk_has_forward(inner, segment_idx + 1, candidate_idx, nodes)
		})
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
