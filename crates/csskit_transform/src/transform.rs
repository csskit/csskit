use crate::{Transformer, TransformerFeatures};
use css_parse::{NodeMetadata, NodeWithMetadata};

/// A rewrite of a tree, run with the other enabled transforms in a single walk, in the order their features are listed
/// in the [`transformer!`](crate::transformer) invocation.
///
/// On each node a transform sees what earlier transforms wrote, and below a node it sees what every transform wrote on
/// entering that node, so a transform that reads the rewritten children of a node must do so in its `exit_*` hook.
pub trait Transform<'a, 'ctx, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> {
	/// Returns true when a subtree described by `metadata` cannot contain anything this transform
	/// rewrites.
	///
	/// [`Transformer`] asks this of the root node before running the transform at all, and the
	/// transform asks it of every stylesheet, style rule and at-rule it considers, through
	/// `VisitNode::rule_subtree_metadata`, pruning the subtree when it holds.
	/// Metadata aggregates upwards, so a kind absent from `metadata` is absent from the whole
	/// subtree: answering `true` for a subtree that does contain such a node silently skips work.
	fn skips_subtree(metadata: &M) -> bool;

	fn new(transformer: &'ctx Transformer<'a, M, N, F>) -> Self;
}
