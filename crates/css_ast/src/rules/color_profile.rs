use crate::Todo;

/// <https://drafts.csswg.org/css-color-5/#at-profile>
pub type ColorProfileRule = Todo;

#[cfg(test)]
mod tests {

	#[test]
	fn test_writes() {
		//assert_parse!(CssAtomSet::ATOMS, ColorProfileRule, "@color-profile --swop5c {\n\tsrc: url(\"https://example.org/SWOP2006_Coated5v2.icc\");}");
	}
}
