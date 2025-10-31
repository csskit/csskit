use crate::{
	CssAtomSet, CssDiagnostic, CssMetadata, SelectorList, StyleValue, UnknownAtRule, UnknownQualifiedRule, rules,
};
use css_parse::{
	BadDeclaration, Cursor, Diagnostic, NodeWithMetadata, Parse, Parser, QualifiedRule, Result as ParserResult,
	RuleVariants,
};
use csskit_derives::{Parse, Peek, SemanticEq, ToCursors, ToSpan};

/// Represents a "Style Rule", such as `body { width: 100% }`. See also the CSS-OM [CSSStyleRule][1] interface.
///
/// The Style Rule is comprised of two child nodes: the [SelectorList] represents the selectors of the rule.
/// Each [Declaration][css_parse::Declaration] will have a [StyleValue], and each rule will be a [NestedGroupRule].
///
/// [1]: https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct StyleRule<'a> {
	pub rule: QualifiedRule<'a, SelectorList<'a>, StyleValue<'a>, NestedGroupRule<'a>, CssMetadata>,
}

impl<'a> NodeWithMetadata<CssMetadata> for StyleRule<'a> {
	fn metadata(&self) -> CssMetadata {
		self.rule.metadata()
	}
}

impl<'a> NodeWithMetadata<CssMetadata> for NestedGroupRule<'a> {
	fn metadata(&self) -> CssMetadata {
		match self {
			Self::Container(r) => r.metadata(),
			Self::Layer(r) => r.metadata(),
			Self::Media(r) => r.metadata(),
			Self::Scope(r) => r.metadata(),
			Self::Supports(r) => r.metadata(),
			Self::UnknownAt(r) => r.metadata(),
			Self::Style(r) => r.metadata(),
			Self::Unknown(r) => r.metadata(),
			Self::BadDeclaration(r) => r.metadata(),
		}
	}
}

// https://drafts.csswg.org/css-nesting/#conditionals
macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Container(ContainerRule<'a>): "container",
			Layer(LayerRule<'a>): "layer",
			Media(MediaRule<'a>): "media",
			Scope(ScopeRule): "scope",
			Supports(SupportsRule<'a>): "supports",
		}
	};
}

macro_rules! nested_group_rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		#[allow(clippy::large_enum_variant)] // TODO: Box?
		// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
		#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum NestedGroupRule<'a> {
			$(
				$name(rules::$ty$(<$a>)?),
			)+
			UnknownAt(UnknownAtRule<'a>),
			Style(StyleRule<'a>),
			Unknown(UnknownQualifiedRule<'a>),
			BadDeclaration(BadDeclaration<'a>),
		}
	}
}
apply_rules!(nested_group_rule);

impl<'a> RuleVariants<'a> for NestedGroupRule<'a> {
	fn parse_at_rule<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		macro_rules! parse_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
			)+ ) => {
				match p.to_atom::<CssAtomSet>(name) {
					$(CssAtomSet::$name => p.parse::<rules::$ty>().map(Self::$name),)+
					_ => Err(Diagnostic::new(name.into(), Diagnostic::unexpected_at_rule))?,
				}
			}
		}
		apply_rules!(parse_rule)
	}

	fn parse_unknown_at_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<UnknownAtRule>().map(Self::UnknownAt)
	}

	fn parse_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<StyleRule>().map(Self::Style)
	}

	fn parse_unknown_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<UnknownQualifiedRule>().map(Self::Unknown)
	}

	fn bad_declaration(b: BadDeclaration<'a>) -> Option<Self> {
		Some(Self::BadDeclaration(b))
	}
}

impl<'a> Parse<'a> for NestedGroupRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_rule_variants(p)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StyleRule>(), 176);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body{}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body,body{}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body{width:1px;}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, "body{opacity:0;}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo *{}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":nth-child(1){opacity:0;}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo{--bar:(baz);}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo{width: calc(1px + (var(--foo)) + 1px);}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ".foo{--bar:1}");
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":root{--custom:{width:0;height:0;};}");
		// Semicolons are "allowed" in geneirc preludes
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":root{a;b{}}");
		// Bad Declarations should be parsable.
		assert_parse!(CssAtomSet::ATOMS, StyleRule, ":root{$(var)-size: 100%;}");
	}
}
