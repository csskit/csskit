use super::{NodeId, NodeVisitor, Visitable};
use crate::*;
use css_lexer::Lexer;
use css_parse::{Arena, Box, Diagnostic, NodeWithMetadata, Parse, Parser, ToCursors, ToSpan, Vec};
use std::marker::PhantomData;
use visit_flow::VisitFlow;

include!(concat!(env!("OUT_DIR"), "/css_root_dispatch.rs"));

/// A node whose type is erased.
pub trait ErasedNode: ToSpan + NodeWithMetadata<CssMetadata> {
	fn accept_dyn(&self, visitor: &mut dyn NodeVisitor) -> VisitFlow;
}

impl<T: ToSpan + Visitable + NodeWithMetadata<CssMetadata>> ErasedNode for T {
	fn accept_dyn(&self, mut visitor: &mut dyn NodeVisitor) -> VisitFlow {
		Visitable::accept(self, &mut visitor)
	}
}

/// What [`parse_root`] gives back.
pub struct ParsedRoot<'a> {
	/// The root node, or `None` if the source does not parse as the node kind.
	pub root: Option<&'a dyn ErasedNode>,
	pub diagnostics: Vec<'a, Diagnostic>,
}

struct ErasedNodeParser<'a, T>(PhantomData<&'a T>);

impl<'a, T> ErasedNodeParser<'a, T>
where
	T: Parse<'a> + ToCursors + ToSpan + Visitable + NodeWithMetadata<CssMetadata> + 'a,
{
	fn parse(&self, arena: &'a Arena, source: &'a str) -> Option<ParsedRoot<'a>> {
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source);
		let result = Parser::new(arena, source, lexer).parse_entirely::<T>();
		let root = result.output.map(|value| &*Box::new_in(arena, value).leak() as &'a dyn ErasedNode);
		Some(ParsedRoot { root, diagnostics: result.errors })
	}
}

trait ParseFallback<'a> {
	fn parse(&self, _: &'a Arena, _: &'a str) -> Option<ParsedRoot<'a>> {
		None
	}
}

impl<'a, T> ParseFallback<'a> for ErasedNodeParser<'a, T> {}

#[cfg(test)]
mod tests {
	use super::{NodeId, NodeVisitor, VisitNode, parse_root};
	use css_parse::Arena;
	use visit_flow::{VisitFlow, VisitFlowExt};

	#[test]
	fn parseability_follows_the_parse_impl_not_a_list() {
		let arena = Arena::default();
		for tag in ["style-value", "font-face-rule-style-value", "font-feature-value"] {
			let id = NodeId::from_tag_name(tag).unwrap_or_else(|| panic!("{tag} is a node kind"));
			assert!(parse_root(id, &arena, "").is_none(), "{tag} has no standalone grammar");
		}
		for tag in ["style-sheet", "length", "angle", "scope-rule", "bg-layer"] {
			let id = NodeId::from_tag_name(tag).unwrap_or_else(|| panic!("{tag} is a node kind"));
			assert!(parse_root(id, &arena, "").is_some(), "{tag} parses on its own");
		}
		// Math functions are generic over their leaf type. You cannot name them as one root.
		assert!(parse_root(NodeId::CalcFunction, &arena, "").is_none());
	}

	#[test]
	fn parse_root_gives_an_erased_root_that_walks() {
		struct Kinds(std::vec::Vec<NodeId>);

		impl NodeVisitor for Kinds {
			fn consider_node(&self, _: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}

			fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
				self.0.extend(node.node_id);
				VisitFlow::DESCEND
			}

			fn exit_node(&mut self, _: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
		}

		let arena = Arena::default();
		let parsed = parse_root(NodeId::StyleSheet, &arena, "a{color:red}").expect("style-sheet is a root");
		let root = parsed.root.expect("the source parses");
		assert!(parsed.diagnostics.is_empty());
		assert_eq!(root.to_span().end().0, 12);

		let mut kinds = Kinds(std::vec::Vec::new());
		let _ = root.accept_dyn(&mut kinds);
		assert!(kinds.0.contains(&NodeId::StyleSheet), "got {:?}", kinds.0);
		assert!(kinds.0.contains(&NodeId::StyleRule), "got {:?}", kinds.0);

		assert!(parse_root(NodeId::StyleValue, &arena, "red").is_none(), "style-value is not a root");
	}

	#[test]
	fn a_walk_of_an_erased_root_prunes_as_a_typed_walk_does() {
		use super::{Visitable, visitor};
		use crate::StyleSheet;
		use css_lexer::Lexer;
		use css_parse::Parser;

		struct Pruner(std::vec::Vec<NodeId>);

		#[visitor]
		impl super::Visit for Pruner {
			fn consider_node(&self, node: VisitNode) -> VisitFlow {
				if node.node_id == Some(NodeId::StyleRule) { VisitFlow::SKIP_CHILDREN } else { VisitFlow::DESCEND }
			}

			fn enter_node(&mut self, node: VisitNode) {
				self.0.extend(node.node_id);
			}
		}

		let arena = Arena::default();
		let source = "a{color:red}";
		let lexer = Lexer::new(&crate::CssAtomSet::ATOMS, source);
		let typed =
			Parser::new(&arena, source, lexer).parse_entirely::<StyleSheet>().output.expect("the source parses");
		let mut typed_kinds = Pruner(std::vec::Vec::new());
		let _ = typed.accept(&mut typed_kinds);

		let parsed = parse_root(NodeId::StyleSheet, &arena, source).expect("style-sheet is a root");
		let mut erased_kinds = Pruner(std::vec::Vec::new());
		let _ = parsed.root.expect("the source parses").accept_dyn(&mut erased_kinds);

		assert_eq!(typed_kinds.0, erased_kinds.0);
		assert_eq!(typed_kinds.0, [NodeId::StyleSheet]);
	}
}
