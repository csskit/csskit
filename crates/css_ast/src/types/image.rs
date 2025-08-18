use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::{Gradient, Url};

/// <https://drafts.csswg.org/css-images-3/#typedef-image>
///
/// ```text
/// <image> = <url> | <gradient>
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Image<'a> {
	Url(Url),
	Gradient(Gradient<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
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
}
