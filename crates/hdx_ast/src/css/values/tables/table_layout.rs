#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-tables-3/#propdef-table-layout
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TableLayout {
	#[default]
	Auto, // atom!("auto")
	Fixed, // atom!("fixed")
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<TableLayout>(), 1);
	}
}
