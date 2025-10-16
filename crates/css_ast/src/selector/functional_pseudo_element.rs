use crate::{CssAtomSet, CssDiagnostic};
use bumpalo::collections::Vec;
use css_lexer::Kind;
use css_parse::{Diagnostic, Parse, Parser, Peek, Result as ParserResult, T};
use csskit_derives::{Parse, ToCursors, ToSpan};

use super::CompoundSelector;

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
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
	ViewTransitionGroup(ViewTransitionGroupPseudoElement<'a>),
	// https://drafts.csswg.org/css-view-transitions-2/#::view-transition-image-pair
	ViewTransitionImagePair(ViewTransitionImagePairPseudoElement<'a>),
	// https://drafts.csswg.org/css-view-transitions-2/#::view-transition-new
	ViewTransitionNew(ViewTransitionNewPseudoElement<'a>),
	// https://drafts.csswg.org/css-view-transitions-2/#::view-transition-old
	ViewTransitionOld(ViewTransitionOldPseudoElement<'a>),
}

impl<'a> Peek<'a> for FunctionalPseudoElement<'a> {
	fn peek(p: &Parser<'a>, _: css_lexer::Cursor) -> bool {
		p.peek::<T![::]>() && p.peek_n(3) == Kind::Function
	}
}

impl<'a> Parse<'a> for FunctionalPseudoElement<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		match p.to_atom::<CssAtomSet>(p.peek_n(3)) {
			CssAtomSet::Highlight => p.parse::<HighlightPseudoElement>().map(Self::Highlight),
			CssAtomSet::Part => p.parse::<PartPseudoElement>().map(Self::Part),
			CssAtomSet::Picker => p.parse::<PickerPseudoElement>().map(Self::Picker),
			CssAtomSet::Slotted => p.parse::<SlottedPseudoElement>().map(Self::Slotted),
			CssAtomSet::ViewTransitionGroup => {
				p.parse::<ViewTransitionGroupPseudoElement>().map(Self::ViewTransitionGroup)
			}
			CssAtomSet::ViewTransitionImagePair => {
				p.parse::<ViewTransitionImagePairPseudoElement>().map(Self::ViewTransitionImagePair)
			}
			CssAtomSet::ViewTransitionNew => p.parse::<ViewTransitionNewPseudoElement>().map(Self::ViewTransitionNew),
			CssAtomSet::ViewTransitionOld => p.parse::<ViewTransitionOldPseudoElement>().map(Self::ViewTransitionOld),
			_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected_pseudo_element))?,
		}
	}
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct HighlightPseudoElement {
	pub colons: T![::],
	#[atom(CssAtomSet::Highlight)]
	pub function: T![Function],
	pub value: T![Ident],
	pub close: T![')'],
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct SlottedPseudoElement<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub colons: T![::],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Slotted)]
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct PartPseudoElement<'a> {
	pub colons: T![::],
	#[atom(CssAtomSet::Part)]
	pub function: T![Function],
	pub value: Vec<'a, T![Ident]>,
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct PickerPseudoElement {
	pub colons: T![::],
	#[atom(CssAtomSet::Picker)]
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ViewTransitionGroupPseudoElement<'a> {
	pub colons: T![::],
	#[atom(CssAtomSet::ViewTransitionGroup)]
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ViewTransitionImagePairPseudoElement<'a> {
	pub colons: T![::],
	#[atom(CssAtomSet::ViewTransitionImagePair)]
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ViewTransitionNewPseudoElement<'a> {
	pub colons: T![::],
	#[atom(CssAtomSet::ViewTransitionNew)]
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ViewTransitionOldPseudoElement<'a> {
	pub colons: T![::],
	#[atom(CssAtomSet::ViewTransitionOld)]
	pub function: T![Function],
	pub value: PtNameAndClassSelector<'a>,
	pub close: Option<T![')']>,
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum PtNameAndClassSelector<'a> {
	Wildcard(T![*]),
	Named(T![Ident], Vec<'a, (T![.], T![Ident])>),
	Classes(Vec<'a, (T![.], T![Ident])>),
}
