use super::prelude::*;
use crate::{MsTrackRepeat, TrackSize};

/// A track size or repeated track sequence in the legacy Microsoft grid syntax.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(NodeWithMetadata)]
pub enum MsTrackSize<'a> {
	Repeat(MsTrackRepeat<'a>),
	TrackSize(TrackSize<'a>),
}
