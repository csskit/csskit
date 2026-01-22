use super::prelude::*;
use crate::units::LengthPercentage;

/// <https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-inline>
///
/// ```text,ignore
/// snap-inline() = snap-inline( <length> , [ left | right | near ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SnapInlineFunction {
	#[atom(CssAtomSet::SnapInline)]
	pub name: T![Function],
	pub params: SnapInlineFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SnapInlineFunctionParams(LengthPercentage, Option<T![,]>, Option<SnapInlineKeyword>, Option<T![,]>);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SnapInlineKeyword {
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::Near)]
	Near(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SnapInlineFunction>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(10%)");
		assert_parse!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(10%,near)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-block(10%)");
		assert_parse_error!(CssAtomSet::ATOMS, SnapInlineFunction, "snap-inline(near)");
	}
}
