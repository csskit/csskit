use css_ast::visit::{NodeId, QueryableNode};
use css_ast::{CssMetadata, PropertyKind};
use css_lexer::Span;
use css_parse::Cursor;
use smallvec::SmallVec;

/// Stores queryable property values extracted from a node.
/// Uses a small inline array since most nodes have 0-1 queryable properties.
#[derive(Clone, Default)]
pub(super) struct PropertyValues(SmallVec<[(PropertyKind, Cursor); 1]>);

impl PropertyValues {
	pub(super) fn from_node<T: QueryableNode>(node: &T) -> Self {
		let mut values = SmallVec::new();
		for &kind in css_ast::PROPERTY_KIND_VARIANTS {
			if let Some(cursor) = node.get_property(kind) {
				values.push((kind, cursor));
			}
		}
		Self(values)
	}

	pub(super) fn get(&self, kind: PropertyKind) -> Option<Cursor> {
		self.0.iter().find(|(k, _)| *k == kind).map(|(_, c)| *c)
	}

	pub(super) fn from_declaration_name(name: Cursor) -> Self {
		Self(smallvec::smallvec![(PropertyKind::Name, name)])
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
pub(super) struct SiblingInfo {
	pub(super) node_id: Option<NodeId>,
	pub(super) span: Span,
}

#[derive(Clone)]
pub(super) struct ParentEntry<'a> {
	pub(super) node_id: Option<NodeId>,
	pub(super) span: Span,
	pub(super) context: MatchContext<'a>,
	pub(super) visited_children: Vec<SiblingInfo>,
}
