use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum ScriptingMediaFeature<"scripting", ScriptingMediaFeatureKeyword>
);

keyword_set!(pub enum ScriptingMediaFeatureKeyword { None: "none", InitialOnly: "initial-only", Enabled: "enabled" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScriptingMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScriptingMediaFeature, "(scripting)");
		assert_parse!(ScriptingMediaFeature, "(scripting:none)");
		assert_parse!(ScriptingMediaFeature, "(scripting:initial-only)");
		assert_parse!(ScriptingMediaFeature, "(scripting:enabled)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScriptingMediaFeature, "(scripting:)");
		assert_parse_error!(ScriptingMediaFeature, "(scripting: yes)");
	}
}
