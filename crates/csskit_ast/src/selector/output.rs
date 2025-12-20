use css_ast::visit::NodeId;
use css_lexer::Span;

#[derive(Debug, Clone)]
pub struct MatchOutput {
	pub node_id: NodeId,
	pub span: Span,
}
