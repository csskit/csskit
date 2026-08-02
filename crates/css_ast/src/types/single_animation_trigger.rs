use super::prelude::*;
use crate::{CalcableValue, DashedIdent, LengthPercentage, SingleAnimationTriggerBehavior, TimelineRangeName};
use css_parse::{Vec, parse_optionals};

/// A single range offset in a [`SingleAnimationTrigger`].
///
/// ```text,ignore
/// [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SingleAnimationTriggerRange<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	Named(TimelineRangeName<'a>, Option<CalcableValue<'a, LengthPercentage>>),
	LengthPercentage(CalcableValue<'a, LengthPercentage>),
}

/// The timeline half of a [`SingleAnimationTrigger`].
///
/// ```text,ignore
/// [ none | auto | [ <dashed-ident> [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]{0,4} ] ]
/// ```
///
/// Note: `<scroll()>` and `<view()>` are not yet modelled, matching
/// [`SingleAnimationTimeline`](crate::SingleAnimationTimeline).
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SingleAnimationTriggerTimeline<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Auto)]
	Auto(T![Ident]),
	Named(DashedIdent, Vec<'a, SingleAnimationTriggerRange<'a>>),
}

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger>
///
/// ```text,ignore
/// <single-animation-trigger> =
///   <single-animation-trigger-behavior> ||
///   [ none | auto | [ [ <dashed-ident> | <scroll()> | <view()> ] [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]{0,4} ] ]
/// ```
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SingleAnimationTrigger<'a> {
	pub behavior: Option<SingleAnimationTriggerBehavior>,
	pub timeline: Option<SingleAnimationTriggerTimeline<'a>>,
}

impl<'a> Peek<'a> for SingleAnimationTrigger<'a> {
	const PEEK_KINDSET: KindSet =
		SingleAnimationTriggerBehavior::PEEK_KINDSET.combine(<SingleAnimationTriggerTimeline<'a>>::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		SingleAnimationTriggerBehavior::peek(p, c) || <SingleAnimationTriggerTimeline<'a>>::peek(p, c)
	}
}

impl<'a> Parse<'a> for SingleAnimationTrigger<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let (behavior, timeline) = parse_optionals!(
			p,
			behavior: SingleAnimationTriggerBehavior,
			timeline: SingleAnimationTriggerTimeline
		);
		Ok(Self { behavior, timeline })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "once");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "repeat");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "state none");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "auto");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "alternate auto");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "--tl");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "--tl normal");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "--tl entry 10%");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "repeat --tl cover 0% cover 100%");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "--tl 10% 90%");
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "--tl calc(10% + 1px)");
	}

	#[test]
	fn test_fields() {
		assert_parse!(CssAtomSet::ATOMS, SingleAnimationTrigger, "repeat --tl entry 10%", |node| {
			assert!(matches!(node.behavior, Some(SingleAnimationTriggerBehavior::Repeat(_))));
			let Some(SingleAnimationTriggerTimeline::Named(_, ranges)) = &node.timeline else {
				panic!("expected a named timeline");
			};
			assert_eq!(ranges.len(), 1);
			assert!(matches!(ranges[0], SingleAnimationTriggerRange::Named(_, Some(_))));
		});
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, SingleAnimationTrigger, "10%");
		assert_parse_error!(CssAtomSet::ATOMS, SingleAnimationTrigger, "once repeat");
		assert_parse_error!(CssAtomSet::ATOMS, SingleAnimationTrigger, "none --tl");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!(
			"--tl 10%",
			SingleAnimationTrigger,
			SingleAnimationTriggerTimeline,
			DashedIdent,
			SingleAnimationTriggerRange,
			LengthPercentage
		);
		assert_visits!("repeat", SingleAnimationTrigger, SingleAnimationTriggerBehavior);
	}
}
