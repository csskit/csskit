use crate::{ImageSet, Url};
use css_parse::{Parse, Result as ParserResult, T};
use csskit_derives::{Peek, ToCursors, ToSpan};

/// <https://drafts.csswg.org/css-ui-4/#typedef-cursor-cursor-image>
///
/// ```text
/// <cursor-image> = [ <url> | <url-set> ] <number>{2}?
/// ```
#[derive(Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum CursorImage<'a> {
	Url(Url, Option<(T![Number], T![Number])>),
	UrlSet(ImageSet<'a>, Option<(T![Number], T![Number])>),
}

impl<'a> Parse<'a> for CursorImage<'a> {
	fn parse(p: &mut css_parse::Parser<'a>) -> ParserResult<Self> {
		if p.peek::<ImageSet>() {
			let image_set = p.parse::<ImageSet>()?;
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
}
