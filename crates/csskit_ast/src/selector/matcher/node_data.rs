use super::PropertyValues;
use css_ast::CssMetadata;
use css_ast::visit::{NodeId, QueryableNode};
use css_lexer::Span;

/// Node-specific data captured during visit (independent of sibling position).
#[derive(Clone, Copy)]
pub(crate) struct NodeData {
	pub(crate) node_id: NodeId,
	pub(crate) span: Span,
	pub(crate) metadata: CssMetadata,
	pub(crate) properties: PropertyValues,
}

impl NodeData {
	#[inline]
	pub(crate) fn from_node<T: QueryableNode>(node: &T) -> Self {
		Self {
			node_id: node.node_id(),
			span: node.to_span(),
			metadata: node.self_metadata(),
			properties: PropertyValues::from_node(node),
		}
	}
}
