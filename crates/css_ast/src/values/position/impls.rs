#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<PositionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TopStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<RightStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BottomStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<LeftStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<InsetBlockStartStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<InsetInlineStartStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<InsetBlockEndStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<InsetInlineEndStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<InsetBlockStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<InsetInlineStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<InsetStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<OverlayStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PositionStyleValue, "sticky");
		assert_parse!(CssAtomSet::ATOMS, PositionStyleValue, "-webkit-sticky");
		assert_parse!(CssAtomSet::ATOMS, InsetBlockStartStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, InsetStyleValue, "1px 2px");
		assert_parse!(CssAtomSet::ATOMS, InsetStyleValue, "1px 2px 3px 4px");
	}
}
