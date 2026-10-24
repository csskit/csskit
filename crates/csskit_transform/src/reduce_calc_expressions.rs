use crate::prelude::*;
use css_ast::{
	Angle, CSSInt, CalcUnit, CssAtomSet, Decibel, DeclarationValue, Flex, Frequency, Length, LengthPercentage,
	LengthPercentageNumber, MathFunction, NumberLength, NumberPercentage, Percentage, ReducedCalc, Resolution,
	Semitones, SupportsFeature, Time, UnitlessZeroResolves, VisitNode, Visitable,
};
use css_parse::{AtomSet, ComponentValues, Declaration, format_in};

const SIGNIFICANT_DIGITS: i32 = 6;

fn round_significant(value: f64) -> f64 {
	if value == 0.0 || !value.is_finite() || value.fract() == 0.0 || value.abs() >= 1e6 {
		return value;
	}
	let factor = 10f64.powi(SIGNIFICANT_DIGITS - 1 - value.abs().log10().floor() as i32);
	(value * factor).round() / factor
}

fn unit_str(unit: CalcUnit) -> &'static str {
	match unit {
		CalcUnit::Number => "",
		CalcUnit::Dimension(atom) => atom.to_str(),
	}
}

#[derive(Default)]
struct CalcLeaf(Option<ReducedCalc>);

macro_rules! visit_calc_leaves {
	($($method:ident($ty:ty)),+ $(,)?) => {
		#[visitor]
		impl Visit for CalcLeaf {
			$(
				fn $method(&mut self, value: &$ty) -> VisitFlow {
					self.0 = ReducedCalc::from_token((*value).into());
					VisitFlow::STOP
				}
			)+
		}
	};
}

visit_calc_leaves!(
	visit_length(Length),
	visit_length_percentage(LengthPercentage),
	visit_length_percentage_number(LengthPercentageNumber),
	visit_number_length(NumberLength),
	visit_percentage(Percentage),
	visit_number_percentage(NumberPercentage),
	visit_time(Time),
	visit_angle(Angle),
	visit_frequency(Frequency),
	visit_resolution(Resolution),
	visit_flex(Flex),
	visit_css_int(CSSInt),
	visit_decibel(Decibel),
	visit_semitones(Semitones),
);

fn reduce_leaf<T: Visitable>(value: &T) -> Option<ReducedCalc> {
	let mut leaf = CalcLeaf::default();
	let _ = value.accept(&mut leaf);
	leaf.0
}

pub struct ReduceCalcExpressions<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
	unitless_zero_resolves: UnitlessZeroResolves,
	math_depth: usize,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceCalcExpressions<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.has_computed()
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer, unitless_zero_resolves: UnitlessZeroResolves::Number, math_depth: 0 }
	}
}

impl<'a, 'ctx, N> ReduceCalcExpressions<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn serialize(&self, reduced: ReducedCalc) -> Option<&'a str> {
		let arena = self.transformer.alloc();
		let terms = reduced.terms();
		let mut rounded =
			terms.iter().map(|&(unit, value)| (unit, round_significant(value))).filter(|&(unit, value)| {
				value != 0.0 || (unit == CalcUnit::Dimension(CssAtomSet::Percent) && terms.len() > 1)
			});
		let Some((unit, value)) = rounded.next() else {
			let (unit, _) = terms[0];
			return Some(match unit {
				CalcUnit::Number => "calc(0)",
				CalcUnit::Dimension(CssAtomSet::Px) if self.unitless_zero_resolves == UnitlessZeroResolves::Length => {
					"0"
				}
				_ => format_in!(in arena, "0{}", unit_str(unit)).into_str(),
			});
		};
		if !value.is_finite() {
			return None;
		}
		let rest: std::vec::Vec<_> = rounded.collect();
		if rest.is_empty() {
			let in_range = value >= 0.0
				&& !matches!(unit, CalcUnit::Number | CalcUnit::Dimension(CssAtomSet::Deg))
				&& !(unit == CalcUnit::Dimension(CssAtomSet::Percent) && value > 100.0);
			let css = if in_range {
				format_in!(in arena, "{}{}", value, unit_str(unit))
			} else {
				format_in!(in arena, "calc({}{})", value, unit_str(unit))
			};
			return Some(css.into_str());
		}
		let mut css = format_in!(in arena, "calc({}{}", value, unit_str(unit));
		for (unit, value) in rest {
			if !value.is_finite() {
				return None;
			}
			let operator = if value < 0.0 { '-' } else { '+' };
			css.push_str(format_in!(in arena, " {} {}{}", operator, value.abs(), unit_str(unit)).as_str());
		}
		css.push(')');
		Some(css.into_str())
	}
}

#[visitor]
impl<'a, 'ctx, N> Visit for ReduceCalcExpressions<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.node_id.is_some() && Self::skips_subtree(&node.subtree_metadata()) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn enter_declaration<'b, T: DeclarationValue<'b, CssMetadata>>(
		&mut self,
		decl: &Declaration<'b, T, CssMetadata>,
		_query: VisitNode,
	) {
		self.unitless_zero_resolves = decl.metadata().unitless_zero_resolves;
	}

	fn exit_declaration<'b, T: DeclarationValue<'b, CssMetadata>>(
		&mut self,
		_decl: &Declaration<'b, T, CssMetadata>,
		_query: VisitNode,
	) {
		self.unitless_zero_resolves = UnitlessZeroResolves::Number;
	}

	fn visit_supports_feature<'b>(&mut self, _feature: &SupportsFeature<'b>) -> VisitFlow {
		VisitFlow::SKIP_CHILDREN
	}

	fn visit_math_function<'b, T: Visitable + ToSpan>(&mut self, _math: &MathFunction<'b, T>) {
		self.math_depth += 1;
	}

	fn exit_math_function<'b, T: Visitable + ToSpan>(&mut self, math: &MathFunction<'b, T>) {
		self.math_depth -= 1;
		// minimal: only the outermost math function is reduced, so a reducible function nested in an unreducible one
		// is kept; reduce nested functions individually if that matters.
		if self.math_depth > 0 {
			return;
		}
		let span = math.to_span();
		if self.transformer.has_pending_edit_covering(span) {
			return;
		}
		let Some(css) = math.reduce_with(&reduce_leaf).and_then(|reduced| self.serialize(reduced)) else {
			return;
		};
		if css.len() < span.len() as usize {
			self.transformer.clear_pending_edits(span);
			self.transformer.replace(span, self.transformer.parse_value::<ComponentValues>(css));
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	macro_rules! calc {
		($from:literal, $to:literal) => {
			assert_transform!(CssMinifierFeature::ReduceCalcExpressions, CssAtomSet, StyleSheet, $from, $to)
		};
	}

	macro_rules! keep {
		($($css:literal),+ $(,)?) => {
			$(assert_no_transform!(CssMinifierFeature::ReduceCalcExpressions, CssAtomSet, StyleSheet, $css);)+
		};
	}

	#[test]
	fn folds_same_unit_arithmetic() {
		calc!("a { width: calc(50px + 50px); }", "a { width: 100px; }");
		calc!("a { width: calc(100px * 2); }", "a { width: 200px; }");
		calc!("a { width: calc(100px / 4); }", "a { width: 25px; }");
		calc!("a { width: calc(2 * 3 * 10px); }", "a { width: 60px; }");
		calc!("a { width: calc(50% + 25%); }", "a { width: 75%; }");
		calc!("a { width: calc(100px); }", "a { width: 100px; }");
	}

	#[test]
	fn folds_nested_and_parenthesized() {
		calc!("a { width: calc(calc(10px + 20px) + calc(30px + 40px)); }", "a { width: 100px; }");
		calc!("a { width: calc(2 * (50px + 25px)); }", "a { width: 150px; }");
		calc!("a { margin-left: calc(-1 * (-10px)); }", "a { margin-left: 10px; }");
	}

	#[test]
	fn converts_absolute_units() {
		calc!("a { width: calc(1in + 0px); }", "a { width: 96px; }");
		calc!("a { transition-duration: calc(1s - 200ms); }", "a { transition-duration: 0.8s; }");
	}

	#[test]
	fn uses_constants_with_six_significant_digits() {
		calc!("a { width: calc(pi * 100px); }", "a { width: 314.159px; }");
		calc!("a { width: calc(100% / 3); }", "a { width: 33.3333%; }");
		calc!("a { width: calc(100% / 11); }", "a { width: 9.09091%; }");
	}

	#[test]
	fn zero_result() {
		calc!("a { width: calc(50px - 50px); }", "a { width: 0; }");
		calc!("a { width: calc(100px * 0); }", "a { width: 0; }");
		calc!("a { line-height: calc(10px - 10px); }", "a { line-height: 0px; }");
	}

	#[test]
	fn keeps_number_results_in_calc() {
		calc!("a { line-height: calc(10 / 2); }", "a { line-height: calc(5); }");
		calc!("a { column-count: calc(2 - 2); }", "a { column-count: calc(0); }");
		calc!("a { z-index: calc(1000000 + 1); }", "a { z-index: calc(1000001); }");
	}

	#[test]
	fn keeps_percentages_over_100_in_calc() {
		calc!("a { width: calc(60% * 2); }", "a { width: calc(120%); }");
	}

	#[test]
	fn keeps_negative_results_in_calc() {
		calc!("a { margin-left: calc(5px - 10px); }", "a { margin-left: calc(-5px); }");
	}

	#[test]
	fn keeps_mixed_units_as_calc() {
		calc!("a { width: calc(100% - 50px + 0%); }", "a { width: calc(100% - 50px); }");
		calc!("a { width: calc(((75.37% - 63.5px) - 900px) + (2 * 100px)); }", "a { width: calc(75.37% - 763.5px); }");
		calc!("a { margin: calc((100% - 5em) / 2); }", "a { margin: calc(50% - 2.5em); }");
		calc!("a { margin: calc(-1 * (1rem + 16px) / 2); }", "a { margin: calc(-0.5rem - 8px); }");
	}

	#[test]
	fn cancels_reciprocal_dimensions() {
		calc!("a { width: calc(1 / (1 / 50px)); }", "a { width: 50px; }");
		calc!("a { width: calc(10px * (2 / 20px) * 3em); }", "a { width: 3em; }");
		calc!("a { line-height: calc(1in / 48px); }", "a { line-height: calc(2); }");
	}

	#[test]
	fn reduces_comparison_functions_of_one_unit() {
		calc!("a { width: min(10px, 20px); }", "a { width: 10px; }");
		calc!("a { width: max(1in, 20px); }", "a { width: 96px; }");
		calc!("a { width: clamp(10px, 50px, 20px); }", "a { width: 20px; }");
		calc!("a { margin: calc(abs(-5px) + 5px); }", "a { margin: 10px; }");
		calc!("a { width: calc(min(10px, 20px) + 10px); }", "a { width: 20px; }");
	}

	#[test]
	fn reduces_each_math_function_in_a_value() {
		calc!("a { margin: calc(1px + 2px) calc(4px - 1px); }", "a { margin: 3px 3px; }");
		calc!("a { width: var(--x, calc(5px + 5px)); }", "a { width: var(--x, 10px); }");
	}

	#[test]
	fn keeps_zero_percentage_beside_lengths() {
		keep!("a { width: calc(100px + 0%); }");
	}

	#[test]
	fn keeps_unresolved_reciprocals() {
		keep!(
			"a { width: calc(1 / (1 / 50px) + 1 / 2px); }",
			"a { transition-duration: calc(1 / 4s); }",
			"a { margin: calc(10 / 4px); }",
			"a { margin: calc(2 * (10 / 4px)); }",
		);
	}

	#[test]
	fn keeps_typed_multipliers_of_unresolvable_operands() {
		keep!(
			"a { transition-delay: calc(var(--x) * .25s); }",
			"li { transition-delay: calc(sibling-index() * .25s); }",
		);
	}

	#[test]
	fn leaves_unresolvable() {
		keep!(
			"a { width: calc(1em + 1px); }",
			"a { width: calc(var(--x) + 10px); }",
			"a { width: calc(min(10px, 1em) + 10px); }",
			"a { width: min(10px, 1em); }",
			"a { width: calc(10px * 1em); }",
			"a { width: calc(1px / 0); }",
			"a { width: calc(infinity * 1px); }",
		);
	}

	#[test]
	fn preserves_nan_comparisons() {
		keep!(
			"a { width: min(NaN * 1px, 100px); }",
			"a { width: max(100px, NaN * 1px); }",
			"a { width: clamp(10px, NaN * 1px, 100px); }",
		);
	}

	#[test]
	fn preserves_relative_unit_resolution() {
		keep!(
			"a { font-size: 0; width: calc(sign(1em) * 100px); }",
			"a { font-size: 0; width: calc(1em / 1em * 100px); }",
			"a { width: calc(2 / (4 / 50%)); }",
			"a { width: calc(1em * (1 / 1em) * 100px); }",
		);
	}

	#[test]
	fn keeps_supports_conditions() {
		keep!("@supports (width: calc(1px + 1px)) { a { color: red } }");
	}

	#[test]
	fn keeps_units_on_zero_outside_declarations() {
		calc!(
			"@media (min-width: calc(1px - 1px)) { a { width: calc(2px + 3px) } }",
			"@media (min-width: 0px) { a { width: 5px } }"
		);
		calc!(
			"@container (width > calc(1px - 1px)) { a { width: 1px } }",
			"@container (width > 0px) { a { width: 1px } }"
		);
	}
}
