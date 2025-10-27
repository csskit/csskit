/// Aggregated metadata for nodes, that can propagate up a node tree.
pub trait NodeMetadata: Sized + Copy + Default {
	/// Merges another NodeMetadata into this one.
	fn merge(&mut self, other: &Self);
}

/// A Node that has NodeMetadata
pub trait NodeWithMetadata<M: NodeMetadata> {
	/// Returns the metadata contributed by this node itself plus and child meta.
	/// Most nodes don't contribute metadata, so can simply return `child`.
	/// Other nodes may want to alter the metadata; supplying their own modifications
	/// to initial.
	fn self_metadata(&self, initial: M) -> M {
		initial
	}

	/// Returns the complete aggregated metadata for this node (self + children)
	fn metadata(&self) -> M;
}

// Stub implementation allowing tests to use () for M
impl NodeMetadata for () {
	fn merge(&mut self, _: &Self) {}
}
