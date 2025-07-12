use css_lexer::Cursor;
use css_parse::{Parse, Parser, Peek, Result as ParserResult, T, diagnostics, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::types::{EasingFunction, SingleTransitionProperty, TransitionBehaviorValue};
use crate::units::Time;

// https://drafts.csswg.org/css-transitions-2/#single-transition
// <single-transition> = [ none | <single-transition-property> ] || <time> || <easing-function> || <time> || <transition-behavior-value>
#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SingleTransition<'a> {
	pub property: Option<SingleTransitionPropertyOrNone>,
	pub duration: Option<Time>,
	pub easing: Option<EasingFunction<'a>>,
	pub delay: Option<Time>,
	pub behavior: Option<TransitionBehaviorValue>,
}

keyword_set!(NoneKeyword, "none");

// [ none | <single-transition-property> ]
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleTransitionPropertyOrNone {
	None(NoneKeyword),
	Property(SingleTransitionProperty),
}

impl<'a> Peek<'a> for SingleTransition<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		SingleTransitionPropertyOrNone::peek(p, c) || EasingFunction::peek(p, c) || Time::peek(p, c)
	}
}

impl SingleTransition<'_> {
	fn is_some_none(&self) -> bool {
		self.property.is_none()
			|| self.duration.is_none()
			|| self.easing.is_none()
			|| self.delay.is_none()
			|| self.behavior.is_none()
	}

	fn is_all_none(&self) -> bool {
		self.property.is_none()
			&& self.duration.is_none()
			&& self.easing.is_none()
			&& self.delay.is_none()
			&& self.behavior.is_none()
	}
}

impl<'a> Parse<'a> for SingleTransition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self { property: None, duration: None, easing: None, delay: None, behavior: None };

		while value.is_some_none() {
			if value.easing.is_none() {
				value.easing = p.parse_if_peek::<EasingFunction>()?;
				if value.easing.is_some() {
					continue;
				}
			}

			if value.property.is_none() {
				value.property = p.parse_if_peek::<SingleTransitionPropertyOrNone>()?;
				if value.property.is_some() {
					continue;
				}
			}

			if value.duration.is_none() {
				value.duration = p.parse_if_peek::<Time>()?;
				if value.duration.is_some() {
					continue;
				}
			}

			if value.delay.is_none() {
				value.delay = p.parse_if_peek::<Time>()?;
				if value.delay.is_some() {
					continue;
				}
			}

			if value.behavior.is_none() {
				value.behavior = p.parse_if_peek::<TransitionBehaviorValue>()?;
				if value.behavior.is_some() {
					continue;
				}
			}

			break;
		}

		if value.is_all_none() {
			let c: Cursor = p.parse::<T![Any]>()?.into();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}

		Ok(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SingleTransition>(), 192);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SingleTransitionPropertyOrNone, "none");
		assert_parse!(SingleTransitionPropertyOrNone, "all");

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
		assert_parse!(SingleTransition, "none 1s normal");
		assert_parse!(SingleTransition, "1s opacity allow-discrete", "opacity 1s allow-discrete");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SingleTransition, "1deg");
		assert_parse_error!(SingleTransition, "none none");
	}
}
