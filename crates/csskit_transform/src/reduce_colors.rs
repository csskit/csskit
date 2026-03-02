use crate::prelude::*;
use chromashift::{ColorSpace, Hex, Named, Srgb};
use css_ast::{Color, ToChromashift, Visitable};

pub struct ReduceColors<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceColors<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn may_change(features: CssMinifierFeature, _node: &N) -> bool {
		features.contains(CssMinifierFeature::ReduceColors)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

impl<'a, 'ctx, N> Visit for ReduceColors<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn visit_color(&mut self, color: &Color) {
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

		let Some(candidate) = [
			Some(Hex::from(Srgb::from(chroma_color)).to_string()),
			Named::try_from(chroma_color).ok().map(|named| named.to_string()),
			Some(Srgb::from(chroma_color).to_string()),
		]
		.into_iter()
		.flatten()
		.min_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b))) else {
			return;
		};

		if candidate.len() < len {
			self.transformer.replace_parsed::<Color>(color.to_span(), &candidate);
		}
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
}
