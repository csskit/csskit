use super::prelude::*;
use crate::{CalcableValue, Color, Image, Percentage, Ranged};

/// <https://drafts.csswg.org/css-images-4/#funcdef-cross-fade>
///
/// ```text,ignore
/// <cross-fade()> = cross-fade( <cf-image># )
/// <cf-image> = [ <image> | <color> ] && <percentage [0,100]>?
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CrossFadeFunction<'a> {
	#[atom(CssAtomSet::CrossFade)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub params: CommaSeparated<'a, CfImage<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// An image or color with an optional percentage in a `cross-fade()` function.
///
/// ```text,ignore
/// <cf-image> = [ <image> | <color> ] && <percentage [0,100]>?
/// ```
///
/// The image and percentage can appear in either order.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CfImage<'a> {
	pub image: CfImageValue<'a>,
	pub percentage: Option<CalcableValue<'a, Ranged<Percentage, 0, 100>>>,
}

impl<'a> Peek<'a> for CfImage<'a> {
	const PEEK_KINDSET: KindSet =
		CfImageValue::PEEK_KINDSET.combine(CalcableValue::<Ranged<Percentage, 0, 100>>::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		CfImageValue::peek(p, c) || CalcableValue::<Ranged<Percentage, 0, 100>>::peek(p, c)
	}
}

impl<'a> Parse<'a> for CfImage<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Either order: <cf-image-value> <percentage>? or <percentage> <cf-image-value>
		let mut image = p.parse_if_peek::<CfImageValue>()?;
		let percentage = p.parse_if_peek::<CalcableValue<Ranged<Percentage, 0, 100>>>()?;
		if image.is_none() {
			image = Some(p.parse::<CfImageValue>()?);
		}
		Ok(Self { image: image.unwrap(), percentage })
	}
}

/// The image or color being mixed by one argument of a `cross-fade()` function.
///
/// ```text,ignore
/// [ <image> | <color> ]
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CfImageValue<'a> {
	Image(Image<'a>),
	Color(Color<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(url(a))");
		assert_parse!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(url(a)20%,url(b)80%)");
		assert_parse!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(red 20%,blue)");
		assert_parse!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(20% red,80% blue)");
		assert_parse!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(linear-gradient(red,blue)50%,url(a))");
		assert_parse!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(url(a)var(--pct))");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade()");
		assert_parse_error!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(20%)");
		assert_parse_error!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(red 120%)");
		assert_parse_error!(CssAtomSet::ATOMS, CrossFadeFunction, "cross-fade(url(a),)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("cross-fade(red 20%,url(a))", CrossFadeFunction, Color, Percentage, Image, Url);
	}
}
