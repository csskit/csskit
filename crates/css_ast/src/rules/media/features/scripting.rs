use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum ScriptingMediaFeature{CssAtomSet::Scripting, ScriptingMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum ScriptingMediaFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::InitialOnly)]
	InitialOnly(T![Ident]),
	#[atom(CssAtomSet::Enabled)]
	Enabled(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScriptingMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ScriptingMediaFeature, "(scripting)");
		assert_parse!(CssAtomSet::ATOMS, ScriptingMediaFeature, "(scripting:none)");
		assert_parse!(CssAtomSet::ATOMS, ScriptingMediaFeature, "(scripting:initial-only)");
		assert_parse!(CssAtomSet::ATOMS, ScriptingMediaFeature, "(scripting:enabled)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ScriptingMediaFeature, "(scripting:)");
		assert_parse_error!(CssAtomSet::ATOMS, ScriptingMediaFeature, "(scripting: yes)");
	}
}
