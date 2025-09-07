use css_parse::{
	Build, CommaSeparated, Cursor, KindSet, Parse, Parser, Result as ParserResult, T, function_set, keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use super::{ForgivingSelector, Nth, RelativeSelector, SelectorList};

macro_rules! apply_functional_pseudo_class {
	($macro: ident) => {
		$macro! {
			Dir: "dir": DirPseudoFunction: DirValue,
			Has: "has": HasPseudoFunction<'a>: RelativeSelector,
			Heading: "heading": HeadingPseudoFunction<'a>: CommaSeparated<'a, Nth>,
			Host: "host": HostPseudoFunction<'a>: SelectorList,
			HostContext: "host-context": HostContextPseudoFunction<'a>: SelectorList,
			Is: "is": IsPseudoFunction<'a>: ForgivingSelector,
			Lang: "lang": LangPseudoFunction<'a>: LangValues,
			Not: "not": NotPseudoFunction<'a>: SelectorList,
			NthChild: "nth-child": NthChildPseudoFunction: Nth,
			NthCol: "nth-col": NthColPseudoFunction: Nth,
			NthLastChild: "nth-last-child": NthLastChildPseudoFunction: Nth,
			NthLastCol: "nth-last-col": NthLastColPseudoFunction: Nth,
			NthLastOfType: "nth-last-of-type": NthLastOfTypePseudoFunction: Nth,
			NthOfType: "nth-of-type": NthOfTypePseudoFunction: Nth,
			State: "state": StatePseudoFunction: T![Ident],
			Where: "where": WherePseudoFunction<'a>: ForgivingSelector,
		}
	};
}

macro_rules! define_functional_pseudo_class {
	( $($ident: ident: $str: tt: $ty: ty: $val_ty: ty $(,)*)+ ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
		#[cfg_attr(
			feature = "serde",
			derive(serde::Serialize),
			serde(tag = "type", content = "value", rename_all = "kebab-case")
		)]
		pub enum FunctionalPseudoClass<'a> {
			$($ident($ty),)+
		}
	}
}
apply_functional_pseudo_class!(define_functional_pseudo_class);

macro_rules! define_functional_pseudo_class_keyword {
	( $($ident: ident: $str: tt: $ty: ty: $val_ty: ty $(,)*)+ ) => {
		function_set!(
			pub enum FunctionalPseudoClassKeyword {
				$($ident: $str,)+
			}
		);
	}
}
apply_functional_pseudo_class!(define_functional_pseudo_class_keyword);

impl<'a> Parse<'a> for FunctionalPseudoClass<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colon = p.parse::<T![:]>();
		let keyword = p.parse::<FunctionalPseudoClassKeyword>();
		p.set_skip(skip);
		let colon = colon?;
		let keyword = keyword?;
		let c: Cursor = keyword.into();
		let function = <T![Function]>::build(p, c);
		macro_rules! match_keyword {
			( $($ident: ident: $str: tt: $ty: ident$(<'a>)?: $val_ty: ty $(,)*)+ ) => {
				Ok(match keyword {
					$(FunctionalPseudoClassKeyword::$ident(_) => {
						let value = p.parse::<$val_ty>()?;
						let close = p.parse_if_peek::<T![')']>()?;
						Self::$ident($ty { colon, function, value, close })
					})+
				})
			}
		}
		apply_functional_pseudo_class!(match_keyword)
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct DirPseudoFunction {
	pub colon: T![:],
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

keyword_set!(pub enum DirValue { Rtl: "rtl", Ltr: "ltr" });

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct HasPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: RelativeSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct HostPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct HostContextPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct IsPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LangPseudoFunction<'a> {
	pub colon: T![:],
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

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NotPseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: SelectorList<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthChildPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthColPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthLastChildPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthLastColPseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthLastOfTypePseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NthOfTypePseudoFunction {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: Nth,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct WherePseudoFunction<'a> {
	#[visit(skip)]
	pub colon: T![:],
	#[visit(skip)]
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct StatePseudoFunction {
	pub colon: T![:],
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HeadingPseudoFunction<'a> {
	pub colon: T![:],
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
