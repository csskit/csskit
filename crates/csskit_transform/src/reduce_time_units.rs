use crate::prelude::*;
use css_ast::{Time, Visitable};

pub struct ReduceTimeUnits<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceTimeUnits<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn may_change(features: CssMinifierFeature, _node: &N) -> bool {
		features.contains(CssMinifierFeature::ReduceTimeUnits)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

impl<'a, 'ctx, N> Visit for ReduceTimeUnits<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn visit_time(&mut self, time: &Time) {
		let original_len = time.to_span().len() as usize;
		let seconds = time.as_seconds();

		if seconds == 0.0 && original_len > 1 {
			self.transformer.replace_parsed::<Time>(time.to_span(), "0");
			return;
		}

		if matches!(time, Time::Ms(_)) {
			let value = if seconds.fract() == 0.0 { format!("{}", seconds as i64) } else { format!("{seconds}") };
			let len = value.len() - value.starts_with("0.") as usize - value.starts_with("-0.") as usize + 1;
			if len < original_len {
				self.transformer.replace_parsed::<Time>(time.to_span(), &format!("{value}s"));
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
	fn normalizes_zero_units() {
		assert_transform!(
			CssMinifierFeature::ReduceTimeUnits,
			CssAtomSet,
			StyleSheet,
			"div { transition-delay: 0ms; animation-duration: 0s; }",
			"div { transition-delay: 0; animation-duration: 0; }"
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
}
