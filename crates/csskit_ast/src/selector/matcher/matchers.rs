use super::context::MatchContext;
use crate::{
	CsskitAtomSet, QueryAttribute, QueryFunctionalPseudoClass, QueryNthPseudo, QueryPrefixedPseudo,
	QueryPropertyTypePseudo, QueryPseudoClass, QuerySelectorComponent, QueryType, QueryWildcard,
};
use css_ast::NodeKinds;
use css_parse::{AtomSet, Cursor};

/// Trait for matching selector components against a node context.
pub(crate) trait Matcher<'a, 'b> {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool;
}

impl<'a, 'b> Matcher<'a, 'b> for QueryType {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		self.node_id(ctx.query_str).is_some_and(|expected| ctx.node.node_id == expected)
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QueryAttribute {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		ctx.node.properties.matches_attribute(self, ctx.query_str, ctx.source)
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QueryPseudoClass {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		let meta = &ctx.node.metadata;
		match self {
			QueryPseudoClass::Important(..) => meta.has_important(),
			QueryPseudoClass::Custom(..) => meta.has_custom_properties(),
			QueryPseudoClass::Computed(..) => meta.has_computed(),
			QueryPseudoClass::Shorthand(..) => meta.has_shorthands(),
			QueryPseudoClass::Longhand(..) => meta.has_longhands(),
			QueryPseudoClass::Unknown(..) => meta.has_unknown() || ctx.node.node_id.tag_name().contains("unknown"),
			QueryPseudoClass::AtRule(..) => meta.node_kinds.contains(NodeKinds::AtRule),
			QueryPseudoClass::Rule(..) => meta.node_kinds.intersects(NodeKinds::StyleRule | NodeKinds::AtRule),
			QueryPseudoClass::Function(..) => {
				meta.has_functions() || ctx.node.node_id.tag_name().ends_with("-function")
			}
			QueryPseudoClass::FirstChild(..) => ctx.sibling_index == 1,
			QueryPseudoClass::Empty(..) => meta.is_empty_container(),
			QueryPseudoClass::Nested(..) => ctx.is_nested,
			QueryPseudoClass::Root(..) => ctx.is_root,
			QueryPseudoClass::Prefixed(..) => {
				meta.single_vendor_prefix().is_some() || ctx.node.properties.vendor_prefix(ctx.source).is_some()
			}
			QueryPseudoClass::OnlyChild(..) => ctx.is_only_child(),
			QueryPseudoClass::LastChild(..) => ctx.is_last_child(),
			QueryPseudoClass::FirstOfType(..) => ctx.is_first_of_type(),
			QueryPseudoClass::LastOfType(..) => ctx.is_last_of_type(),
			QueryPseudoClass::OnlyOfType(..) => ctx.is_only_of_type(),
		}
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QueryWildcard {
	fn matches(&self, _ctx: &MatchContext<'a, 'b>) -> bool {
		true
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QueryNthPseudo {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		self.value.matches(ctx.sibling_index)
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QueryPropertyTypePseudo {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		self.property_group().is_some_and(|g| ctx.node.metadata.property_groups.contains(g))
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QueryPrefixedPseudo {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		self.vendor_prefix().is_some_and(|v| ctx.node.metadata.vendor_prefixes.contains(v))
			|| ctx.node.properties.vendor_prefix(ctx.source).is_some_and(|prefix| {
				CsskitAtomSet::from_str(prefix) == CsskitAtomSet::from_bits(Cursor::from(self.vendor).atom_bits())
			})
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QueryFunctionalPseudoClass<'b> {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		match self {
			// :not() and :has() require recursive selector matching - handled separately by caller.
			// Return true here so they don't reject the match on their own.
			QueryFunctionalPseudoClass::Not(_) => true,
			QueryFunctionalPseudoClass::Has(_) => true,
			QueryFunctionalPseudoClass::NthChild(nth) => nth.matches(ctx),
			QueryFunctionalPseudoClass::NthLastChild(nth) => nth.value.matches(ctx.index_from_end()),
			QueryFunctionalPseudoClass::NthOfType(nth) => nth.value.matches(ctx.type_index()),
			QueryFunctionalPseudoClass::NthLastOfType(nth) => nth.value.matches(ctx.type_index_from_end()),
			QueryFunctionalPseudoClass::PropertyType(pt) => pt.matches(ctx),
			QueryFunctionalPseudoClass::Prefixed(pf) => pf.matches(ctx),
			QueryFunctionalPseudoClass::Size(sz) => sz.matches(ctx.node.metadata.size),
		}
	}
}

impl<'a, 'b> Matcher<'a, 'b> for QuerySelectorComponent<'b> {
	fn matches(&self, ctx: &MatchContext<'a, 'b>) -> bool {
		match self {
			QuerySelectorComponent::Type(t) => t.matches(ctx),
			QuerySelectorComponent::Wildcard(w) => w.matches(ctx),
			QuerySelectorComponent::Combinator(_) => unreachable!("combinators should not appear in selector parts"),
			QuerySelectorComponent::Attribute(a) => a.matches(ctx),
			QuerySelectorComponent::PseudoClass(p) => p.matches(ctx),
			QuerySelectorComponent::FunctionalPseudoClass(p) => p.matches(ctx),
		}
	}
}
