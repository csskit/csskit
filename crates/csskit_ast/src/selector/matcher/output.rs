use css_ast::NodeId;
use css_lexer::Span;
use indexmap::IndexSet;

/// A unique match identified by span and node type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatchOutput {
	/// The source span of the matched node.
	pub span: Span,
	/// The type of the matched node.
	pub node_id: NodeId,
}

pub type Matches = IndexSet<MatchOutput>;
