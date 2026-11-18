use crate::prelude::*;
use css_ast::{CssTypes, Time, VisitMut, VisitNode};
use css_parse::format_in;

pub struct ReduceTimeUnits<'a, 'ctx, N: NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceTimeUnits<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.has_value_kinds(CssTypes::Time)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

#[visitor]
impl<'a, 'ctx, N> VisitMut<'a> for ReduceTimeUnits<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.rule_subtree_metadata().is_some_and(|meta| Self::skips_subtree(&meta)) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn visit_time(&mut self, time: &mut Time) {
		if let Time::Ms(_) = *time {
			let seconds = time.as_seconds();
			if !seconds.is_finite() {
				return;
			}
			let seconds_text = format_in!(in self.transformer.alloc(), "{seconds}s").into_str();
			let Some(compacted) = self.transformer.compact(seconds_text) else {
				return;
			};
			if compacted.len() < self.transformer.compacted_len(time)
				&& let Some(reduced) = self.transformer.parse::<Time>(seconds_text)
			{
				self.transformer.replace(time, reduced);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn converts_milliseconds_to_seconds() {
		assert_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { transition-duration: 500ms; }",
			"div { transition-duration: 0.5s; }"
		);
	}

	#[test]
	fn keeps_shorter_millisecond_values() {
		assert_no_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { transition-duration: 50ms; }"
		);
	}

	#[test]
	fn keeps_second_values_when_not_shorter() {
		assert_no_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { transition-duration: 2s; }"
		);
	}

	#[test]
	fn converts_whole_seconds() {
		assert_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { animation-duration: 1000ms; }",
			"div { animation-duration: 1s; }"
		);
	}

	#[test]
	fn converts_only_when_compact_is_larger() {
		assert_no_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { animation-duration: 00050ms; }"
		);
	}

	#[test]
	fn keeps_milliseconds_beyond_f32_range() {
		assert_no_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { transition-duration: 4e38ms; }"
		);
	}

	#[test]
	fn converts_milliseconds_beyond_f32_precision() {
		assert_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { transition-duration: 300000000000000000000000000000ms; }",
			"div { transition-duration: 300000000000000000000000000s; }"
		);
	}

	#[test]
	fn compares_lengths_after_scientific_notation() {
		assert_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { transition-duration: 1000000ms; }",
			"div { transition-duration: 1000s; }"
		);
	}

	#[test]
	fn converts_in_rules_nested_in_rules_without_times() {
		assert_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { color: red; & span { transition-duration: 500ms; } }",
			"div { color: red; & span { transition-duration: 0.5s; } }"
		);
	}
}
