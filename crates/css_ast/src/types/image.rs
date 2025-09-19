use super::prelude::*;

use crate::{Gradient, Url};

/// <https://drafts.csswg.org/css-images-3/#typedef-image>
///
/// ```text
/// <image> = <url> | <gradient>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum Image<'a> {
	Url(Url),
	Gradient(Gradient<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_visits;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Image>(), 216);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Image, "url('foo')");
		assert_parse!(Image, "url(\"foo\")");
		assert_parse!(Image, "url(foo)");
	}

	#[test]
	fn test_visits() {
		assert_visits!("url('foo')", Image, Url);
		assert_visits!("linear-gradient(red, blue)", Image, Gradient, LinearGradientFunction);
	}
}
