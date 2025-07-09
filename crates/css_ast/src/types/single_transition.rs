#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset, Span};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};
use csskit_derives::{Parse, Peek, ToCursors};

use crate::types::{EasingFunction, SingleTransitionProperty};
use crate::units::Time;

// https://drafts.csswg.org/css-transitions-1/#single-transition
// <single-transition> = [ none | <single-transition-property> ] || <time> || <easing-function> || <time>
#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SingleTransition<'a>(
	pub Option<SingleTransitionPropertyOrNone>,
	pub Option<Time>,
	pub Option<EasingFunction<'a>>,
	pub Option<Time>,
);

// [ none | <single-transition-property> ]
#[derive(ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleTransitionPropertyOrNone {
	None(T![Ident]),
	Property(SingleTransitionProperty),
}

impl<'a> Parse<'a> for SingleTransitionPropertyOrNone {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(property) = p.parse_if_peek::<SingleTransitionProperty>()? {
			return Ok(Self::Property(property));
		}

		let ident = p.parse::<T![Ident]>()?;
		if !p.eq_ignore_ascii_case(ident.into(), "none") {
			Err(diagnostics::UnexpectedIdent(p.parse_str(ident.into()).into(), (&ident).into()))?
		}

		Ok(Self::None(ident))
	}
}

impl<'a> Peek<'a> for SingleTransition<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		SingleTransitionPropertyOrNone::peek(p, c) || EasingFunction::peek(p, c) || Time::peek(p, c)
	}
}

impl<'a> Parse<'a> for SingleTransition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut property = None;
		let mut duration = None;
		let mut timing_function = None;
		let mut delay = None;

		let mut has_thing = false;

		for _ in 0..3 {
			if p.peek::<EasingFunction>() {
				if timing_function.is_some() {
					let c: Cursor = p.parse::<T![Any]>()?.into();
					Err(diagnostics::Unexpected(c.into(), c.into()))?
				}

				timing_function = Some(p.parse::<EasingFunction>()?);
				has_thing = true;
			}

			if p.peek::<SingleTransitionPropertyOrNone>() {
				if property.is_some() {
					let c: Cursor = p.parse::<T![Any]>()?.into();
					Err(diagnostics::Unexpected(c.into(), c.into()))?
				}

				property = Some(p.parse::<SingleTransitionPropertyOrNone>()?);
				has_thing = true;
			}

			if p.peek::<Time>() {
				if duration.is_some() && delay.is_some() {
					let c: Cursor = p.parse::<T![Any]>()?.into();
					Err(diagnostics::Unexpected(c.into(), c.into()))?
				}

				let time = p.parse::<Time>()?;
				has_thing = true;

				if duration.is_none() {
					duration = Some(time);
				} else {
					delay = Some(time);
				}
			}
		}

		if has_thing == false {
			let c: Cursor = p.parse::<T![Any]>()?.into();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}

		Ok(Self(property, duration, timing_function, delay))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SingleTransition>(), 176);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SingleTransition, "none");
		assert_parse!(SingleTransition, "opacity");
		assert_parse!(SingleTransition, "opacity 1s");
		assert_parse!(SingleTransition, "opacity 1s ease-in");
		assert_parse!(SingleTransition, "opacity 1s ease-in 2s");
		assert_parse!(SingleTransition, "2s ease-in");
		assert_parse!(SingleTransition, "1s opacity", "opacity 1s");
		assert_parse!(SingleTransition, "ease-in 1s opacity", "opacity 1s ease-in");
		assert_parse!(SingleTransition, "1s 2s ease-in opacity", "opacity 1s ease-in 2s");
		assert_parse!(SingleTransition, "ease-in opacity 1s 2s", "opacity 1s ease-in 2s");
		assert_parse!(SingleTransition, "ease-in");
		assert_parse!(SingleTransition, "1s");
		assert_parse!(SingleTransition, "1s 2s");
		assert_parse!(SingleTransition, "all 1s ease-in 2s");
		assert_parse!(SingleTransition, "none 1s");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SingleTransition, "1deg");
		assert_parse_error!(SingleTransition, "none none");
	}
}
