use crate::prelude::*;
use css_ast::{ColumnGapStyleValue, DeclarationValue, RowGapStyleValue, StyleValue, VisitMut, VisitNode};
use css_parse::{Declaration, SemanticEq};

/// Removes redundant parts from values of CSS shorthand declarations, for example `margin: 1px 1px 1px 1px` becomes
/// `margin: 1px`.
pub struct ReduceShorthandValues<'a, 'ctx, N: NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> ReduceShorthandValues<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn values_equal<T: SemanticEq>(&self, first: &T, second: &T) -> bool {
		first.semantic_eq(second, &self.transformer.text())
	}

	fn delete<T>(&self, value: &mut Option<T>) {
		*value = None;
		self.transformer.mark_changed();
	}

	fn reduce_two<T: SemanticEq>(&self, first: &T, second: &mut Option<T>) {
		if let Some(second_value) = second
			&& self.values_equal(second_value, first)
		{
			self.delete(second);
		}
	}

	fn reduce_four<T: SemanticEq>(
		&self,
		first: &T,
		second: &mut Option<T>,
		third: &mut Option<T>,
		fourth: &mut Option<T>,
	) {
		if let Some(fourth_value) = fourth {
			let Some(second_value) = second else { return };
			if !self.values_equal(fourth_value, second_value) {
				return;
			}
			self.delete(fourth);
		}
		if let Some(third_value) = third {
			if !self.values_equal(third_value, first) {
				return;
			}
			self.delete(third);
		}
		self.reduce_two(first, second);
	}
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceShorthandValues<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.has_shorthands()
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

#[visitor]
impl<'a, 'ctx, N> VisitMut<'a> for ReduceShorthandValues<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.rule_subtree_metadata().is_some_and(|meta| Self::skips_subtree(&meta)) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn enter_declaration<T: DeclarationValue<'a, CssMetadata>>(
		&mut self,
		declaration: &mut Declaration<'a, T, CssMetadata>,
	) -> VisitFlow {
		if declaration.metadata().has_substitution() { VisitFlow::SKIP_CHILDREN } else { VisitFlow::DESCEND }
	}

	fn visit_style_value(&mut self, _value: &mut StyleValue<'a>) -> VisitFlow {
		VisitFlow::SKIP_CHILDREN
	}

	fn exit_style_value(&mut self, value: &mut StyleValue<'a>) {
		match value {
			StyleValue::Margin(value) => {
				let value = &mut **value;
				self.reduce_four(&value.0, &mut value.1, &mut value.2, &mut value.3);
			}
			StyleValue::Padding(value) => {
				let value = &mut **value;
				self.reduce_four(&value.0, &mut value.1, &mut value.2, &mut value.3);
			}
			StyleValue::Inset(value) => {
				let value = &mut **value;
				self.reduce_four(&value.0, &mut value.1, &mut value.2, &mut value.3);
			}
			StyleValue::BorderColor(value) => {
				let value = &mut **value;
				self.reduce_four(&value.0, &mut value.1, &mut value.2, &mut value.3);
			}
			StyleValue::BorderStyle(value) => {
				let value = &mut **value;
				self.reduce_four(&value.0, &mut value.1, &mut value.2, &mut value.3);
			}
			StyleValue::BorderWidth(value) => {
				let value = &mut **value;
				self.reduce_four(&value.0, &mut value.1, &mut value.2, &mut value.3);
			}
			StyleValue::BorderRadius(value) => {
				let value = &mut **value;
				self.reduce_four(&value.0, &mut value.1, &mut value.2, &mut value.3);
				if let Some((_, first, second, third, fourth)) = &mut value.4 {
					self.reduce_four(first, second, third, fourth);
				}
			}
			StyleValue::Gap(value) => {
				let value = &mut **value;
				if let Some(second) = &value.1
					&& gap_values_equal(&value.0, second, &self.transformer.text())
				{
					self.delete(&mut value.1);
				}
			}
			StyleValue::Overflow(value) => {
				let value = &mut **value;
				self.reduce_two(&value.0, &mut value.1);
			}
			_ => {}
		}
	}
}

pub(crate) fn gap_values_equal<'a>(
	first: &RowGapStyleValue<'a>,
	second: &ColumnGapStyleValue<'a>,
	source_text: &str,
) -> bool {
	match (first, second) {
		(RowGapStyleValue::Normal(first), ColumnGapStyleValue::Normal(second)) => {
			first.semantic_eq(second, source_text)
		}
		(RowGapStyleValue::LengthPercentage(first), ColumnGapStyleValue::LengthPercentage(second)) => {
			first.semantic_eq(second, source_text)
		}
		(RowGapStyleValue::LineWidth(first), ColumnGapStyleValue::LineWidth(second)) => {
			first.semantic_eq(second, source_text)
		}
		_ => false,
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn reduces_four_value_shorthands() {
		assert_transform!(
			CssMinifierFeature::ReduceShorthandValues,
			CssAtomSet,
			StyleSheet,
			"a { margin: 1px 1px 1px 1px; padding: 1px 2px 1px 2px; inset: 1px 2px 3px 2px; }",
			"a { margin: 1px; padding: 1px 2px; inset: 1px 2px 3px; }"
		);
	}

	#[test]
	fn reduces_shorthands_in_nested_rules() {
		assert_transform!(
			CssMinifierFeature::ReduceShorthandValues,
			CssAtomSet,
			StyleSheet,
			"@media print { a { margin: 1px 1px; } } b { c { padding: 2px 2px; } }",
			"@media print { a { margin: 1px; } } b { c { padding: 2px; } }"
		);
	}

	#[test]
	fn reduces_border_and_overflow_shorthands() {
		assert_transform!(
			CssMinifierFeature::ReduceShorthandValues,
			CssAtomSet,
			StyleSheet,
			"a { border-color: red red red red; border-radius: 10px 10px 10px 10px / 5px 5px 5px 5px; border-style: solid solid; border-width: 1px 2px 1px 2px; gap: 1px 1px; overflow: hidden hidden; }",
			"a { border-color: red; border-radius: 10px / 5px; border-style: solid; border-width: 1px 2px; gap: 1px; overflow: hidden; }"
		);
	}

	#[test]
	fn keeps_irreducible_shorthands() {
		assert_no_transform!(
			CssMinifierFeature::ReduceShorthandValues,
			CssAtomSet,
			StyleSheet,
			"a { margin: 1px 2px 3px 4px; }"
		);
	}

	#[test]
	fn keeps_shorthands_using_substitution() {
		assert_no_transform!(
			CssMinifierFeature::ReduceShorthandValues,
			CssAtomSet,
			StyleSheet,
			"a { margin: var(--a) var(--a); gap: var(--g) var(--g); }"
		);
	}

	#[test]
	fn keeps_declarations_intact() {
		assert_no_transform!(
			CssMinifierFeature::ReduceShorthandValues,
			CssAtomSet,
			StyleSheet,
			"a { border-width: 1px; border-style: solid; border-color: red; margin-top: 1px; margin: 2px; }"
		);
	}
}
