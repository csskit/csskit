use crate::Todo;

/// <https://drafts.csswg.org/css-cascade-6/#at-ruledef-scope>
pub type ScopeRule = Todo;

#[cfg(test)]
mod tests {

	#[test]
	fn test_writes() {
		//assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope");
	}
}
