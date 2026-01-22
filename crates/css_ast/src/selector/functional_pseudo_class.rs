use crate::CssAtomSet;
use css_lexer::Kind;
use css_parse::{CommaSeparated, Cursor, Diagnostic, Parse, Parser, Peek, Result as ParserResult, State, T};
use csskit_derives::{Parse, Peek, SemanticEq, ToCursors, ToSpan};

use super::{ForgivingSelector, Nth, RelativeSelector, SelectorList};

macro_rules! apply_functional_pseudo_class {
	($macro: ident) => {
		$macro! {
			Dir(DirPseudoFunction) CssAtomSet::Dir,
			Has(HasPseudoFunction<'a>) CssAtomSet::Has,
			Heading(HeadingPseudoFunction<'a>) CssAtomSet::Heading,
			Host(HostPseudoFunction<'a>) CssAtomSet::Host,
			HostContext(HostContextPseudoFunction<'a>) CssAtomSet::HostContext,
			Is(IsPseudoFunction<'a>) CssAtomSet::Is,
			Lang(LangPseudoFunction<'a>) CssAtomSet::Lang,
			Not(NotPseudoFunction<'a>) CssAtomSet::Not,
			NthChild(NthChildPseudoFunction) CssAtomSet::NthChild,
			NthCol(NthColPseudoFunction) CssAtomSet::NthCol,
			NthLastChild(NthLastChildPseudoFunction) CssAtomSet::NthLastChild,
			NthLastCol(NthLastColPseudoFunction) CssAtomSet::NthLastCol,
			NthLastOfType(NthLastOfTypePseudoFunction) CssAtomSet::NthLastOfType,
			NthOfType(NthOfTypePseudoFunction) CssAtomSet::NthOfType,
			State(StatePseudoFunction) CssAtomSet::State,
			Where(WherePseudoFunction<'a>) CssAtomSet::Where,
		}
	};
}

macro_rules! define_functional_pseudo_class {
	( $($ident: ident($ty: ty) $pat: pat $(,)*)+ ) => {
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
		#[derive( ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum FunctionalPseudoClass<'a> {
			$($ident($ty),)+
		}
	}
}
apply_functional_pseudo_class!(define_functional_pseudo_class);

impl<'a> Peek<'a> for FunctionalPseudoClass<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: css_lexer::Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![:]>::peek(p, c) && p.peek_n(2) == Kind::Function
	}
}

impl<'a> Parse<'a> for FunctionalPseudoClass<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		macro_rules! match_keyword {
			( $($ident: ident($ty: ty) $pat: pat $(,)*)+ ) => {
				match p.to_atom::<CssAtomSet>(p.peek_n(2)) {
					CssAtomSet::Has if p.is(State::DisallowRelativeSelector) => {
						Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
					}
					$($pat => p.parse::<$ty>().map(Self::$ident),)+
					_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
				}
			}
		}
		apply_functional_pseudo_class!(match_keyword)
	}
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct DirPseudoFunction {
	pub colon: T![:],
	#[atom(CssAtomSet::Dir)]
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum DirValue {
	#[atom(CssAtomSet::Rtl)]
	Rtl(T![Ident]),
	#[atom(CssAtomSet::Ltr)]
	Ltr(T![Ident]),
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HasPseudoFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Has)]
	pub function: T![Function],
	#[parse(state = State::DisallowRelativeSelector)]
	pub value: RelativeSelector<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HostPseudoFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Host)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HostContextPseudoFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::HostContext)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct IsPseudoFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Is)]
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LangPseudoFunction<'a> {
	pub colon: T![:],
	#[atom(CssAtomSet::Lang)]
	pub function: T![Function],
	pub value: LangValues<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToSpan, Parse, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LangValues<'a>(pub CommaSeparated<'a, LangValue>);

#[derive(Parse, ToSpan, Peek, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LangValue {
	Ident(T![Ident]),
	String(T![String]),
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NotPseudoFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Not)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NthChildPseudoFunction {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::NthChild)]
	pub function: T![Function],
	pub value: Nth,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NthColPseudoFunction {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::NthCol)]
	pub function: T![Function],
	pub value: Nth,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NthLastChildPseudoFunction {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::NthLastChild)]
	pub function: T![Function],
	pub value: Nth,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NthLastColPseudoFunction {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::NthLastCol)]
	pub function: T![Function],
	pub value: Nth,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NthLastOfTypePseudoFunction {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::NthLastOfType)]
	pub function: T![Function],
	pub value: Nth,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NthOfTypePseudoFunction {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::NthOfType)]
	pub function: T![Function],
	pub value: Nth,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WherePseudoFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colon: T![:],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Where)]
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StatePseudoFunction {
	pub colon: T![:],
	#[atom(CssAtomSet::State)]
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HeadingPseudoFunction<'a> {
	pub colon: T![:],
	#[atom(CssAtomSet::Heading)]
	pub function: T![Function],
	pub value: CommaSeparated<'a, Nth>,
	pub close: Option<T![')']>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::selector::SelectorList;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FunctionalPseudoClass>(), 104);
		assert_eq!(std::mem::size_of::<DirValue>(), 16);
	}

	#[test]
	fn test_has_parses() {
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":has(.foo)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":has(> .foo)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":has(+ .sibling)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":has(:is(.a,.b))");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":has(:not(.foo))");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":has(:where(.foo))");
	}

	#[test]
	fn test_nested_has_disallowed() {
		// Nested :has() is invalid CSS - :has() cannot contain :has()
		assert_parse_error!(CssAtomSet::ATOMS, SelectorList, ":has(:has(.foo))");
		assert_parse_error!(CssAtomSet::ATOMS, SelectorList, ":has(:has(:has(.foo)))");
		assert_parse_error!(CssAtomSet::ATOMS, SelectorList, ":has(.bar :has(.foo))");
		assert_parse_error!(CssAtomSet::ATOMS, SelectorList, ":has(:is(:has(.foo)))");
		assert_parse_error!(CssAtomSet::ATOMS, SelectorList, ":has(:not(:has(.foo)))");
		assert_parse_error!(CssAtomSet::ATOMS, SelectorList, ":has(:where(:has(.foo)))");
	}
}
