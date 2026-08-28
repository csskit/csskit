use crate::prelude::*;
use css_ast::{
	BorderColorStyleValue, BorderRadiusStyleValue, BorderStyleStyleValue, BorderWidthStyleValue, ColumnGapStyleValue,
	DeclarationValue, GapStyleValue, InsetStyleValue, MarginStyleValue, OverflowStyleValue, PaddingStyleValue,
	RowGapStyleValue, VisitNode, Visitable,
};
use css_parse::{Declaration, SemanticEq};

/// Removes redundant parts from values of CSS shorthand declarations, for example `margin: 1px 1px 1px 1px` becomes
/// `margin: 1px`.
pub struct ReduceShorthandValues<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> ReduceShorthandValues<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn delete<T: ToSpan>(&self, value: &T) {
		let span = value.to_span();
		self.transformer.clear_pending_edits(span);
		self.transformer.delete(span);
	}

	fn reduce_two<T>(&self, first: &T, second: &Option<T>)
	where
		T: SemanticEq + ToSpan,
	{
		if let Some(second) = second
			&& second.semantic_eq(first)
		{
			self.delete(second);
		}
	}

	fn reduce_four<T>(&self, first: &T, second: &Option<T>, third: &Option<T>, fourth: &Option<T>)
	where
		T: SemanticEq + ToSpan,
	{
		if let Some(fourth) = fourth {
			let Some(second) = second else { return };
			if !fourth.semantic_eq(second) {
				return;
			}
			self.delete(fourth);
		}
		if let Some(third) = third {
			if !third.semantic_eq(first) {
				return;
			}
			self.delete(third);
		}
		self.reduce_two(first, second);
	}
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceShorthandValues<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.has_shorthands()
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

#[visitor]
impl<'a, 'ctx, N> Visit for ReduceShorthandValues<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn enter_declaration<'b, T: DeclarationValue<'b, CssMetadata>>(
		&mut self,
		declaration: &Declaration<'b, T, CssMetadata>,
		_query: VisitNode,
	) -> VisitFlow {
		if declaration.metadata().has_substitution() {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn visit_margin_style_value(&mut self, value: &MarginStyleValue) {
		self.reduce_four(&value.0, &value.1, &value.2, &value.3);
	}

	fn visit_padding_style_value(&mut self, value: &PaddingStyleValue) {
		self.reduce_four(&value.0, &value.1, &value.2, &value.3);
	}

	fn visit_inset_style_value(&mut self, value: &InsetStyleValue) {
		self.reduce_four(&value.0, &value.1, &value.2, &value.3);
	}

	fn visit_border_color_style_value(&mut self, value: &BorderColorStyleValue) {
		self.reduce_four(&value.0, &value.1, &value.2, &value.3);
	}

	fn visit_border_style_style_value(&mut self, value: &BorderStyleStyleValue) {
		self.reduce_four(&value.0, &value.1, &value.2, &value.3);
	}

	fn visit_border_width_style_value(&mut self, value: &BorderWidthStyleValue) {
		self.reduce_four(&value.0, &value.1, &value.2, &value.3);
	}
	fn visit_border_radius_style_value(&mut self, value: &BorderRadiusStyleValue) {
		self.reduce_four(&value.0, &value.1, &value.2, &value.3);
		if let Some((_, first, second, third, fourth)) = &value.4 {
			self.reduce_four(first, second, third, fourth);
		}
	}

	fn visit_gap_style_value(&mut self, value: &GapStyleValue) {
		if let Some(second) = &value.1
			&& gap_values_equal(&value.0, second)
		{
			self.delete(second);
		}
	}

	fn visit_overflow_style_value(&mut self, value: &OverflowStyleValue) {
		self.reduce_two(&value.0, &value.1);
	}
}

pub(crate) fn gap_values_equal<'a>(first: &RowGapStyleValue<'a>, second: &ColumnGapStyleValue<'a>) -> bool {
	match (first, second) {
		(RowGapStyleValue::Normal(first), ColumnGapStyleValue::Normal(second)) => first.semantic_eq(second),
		(RowGapStyleValue::LengthPercentage(first), ColumnGapStyleValue::LengthPercentage(second)) => {
			first.semantic_eq(second)
		}
		(RowGapStyleValue::LineWidth(first), ColumnGapStyleValue::LineWidth(second)) => first.semantic_eq(second),
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
