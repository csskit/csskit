use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Result as ParserResult, T, ToCursors, diagnostics};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable};

use super::CompoundSelector;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a> Parse<'a> for FunctionalPseudoElement<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colons = p.parse::<T![::]>()?;
		let function = p.parse::<T![Function]>()?;
		match p.parse_str_lower(function.into()) {
			"highlight" => {
				let value = p.parse::<T![Ident]>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Highlight(HighlightPseudoElement { colons, function, value, close }))
			}
			"part" => {
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
			"slotted" => {
				let value = p.parse::<CompoundSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Slotted(SlottedPseudoElement { colons, function, value, close }))
			}
			ident => {
				let c: Cursor = function.into();
				Err(diagnostics::UnexpectedFunction(ident.into(), c.into()))?
			}
		}
	}
}

impl<'a> ToCursors for FunctionalPseudoElement<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Highlight(c) => ToCursors::to_cursors(c, s),
			Self::Slotted(c) => ToCursors::to_cursors(c, s),
			Self::Part(c) => ToCursors::to_cursors(c, s),
		}
	}
}

impl<'a> Visitable<'a> for FunctionalPseudoElement<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_functional_pseudo_element(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct HighlightPseudoElement {
	pub colons: T![::],
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for HighlightPseudoElement {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.colons, s);
		s.append(self.function.into());
		s.append(self.value.into());
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SlottedPseudoElement<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for SlottedPseudoElement<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.colons, s);
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PartPseudoElement<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: Vec<'a, T![Ident]>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for PartPseudoElement<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.colons, s);
		s.append(self.function.into());
		for value in &self.value {
			s.append(value.into());
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}
