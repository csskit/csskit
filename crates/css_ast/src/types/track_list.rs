use super::prelude::*;
use crate::{AutoRepeat, FixedRepeat, FixedSize, LineNames, NameRepeat, NonEmpty, TrackRepeat, TrackSize};
use css_parse::Either;

/// <https://drafts.csswg.org/css-grid-2/#typedef-track-list>
///
/// ```text,ignore
/// <track-list> = [ <line-names>? [ <track-size> | <track-repeat> ] ]+ <line-names>?
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TrackList<'a> {
	pub leading_names: Option<LineNames<'a>>,
	pub items: NonEmpty<Vec<'a, TrackListItem<'a>>>,
}

type TrackListItem<'a> = (Either<TrackSize<'a>, TrackRepeat<'a>>, Option<LineNames<'a>>);

impl<'a> Peek<'a> for TrackList<'a> {
	const PEEK_KINDSET: KindSet =
		LineNames::PEEK_KINDSET.combine(TrackSize::PEEK_KINDSET).combine(TrackRepeat::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		LineNames::peek(p, c) || TrackSize::peek(p, c) || TrackRepeat::peek(p, c)
	}
}

/// <https://drafts.csswg.org/css-grid-2/#typedef-auto-track-list>
///
/// ```text,ignore
/// <auto-track-list> = [ <line-names>? [ <fixed-size> | <fixed-repeat> ] ]* <line-names>? <auto-repeat>
///                     [ <line-names>? [ <fixed-size> | <fixed-repeat> ] ]* <line-names>?
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AutoTrackList<'a> {
	pub start_leading_names: Option<LineNames<'a>>,
	pub start_items: Vec<'a, AutoTrackListItem<'a>>,
	pub auto_repeat: AutoRepeat<'a>,
	pub end_leading_names: Option<LineNames<'a>>,
	pub end_items: Vec<'a, AutoTrackListItem<'a>>,
}

type AutoTrackListItem<'a> = (Either<FixedSize<'a>, FixedRepeat<'a>>, Option<LineNames<'a>>);

impl<'a> Peek<'a> for AutoTrackList<'a> {
	const PEEK_KINDSET: KindSet = LineNames::PEEK_KINDSET
		.combine(FixedSize::PEEK_KINDSET)
		.combine(FixedRepeat::PEEK_KINDSET)
		.combine(AutoRepeat::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		LineNames::peek(p, c) || FixedSize::peek(p, c) || FixedRepeat::peek(p, c) || AutoRepeat::peek(p, c)
	}
}

/// <https://drafts.csswg.org/css-grid-2/#typedef-explicit-track-list>
///
/// ```text,ignore
/// <explicit-track-list> = [ <line-names>? <track-size> ]+ <line-names>?
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ExplicitTrackList<'a> {
	pub leading_names: Option<LineNames<'a>>,
	pub items: NonEmpty<Vec<'a, ExplicitTrackListItem<'a>>>,
}

type ExplicitTrackListItem<'a> = (TrackSize<'a>, Option<LineNames<'a>>);

impl<'a> Peek<'a> for ExplicitTrackList<'a> {
	const PEEK_KINDSET: KindSet = LineNames::PEEK_KINDSET.combine(TrackSize::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		LineNames::peek(p, c) || TrackSize::peek(p, c)
	}
}

/// <https://drafts.csswg.org/css-grid-2/#typedef-line-name-list>
///
/// ```text,ignore
/// <line-name-list> = [ <line-names> | <name-repeat> ]+
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LineNameList<'a>(pub NonEmpty<Vec<'a, Either<LineNames<'a>, NameRepeat<'a>>>>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TrackList>(), 72);
		assert_eq!(std::mem::size_of::<AutoTrackList>(), 272);
		assert_eq!(std::mem::size_of::<ExplicitTrackList>(), 72);
		assert_eq!(std::mem::size_of::<LineNameList>(), 24);
	}

	#[test]
	fn test_writes_track_list() {
		assert_parse!(CssAtomSet::ATOMS, TrackList, "10px");
		assert_parse!(CssAtomSet::ATOMS, TrackList, "[a] 10px [b]");
		assert_parse!(CssAtomSet::ATOMS, TrackList, "10px [a]");
		assert_parse!(CssAtomSet::ATOMS, TrackList, "10px [a] 20px");
		assert_parse!(CssAtomSet::ATOMS, TrackList, "10px 1fr repeat(2,20px)");
		assert_parse!(CssAtomSet::ATOMS, TrackList, "[a] 10px [b] repeat(2,1fr) [c]");
	}

	#[test]
	fn test_errors_track_list() {
		assert_peek_false!(CssAtomSet::ATOMS, TrackList, "none");
		assert_parse_error!(CssAtomSet::ATOMS, TrackList, "[a]");
	}

	#[test]
	fn test_writes_auto_track_list() {
		assert_parse!(CssAtomSet::ATOMS, AutoTrackList, "repeat(auto-fill,10px)");
		assert_parse!(CssAtomSet::ATOMS, AutoTrackList, "10px repeat(auto-fill,20px) 30px");
		assert_parse!(CssAtomSet::ATOMS, AutoTrackList, "[a] 10px repeat(auto-fit,[b] 20px) [c] 30px [d]");
	}

	#[test]
	fn test_errors_auto_track_list() {
		assert_peek_false!(CssAtomSet::ATOMS, AutoTrackList, "none");
		assert_parse_error!(CssAtomSet::ATOMS, AutoTrackList, "10px 20px");
	}

	#[test]
	fn test_writes_explicit_track_list() {
		assert_parse!(CssAtomSet::ATOMS, ExplicitTrackList, "10px");
		assert_parse!(CssAtomSet::ATOMS, ExplicitTrackList, "[a] 10px [b] 1fr [c]");
	}

	#[test]
	fn test_errors_explicit_track_list() {
		assert_peek_false!(CssAtomSet::ATOMS, ExplicitTrackList, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ExplicitTrackList, "[a]");
	}

	#[test]
	fn test_writes_line_name_list() {
		assert_parse!(CssAtomSet::ATOMS, LineNameList, "[a]");
		assert_parse!(CssAtomSet::ATOMS, LineNameList, "[a] [b] repeat(2,[c])");
	}

	#[test]
	fn test_errors_line_name_list() {
		assert_peek_false!(CssAtomSet::ATOMS, LineNameList, "10px");
	}
}
