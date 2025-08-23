use css_parse::{Function, function_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::NumberOrInfinity;

function_set!(pub struct SuperellipseFunctionName "superellipse");

/// <https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value>
///
/// ```text,ignore
/// superellipse() = superellipse(<number [-∞,∞]> | infinity | -infinity)
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
pub struct SuperellipseFunction(Function<SuperellipseFunctionName, NumberOrInfinity>);
