use crate::CssAtomSet;
use css_lexer::Kind;
use css_parse::{CommaSeparated, Diagnostic, Parse, Parser, Peek, Result as ParserResult, T};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

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
		#[derive( ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum FunctionalPseudoClass<'a> {
			$($ident($ty),)+
		}
	}
}
apply_functional_pseudo_class!(define_functional_pseudo_class);

impl<'a> Peek<'a> for FunctionalPseudoClass<'a> {
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		<T![:]>::peek(p, c) && p.peek_n(2) == Kind::Function
	}
}

impl<'a> Parse<'a> for FunctionalPseudoClass<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		macro_rules! match_keyword {
			( $($ident: ident($ty: ty) $pat: pat $(,)*)+ ) => {
				match p.to_atom::<CssAtomSet>(p.peek_n(2)) {
					$($pat => p.parse::<$ty>().map(Self::$ident),)+
					_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
				}
			}
		}
		apply_functional_pseudo_class!(match_keyword)
	}
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct DirPseudoFunction {
	pub colon: T![:],
	#[atom(CssAtomSet::Dir)]
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum DirValue {
	#[atom(CssAtomSet::Rtl)]
	Rtl(T![Ident]),
	#[atom(CssAtomSet::Ltr)]
	Ltr(T![Ident]),
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct HasPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::Has)]
	pub function: T![Function],
	pub value: RelativeSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct HostPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::Host)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct HostContextPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::HostContext)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct IsPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::Is)]
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LangPseudoFunction<'a> {
	pub colon: T![:],
	#[atom(CssAtomSet::Lang)]
	pub function: T![Function],
	pub value: LangValues<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToSpan, Parse, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LangValues<'a>(pub CommaSeparated<'a, LangValue>);

#[derive(Parse, ToSpan, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LangValue {
	Ident(T![Ident]),
	String(T![String]),
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NotPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::Not)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthChildPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::NthChild)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthColPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::NthCol)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthLastChildPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::NthLastChild)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthLastColPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::NthLastCol)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthLastOfTypePseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::NthLastOfType)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthOfTypePseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::NthOfType)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct WherePseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	#[atom(CssAtomSet::Where)]
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct StatePseudoFunction {
	pub colon: T![:],
	#[atom(CssAtomSet::State)]
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
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

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FunctionalPseudoClass>(), 104);
		assert_eq!(std::mem::size_of::<DirValue>(), 16);
	}
}
