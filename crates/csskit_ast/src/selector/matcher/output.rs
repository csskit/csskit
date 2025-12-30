use super::PropertyValues;
use css_ast::NodeId;
use css_lexer::Span;
use indexmap::IndexSet;
use smallvec::SmallVec;

/// A unique match identified by span and node type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchOutput {
	/// The source span of the matched node.
	pub span: Span,
	/// The type of the matched node.
	pub node_id: NodeId,
	/// Property values for the matched node (used for diagnostic attr() function).
	pub properties: PropertyValues,
	/// Size of the matched node (number of children, declarations, selectors, etc.).
	pub size: u16,
	/// Snapshot of stat counter values at match time (for diagnostic interpolation).
	/// Only contains stats referenced in the diagnostic message. Empty during matching, populated during collection.
	pub stat_snapshot: SmallVec<[(u32, usize); 1]>,
}

pub type Matches = IndexSet<MatchOutput>;
