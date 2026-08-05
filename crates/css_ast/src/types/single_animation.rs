use super::prelude::*;
use crate::{
	AutoOr, CalcableValue, EasingFunction, KeyframesName, NonNegative, NoneOr, SingleAnimationDirection,
	SingleAnimationFillMode, SingleAnimationIterationCount, SingleAnimationPlayState, SingleAnimationTimeline, Time,
};
use css_parse::parse_optionals;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation>
///
/// ```text,ignore
/// <single-animation> =
///   <'animation-duration'> ||
///   <easing-function> ||
///   <'animation-delay-start'> ||
///   <single-animation-iteration-count> ||
///   <single-animation-direction> ||
///   <single-animation-fill-mode> ||
///   <single-animation-play-state> ||
///   [ none | <keyframes-name> ] ||
///   <single-animation-timeline>
/// ```
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SingleAnimation<'a> {
	pub duration: Option<AutoOr<CalcableValue<'a, NonNegative<Time>>>>,
	pub easing: Option<EasingFunction<'a>>,
	pub delay: Option<CalcableValue<'a, Time>>,
	pub iteration_count: Option<SingleAnimationIterationCount<'a>>,
	pub direction: Option<SingleAnimationDirection>,
	pub fill_mode: Option<SingleAnimationFillMode>,
	pub play_state: Option<SingleAnimationPlayState>,
	pub name: Option<NoneOr<KeyframesName>>,
	pub timeline: Option<SingleAnimationTimeline>,
}

impl<'a> Peek<'a> for SingleAnimation<'a> {
	const PEEK_KINDSET: KindSet = <AutoOr<CalcableValue<'a, NonNegative<Time>>>>::PEEK_KINDSET
		.combine(EasingFunction::PEEK_KINDSET)
		.combine(<CalcableValue<'a, Time>>::PEEK_KINDSET)
		.combine(<SingleAnimationIterationCount<'a>>::PEEK_KINDSET)
		.combine(<NoneOr<KeyframesName>>::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<AutoOr<CalcableValue<'a, NonNegative<Time>>>>::peek(p, c)
			|| EasingFunction::peek(p, c)
			|| <CalcableValue<'a, Time>>::peek(p, c)
			|| <SingleAnimationIterationCount<'a>>::peek(p, c)
			|| <NoneOr<KeyframesName>>::peek(p, c)
	}
}

impl<'a> Parse<'a> for SingleAnimation<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let (duration, easing, delay, iteration_count, direction, fill_mode, play_state, name, timeline) = parse_optionals!(
			p,
			duration: AutoOr<CalcableValue<NonNegative<Time>>>,
			easing: EasingFunction,
			delay: CalcableValue<Time>,
			iteration_count: SingleAnimationIterationCount,
			direction: SingleAnimationDirection,
			fill_mode: SingleAnimationFillMode,
			play_state: SingleAnimationPlayState,
			name: NoneOr<KeyframesName>,
			timeline: SingleAnimationTimeline
		);
		Ok(Self { duration, easing, delay, iteration_count, direction, fill_mode, play_state, name, timeline })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "slidein");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "3s");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "auto");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "3s slidein");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "3s 1s slidein");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "3s ease-in 1s infinite alternate both paused slidein");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "slidein 3s ease-in 1s 2 reverse forwards running");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "3s auto");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "\"slide in\"");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "cubic-bezier(0.1,0.7,1,0.1) 3s");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "calc(1s + 200ms) var(--delay) slidein");
	}

	#[test]
	fn test_fields() {
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "3s 1s slidein", |node| {
			assert!(matches!(node.duration, Some(AutoOr::Some(_))));
			assert!(node.delay.is_some());
			assert!(matches!(node.name, Some(NoneOr::Some(_))));
			assert!(node.easing.is_none());
		});
		assert_parse!(CssAtomSet::ATOMS, SingleAnimation, "auto none", |node| {
			assert!(matches!(node.duration, Some(AutoOr::Auto(_))));
			assert!(matches!(node.fill_mode, Some(SingleAnimationFillMode::None(_))));
		});
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, SingleAnimation, "1deg");
		assert_parse_error!(CssAtomSet::ATOMS, SingleAnimation, "slidein slideout");
		assert_parse_error!(CssAtomSet::ATOMS, SingleAnimation, "3s 1s 2s");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("3s", SingleAnimation, Time);
		assert_visits!("ease-in", SingleAnimation, EasingFunction);
		assert_visits!("3s ease-in 1s", SingleAnimation, Time, EasingFunction, Time);
		assert_visits!("slidein", SingleAnimation, KeyframesName);
		assert_visits!("reverse", SingleAnimation, SingleAnimationDirection);
		assert_visits!("3s slidein forwards", SingleAnimation, Time, SingleAnimationFillMode, KeyframesName);
	}
}
