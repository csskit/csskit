use super::prelude::*;
use crate::{Flex, Percentage};

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
		#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self), metadata(skip))]
		pub enum Length {
			Zero(#[in_range(0.0..0.0)] T![Number]),
			$(
				#[atom(CssAtomSet::$name)]
				$name(T![Dimension]),
			)+
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

#[cfg(feature = "visitable")]
impl css_parse::NodeWithMetadata<crate::CssMetadata> for Length {
	fn metadata(&self) -> crate::CssMetadata {
		crate::CssMetadata { node_kinds: crate::NodeKinds::Dimension, ..Default::default() }
	}
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum LengthPercentage {
	#[cfg_attr(feature = "visitable", visit(skip))]
	Zero(#[in_range(0.0..0.0)] T![Number]),
	Length(Length),
	#[cfg_attr(feature = "visitable", visit(skip))]
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

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub enum LengthPercentageOrFlex {
	Flex(Flex),
	LengthPercentage(LengthPercentage),
}

#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum NumberLength {
	#[cfg_attr(feature = "visitable", visit(skip))]
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Length>(), 16);
		assert_eq!(std::mem::size_of::<LengthPercentage>(), 16);
		assert_eq!(std::mem::size_of::<NumberLength>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Length, "10px");
		// Truncates to 7dp
		assert_parse!(CssAtomSet::ATOMS, Length, "1.2345679px");
		// Removes redundant dp
		assert_parse!(CssAtomSet::ATOMS, Length, "-1px");
		// Percent
		assert_parse!(CssAtomSet::ATOMS, LengthPercentage, "1%");
	}
}
