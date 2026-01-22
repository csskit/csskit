use super::prelude::*;

use crate::TransformFunction;

// https://drafts.csswg.org/css-transforms-1/#typedef-transform-list
// <transform-list> = <transform-function>+
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TransformList<'a>(pub Vec<'a, TransformFunction>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransformList>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TransformList, "matrix(1,0,0,1,0,0)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "translate(1rem)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "translateX(1rem)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "translateY(1rem)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "scale(2)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "scale(1,2)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "scaleX(2)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "scaleY(2)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "rotate(45deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "skew(1deg,2deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "skewX(1deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "skewY(1deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "rotate(180deg)scale(2,3)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "skewX(10deg)skewY(20deg)rotate(45deg)");
		assert_parse!(CssAtomSet::ATOMS, TransformList, "scale(1.5)rotate(90deg)skew(15deg,30deg)");
		assert_parse!(
			CssAtomSet::ATOMS,
			TransformList,
			"matrix(1,0,0,1,0,0)translate(1rem)translateX(1rem)translateY(1rem)scale(2)scale(1,2)scaleX(2)scaleY(2)rotate(45deg)skew(1deg,2deg)skewX(1deg)skewY(1deg)"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TransformList, "rotate(45deg) auto");
		assert_parse_error!(CssAtomSet::ATOMS, TransformList, "auto rotate(45deg)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("scale(2)", TransformList, TransformFunction, ScaleFunction);
		assert_visits!(
			"rotate(45deg) scale(2)",
			TransformList,
			TransformFunction,
			RotateFunction,
			TransformFunction,
			ScaleFunction
		);
		assert_visits!(
			"translate(1rem) rotate(90deg) scale(1.5)",
			TransformList,
			TransformFunction,
			TranslateFunction,
			TransformFunction,
			RotateFunction,
			TransformFunction,
			ScaleFunction
		);
	}
}
