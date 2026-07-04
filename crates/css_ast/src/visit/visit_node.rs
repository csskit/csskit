use css_lexer::Span;
use css_parse::Cursor;

use crate::{CssMetadata, PropertyKind};

use super::NodeId;

pub(crate) trait QueryNodeData {
	/// Metadata for *this node only* (no subtree aggregation).
	fn self_metadata(&self) -> CssMetadata;
	/// Returns a cursor for the given property kind, if the node has that property.
	fn get_property(&self, kind: PropertyKind) -> Option<Cursor>;
}

/// The single node view passed to every [`Visit`](super::Visit) callback.
#[derive(Clone, Copy)]
pub struct VisitNode<'n> {
	pub span: Span,
	pub node_id: Option<NodeId>,
	subtree_metadata: CssMetadata,
	source: Option<&'n dyn QueryNodeData>,
}

impl<'n> VisitNode<'n> {
	/// Construct for a queryable node. For Nodes without `NodeId` or meta see [VisitNode::new_transparent].
	#[inline]
	pub(crate) fn new(
		span: Span,
		node_id: NodeId,
		subtree_metadata: CssMetadata,
		source: &'n dyn QueryNodeData,
	) -> Self {
		Self { span, node_id: Some(node_id), subtree_metadata, source: Some(source) }
	}

	/// Construct for a "transparent node" (no `NodeId`, no meta).
	#[inline]
	pub fn new_transparent(span: Span) -> Self {
		Self { span, node_id: None, subtree_metadata: CssMetadata::default(), source: None }
	}

	/// Aggregated metadata for this node *and its entire subtree*.
	#[inline]
	pub fn subtree_metadata(&self) -> CssMetadata {
		self.subtree_metadata
	}

	/// Metadata for *this node only* (no subtree aggregation).
	#[inline]
	pub fn self_metadata(&self) -> CssMetadata {
		self.source.map(QueryNodeData::self_metadata).unwrap_or_default()
	}

	/// Retrieve a named property from this node, if present.
	#[inline]
	pub fn property(&self, kind: PropertyKind) -> Option<Cursor> {
		self.source.and_then(|s| s.get_property(kind))
	}
}
