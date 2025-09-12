use super::{GlyphOrientationVerticalKeywords, GlyphOrientationVerticalStyleValue};
use css_parse::{Parse, Parser, Peek, Result as ParseResult, T};

pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

impl<'a> Parse<'a> for GlyphOrientationVerticalStyleValue {
	fn parse(p: &mut Parser<'a>) -> ParseResult<Self> {
		match p.parse_if_peek::<GlyphOrientationVerticalKeywords>()? {
			Some(GlyphOrientationVerticalKeywords::Auto(ident)) => Ok(Self::Auto(ident)),
			None => {
				if let Some(tk) = p.parse_if_peek::<crate::CSSInt>()? {
					match tk.into() {
						0i32 => {
							return Ok(Self::Literal0(tk));
						}
						90i32 => {
							return Ok(Self::Literal90(tk));
						}
						_ => {}
					}
				}
				if let Some(tk) = p.parse_if_peek::<T![Dimension]>()? {
					match tk.into() {
						(0f32, ::css_parse::DimensionUnit::Deg) => {
							return Ok(Self::Literal0deg(tk));
						}
						(90f32, ::css_parse::DimensionUnit::Deg) => {
							return Ok(Self::Literal90deg(tk));
						}
						_ => {}
					}
				}
				Err(crate::diagnostics::Unexpected(p.next()))?
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DirectionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<UnicodeBidiStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<WritingModeStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextOrientationStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<GlyphOrientationVerticalStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TextCombineUprightStyleValue>(), 28);
	}

	#[test]
	fn test_parse() {
		assert_parse!(GlyphOrientationVerticalStyleValue, "auto");
		assert_parse!(GlyphOrientationVerticalStyleValue, "0");
		assert_parse!(GlyphOrientationVerticalStyleValue, "90");
		assert_parse!(GlyphOrientationVerticalStyleValue, "90deg");
		assert_parse!(TextCombineUprightStyleValue, "none");
		assert_parse!(TextCombineUprightStyleValue, "all");
		assert_parse!(TextCombineUprightStyleValue, "digits");
		assert_parse!(TextCombineUprightStyleValue, "digits 2");
		assert_parse!(TextCombineUprightStyleValue, "digits 4");
	}

	#[test]
	fn test_parse_error() {
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "128");
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "50deg");
		assert_parse_error!(GlyphOrientationVerticalStyleValue, "50deg");
		assert_parse_error!(TextCombineUprightStyleValue, "digits 1");
		assert_parse_error!(TextCombineUprightStyleValue, "digits 2 2");
		assert_parse_error!(TextCombineUprightStyleValue, "digits 5");
	}

	#[cfg(feature = "css_feature_data")]
	#[test]
	fn test_feature_data() {
		use crate::assert_feature_id;
		assert_feature_id!("all", TextCombineUprightStyleValue, "css.properties.text-combine-upright.all");
		assert_feature_id!("none", TextCombineUprightStyleValue, "css.properties.text-combine-upright.none");
		assert_feature_id!("digits 2", TextCombineUprightStyleValue, "css.properties.text-combine-upright");
	}
}
