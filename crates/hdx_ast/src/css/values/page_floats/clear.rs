#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-page-floats-3/#propdef-clear
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Clear {
	InlineStart, // atom!("inline-start")
	InlineEnd,   // atom!("inline-end")
	BlockStart,  // atom!("block-start")
	BlockEnd,    // atom!("block-end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Top,         // atom!("top")
	Bottom,      // atom!("bottom")
	BothInline,  // atom!("both-inline")
	BothBlock,   // atom!("both-block")
	Both,        // atom!("both")
	#[default]
	None, // atom!("none")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Clear>(), 1);
	}
}
