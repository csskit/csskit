use crate::prelude::*;
use css_ast::{DeclarationValue, VisitMut, VisitNode};
use css_parse::Declaration;

/// Removes nodes which have no effect on rendering, such as rules with an empty block, or rules holding nothing but
/// such rules.
pub struct RemoveInertNodes<'a, 'ctx, N: NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for RemoveInertNodes<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.is_inert()
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

#[visitor]
impl<'a, 'ctx, N> VisitMut<'a> for RemoveInertNodes<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.rule_subtree_metadata().is_some_and(|meta| Self::skips_subtree(&meta)) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn retain(&mut self, node: VisitNode) -> bool {
		if node.self_metadata().is_inert() {
			self.transformer.mark_changed();
			return false;
		}
		true
	}

	fn enter_declaration<T: DeclarationValue<'a, CssMetadata>>(
		&mut self,
		_decl: &mut Declaration<'a, T, CssMetadata>,
	) -> VisitFlow {
		VisitFlow::SKIP_CHILDREN
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn removes_empty_style_rule() {
		assert_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"a {}\nb { color: red }",
			"b { color: red }"
		);
	}

	#[test]
	fn removes_consecutive_empty_style_rules() {
		assert_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"kbd {}\nul li:nth-of-type(2n) {}\nfieldset > legend {}",
			""
		);
	}

	#[test]
	fn removes_empty_media_rule() {
		assert_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"@media screen {}\nb { color: red }",
			"b { color: red }"
		);
	}

	#[test]
	fn removes_empty_nested_rule() {
		assert_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"a { color: red; & b {} }",
			"a { color: red; }"
		);
	}

	#[test]
	fn removes_rule_containing_only_inert_rules() {
		assert_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"nav { a {} }\nb { color: red }",
			"b { color: red }"
		);
	}

	#[test]
	fn removes_keyframes_with_only_empty_keyframes() {
		assert_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"@keyframes fade { 0% {} 100% {} }\nb { color: red }",
			"b { color: red }"
		);
	}

	#[test]
	fn removes_empty_font_face_rule() {
		assert_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"@font-face {}\nb { color: red }",
			"b { color: red }"
		);
	}

	#[test]
	fn keeps_rule_with_declarations() {
		assert_no_transform!(CssMinifierFeature::RemoveInertNodes, CssAtomSet, StyleSheet, "a { color: red }");
	}

	#[test]
	fn keeps_rule_with_unknown_declaration() {
		assert_no_transform!(CssMinifierFeature::RemoveInertNodes, CssAtomSet, StyleSheet, "a { fnord: red }");
	}

	#[test]
	fn keeps_rule_containing_a_statement_at_rule() {
		assert_no_transform!(
			CssMinifierFeature::RemoveInertNodes,
			CssAtomSet,
			StyleSheet,
			"@media screen { @layer a; }"
		);
	}

	#[test]
	fn keeps_empty_layer_rule() {
		assert_no_transform!(CssMinifierFeature::RemoveInertNodes, CssAtomSet, StyleSheet, "@layer a {}");
	}

	#[test]
	fn keeps_empty_unknown_at_rule() {
		assert_no_transform!(CssMinifierFeature::RemoveInertNodes, CssAtomSet, StyleSheet, "@fnord {}");
	}

	#[test]
	fn keeps_rule_containing_an_unknown_at_rule() {
		assert_no_transform!(CssMinifierFeature::RemoveInertNodes, CssAtomSet, StyleSheet, "a { @fnord {} }");
	}
}
