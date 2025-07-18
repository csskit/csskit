use bumpalo::collections::Vec;
use css_parse::{Build, Parse, Parser, Result as ParserResult, T, function_set};
use csskit_derives::{ToCursors, ToSpan, Visitable};

use super::CompoundSelector;

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
#[visit]
pub enum FunctionalPseudoElement<'a> {
	// https://drafts.csswg.org/css-highlight-api/#custom-highlight-pseudo
	Highlight(HighlightPseudoElement),
	// https://drafts.csswg.org/css-shadow-parts/#part
	Part(PartPseudoElement<'a>),
	// https://drafts.csswg.org/css-scoping/#slotted-pseudo
	Slotted(SlottedPseudoElement<'a>),
}

function_set!(
	enum FunctionKeywords {
		Highlight: "highlight",
		Part: "part",
		Slotted: "slotted",
	}
);

impl<'a> Parse<'a> for FunctionalPseudoElement<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colons = p.parse::<T![::]>()?;
		let kw = p.parse::<FunctionKeywords>()?;
		let function = <T![Function]>::build(p, kw.into());
		match kw {
			FunctionKeywords::Highlight(_) => {
				let value = p.parse::<T![Ident]>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Highlight(HighlightPseudoElement { colons, function, value, close }))
			}
			FunctionKeywords::Part(_) => {
				let mut value = Vec::new_in(p.bump());
				loop {
					if p.peek::<T![')']>() {
						break;
					}
					value.push(p.parse::<T![Ident]>()?);
				}
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Part(PartPseudoElement { colons, function, value, close }))
			}
			FunctionKeywords::Slotted(_) => {
				let value = p.parse::<CompoundSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Slotted(SlottedPseudoElement { colons, function, value, close }))
			}
		}
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HighlightPseudoElement {
	pub colons: T![::],
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct SlottedPseudoElement<'a> {
	#[visit(skip)]
	pub colons: T![::],
	#[visit(skip)]
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct PartPseudoElement<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: Vec<'a, T![Ident]>,
	pub close: Option<T![')']>,
}
