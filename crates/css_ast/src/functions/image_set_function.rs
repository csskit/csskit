use super::prelude::*;
use crate::{CalcableValue, Image, Resolution};

/// <https://drafts.csswg.org/css-images-4/#funcdef-image-set>
///
/// ```text
/// <image-set()> = image-set( <image-set-option># )
/// <image-set-option> = [ <image> | <string> ] [ <resolution> || type(<string>) ]?
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ImageSetFunction<'a> {
	#[atom(CssAtomSet::ImageSet)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, ImageSetParams<'a>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ImageSetParams<'a> {
	Image(Image<'a>, Option<ResolutionOrType<'a>>),
	String(T![String], Option<ResolutionOrType<'a>>),
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ResolutionOrType<'a> {
	Resolution(CalcableValue<'a, Resolution>),
	Type(#[atom(CssAtomSet::Type)] T![Function], T![String], #[semantic_eq(skip)] T![')']),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImageSetFunction>(), 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ImageSetFunction, "image-set('image.jpg'1x,'image.jpg'2x)");
		assert_parse!(
			CssAtomSet::ATOMS,
			ImageSetFunction,
			"image-set(url('1.avif')type('image/avif'),url('2.jpg')type('image/jpeg'))"
		);
		assert_parse!(CssAtomSet::ATOMS, ImageSetFunction, "image-set(url(foo))");
		assert_parse!(CssAtomSet::ATOMS, ImageSetFunction, "image-set('a.jpg'calc(1x * 2))");
		assert_parse!(CssAtomSet::ATOMS, ImageSetFunction, "image-set('a.jpg'var(--dpi))");
	}
}
