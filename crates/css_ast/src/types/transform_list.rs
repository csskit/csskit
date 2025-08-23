use bumpalo::collections::Vec;
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::TransformFunction;

// https://drafts.csswg.org/css-transforms-1/#typedef-transform-list
// <transform-list> = <transform-function>+
#[derive(ToSpan, Peek, Parse, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TransformList<'a>(pub Vec<'a, TransformFunction>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransformList>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransformList, "matrix(1,0,0,1,0,0)");
		assert_parse!(TransformList, "translate(1rem)");
		assert_parse!(TransformList, "translateX(1rem)");
		assert_parse!(TransformList, "translateY(1rem)");
		assert_parse!(TransformList, "scale(2)");
		assert_parse!(TransformList, "scale(1,2)");
		assert_parse!(TransformList, "scaleX(2)");
		assert_parse!(TransformList, "scaleY(2)");
		assert_parse!(TransformList, "rotate(45deg)");
		assert_parse!(TransformList, "skew(1deg,2deg)");
		assert_parse!(TransformList, "skewX(1deg)");
		assert_parse!(TransformList, "skewY(1deg)");
		assert_parse!(TransformList, "rotate(180deg)scale(2,3)");
		assert_parse!(TransformList, "skewX(10deg)skewY(20deg)rotate(45deg)");
		assert_parse!(TransformList, "scale(1.5)rotate(90deg)skew(15deg,30deg)");
		assert_parse!(
			TransformList,
			"matrix(1,0,0,1,0,0)translate(1rem)translateX(1rem)translateY(1rem)scale(2)scale(1,2)scaleX(2)scaleY(2)rotate(45deg)skew(1deg,2deg)skewX(1deg)skewY(1deg)"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TransformList, "rotate(45deg) auto");
		assert_parse_error!(TransformList, "auto rotate(45deg)");
	}
}
