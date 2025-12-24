use super::NodeData;

/// Context for matching nodes/declarations against selectors.
#[derive(Clone, Copy)]
pub(crate) struct MatchContext<'a, 'b> {
	pub(crate) node: NodeData,
	/// 1-based sibling index (for :first-child, :nth-child matching).
	pub(crate) sibling_index: i32,
	/// Number of same-type siblings to the left (for :nth-of-type, :first-of-type).
	pub(crate) type_left: usize,
	/// Total number of siblings (for :last-child, :only-child, :nth-last-child).
	pub(crate) total: usize,
	/// Number of same-type siblings to the right (for :last-of-type, :only-of-type, :nth-last-of-type).
	pub(crate) type_right: usize,
	/// Whether this is the root node (no parents).
	pub(crate) is_root: bool,
	/// Whether this node is nested inside a StyleRule.
	pub(crate) is_nested: bool,
	/// Source string for the CSS being matched.
	pub(crate) source: &'a str,
	/// Source string for the query selector.
	pub(crate) query_str: &'b str,
}

impl<'a, 'b> MatchContext<'a, 'b> {
	#[inline]
	pub(crate) fn is_only_child(&self) -> bool {
		self.total == 1
	}

	#[inline]
	pub(crate) fn is_last_child(&self) -> bool {
		self.sibling_index as usize == self.total
	}

	#[inline]
	pub(crate) fn is_first_of_type(&self) -> bool {
		self.type_left == 0
	}

	#[inline]
	pub(crate) fn is_last_of_type(&self) -> bool {
		self.type_right == 0
	}

	#[inline]
	pub(crate) fn is_only_of_type(&self) -> bool {
		self.type_left == 0 && self.type_right == 0
	}

	/// Returns the 1-based index from the end (for :nth-last-child).
	#[inline]
	pub(crate) fn index_from_end(&self) -> i32 {
		(self.total - self.sibling_index as usize + 1) as i32
	}

	/// Returns the 1-based type index from the start (for :nth-of-type).
	#[inline]
	pub(crate) fn type_index(&self) -> i32 {
		(self.type_left + 1) as i32
	}

	/// Returns the 1-based type index from the end (for :nth-last-of-type).
	#[inline]
	pub(crate) fn type_index_from_end(&self) -> i32 {
		(self.type_right + 1) as i32
	}
}
