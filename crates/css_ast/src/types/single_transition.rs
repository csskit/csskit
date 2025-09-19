use super::prelude::*;
use crate::{EasingFunction, NoneOr, SingleTransitionProperty, Time, TransitionBehaviorValue};
use css_parse::parse_optionals;

// https://drafts.csswg.org/css-transitions-2/#single-transition
// <single-transition> = [ none | <single-transition-property> ] || <time> || <easing-function> || <time> || <transition-behavior-value>
#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct SingleTransition<'a> {
	#[visit(skip)]
	pub property: Option<NoneOr<SingleTransitionProperty>>,
	pub duration: Option<Time>,
	pub easing: Option<EasingFunction<'a>>,
	pub delay: Option<Time>,
	#[visit(skip)]
	pub behavior: Option<TransitionBehaviorValue>,
}

impl<'a> Peek<'a> for SingleTransition<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<NoneOr<SingleTransitionProperty>>::peek(p, c) || EasingFunction::peek(p, c) || Time::peek(p, c)
	}
}

impl<'a> Parse<'a> for SingleTransition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (easing, property, duration, delay, behavior) = parse_optionals!(p, easing: EasingFunction, property: NoneOr<SingleTransitionProperty>, duration: Time, delay: Time, behavior: TransitionBehaviorValue);
		Ok(Self { easing, property, duration, delay, behavior })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_visits;
	use css_parse::{assert_parse, assert_parse_error};

	type NoneOrSingleTransitionProperty = NoneOr<SingleTransitionProperty>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SingleTransition>(), 192);
	}

	#[test]
	fn test_writes() {
		assert_parse!(NoneOrSingleTransitionProperty, "none", NoneOrSingleTransitionProperty::None(_));
		assert_parse!(NoneOrSingleTransitionProperty, "all", NoneOrSingleTransitionProperty::Some(_));

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

	#[test]
	fn test_visits() {
		assert_visits!("1s", SingleTransition, Time);
		assert_visits!("ease-in", SingleTransition, EasingFunction);
		assert_visits!("1s 2s", SingleTransition, Time, Time);
		assert_visits!("1s ease-in 2s", SingleTransition, Time, EasingFunction, Time);
	}
}
