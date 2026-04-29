#[cfg(test)]
mod tests {
	use super::super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FillBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillColorStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<FillOpacityStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillOriginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillPositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<FillRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<FillRuleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillSizeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeColorStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeDashCornerStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeDasharrayStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeDashoffsetStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeLinecapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeMiterlimitStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<StrokeOpacityStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeOriginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokePositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeSizeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeWidthStyleValue>(), 32);
	}
}
