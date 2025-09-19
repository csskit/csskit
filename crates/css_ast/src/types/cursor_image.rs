use super::prelude::*;
use crate::{ImageSetFunction, Url};

/// <https://drafts.csswg.org/css-ui-4/#typedef-cursor-cursor-image>
///
/// ```text
/// <cursor-image> = [ <url> | <url-set> ] <number>{2}?
/// ```
///
/// `<url-set>` is a limited version of image-set(), where the `<image>` sub-production is restricted to `<url>` only.
#[derive(Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum CursorImage<'a> {
	Url(Url, #[visit(skip)] Option<(T![Number], T![Number])>),
	UrlSet(ImageSetFunction<'a>, #[visit(skip)] Option<(T![Number], T![Number])>),
}

impl<'a> Parse<'a> for CursorImage<'a> {
	fn parse(p: &mut css_parse::Parser<'a>) -> ParserResult<Self> {
		if p.peek::<ImageSetFunction>() {
			let image_set = p.parse::<ImageSetFunction>()?;
			let mut numbers = None;
			if p.peek::<T![Number]>() {
				let a = p.parse::<T![Number]>()?;
				let b = p.parse::<T![Number]>()?;
				numbers = Some((a, b));
			}
			Ok(Self::UrlSet(image_set, numbers))
		} else {
			let url = p.parse::<Url>()?;
			let mut numbers = None;
			if p.peek::<T![Number]>() {
				let a = p.parse::<T![Number]>()?;
				let b = p.parse::<T![Number]>()?;
				numbers = Some((a, b));
			}
			Ok(Self::Url(url, numbers))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_visits;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CursorImage>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CursorImage, "url(hyper.cur)");
		assert_parse!(CursorImage, "url(hyper.png)2 3");
	}

	#[test]
	fn test_visits() {
		assert_visits!("url(hyper.cur)", CursorImage, Url);
		assert_visits!("url(hyper.png) 2 3", CursorImage, Url);
		assert_visits!("image-set(url('foo.jpg') 1x)", CursorImage, ImageSetFunction);
	}
}
