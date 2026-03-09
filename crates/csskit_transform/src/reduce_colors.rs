use crate::prelude::*;
use chromashift::{COLOR_EPSILON, ColorDistance, ColorSpace, Hex, Named, Srgb, ToAlpha};
use css_ast::{
	Color, ColorFunction, ColorMixFunction, HueInterpolationDirection, InterpolationColorSpace, ToChromashift,
	Visitable,
};

pub struct ReduceColors<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
	/// When true, the outer color-mix() is being replaced entirely, so inner
	/// `visit_color` calls should be suppressed to avoid overlapping edits.
	replacing_outer: bool,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceColors<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn may_change(features: CssMinifierFeature, _node: &N) -> bool {
		features.contains(CssMinifierFeature::ReduceColors)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer, replacing_outer: false }
	}
}

trait Shortest {
	fn shortest(&self) -> Option<String>;
}

impl Shortest for chromashift::Color {
	fn shortest(&self) -> Option<String> {
		[
			Some(Hex::from(*self).to_string()),
			Named::try_from(*self).ok().map(|named| named.to_string()),
			Some(Srgb::from(*self).to_string()),
		]
		.into_iter()
		.flatten()
		.min_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)))
	}
}

impl<'a, 'ctx, N> Visit for ReduceColors<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn visit_color(&mut self, color: &Color) {
		if self.replacing_outer {
			return;
		}
		// color-mix() is handled by visit_color_mix_function
		if matches!(color, Color::Function(ColorFunction::ColorMix(_))) {
			return;
		}
		let Some(chroma_color) = color.to_chromashift() else {
			return;
		};
		let len = color.to_span().len() as usize;

		// Only generate sRGB-based candidates if the colour is within the sRGB gamut.
		// Converting an out-of-gamut colour (e.g. display-p3 1 0 0) to sRGB would silently
		// clamp the values, changing the actual colour.
		if !chroma_color.in_gamut_of(ColorSpace::Srgb) {
			return;
		}

		let Some(candidate) = chroma_color.shortest() else {
			return;
		};

		if candidate.len() < len {
			self.transformer.replace_parsed::<Color>(color.to_span(), &candidate);
		}
	}

	fn visit_color_mix_function<'b>(&mut self, mix: &ColorMixFunction<'b>) {
		let outer_span = mix.to_span();
		let outer_len = outer_span.len() as usize;

		let first_chroma = mix.first.color.to_chromashift();
		let second_chroma = mix.second.color.to_chromashift();
		let delta_e = first_chroma.and_then(|first| second_chroma.map(|second| first.delta_e(second)));

		// Compute effective percentages per CSS Color 5 3.2:
		// - If only one percentage is given, the other is 100% - given
		// - If neither is given, both default to 50%
		// - If both are given, they're used as-is (and may not sum to 100%)
		let p1_explicit = mix.first.percentage.as_ref().map(|p| p.value());
		let p2_explicit = mix.second.percentage.as_ref().map(|p| p.value());
		let (p1, p2) = match (p1_explicit, p2_explicit) {
			(Some(a), Some(b)) => (a, b),
			(Some(a), None) => (a, 100.0 - a),
			(None, Some(b)) => (100.0 - b, b),
			(None, None) => (50.0, 50.0),
		};
		let sum = p1 + p2;

		// The same color on both sides should just shrink to the one color,
		// but only when alpha_mult is 1 (sum >= 100)
		if delta_e.is_some_and(|delta| delta < COLOR_EPSILON) && sum >= 100.0 {
			let str = first_chroma.and_then(|color| color.shortest()).unwrap_or_else(|| {
				let span = mix.first.color.to_span();
				self.transformer.source_text[span.start().0 as usize..span.end().0 as usize].to_string()
			});
			self.transformer.clear_pending_edits(outer_span);
			self.transformer.replace_parsed::<Color>(outer_span, &str);
			self.replacing_outer = true;
			return;
		}

		// 100%/0% elimination — only when sum == 100 (no alpha multiplier, no normalization)
		if sum == 100.0 && (p1 == 100.0 || p2 == 0.0) {
			let str = first_chroma.and_then(|color| color.shortest()).unwrap_or_else(|| {
				let span = mix.first.color.to_span();
				self.transformer.source_text[span.start().0 as usize..span.end().0 as usize].to_string()
			});
			self.transformer.clear_pending_edits(outer_span);
			self.transformer.replace_parsed::<Color>(outer_span, &str);
			self.replacing_outer = true;
			return;
		}

		// 0%/100% elimination — only when sum == 100
		if sum == 100.0 && (p2 == 100.0 || p1 == 0.0) {
			let str = second_chroma.and_then(|color| color.shortest()).unwrap_or_else(|| {
				let span = mix.second.color.to_span();
				self.transformer.source_text[span.start().0 as usize..span.end().0 as usize].to_string()
			});
			self.transformer.clear_pending_edits(outer_span);
			self.transformer.replace_parsed::<Color>(outer_span, &str);
			self.replacing_outer = true;
			return;
		}

		// Try to statically mix both colors if they're both known
		if let (Some(first), Some(second)) = (first_chroma, second_chroma)
			&& sum > 0.0
		{
			// Normalize so that p1_norm + p2_norm = 100
			let np1 = (p1 as f64) / (sum as f64) * 100.0;
			// The percentage for mixing is "how much of the second color"
			let percentage = 100.0 - np1;

			let mixed = mix.interpolation.color_space.mix(first, second, percentage);

			// Apply alpha multiplier per CSS Color 5 3.3:
			// alpha_mult = 1 - leftover, where leftover = max(1 - sum/100, 0)
			let alpha_mult = (sum as f64 / 100.0).min(1.0);
			let mixed_alpha = (mixed.to_alpha() as f64 / 100.0 * alpha_mult * 100.0) as f32;
			let mixed = mixed.with_alpha(mixed_alpha);

			// Only convert to sRGB hex/named if the result is in sRGB gamut.
			// Converting out-of-gamut colors to hex silently clamps, changing the color.
			let candidate = if mixed.in_gamut_of(ColorSpace::Srgb) {
				mixed.shortest()
			} else {
				// Out of sRGB gamut — output in the native interpolation color space
				Some(mixed.to_string())
			};

			if let Some(candidate) = candidate
				&& candidate.len() < outer_len
			{
				self.transformer.replace_parsed::<Color>(outer_span, &candidate);
				self.replacing_outer = true;
				return;
			}
		}

		// Partial optimizations (only when not replacing the entire expression)

		// Remove redundant 50% percentages — only when both effective percentages are 50%
		// (i.e. the sum is 100, so no alpha multiplier effect)
		if sum == 100.0 {
			if p1 == 50.0
				&& let Some(ref pct) = mix.first.percentage
			{
				self.transformer.delete(pct.to_span());
			}
			if p2 == 50.0
				&& let Some(ref pct) = mix.second.percentage
			{
				self.transformer.delete(pct.to_span());
			}
		}

		// Remove redundant "shorter hue" (shorter is the default hue interpolation direction)
		if let InterpolationColorSpace::Polar(_, Some(ref hue_method)) = mix.interpolation.color_space
			&& matches!(hue_method.direction, HueInterpolationDirection::Shorter(_))
		{
			self.transformer.delete(hue_method.to_span());
		}
	}

	fn exit_color_mix_function<'b>(&mut self, _mix: &ColorMixFunction<'b>) {
		self.replacing_outer = false;
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn reduces_full_length_hex() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: #ffffff; }",
			"body { color: #fff; }"
		);
	}

	#[test]
	fn prefers_shorthand_hex_over_keyword() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: #000000; }",
			"body { color: #000; }"
		);
	}

	#[test]
	fn prefers_named_over_rgb() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: rgb(210, 180, 140); }",
			"body { color: tan; }"
		);
	}

	#[test]
	fn shortens_alpha_hex() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: rgba(255, 0, 0, 0.5); }",
			"body { color: #ff000080; }"
		);
	}

	#[test]
	fn no_transform_when_already_short() {
		assert_no_transform!(CssMinifierFeature::ReduceColors, CssAtomSet, StyleSheet, "body { color: red; }");
	}

	#[test]
	fn no_transform_for_currentcolor() {
		assert_no_transform!(CssMinifierFeature::ReduceColors, CssAtomSet, StyleSheet, "body { color: currentcolor; }");
	}

	#[test]
	fn reduces_color_srgb_function() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color(srgb 1 0 0); }",
			"a { color: red; }"
		);
	}

	#[test]
	fn reduces_color_display_p3() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color(display-p3 0.5 0.5 0.5); }",
			"a { color: gray; }"
		);
	}

	#[test]
	fn no_transform_for_out_of_gamut_display_p3() {
		assert_no_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color(display-p3 1 0 0); }"
		);
	}
	#[test]

	fn color_mix_100_percent_first() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 100%, blue); }",
			"a { color: red; }"
		);
	}

	#[test]
	fn color_mix_0_percent_first() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 0%, blue); }",
			"a { color: #00f; }"
		);
	}

	#[test]
	fn color_mix_same_color_both_sides() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red, red); }",
			"a { color: red; }"
		);
	}

	#[test]
	fn color_mix_removes_redundant_50_50() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, currentcolor 50%, red 50%); }",
			"a { color: color-mix(in srgb, currentcolor, red); }"
		);
	}

	#[test]
	fn color_mix_removes_single_redundant_50() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, currentcolor 50%, red); }",
			"a { color: color-mix(in srgb, currentcolor, red); }"
		);
	}

	#[test]
	fn color_mix_removes_shorter_hue() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch shorter hue, currentcolor, red); }",
			"a { color: color-mix(in oklch, currentcolor, red); }"
		);
	}

	#[test]
	fn color_mix_keeps_longer_hue() {
		assert_no_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch longer hue,currentcolor,red); }"
		);
	}

	#[test]
	fn color_mix_no_transform_when_already_compact() {
		assert_no_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch longer hue,currentcolor,red); }"
		);
	}

	#[test]
	fn color_mix_minifies_inner_colors() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch, rgba(255, 255, 255, 1), currentcolor); }",
			"a { color: color-mix(in oklch, #fff, currentcolor); }"
		);
	}

	#[test]
	fn color_mix_minifies_inner_rgb_to_named() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, hsl(0, 100%, 50%), currentcolor); }",
			"a { color: color-mix(in srgb, red, currentcolor); }"
		);
	}

	#[test]
	fn color_mix_mixes_static_colors() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red, blue); }",
			"a { color: purple; }"
		);
	}

	#[test]
	fn color_mix_normalizes_percentages_over_100() {
		// 80% + 40% = 120%, normalizes to 66.67%/33.33%, giving rgb(170, 0, 85) = #a05
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 80%, blue 40%); }",
			"a { color: #a05; }"
		);
	}

	#[test]
	fn color_mix_alpha_multiplier_under_100() {
		// 30% + 30% = 60%, alpha_mult = 0.6, result is semi-transparent purple
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 30%, blue 30%); }",
			"a { color: #80008099; }"
		);
	}

	#[test]
	fn color_mix_no_100_shortcircuit_when_both_explicit() {
		// red 100% + blue 50% sums to 150%, must normalize to 67/33 — not short-circuit to red
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 100%, blue 50%); }",
			"a { color: #a05; }"
		);
	}

	#[test]
	fn color_mix_oklch_out_of_gamut_uses_native_space() {
		// oklch mix of lime+blue is out of sRGB gamut — resolved to oklch(), not hex
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch, lime, blue); }",
			"a { color: oklch(0.66 0.304 203.27); }"
		);
	}
}
