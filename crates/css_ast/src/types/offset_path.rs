use super::prelude::*;
use crate::{BasicShape, RayFunction, Url};

/// <https://drafts.csswg.org/css-fonts-4/#numeric-spacing-values>
///
/// ```text,ignore
/// <offset-path> = <ray()> | <url> | <basic-shape>
/// ```
#[derive(Parse, Peek, SemanticEq, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum OffsetPath {
	RayFunction(RayFunction),
	Url(Url),
	BasicShape(BasicShape),
}
