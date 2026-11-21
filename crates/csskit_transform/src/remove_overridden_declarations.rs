use crate::prelude::*;
use css_ast::{
	CssAtomSet, DeclarationValue, NestedGroupRule, NodeId, Shorthand, StyleRule, StyleValue, VisitMut, VisitNode,
};
use css_parse::{AtomSet, Cursor, Declaration};

/// Removes declarations which a later declaration of the same block fully overrides, for example the `margin-top` of
/// `margin-top: 1px; margin: 2px`.
///
/// A declaration is overridden when every property it sets is also set, or reset, by a later declaration of equal or
/// greater importance.
pub struct RemoveOverriddenDeclarations<'a, 'ctx, N: NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
	overriders: Vec<Overrider>,
	overridden: Vec<usize>,
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
	N: NodeWithMetadata<CssMetadata>,
{
	fn can_override<'s>(declaration: &Declaration<'s, StyleValue<'s>, CssMetadata>) -> bool {
		!declaration.is_unknown() && !declaration.metadata().has_substitution()
	}

	fn may_contain_overrider(metadata: &CssMetadata) -> bool {
		metadata.has_shorthands() || metadata.has_css_wide_keywords()
	}

	fn property<'s>(declaration: &Declaration<'s, StyleValue<'s>, CssMetadata>) -> Property {
		let cursor: Cursor = declaration.name.into();
		let name = CssAtomSet::from_bits(cursor.token().atom_bits());
		let metadata = declaration.value.property_metadata();
		let not_shorthand =
			!metadata.has_shorthands() && (metadata.has_longhands() || metadata.has_custom_properties());
		let shorthand = if not_shorthand { None } else { StyleValue::shorthand_by_name(name) };
		Property { name, important: declaration.important.is_some(), shorthand }
	}

	fn overrider<'s>(index: usize, declaration: &Declaration<'s, StyleValue<'s>, CssMetadata>) -> Option<Overrider> {
		if !Self::may_contain_overrider(&declaration.value.property_metadata()) || !Self::can_override(declaration) {
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

	fn find_overridden<'b, 's, I>(&mut self, declarations: I)
	where
		I: Clone + Iterator<Item = &'b Declaration<'s, StyleValue<'s>, CssMetadata>>,
		's: 'b,
	{
		self.overriders.clear();
		self.overridden.clear();
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
				self.overridden.push(index);
			}
		}
		if !self.overridden.is_empty() {
			self.transformer.mark_changed();
		}
	}

	fn is_overridden(&self, index: usize) -> bool {
		self.overridden.binary_search(&index).is_ok()
	}
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for RemoveOverriddenDeclarations<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!Self::may_contain_overrider(metadata)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer, overriders: Vec::new(), overridden: Vec::new() }
	}
}

#[visitor]
impl<'a, 'ctx, N> VisitMut<'a> for RemoveOverriddenDeclarations<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.node_id == Some(NodeId::SelectorList)
			|| node.rule_subtree_metadata().is_some_and(|meta| Self::skips_subtree(&meta))
		{
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn enter_declaration<T: DeclarationValue<'a, CssMetadata>>(
		&mut self,
		_declaration: &mut Declaration<'a, T, CssMetadata>,
	) -> VisitFlow {
		VisitFlow::SKIP_CHILDREN
	}

	fn exit_style_rule(&mut self, rule: &mut StyleRule<'a>) {
		self.find_overridden(rule.rule.block.declarations.iter());
		if !self.overridden.is_empty() {
			let mut index = 0;
			rule.rule.block.declarations.retain(|_| {
				let keep = !self.is_overridden(index);
				index += 1;
				keep
			});
		}
		for nested_rule in rule.rule.block.rules.iter_mut() {
			if let NestedGroupRule::Declarations(group) = nested_rule {
				let declarations = || {
					group.declarations.iter().filter_map(|item| match item {
						css_parse::DeclarationOrBad::Declaration(declaration) => Some(declaration),
						css_parse::DeclarationOrBad::Bad(_) => None,
					})
				};
				self.find_overridden(declarations());
				if self.overridden.is_empty() {
					continue;
				}
				let mut index = 0;
				group.declarations.retain(|item| match item {
					css_parse::DeclarationOrBad::Declaration(_) => {
						let keep = !self.is_overridden(index);
						index += 1;
						keep
					}
					css_parse::DeclarationOrBad::Bad(_) => true,
				});
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
	fn removes_substituted_declarations_before_overriding_shorthands() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"b { border-top: var(--x); border: 0; }",
			"b { border: 0; }"
		);
	}

	#[test]
	fn removes_declarations_overridden_by_keyword_shorthands() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px; margin: inherit; }",
			"a { margin: inherit; }"
		);
	}

	#[test]
	fn removes_declarations_overridden_by_math_shorthands() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"a { margin-top: 1px; margin: calc(1px + 2px); }",
			"a { margin: calc(1px + 2px); }"
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
	fn removes_overridden_declarations_in_at_rules() {
		assert_transform!(
			CssMinifierFeature::RemoveOverriddenDeclarations,
			CssAtomSet,
			StyleSheet,
			"@media print { a { margin-top: 1px; margin: 2px; } }",
			"@media print { a { margin: 2px; } }"
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
			"a { margin-top: 1px; margin: var(--margin); } b { margin-top: 1px; margin: attr(data-margin type(<length>)); }"
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
