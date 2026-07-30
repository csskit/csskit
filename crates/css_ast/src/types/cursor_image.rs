use super::prelude::*;
use crate::{CalcableValue, ImageSetFunction, Url};

/// <https://drafts.csswg.org/css-ui-4/#typedef-cursor-cursor-image>
///
/// ```text
/// <cursor-image> = [ <url> | <url-set> ] <number>{2}?
/// ```
///
/// `<url-set>` is a limited version of image-set(), where the `<image>` sub-production is restricted to `<url>` only.
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CursorImage<'a> {
	Url(
		Url,
		#[cfg_attr(feature = "visitable", visit(skip))]
		Option<(CalcableValue<'a, T![Number]>, CalcableValue<'a, T![Number]>)>,
	),
	UrlSet(
		ImageSetFunction<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		Option<(CalcableValue<'a, T![Number]>, CalcableValue<'a, T![Number]>)>,
	),
}

impl<'a> Parse<'a> for CursorImage<'a> {
	fn parse<I>(p: &mut css_parse::Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<ImageSetFunction>() {
			let image_set = p.parse::<ImageSetFunction>()?;
			let mut numbers = None;
			if p.peek::<CalcableValue<T![Number]>>() {
				let a = p.parse::<CalcableValue<T![Number]>>()?;
				let b = p.parse::<CalcableValue<T![Number]>>()?;
				numbers = Some((a, b));
			}
			Ok(Self::UrlSet(image_set, numbers))
		} else {
			let url = p.parse::<Url>()?;
			let mut numbers = None;
			if p.peek::<CalcableValue<T![Number]>>() {
				let a = p.parse::<CalcableValue<T![Number]>>()?;
				let b = p.parse::<CalcableValue<T![Number]>>()?;
				numbers = Some((a, b));
			}
			Ok(Self::Url(url, numbers))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CursorImage, "url(hyper.cur)");
		assert_parse!(CssAtomSet::ATOMS, CursorImage, "url(hyper.png)2 3");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, CursorImage, "url(hyper.png)var(--x) 3");
		assert_parse!(CssAtomSet::ATOMS, CursorImage, "url(hyper.png)calc(1 + 1) calc(2 + 1)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("url(hyper.cur)", CursorImage, Url);
		assert_visits!("url(hyper.png) 2 3", CursorImage, Url);
		assert_visits!("image-set(url('foo.jpg') 1x)", CursorImage, ImageSetFunction);
	}
}
