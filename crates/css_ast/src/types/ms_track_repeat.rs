use super::prelude::*;
use crate::{CSSInt, NonEmpty, Positive, TrackSize};

/// A repeated track sequence in the legacy Microsoft grid syntax, such as `(250px 10px)[4]`.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(NodeWithMetadata)]
pub struct MsTrackRepeat<'a> {
	pub open_paren: T!['('],
	pub tracks: NonEmpty<Vec<'a, TrackSize<'a>>>,
	pub close_paren: T![')'],
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub open_bracket: T!['['],
	pub count: Positive<CSSInt>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close_bracket: T![']'],
}
