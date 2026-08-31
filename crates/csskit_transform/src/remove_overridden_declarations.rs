use crate::prelude::*;
use css_ast::{CssAtomSet, NestedGroupRule, StyleRule, StyleValue, VisitNode, Visitable};
use css_parse::{AtomSet, Cursor, Declaration};

/// Removes declarations which a later declaration of the same block fully overrides, for example the `margin-top` of
/// `margin-top: 1px; margin: 2px`.
///
/// A declaration is overridden when every property it sets is also set, or reset, by a later declaration of equal or
/// greater importance.
pub struct RemoveOverriddenDeclarations<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> RemoveOverriddenDeclarations<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn delete<T: ToSpan>(&self, value: &T) {
		let span = value.to_span();
		self.transformer.clear_pending_edits(span);
		self.transformer.delete(span);
	}

	fn property_name<'s>(declaration: &Declaration<'s, StyleValue<'s>, CssMetadata>) -> CssAtomSet {
		let cursor: Cursor = declaration.name.into();
		CssAtomSet::from_bits(cursor.token().atom_bits())
	}

	fn later_declaration_overrides<'s>(
		earlier: &Declaration<'s, StyleValue<'s>, CssMetadata>,
		later: &Declaration<'s, StyleValue<'s>, CssMetadata>,
	) -> bool {
		if earlier.important.is_some() && later.important.is_none()
			|| later.is_unknown()
			|| later.metadata().has_substitution()
		{
			return false;
		}
		let earlier_name = Self::property_name(earlier);
		let later_name = Self::property_name(later);
		if earlier_name == later_name {
			return false;
		}
		if let Some(later) = StyleValue::shorthand_by_name(later_name) {
			let covered = match StyleValue::shorthand_by_name(earlier_name) {
				// `all` expresses no property of its own, and the properties it resets are not recorded,
				// so no later declaration is known to cover it.
				Some(earlier) if earlier.longhands.is_empty() => false,
				Some(earlier) => earlier.longhands.iter().all(|longhand| later.longhands.contains(longhand)),
				None => later.longhands.contains(&earlier_name),
			};
			if covered {
				return true;
			}
		}
		StyleValue::longhand_by_name(earlier_name).is_some_and(|earlier| earlier.reset_by.contains(&later_name))
	}

	fn remove_overridden<'b, 's, I>(&self, declarations: I)
	where
		I: Clone + Iterator<Item = &'b Declaration<'s, StyleValue<'s>, CssMetadata>>,
		's: 'b,
	{
		let mut declarations = declarations;
		while let Some(declaration) = declarations.next() {
			if declaration.is_unknown() {
				continue;
			}
			if declarations.clone().any(|later| Self::later_declaration_overrides(declaration, later)) {
				self.delete(declaration);
			}
		}
	}
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for RemoveOverriddenDeclarations<'a, 'ctx, N>
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
impl<'a, 'ctx, N> Visit for RemoveOverriddenDeclarations<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn exit_style_rule(&mut self, rule: &StyleRule) {
		self.remove_overridden(rule.rule.block.declarations.iter());
		for nested_rule in &rule.rule.block.rules {
			if let NestedGroupRule::Declarations(group) = nested_rule {
				let declarations = || {
					group.declarations.iter().filter_map(|item| match item {
						css_parse::DeclarationOrBad::Declaration(declaration) => Some(declaration),
						css_parse::DeclarationOrBad::Bad(_) => None,
					})
				};
				self.remove_overridden(declarations());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn removes_declarations_overridden_by_later_shorthands() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px; margin: 2px; border-image: url(border.png) 30; border: 1px solid; font-weight: bold; font: 16px serif; }",
			"a { margin: 2px; border: 1px solid; font: 16px serif; }"
		);
	}

	#[test]
	fn keeps_important_declarations_before_normal_shorthands() {
		assert_no_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px !important; margin: 2px; }"
		);
	}

	#[test]
	fn keeps_declarations_before_substituted_shorthands() {
		assert_no_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px; margin: var(--margin); }"
		);
	}

	#[test]
	fn keeps_declarations_a_later_shorthand_does_not_cover() {
		assert_no_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px; padding: 2px; }"
		);
	}
}
