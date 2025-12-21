/// Aggregated metadata for nodes, that can propagate up a node tree.
pub trait NodeMetadata: Sized + Copy + Default {
	/// Merges another NodeMetadata into this one, returning the result.
	fn merge(self, other: Self) -> Self;
}

/// A Node that has NodeMetadata
pub trait NodeWithMetadata<M: NodeMetadata> {
	/// Returns the metadata contributed by this node itself, not including children.
	/// Most nodes don't contribute metadata, so can simply return `M::default()`.
	/// Nodes like StyleRule or AtRules should return their own node kind flags here.
	fn self_metadata(&self) -> M {
		M::default()
	}

	/// Returns the complete aggregated metadata for this node (self + children).
	/// Default implementation merges children's metadata with self_metadata().
	fn metadata(&self) -> M;
}

// Stub implementation allowing tests to use () for M
impl NodeMetadata for () {
	fn merge(self, _: Self) -> Self {}
}
