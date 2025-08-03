use crate::{Image, Resolution};
use css_parse::{CommaSeparated, Function, T, function_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

function_set!(pub struct ImageSetFunctionName "image-set");

function_set!(pub struct TypeFunctionName "type");

/// <https://drafts.csswg.org/css-images-4/#funcdef-image-set>
///
/// ```text
/// <image-set()> = image-set( <image-set-option># )
/// <image-set-option> = [ <image> | <string> ] [ <resolution> || type(<string>) ]?
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImageSet<'a>(Function<'a, ImageSetFunctionName, CommaSeparated<'a, ImageSetOption<'a>>>);

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ImageSetOption<'a> {
	Image(Image<'a>, Option<ResolutionOrType<'a>>),
	String(T![String], Option<ResolutionOrType<'a>>),
}

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ResolutionOrType<'a> {
	Resolution(Resolution),
	Type(Function<'a, TypeFunctionName, T![String]>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImageSet>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ImageSet, "image-set('image.jpg'1x,'image.jpg'2x)");
		assert_parse!(ImageSet, "image-set(url('1.avif')type('image/avif'),url('2.jpg')type('image/jpeg'))");
		assert_parse!(ImageSet, "image-set(url(foo))");
	}
}
