use css_lexer::Cursor;
use css_parse::{Build, Parser, Peek, T};
use csskit_derives::{IntoCursor, Peek, ToCursors, Visitable};

use super::Flex;

// const PX_CM: f32 = PX_IN / 2.54;
// const PX_MM: f32 = PX_IN / 25.4;
// const PX_Q: f32 = PX_MM / 4.0;
// const PX_IN: f32 = 96.0;
// const PX_PC: f32 = PX_IN / 6.0;
// const PX_PT: f32 = PX_IN / 72.0;

macro_rules! apply_lengths {
	($ident: ident) => {
		$ident! {
			// https://drafts.csswg.org/css-values/#font-relative-lengths
			Em,
			Rem,
			Ex,
			Rex,
			Cap,
			Rcap,
			Ch,
			Rch,
			Ic,
			Ric,
			Lh,
			Rlh,

			// https://drafts.csswg.org/css-values/#viewport-relative-units
			Vw,
			Svw,
			Lvw,
			Dvw,
			Vh,
			Svh,
			Lvh,
			Dvh,
			Vi,
			Svi,
			Lvi,
			Dvi,
			Vb,
			Svb,
			Lvb,
			Dvb,
			Vmin,
			Svmin,
			Lvmin,
			Dvmin,
			Vmax,
			Svmax,
			Lvmax,
			Dvmax,

			// https://drafts.csswg.org/css-values/#absolute-lengths
			Cm,
			Mm,
			Q,
			In,
			Pc,
			Pt,
			Px,

			// https://www.w3.org/TR/css-contain-3/#container-lengths
			Cqw,
			Cqh,
			Cqi,
			Cqb,
			Cqmin,
			Cqmax,
		}
	};
}

macro_rules! define_length {
	( $($name: ident),+ $(,)* ) => {
		#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		#[visit(self)]
		pub enum Length {
			Zero(T![Number]),
			$($name(T![Dimension::$name]),)+
		}
	}
}
apply_lengths!(define_length);

impl From<Length> for f32 {
	fn from(val: Length) -> Self {
		macro_rules! match_length {
			( $($name: ident),+ $(,)* ) => {
				match val {
					Length::Zero(_) => 0.0,
					$(Length::$name(f) => f.into()),+
				}
			}
		}
		apply_lengths!(match_length)
	}
}

impl<'a> Peek<'a> for Length {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		macro_rules! is_checks {
			( $($name: ident),+ $(,)* ) => {
				(<T![Number]>::peek(p, c) && c.token().value() == 0.0)
					$(|| <T![Dimension::$name]>::peek(p, c))+
			}
		}
		apply_lengths!(is_checks)
	}
}

impl<'a> Build<'a> for Length {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		macro_rules! build_steps {
			( $($name: ident),+ $(,)* ) => {
				$(if <T![Dimension::$name]>::peek(p, c) {
					Self::$name(<T![Dimension::$name]>::build(p, c))
				} else )+ {
					Self::Zero(<T![Number]>::build(p, c))
				}
			}
		}
		apply_lengths!(build_steps)
	}
}

macro_rules! define_length_percentage {
	( $($name: ident),+ $(,)* ) => {
		#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		#[visit(self)]
		pub enum LengthPercentage {
			Zero(T![Number]),
			$($name(T![Dimension::$name]),)+
			Percent(T![Dimension::%]),
		}
	}
}
apply_lengths!(define_length_percentage);

impl From<LengthPercentage> for f32 {
	fn from(val: LengthPercentage) -> Self {
		macro_rules! match_length {
			( $($name: ident),+ $(,)* ) => {
				match val {
					LengthPercentage::Zero(_) => 0.0,
					LengthPercentage::Percent(f) => f.into(),
					$(LengthPercentage::$name(f) => f.into()),+
				}
			}
		}
		apply_lengths!(match_length)
	}
}

impl<'a> Peek<'a> for LengthPercentage {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		macro_rules! is_checks {
			( $($name: ident),+ $(,)* ) => {
				(<T![Number]>::peek(p, c) && c.token().value() == 0.0)
				|| <T![Dimension::%]>::peek(p, c)
					$(|| <T![Dimension::$name]>::peek(p, c))+
			}
		}
		apply_lengths!(is_checks)
	}
}

impl<'a> Build<'a> for LengthPercentage {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		macro_rules! build_steps {
			( $($name: ident),+ $(,)* ) => {
				$(if <T![Dimension::$name]>::peek(p, c) {
					Self::$name(<T![Dimension::$name]>::build(p, c))
				} else )+ if <T![Dimension::%]>::peek(p, c) {
					Self::Percent(<T![Dimension::%]>::build(p, c))
				} else {
					Self::Zero(<T![Number]>::build(p, c))
				}
			}
		}
		apply_lengths!(build_steps)
	}
}

#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum LengthOrAuto {
	#[visit(skip)]
	Auto(T![Ident]),
	Length(Length),
}

impl<'a> Peek<'a> for LengthOrAuto {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Length::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "auto"))
	}
}

impl<'a> Build<'a> for LengthOrAuto {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if Length::peek(p, c) { Self::Length(Length::build(p, c)) } else { Self::Auto(<T![Ident]>::build(p, c)) }
	}
}

#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum LengthOrNone {
	#[visit(skip)]
	None(T![Ident]),
	Length(Length),
}

impl<'a> Peek<'a> for LengthOrNone {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Length::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "none"))
	}
}

impl<'a> Build<'a> for LengthOrNone {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if Length::peek(p, c) { Self::Length(Length::build(p, c)) } else { Self::None(<T![Ident]>::build(p, c)) }
	}
}

#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum LengthPercentageOrAuto {
	#[visit(skip)]
	Auto(T![Ident]),
	LengthPercentage(LengthPercentage),
}

impl<'a> Peek<'a> for LengthPercentageOrAuto {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		LengthPercentage::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "auto"))
	}
}

impl<'a> Build<'a> for LengthPercentageOrAuto {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if LengthPercentage::peek(p, c) {
			Self::LengthPercentage(LengthPercentage::build(p, c))
		} else {
			Self::Auto(<T![Ident]>::build(p, c))
		}
	}
}

#[derive(IntoCursor, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(children)]
pub enum LengthPercentageOrFlex {
	Flex(Flex),
	LengthPercentage(LengthPercentage),
}

impl<'a> Build<'a> for LengthPercentageOrFlex {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if Flex::peek(p, c) {
			Self::Flex(Flex::build(p, c))
		} else {
			Self::LengthPercentage(LengthPercentage::build(p, c))
		}
	}
}

#[derive(Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum NumberLength {
	#[visit(skip)]
	Number(T![Number]),
	Length(Length),
}

impl From<NumberLength> for f32 {
	fn from(val: NumberLength) -> Self {
		match val {
			NumberLength::Number(n) => n.into(),
			NumberLength::Length(n) => n.into(),
		}
	}
}

impl<'a> Build<'a> for NumberLength {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if Length::peek(p, c) { Self::Length(Length::build(p, c)) } else { Self::Number(<T![Number]>::build(p, c)) }
	}
}

#[derive(Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum NumberPercentage {
	Number(T![Number]),
	Percentage(T![Dimension::%]),
}

impl From<NumberPercentage> for f32 {
	fn from(val: NumberPercentage) -> Self {
		match val {
			NumberPercentage::Number(n) => n.into(),
			NumberPercentage::Percentage(n) => n.into(),
		}
	}
}

impl<'a> Build<'a> for NumberPercentage {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else {
			Self::Percentage(<T![Dimension::%]>::build(p, c))
		}
	}
}

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NumberLengthOrAuto {
	NumberLength(NumberLength),
	Auto(T![Ident]),
}

impl From<NumberLengthOrAuto> for f32 {
	fn from(val: NumberLengthOrAuto) -> Self {
		match val {
			NumberLengthOrAuto::NumberLength(n) => n.into(),
			NumberLengthOrAuto::Auto(_) => 0.0,
		}
	}
}

impl<'a> Peek<'a> for NumberLengthOrAuto {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		NumberLength::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "auto"))
	}
}

impl<'a> Build<'a> for NumberLengthOrAuto {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		if NumberLength::peek(p, c) {
			Self::NumberLength(NumberLength::build(p, c))
		} else {
			Self::Auto(<T![Ident]>::build(p, c))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Length>(), 16);
		assert_eq!(std::mem::size_of::<LengthPercentage>(), 16);
		assert_eq!(std::mem::size_of::<LengthPercentageOrAuto>(), 16);
		assert_eq!(std::mem::size_of::<NumberLength>(), 16);
		assert_eq!(std::mem::size_of::<NumberPercentage>(), 16);
		assert_eq!(std::mem::size_of::<NumberLengthOrAuto>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Length, "10px");
		// Truncates to 7dp
		assert_parse!(Length, "1.2345679px");
		// Removes redundant dp
		assert_parse!(Length, "-1px");
		// Percent
		assert_parse!(LengthPercentage, "1%");
		assert_parse!(LengthPercentageOrAuto, "auto");
	}
}
