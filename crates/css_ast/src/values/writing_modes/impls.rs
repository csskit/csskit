use super::GlyphOrientationVerticalStyleValue;
use crate::{CSSInt, CssAtomSet, Exact};
use css_parse::{Cursor, Diagnostic, Parse, Parser, Peek, Result as ParseResult, T};

impl<'a> Parse<'a> for GlyphOrientationVerticalStyleValue {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if <T![Ident]>::peek(p, c) && p.equals_atom(c, &CssAtomSet::Auto) {
			p.parse::<T![Ident]>().map(Self::Auto)
		} else {
			if let Some(int) = p.parse_if_peek::<CSSInt>()? {
				match int.into() {
					0i32 => {
						return Ok(Self::Literal0(Exact(int)));
					}
					90i32 => {
						return Ok(Self::Literal90(Exact(int)));
					}
					_ => {}
				}
			}
			if let Some(dimension) = p.parse_if_peek::<T![Dimension]>()? {
				match (dimension.value(), p.to_atom::<CssAtomSet>(dimension.into())) {
					(0f32, CssAtomSet::Deg) => {
						return Ok(Self::Literal0deg(Exact(dimension)));
					}
					(90f32, CssAtomSet::Deg) => {
						return Ok(Self::Literal90deg(Exact(dimension)));
					}
					_ => {}
				}
			}
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
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
		assert_parse!(CssAtomSet::ATOMS, GlyphOrientationVerticalStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, GlyphOrientationVerticalStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, GlyphOrientationVerticalStyleValue, "90");
		assert_parse!(CssAtomSet::ATOMS, GlyphOrientationVerticalStyleValue, "90deg");
		assert_parse!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "all");
		assert_parse!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "digits");
		assert_parse!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "digits 2");
		assert_parse!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "digits 4");
	}

	#[test]
	fn test_parse_error() {
		assert_parse_error!(CssAtomSet::ATOMS, GlyphOrientationVerticalStyleValue, "128");
		assert_parse_error!(CssAtomSet::ATOMS, GlyphOrientationVerticalStyleValue, "50deg");
		assert_parse_error!(CssAtomSet::ATOMS, GlyphOrientationVerticalStyleValue, "50deg");
		assert_parse_error!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "digits 1");
		assert_parse_error!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "digits 2 2");
		assert_parse_error!(CssAtomSet::ATOMS, TextCombineUprightStyleValue, "digits 5");
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
