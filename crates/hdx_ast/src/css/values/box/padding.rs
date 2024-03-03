use crate::{css::values::units::LengthPercentageOrAuto, macros::*, Parsable, Value, Writable};

// https://drafts.csswg.org/css-box-4/#padding-physical
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Padding(pub PaddingBlockStart, pub PaddingBlockEnd, pub PaddingInlineStart, pub PaddingInlineEnd);

parse_rect!(Padding, LengthPercentageOrAuto, PaddingBlockStart, PaddingBlockEnd, PaddingInlineStart, PaddingInlineEnd);
write_rect!(Padding);

// https://drafts.csswg.org/css-logical-1/#propdef-padding-inline
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingInline(pub PaddingInlineStart, pub PaddingInlineEnd);

parse_logical_sides!(PaddingInline, LengthPercentageOrAuto, PaddingInlineStart, PaddingInlineEnd);
write_logical_sides!(PaddingInline);

// https://drafts.csswg.org/css-logical-1/#propdef-padding-inline
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingBlock(pub PaddingBlockStart, pub PaddingBlockEnd);

parse_logical_sides!(PaddingBlock, LengthPercentageOrAuto, PaddingBlockStart, PaddingBlockEnd);
write_logical_sides!(PaddingBlock);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingBlockStart(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingBlockEnd(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingInlineStart(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingInlineEnd(#[parsable(FromToken)] pub LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingTop(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingBottom(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingLeft(#[parsable(FromToken)] LengthPercentageOrAuto);

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaddingRight(#[parsable(FromToken)] LengthPercentageOrAuto);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Padding, 32);
		assert_size!(PaddingBlockStart, 8);
		assert_size!(PaddingBlockEnd, 8);
		assert_size!(PaddingInlineStart, 8);
		assert_size!(PaddingInlineEnd, 8);
		assert_size!(PaddingTop, 8);
		assert_size!(PaddingRight, 8);
		assert_size!(PaddingLeft, 8);
		assert_size!(PaddingBottom, 8);
	}

	#[test]
	fn test_writes() {
		// assert_parse!(PaddingLeft, "auto");
		// assert_parse!(PaddingBlock, "1px");
		assert_parse!(PaddingBlock, "1px 2px");
		// assert_parse!(PaddingInline, "1px");
		// assert_parse!(PaddingInline, "1px 2px");
		// assert_parse!(Padding, "1px");
		// assert_parse!(Padding, "1px 2px");
		// assert_parse!(Padding, "1px 2px 3px");
		// assert_parse!(Padding, "1px 2px 3px 4px");
	}
}
