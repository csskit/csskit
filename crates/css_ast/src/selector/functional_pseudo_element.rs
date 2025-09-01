use bumpalo::collections::Vec;
use css_parse::{Parse, Parser, Result as ParserResult, T, function_set};
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};

use super::CompoundSelector;

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
#[visit]
pub enum FunctionalPseudoElement<'a> {
	// https://drafts.csswg.org/css-highlight-api/#custom-highlight-pseudo
	Highlight(HighlightPseudoElement),
	// https://drafts.csswg.org/css-shadow-parts/#part
	Part(PartPseudoElement<'a>),
	// https://drafts.csswg.org/css-forms-1/#picker-pseudo
	Picker(PickerPseudoElement),
	// https://drafts.csswg.org/css-scoping/#slotted-pseudo
	Slotted(SlottedPseudoElement<'a>),
	// https://drafts.csswg.org/css-view-transitions-2/#view-transition-pseudo
	// https://drafts.csswg.org/css-view-transitions-2/#::view-transition-group
	ViewTransitionGroup(ViewTransitionGroupPseudoFunction<'a>),
	// https://drafts.csswg.org/css-view-transitions-2/#::view-transition-image-pair
	ViewTransitionImagePair(ViewTransitionImagePairPseudoFunction<'a>),
	// https://drafts.csswg.org/css-view-transitions-2/#::view-transition-new
	ViewTransitionNew(ViewTransitionNewPseudoFunction<'a>),
	// https://drafts.csswg.org/css-view-transitions-2/#::view-transition-old
	ViewTransitionOld(ViewTransitionOldPseudoFunction<'a>),
}

function_set!(
	enum FunctionKeywords {
		Highlight: "highlight",
		Part: "part",
		Slotted: "slotted",
		Picker: "picker",
		ViewTransitionGroup: "view-transition-group",
		ViewTransitionImagePair: "view-transition-image-pair",
		ViewTransitionNew: "view-transition-new",
		ViewTransitionOld: "view-transition-old",
	}
);

impl<'a> Parse<'a> for FunctionalPseudoElement<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colons = p.parse::<T![::]>()?;
		let kw = p.parse::<FunctionKeywords>()?;
		match kw {
			FunctionKeywords::Highlight(function) => {
				let value = p.parse::<T![Ident]>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Highlight(HighlightPseudoElement { colons, function, value, close }))
			}
			FunctionKeywords::Part(function) => {
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
			FunctionKeywords::Picker(function) => {
				let value = p.parse::<T![Ident]>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Picker(PickerPseudoElement { colons, function, value, close }))
			}
			FunctionKeywords::Slotted(function) => {
				let value = p.parse::<CompoundSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Slotted(SlottedPseudoElement { colons, function, value, close }))
			}
			FunctionKeywords::ViewTransitionGroup(function) => {
				let value = p.parse::<PtNameAndClassSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::ViewTransitionGroup(ViewTransitionGroupPseudoFunction { colons, function, value, close }))
			}
			FunctionKeywords::ViewTransitionImagePair(function) => {
				let value = p.parse::<PtNameAndClassSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::ViewTransitionImagePair(ViewTransitionImagePairPseudoFunction {
					colons,
					function,
					value,
					close,
				}))
			}
			FunctionKeywords::ViewTransitionNew(function) => {
				let value = p.parse::<PtNameAndClassSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::ViewTransitionNew(ViewTransitionNewPseudoFunction { colons, function, value, close }))
			}
			FunctionKeywords::ViewTransitionOld(function) => {
				let value = p.parse::<PtNameAndClassSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::ViewTransitionOld(ViewTransitionOldPseudoFunction { colons, function, value, close }))
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

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct PickerPseudoElement {
	pub colons: T![::],
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ViewTransitionGroupPseudoFunction<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ViewTransitionImagePairPseudoFunction<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ViewTransitionNewPseudoFunction<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ViewTransitionOldPseudoFunction<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum PtNameAndClassSelector<'a> {
	Wildcard(T![*]),
	Named(T![Ident], Vec<'a, (T![.], T![Ident])>),
	Classes(Vec<'a, (T![.], T![Ident])>),
}
