use crate::{Transformer, TransformerFeatures};
use css_parse::{NodeMetadata, NodeWithMetadata};

pub trait Transform<'a, 'ctx, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> {
	/// Returns true when a subtree described by `metadata` cannot contain anything this transform
	/// rewrites.
	///
	/// [`Transformer`] asks this of the root node before running the transform at all, and the
	/// transform asks it of every queryable node it enters, pruning the subtree when it holds.
	/// Metadata aggregates upwards, so a kind absent from `metadata` is absent from the whole
	/// subtree: answering `true` for a subtree that does contain such a node silently skips work.
	fn skips_subtree(metadata: &M) -> bool;

	fn new(transformer: &'ctx Transformer<'a, M, N, F>) -> Self;
}
