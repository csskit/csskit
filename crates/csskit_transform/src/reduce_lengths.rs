use crate::prelude::*;
use css_ast::{Length, Visitable};

pub struct ReduceLengths<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceLengths<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn may_change(features: CssMinifierFeature, _node: &N) -> bool {
		features.contains(CssMinifierFeature::ReduceLengths)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

impl<'a, 'ctx, N> Visit for ReduceLengths<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn visit_length(&mut self, length: &Length) {
		enum ResolvedType {
			UnitlessZero,
			UnitedZero,
			Resolved(f32),
			Unresolved,
		}

		let resolved = match length {
			Length::Zero(_) => ResolvedType::UnitlessZero,
			_ if Into::<f32>::into(*length) == 0.0 => ResolvedType::UnitedZero,
			_ => {
				if let Some(px) = length.to_px() {
					ResolvedType::Resolved(px)
				} else {
					ResolvedType::Unresolved
				}
			}
		};

		if matches!(resolved, ResolvedType::UnitedZero | ResolvedType::Resolved(0.0)) {
			self.transformer.replace(length, self.transformer.parse_value::<Length>("0"));
		} else if let ResolvedType::Resolved(px) = resolved {
			let replacement = bumpalo::format!(in self.transformer.bump(), "{}px", px);
			let original_span = length.to_span();
			let original_len = (original_span.end().0 - original_span.start().0) as usize;
			if replacement.len() <= original_len {
				self.transformer.replace(length, self.transformer.parse_value::<Length>(replacement.into_bump_str()));
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn test_reduce_zero_lengths() {
		assert_transform!(
			CssMinifierFeature::ReduceLengths,
			CssAtomSet,
			StyleSheet,
			"body { width: 0px; height: 0rem; margin: 0em; }",
			"body { width: 0; height: 0; margin: 0; }"
		);
	}

	#[test]
	fn test_length_shortening_guard() {
		assert_transform!(
			CssMinifierFeature::ReduceLengths,
			CssAtomSet,
			StyleSheet,
			"div { font-size: 12pt; }",
			"div { font-size: 16px; }"
		);
	}

	#[test]
	fn test_length_noop() {
		assert_no_transform!(CssMinifierFeature::ReduceLengths, CssAtomSet, StyleSheet, "body { width: 10rem; }");
	}
}
