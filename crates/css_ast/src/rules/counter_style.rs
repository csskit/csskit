use super::prelude::*;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};
use crate::{Computed, Inherits, PropertyGroup, Todo};
use csskit_derives::{DeclarationMetadata, IntoCursor, Parse, Peek, SemanticEq, ToCursors, ToSpan};
use csskit_proc_macro::syntax;

/// <https://drafts.csswg.org/css-counter-styles-3/#the-counter-style-rule>
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip), queryable(skip))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CounterStyleRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::CounterStyle)]
	pub name: T![AtKeyword],
	pub prelude: CounterStyleName,
	pub block: CounterStyleRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for CounterStyleRule<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata {
			used_at_rules: AtRuleId::CounterStyle,
			node_kinds: NodeKinds::AtRule,
			property_kinds: PropertyKind::Name,
			..Default::default()
		}
	}

	fn metadata(&self) -> CssMetadata {
		self.block.0.metadata().merge(self.self_metadata())
	}
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for CounterStyleRule<'a> {
	const NODE_ID: NodeId = NodeId::CounterStyleRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.prelude.into()),
			_ => None,
		}
	}
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CounterStyleName(T![Ident]);

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CounterStyleRuleBlock<'a>(DeclarationList<'a, CounterStyleRuleStyleValue<'a>, CssMetadata>);

#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
pub enum CounterStyleRuleStyleValue<'a> {
	Unknown(ComponentValues<'a>),
	System(SystemStyleValue),
	Symbols(SymbolsStyleValue<'a>),
	AdditiveSymbols(AdditiveSymbolsStyleValue<'a>),
	Negative(NegativeStyleValue<'a>),
	Prefix(PrefixStyleValue<'a>),
	Suffix(SuffixStyleValue<'a>),
	Range(RangeStyleValue),
	Pad(PadStyleValue<'a>),
	SpeakAs(SpeakAsStyleValue),
	Fallback(FallbackStyleValue),
}

impl<'a> NodeWithMetadata<CssMetadata> for CounterStyleRuleStyleValue<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata { node_kinds: NodeKinds::Declaration, property_kinds: PropertyKind::Name, ..Default::default() }
	}

	fn metadata(&self) -> CssMetadata {
		self.self_metadata()
	}
}

impl<'a> DeclarationValue<'a, CssMetadata> for CounterStyleRuleStyleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::InitialValue | CssAtomSet::Inherits | CssAtomSet::Syntax)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn is_initial(&self) -> bool {
		false
	}

	fn is_inherit(&self) -> bool {
		false
	}

	fn is_unset(&self) -> bool {
		false
	}

	fn is_revert(&self) -> bool {
		false
	}

	fn is_revert_layer(&self) -> bool {
		false
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn parse_declaration_value<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::System => Self::System(p.parse::<SystemStyleValue>()?),
			CssAtomSet::Symbols => Self::Symbols(p.parse::<SymbolsStyleValue<'a>>()?),
			CssAtomSet::AdditiveSymbols => Self::AdditiveSymbols(p.parse::<AdditiveSymbolsStyleValue<'a>>()?),
			CssAtomSet::Negative => Self::Negative(p.parse::<NegativeStyleValue<'a>>()?),
			CssAtomSet::Prefix => Self::Prefix(p.parse::<PrefixStyleValue<'a>>()?),
			CssAtomSet::Suffix => Self::Suffix(p.parse::<SuffixStyleValue<'a>>()?),
			CssAtomSet::Range => Self::Range(p.parse::<RangeStyleValue>()?),
			CssAtomSet::Pad => Self::Pad(p.parse::<PadStyleValue<'a>>()?),
			CssAtomSet::SpeakAs => Self::SpeakAs(p.parse::<SpeakAsStyleValue>()?),
			CssAtomSet::Fallback => Self::Fallback(p.parse::<FallbackStyleValue>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

#[syntax("	[ <integer [0,∞]> && <symbol> ]# ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct AdditiveSymbolsStyleValue<'a>;

#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct FallbackStyleValue(CounterStyleName);

#[syntax(" <symbol> <symbol>? ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct NegativeStyleValue<'a>;

#[syntax(" <integer [0,∞]> && <symbol> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct PadStyleValue<'a>;

#[syntax(" <symbol> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct PrefixStyleValue<'a>;

// #[syntax(" [ [ <integer> | infinite ]{2} ]# | auto ")]
// #[derive(Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
// #[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
// pub struct RangeStyleValue<'a>;
pub type RangeStyleValue = Todo;

#[syntax(" auto | bullets | numbers | words | spell-out | <counter-style-name> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum SpeakAsStyleValue {}

#[syntax(" <symbol> ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct SuffixStyleValue<'a>;

#[syntax(" <symbol>+ ")]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct SymbolsStyleValue<'a>;

#[syntax(
	" cyclic | numeric | alphabetic | symbolic | additive | [fixed <integer>?] | [ extends <counter-style-name> ] "
)]
#[derive(
	Peek, Parse, ToCursors, ToSpan, DeclarationMetadata, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[declaration_metadata(initial = "n/a", inherits = False, property_group = CounterStyle)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum SystemStyleValue {}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CounterStyleRule>(), 112);
		assert_eq!(std::mem::size_of::<CounterStyleName>(), 12);
		assert_eq!(std::mem::size_of::<CounterStyleRuleBlock>(), 88);
		assert_eq!(std::mem::size_of::<CounterStyleRuleStyleValue>(), 416);
		assert_eq!(std::mem::size_of::<AdditiveSymbolsStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<FallbackStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<NegativeStyleValue>(), 416);
		assert_eq!(std::mem::size_of::<PadStyleValue>(), 224);
		assert_eq!(std::mem::size_of::<PrefixStyleValue>(), 208);
		// assert_eq!(std::mem::size_of::<RangeStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<SpeakAsStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<SuffixStyleValue>(), 208);
		assert_eq!(std::mem::size_of::<SymbolsStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<SystemStyleValue>(), 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CounterStyleRule, "@counter-style thumbs {}");
	}
}
