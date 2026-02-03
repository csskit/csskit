use super::prelude::*;
use crate::{CSSInt, NonNegative, OpentypeTag};

/// `<feature-tag-value>` as defined in [css-fonts-4](https://drafts.csswg.org/css-fonts-4/#font-feature-settings-prop).
///
/// ```text,ignore
/// <feature-tag-value> = <opentype-tag> [ <integer [0,∞]> | on | off ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FeatureTagValue(pub OpentypeTag, pub Option<FeatureTagToggle>);

/// The optional value for a feature tag: `<integer [0,∞]> | on | off`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum FeatureTagToggle {
	#[atom(CssAtomSet::On)]
	On(T![Ident]),
	#[atom(CssAtomSet::Off)]
	Off(T![Ident]),
	Integer(NonNegative<CSSInt>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FeatureTagValue>(), 28);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"kern\"");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"liga\"");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"kern\" on");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"kern\" off");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"liga\" on");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"kern\" 1");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"kern\" 0");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "\"ss01\" 2");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "'smcp'");
		assert_parse!(CssAtomSet::ATOMS, FeatureTagValue, "'smcp' on");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FeatureTagValue, "\"ker\"");
		assert_parse_error!(CssAtomSet::ATOMS, FeatureTagValue, "\"kerns\"");
		assert_parse_error!(CssAtomSet::ATOMS, FeatureTagValue, "\"kern\" -1");
		assert_parse_error!(CssAtomSet::ATOMS, FeatureTagValue, "kern");
	}
}
