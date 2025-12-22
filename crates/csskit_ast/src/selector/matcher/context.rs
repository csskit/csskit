use css_ast::visit::{NodeId, QueryableNode};
use css_ast::{CssMetadata, PropertyKind};
use css_lexer::Span;
use css_parse::Cursor;

/// Stores queryable property values extracted from a node.
/// Uses direct field access for O(1) lookup (currently only `name` property exists).
#[derive(Clone, Copy, Default)]
pub(super) struct PropertyValues {
	/// The `name` property value (for declarations, named at-rules, functions).
	pub name: Option<Cursor>,
}

impl PropertyValues {
	#[inline]
	pub(super) fn from_node<T: QueryableNode>(node: &T) -> Self {
		Self { name: node.get_property(PropertyKind::Name) }
	}

	#[inline]
	pub(super) fn get(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => self.name,
			_ => None,
		}
	}

	#[inline]
	pub(super) fn from_declaration_name(name: Cursor) -> Self {
		Self { name: Some(name) }
	}
}

/// Context for matching declarations against selectors.
/// Stores metadata directly rather than copying individual fields.
#[derive(Clone, Default)]
pub(super) struct MatchContext<'a> {
	pub(super) metadata: Option<CssMetadata>,
	pub(super) is_important: bool,
	pub(super) is_custom_property: bool,
	pub(super) properties: PropertyValues,
	pub(super) source: &'a str,
	pub(super) sibling_index: i32,
}

#[derive(Clone)]
pub(super) struct SiblingInfo<'a> {
	pub(super) node_id: Option<NodeId>,
	pub(super) span: Span,
	pub(super) context: MatchContext<'a>,
}

#[derive(Clone)]
pub(super) struct ParentEntry<'a> {
	pub(super) node_id: Option<NodeId>,
	pub(super) span: Span,
	pub(super) context: MatchContext<'a>,
	pub(super) visited_children: Vec<SiblingInfo<'a>>,
}
