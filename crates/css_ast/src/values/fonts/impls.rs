#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<FontFamilyStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FontWeightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FontWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FontStyleStyleValue>(), 28);
		assert_eq!(std::mem::size_of::<FontSizeStyleValue>(), 20);
		// assert_eq!(std::mem::size_of::<FontSizeAdjustStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<FontStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FontSynthesisWeightStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FontSynthesisStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FontSynthesisSmallCapsStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FontSynthesisPositionStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<FontSynthesisStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FontKerningStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<FontVariantLigaturesStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FontVariantPositionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FontVariantCapsStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<FontVariantNumericStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<FontVariantAlternatesStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<FontVariantEastAsianStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<FontVariantStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<FontFeatureSettingsStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FontLanguageOverrideStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FontOpticalSizingStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<FontVariationSettingsStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<FontPaletteStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<FontVariantEmojiStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontStyleStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, FontStyleStyleValue, "oblique 45deg");
		assert_parse!(CssAtomSet::ATOMS, FontSizeStyleValue, "45rem");
		assert_parse!(CssAtomSet::ATOMS, FontSizeStyleValue, "smaller");
		assert_parse!(CssAtomSet::ATOMS, FontSizeStyleValue, "80%");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FontStyleStyleValue, "oblique 45px");
		assert_parse_error!(CssAtomSet::ATOMS, FontStyleStyleValue, "oblique 91deg");
		assert_parse_error!(CssAtomSet::ATOMS, FontStyleStyleValue, "oblique -91deg");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("12px", FontSizeStyleValue, LengthPercentage, Length);
	}
}
