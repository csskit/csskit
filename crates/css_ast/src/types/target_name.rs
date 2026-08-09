use super::prelude::*;

/// <https://www.w3.org/TR/css3-hyperlinks/#target-name>
///
/// ```text,ignore
/// <target-name> = <string>
/// ```
#[node]
#[derive(
	Parse,
	Peek,
	IntoCursor,
	ToCursors,
	ToSpan,
	SemanticEq,
	Debug,
	Default,
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TargetName(T![String]);
