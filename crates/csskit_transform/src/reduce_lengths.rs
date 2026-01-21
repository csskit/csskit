use crate::prelude::*;
use css_ast::{DeclarationValue, Length, QueryableNode, UnitlessZeroResolves, Visitable};
use css_parse::Declaration;
use std::cell::Cell;

pub struct ReduceLengths<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
	/// Tracks how unitless zero resolves in the current declaration context. When visiting inside a declaration where
	/// unitless zero resolves to Number, we skip the converting zero lengths to unitless zero.
	unitless_zero_resolves: Cell<UnitlessZeroResolves>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceLengths<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn may_change(features: CssMinifierFeature, _node: &N) -> bool {
		features.contains(CssMinifierFeature::ReduceLengths)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer, unitless_zero_resolves: Cell::new(UnitlessZeroResolves::Length) }
	}
}

impl<'a, 'ctx, N> Visit for ReduceLengths<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn visit_declaration<'b, T: DeclarationValue<'b, CssMetadata> + QueryableNode>(
		&mut self,
		decl: &Declaration<'b, T, CssMetadata>,
	) {
		self.unitless_zero_resolves.set(decl.metadata().unitless_zero_resolves);
	}

	fn exit_declaration<'b, T: DeclarationValue<'b, CssMetadata> + QueryableNode>(
		&mut self,
		_decl: &Declaration<'b, T, CssMetadata>,
	) {
		self.unitless_zero_resolves.set(UnitlessZeroResolves::Length);
	}

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

		let can_reduce_to_unitless = self.unitless_zero_resolves.get() == UnitlessZeroResolves::Length;

		if can_reduce_to_unitless && matches!(resolved, ResolvedType::UnitedZero | ResolvedType::Resolved(0.0)) {
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

	#[test]
	fn test_unitless_zero_resolves_to_number() {
		// line-height is not safe to reduce to `0` as they're semantically different.
		assert_no_transform!(CssMinifierFeature::ReduceLengths, CssAtomSet, StyleSheet, "body { line-height: 0px; }");
		// tab-size is not safe to reduce to `0` as they're semantically different.
		assert_no_transform!(CssMinifierFeature::ReduceLengths, CssAtomSet, StyleSheet, "body { tab-size: 0px; }");
		// calc is not safe to reduce to `0` as it changes the return type
		assert_no_transform!(
			CssMinifierFeature::ReduceLengths,
			CssAtomSet,
			StyleSheet,
			"body { width: calc(100px - 0px); }"
		);
	}

	#[test]
	fn test_unitless_zero_resolves_to_length() {
		assert_transform!(
			CssMinifierFeature::ReduceLengths,
			CssAtomSet,
			StyleSheet,
			"div { width: 0px; }",
			"div { width: 0; }"
		);
	}
}
