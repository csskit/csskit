use super::prelude::*;
use crate::{Color, UrlOrString};

/// <https://drafts.csswg.org/css-images-4/#funcdef-image>
///
/// ```text,ignore
/// <image()> = image( <image-tags>? [ <image-src>? , <color>? ]! )
/// <image-tags> = [ ltr | rtl ]
/// <image-src> = [ <url> | <string> ]
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ImageFunction<'a> {
	#[atom(CssAtomSet::Image)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub params: ImageFunctionParams<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// The arguments of an `image()` function.
///
/// ```text,ignore
/// <image-tags>? [ <image-src>? , <color>? ]!
/// ```
///
/// At least one of the `<image-src>` or the `<color>` must be present, and the comma is only
/// written when both are.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ImageFunctionParams<'a> {
	pub tags: Option<ImageTags>,
	pub src: Option<UrlOrString>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub color: Option<Color<'a>>,
}

impl<'a> Peek<'a> for ImageFunctionParams<'a> {
	const PEEK_KINDSET: KindSet =
		ImageTags::PEEK_KINDSET.combine(UrlOrString::PEEK_KINDSET).combine(Color::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		ImageTags::peek(p, c) || UrlOrString::peek(p, c) || Color::peek(p, c)
	}
}

impl<'a> Parse<'a> for ImageFunctionParams<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let tags = p.parse_if_peek::<ImageTags>()?;
		let src = p.parse_if_peek::<UrlOrString>()?;
		// Omitting the `<image-src>` also omits the comma, in which case the `<color>` is required.
		let (comma, color) = if src.is_some() {
			match p.parse_if_peek::<T![,]>()? {
				Some(comma) => (Some(comma), Some(p.parse::<Color>()?)),
				None => (None, None),
			}
		} else {
			(None, Some(p.parse::<Color>()?))
		};
		Ok(Self { tags, src, comma, color })
	}
}

/// The directionality of an `image()` function.
///
/// ```text,ignore
/// <image-tags> = [ ltr | rtl ]
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ImageTags {
	#[atom(CssAtomSet::Ltr)]
	Ltr(T![Ident]),
	#[atom(CssAtomSet::Rtl)]
	Rtl(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ImageFunction, "image(url(foo))");
		assert_parse!(CssAtomSet::ATOMS, ImageFunction, "image('foo.png')");
		assert_parse!(CssAtomSet::ATOMS, ImageFunction, "image(red)");
		assert_parse!(CssAtomSet::ATOMS, ImageFunction, "image(url(foo),red)");
		assert_parse!(CssAtomSet::ATOMS, ImageFunction, "image(ltr url(foo))");
		assert_parse!(CssAtomSet::ATOMS, ImageFunction, "image(rtl 'foo.png',rgb(0 0 0))");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ImageFunction, "image()");
		assert_parse_error!(CssAtomSet::ATOMS, ImageFunction, "image(ltr)");
		assert_parse_error!(CssAtomSet::ATOMS, ImageFunction, "image(url(foo),)");
		assert_parse_error!(CssAtomSet::ATOMS, ImageFunction, "image(,red)");
		assert_parse_error!(CssAtomSet::ATOMS, ImageFunction, "image(url(foo)red)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("image(url(foo),red)", ImageFunction, UrlOrString, Color);
	}
}
