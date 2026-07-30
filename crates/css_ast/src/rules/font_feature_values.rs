use crate::Todo;

/// <https://drafts.csswg.org/css-fonts/#at-ruledef-font-feature-values>
pub type FontFeatureValuesRule = Todo;

#[cfg(test)]
mod tests {

	#[test]
	fn test_writes() {
		//assert_parse!(CssAtomSet::ATOMS, FontFeatureValuesRule, "@font-feature-values Taisho Gothic {}");
	}
}
