use crate::{Flex, Percentage};
use css_parse::{Cursor, Diagnostic, DimensionUnit, Parse, Parser, Peek, Result, T, ToNumberValue};
use csskit_derives::{IntoCursor, Peek, ToCursors, Visitable};

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
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[visit(self)]
		pub enum Length {
			Zero(T![Number]),
			$($name(T![Dimension]),)+
		}
	}
}
apply_lengths!(define_length);

impl Length {
	const PX_CM: f32 = Self::PX_IN / 2.54;
	const PX_MM: f32 = Self::PX_IN / 25.4;
	const PX_Q: f32 = Self::PX_MM / 4.0;
	const PX_IN: f32 = 96.0;
	const PX_PC: f32 = Self::PX_IN / 6.0;
	const PX_PT: f32 = Self::PX_IN / 72.0;

	pub fn to_px(&self) -> Option<f32> {
		match self {
			Self::Zero(_) => Some(0.0),
			Self::Cm(d) => Some(Into::<f32>::into(*d) * Self::PX_CM),
			Self::Mm(d) => Some(Into::<f32>::into(*d) * Self::PX_MM),
			Self::Q(d) => Some(Into::<f32>::into(*d) * Self::PX_Q),
			Self::In(d) => Some(Into::<f32>::into(*d) * Self::PX_IN),
			Self::Pc(d) => Some(Into::<f32>::into(*d) * Self::PX_PC),
			Self::Pt(d) => Some(Into::<f32>::into(*d) * Self::PX_PT),
			_ => None,
		}
	}
}

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

impl PartialEq<f32> for Length {
	fn eq(&self, other: &f32) -> bool {
		let f: f32 = (*self).into();
		f == *other
	}
}

impl ToNumberValue for Length {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl<'a> Peek<'a> for Length {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		macro_rules! is_checks {
			( $($name: ident),+ $(,)* ) => {
				(<T![Number]>::peek(p, c) && c.token().value() == 0.0)
					|| (<T![Dimension]>::peek(p, c) && matches!(c.token().dimension_unit(), $(DimensionUnit::$name)|*))
			}
		}
		apply_lengths!(is_checks)
	}
}

impl<'a> Parse<'a> for Length {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let c = p.peek_n(1);
		macro_rules! build_steps {
			( $($name: ident),+ $(,)* ) => {
				match c.token().dimension_unit() {
					$(DimensionUnit::$name => p.parse::<T![Dimension]>().map(Self::$name),)+
					_ => p.parse::<T![Number]>().map(Self::Zero)
				}
			}
		}
		apply_lengths!(build_steps)
	}
}

#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum LengthPercentage {
	Zero(T![Number]),
	Length(Length),
	Percent(Percentage),
}

impl From<LengthPercentage> for f32 {
	fn from(val: LengthPercentage) -> Self {
		match val {
			LengthPercentage::Zero(_) => 0.0,
			LengthPercentage::Percent(f) => f.into(),
			LengthPercentage::Length(f) => f.into(),
		}
	}
}

impl ToNumberValue for LengthPercentage {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl<'a> Peek<'a> for LengthPercentage {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		(<T![Number]>::peek(p, c) && c.token().value() == 0.0) || <Percentage>::peek(p, c) || <Length>::peek(p, c)
	}
}

impl<'a> Parse<'a> for LengthPercentage {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Length>() {
			p.parse::<Length>().map(Self::Length)
		} else if p.peek::<Percentage>() {
			p.parse::<Percentage>().map(Self::Percent)
		} else {
			p.parse::<T![Number]>().map(Self::Zero)
		}
	}
}

#[derive(IntoCursor, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum LengthPercentageOrFlex {
	Flex(Flex),
	LengthPercentage(LengthPercentage),
}

impl<'a> Parse<'a> for LengthPercentageOrFlex {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			if p.peek::<Flex>() {
				Ok(Self::Flex(p.parse::<Flex>()?))
			} else {
				Ok(Self::LengthPercentage(p.parse::<LengthPercentage>()?))
			}
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
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

impl ToNumberValue for NumberLength {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl<'a> Parse<'a> for NumberLength {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			if p.peek::<Length>() {
				Ok(Self::Length(p.parse::<Length>()?))
			} else {
				let c = p.next();
				Ok(Self::Number(T![Number](c)))
			}
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
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
		assert_eq!(std::mem::size_of::<NumberLength>(), 16);
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
	}
}
