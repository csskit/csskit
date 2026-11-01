use crate::prelude::*;
use css_ast::{
	CssAtomSet, DeclarationValue, NestedGroupRule, SelectorList, Shorthand, StyleRule, StyleValue, VisitNode, Visitable,
};
use css_parse::{AtomSet, Cursor, Declaration};

/// Removes declarations which a later declaration of the same block fully overrides, for example the `margin-top` of
/// `margin-top: 1px; margin: 2px`.
///
/// A declaration is overridden when every property it sets is also set, or reset, by a later declaration of equal or
/// greater importance.
pub struct RemoveOverriddenDeclarations<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
	overriders: Vec<Overrider>,
}

struct Property {
	name: CssAtomSet,
	important: bool,
	shorthand: Option<&'static Shorthand>,
}

struct Overrider {
	index: usize,
	name: CssAtomSet,
	important: bool,
	shorthand: &'static Shorthand,
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

	fn can_override<'s>(declaration: &Declaration<'s, StyleValue<'s>, CssMetadata>) -> bool {
		!declaration.is_unknown() && !declaration.metadata().has_substitution()
	}

	fn may_contain_overrider(metadata: &CssMetadata) -> bool {
		metadata.has_shorthands() || metadata.has_css_wide_keywords()
	}

	fn property<'s>(declaration: &Declaration<'s, StyleValue<'s>, CssMetadata>) -> Property {
		let cursor: Cursor = declaration.name.into();
		let name = CssAtomSet::from_bits(cursor.token().atom_bits());
		let metadata = declaration.metadata();
		let not_shorthand =
			!metadata.has_shorthands() && (metadata.has_longhands() || metadata.has_custom_properties());
		let shorthand = if not_shorthand { None } else { StyleValue::shorthand_by_name(name) };
		Property { name, important: declaration.important.is_some(), shorthand }
	}

	fn overrider<'s>(index: usize, declaration: &Declaration<'s, StyleValue<'s>, CssMetadata>) -> Option<Overrider> {
		if !Self::may_contain_overrider(&declaration.metadata()) || !Self::can_override(declaration) {
			return None;
		}
		let Property { name, important, shorthand } = Self::property(declaration);
		Some(Overrider { index, name, important, shorthand: shorthand? })
	}

	fn later_property_overrides(earlier: &Property, earlier_reset_by: &[CssAtomSet], later: &Overrider) -> bool {
		if earlier.important && !later.important || earlier.name == later.name {
			return false;
		}
		let covered = match earlier.shorthand {
			// `all` expresses no property of its own, and the properties it resets are not recorded,
			// so no later declaration is known to cover it.
			Some(earlier) if earlier.longhands.is_empty() => false,
			Some(earlier) => earlier.longhands.iter().all(|longhand| later.shorthand.longhands.contains(longhand)),
			None => later.shorthand.longhands.contains(&earlier.name),
		};
		covered || earlier_reset_by.contains(&later.name)
	}

	fn remove_overridden<'b, 's, I>(&mut self, declarations: I)
	where
		I: Clone + Iterator<Item = &'b Declaration<'s, StyleValue<'s>, CssMetadata>>,
		's: 'b,
	{
		self.overriders.clear();
		for (index, declaration) in declarations.clone().enumerate().skip(1) {
			let Some(overrider) = Self::overrider(index, declaration) else {
				continue;
			};
			if let Some(earlier) = self
				.overriders
				.iter()
				.position(|earlier| earlier.name == overrider.name && earlier.important == overrider.important)
			{
				self.overriders.remove(earlier);
			}
			self.overriders.push(overrider);
		}
		let Some(last) = self.overriders.last().map(|later| later.index) else {
			return;
		};
		for (index, declaration) in declarations.enumerate().take(last) {
			if declaration.is_unknown() {
				continue;
			}
			let earlier = Self::property(declaration);
			let earlier_reset_by =
				StyleValue::longhand_by_name(earlier.name).map_or(&[][..], |longhand| longhand.reset_by);
			let later = &self.overriders[self.overriders.partition_point(|later| later.index <= index)..];
			if later.iter().any(|later| Self::later_property_overrides(&earlier, earlier_reset_by, later)) {
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
		Self { transformer, overriders: Vec::new() }
	}
}

#[visitor]
impl<'a, 'ctx, N> Visit for RemoveOverriddenDeclarations<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn visit_selector_list(&mut self, _list: &SelectorList) -> VisitFlow {
		VisitFlow::SKIP_CHILDREN
	}

	fn enter_declaration<'b, T: DeclarationValue<'b, CssMetadata>>(
		&mut self,
		_declaration: &Declaration<'b, T, CssMetadata>,
		_query: VisitNode,
	) -> VisitFlow {
		VisitFlow::SKIP_CHILDREN
	}

	fn visit_style_rule(&mut self, rule: &StyleRule) -> VisitFlow {
		if !Self::may_contain_overrider(&rule.rule.block.meta) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn exit_style_rule(&mut self, rule: &StyleRule) {
		if !Self::may_contain_overrider(&rule.rule.block.meta) {
			return;
		}
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
	fn keeps_declarations_after_the_last_overriding_shorthand() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { --a: 1px; margin-top: 1px; color: red; margin: 2px; margin-top: 3px; padding: 4px; padding-top: 5px; }",
			"a { --a: 1px; color: red; margin: 2px; margin-top: 3px; padding: 4px; padding-top: 5px; }"
		);
	}

	#[test]
	fn removes_declarations_before_repeated_overriding_shorthands() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px; margin: 0; margin-top: 2px; margin: 1px; margin-left: 3px; }",
			"a { margin: 0; margin: 1px; margin-left: 3px; }"
		);
	}

	#[test]
	fn removes_declarations_before_repeated_shorthands_of_their_importance() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px !important; margin: 0 !important; margin-top: 2px !important; margin: 3px; }",
			"a { margin: 0 !important; margin-top: 2px !important; margin: 3px; }"
		);
	}

	#[test]
	fn removes_declarations_overridden_by_keyword_shorthands_or_overriding_substituted_shorthands() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px; margin: inherit; } b { border-top: var(--x); border: 0; }",
			"a { margin: inherit; } b { border: 0; }"
		);
	}

	#[test]
	fn removes_overridden_declarations_in_rules_nested_in_rules_without_shorthands() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { color: red; & b { margin-top: 1px; margin: 2px; } }",
			"a { color: red; & b { margin: 2px; } }"
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
