use crate::{Transformer, TransformerFeatures};
use css_parse::{NodeMetadata, NodeWithMetadata};

pub trait Transform<'a, 'ctx, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> {
	fn may_change(features: F, node: &N) -> bool;

	fn new(transformer: &'ctx Transformer<'a, M, N, F>) -> Self;
}
