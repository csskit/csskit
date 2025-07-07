use bumpalo::collections::Vec;
use css_lexer::{Cursor, KindSet};
use css_parse::{Build, Parse, Parser, Result as ParserResult, T, function_set, keyword_set};
use csskit_derives::ToCursors;

use crate::{Visit, Visitable};

use super::{ForgivingSelector, Nth, RelativeSelector, SelectorList};

macro_rules! apply_functional_pseudo_class {
	($macro: ident) => {
		$macro! {
			Dir: "dir": DirPseudoFunction: DirValue,
			Has: "has": HasPseudoFunction<'a>: RelativeSelector,
			Host: "host": HostPseudoFunction<'a>: SelectorList,
			HostContext: "host-context": HostContextPseudoFunction<'a>: SelectorList,
			Is: "is": IsPseudoFunction<'a>: ForgivingSelector,
			Lang: "lang": LangPseudoFunction<'a>: LangValues,
			Not: "not": NotPseudoFunction<'a>: SelectorList,
			NthChild: "nth-child": NthChildPseudoFunction<'a>: Nth,
			NthCol: "nth-col": NthColPseudoFunction<'a>: Nth,
			NthLastChild: "nth-last-child": NthLastChildPseudoFunction<'a>: Nth,
			NthLastCol: "nth-last-col": NthLastColPseudoFunction<'a>: Nth,
			NthLastOfType: "nth-last-of-type": NthLastOfTypePseudoFunction<'a>: Nth,
			NthOfType: "nth-of-type": NthOfTypePseudoFunction<'a>: Nth,
			State: "state": StatePseudoFunction: T![Ident],
			Where: "where": WherePseudoFunction<'a>: ForgivingSelector,
		}
	};
}

macro_rules! define_functional_pseudo_class {
	( $($ident: ident: $str: tt: $ty: ty: $val_ty: ty $(,)*)+ ) => {
		#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
		function_set!(FunctionalPseudoClassKeyword {
			$($ident: $str,)+
		});
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

impl<'a> Visitable<'a> for FunctionalPseudoClass<'a> {
	fn accept<V: Visit<'a>>(&self, _v: &mut V) {
		// macro_rules! match_keyword {
		// 	( $($ident: ident: $str: tt: $ty: ty: $val_ty: ty $(,)*)+ ) => {
		// 		match self {
		// 			$(Self::$ident(c) => Visitable::accept(c, v),)+
		// 		}
		// 	}
		// }
		// apply_functional_pseudo_class!(match_keyword);
	}
}

#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DirPseudoFunction {
	pub colon: T![:],
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

keyword_set!(DirValue { Rtl: "rtl", Ltr: "ltr" });

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct HasPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: RelativeSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct HostPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: SelectorList<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct HostContextPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: SelectorList<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct IsPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LangPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: LangValues<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LangValues<'a>(Vec<'a, LangValue>);

impl<'a> Parse<'a> for LangValues<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = Vec::new_in(p.bump());
		loop {
			values.push(p.parse::<LangValue>()?);
			if p.peek::<T![')']>() {
				break;
			}
		}
		Ok(Self(values))
	}
}

#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LangValue {
	Ident(T![Ident], Option<T![,]>),
	String(T![String], Option<T![,]>),
}

impl<'a> Parse<'a> for LangValue {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Ident]>() {
			let value = p.parse::<T![Ident]>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			Ok(Self::Ident(value, comma))
		} else {
			let value = p.parse::<T![String]>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			Ok(Self::String(value, comma))
		}
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NotPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: SelectorList<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthChildPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthColPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthLastChildPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthLastColPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthLastOfTypePseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthOfTypePseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct WherePseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct StatePseudoFunction {
	pub colon: T![:],
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FunctionalPseudoClass>(), 96);
		assert_eq!(std::mem::size_of::<DirValue>(), 16);
	}
}
