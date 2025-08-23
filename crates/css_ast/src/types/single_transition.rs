use css_lexer::Cursor;
use css_parse::{Parse, Parser, Peek, Result as ParserResult, parse_optionals};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::{EasingFunction, NoneKeyword, SingleTransitionProperty, Time, TransitionBehaviorValue};

// https://drafts.csswg.org/css-transitions-2/#single-transition
// <single-transition> = [ none | <single-transition-property> ] || <time> || <easing-function> || <time> || <transition-behavior-value>
#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct SingleTransition<'a> {
	#[visit(skip)]
	pub property: Option<SingleTransitionPropertyOrNone>,
	pub duration: Option<Time>,
	pub easing: Option<EasingFunction<'a>>,
	pub delay: Option<Time>,
	#[visit(skip)]
	pub behavior: Option<TransitionBehaviorValue>,
}

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

impl<'a> Parse<'a> for SingleTransition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (easing, property, duration, delay, behavior) = parse_optionals!(p, easing: EasingFunction, property: SingleTransitionPropertyOrNone, duration: Time, delay: Time, behavior: TransitionBehaviorValue);
		Ok(Self { easing, property, duration, delay, behavior })
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
		assert_parse!(SingleTransitionPropertyOrNone, "none", SingleTransitionPropertyOrNone::None(_));
		assert_parse!(SingleTransitionPropertyOrNone, "all", SingleTransitionPropertyOrNone::Property(_));

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
