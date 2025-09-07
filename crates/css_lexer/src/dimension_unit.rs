use std::hash::Hash;

/// Represents a [Kind::Dimension's][crate::Kind::Dimension] unit, if it is "known": defined by the CSS grammar.
///
/// In order to more efficiently lex CSS, the known dimension units are encoded into [Tokens][crate::Token]. This
/// avoids the need for subsequent downstream passes to consult the underlying string data to establish what kind of
/// dimension unit the [Kind::Dimension][crate::Kind::Dimension] represents.
///
/// This enum captures all known and defined units per the CSS specifications. Custom units (e.g. those beginning with a
/// double dash (`--`)), unknown units, or units using escape characters will all be represented using the
/// [DimensionUnit::Unknown] variant which means the underlying `&str` must be consulted.
///
/// # Example
///
/// ```
/// use css_lexer::*;
/// let mut lexer = Lexer::new("10px");
/// {
///     let token = lexer.advance();
///     assert_eq!(token, Kind::Dimension);
///     assert_eq!(token, DimensionUnit::Px);
///     let unit = token.dimension_unit();
///     let str: &'static str = unit.into();
///     assert_eq!(str, "px");
/// }
/// ```
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "lowercase"))]
pub enum DimensionUnit {
	#[default]
	Unknown = 0,
	Cap,
	Ch,
	Cm,
	Cqb,
	Cqh,
	Cqi,
	Cqmax,
	Cqmin,
	Cqw,
	Db,
	Deg,
	Dpcm,
	Dpi,
	Dppx,
	Dvb,
	Dvh,
	Dvi,
	Dvmax,
	Dvmin,
	Dvw,
	Em,
	Ex,
	Fr,
	Grad,
	Hz,
	Ic,
	In,
	Khz,
	Lh,
	Lvb,
	Lvh,
	Lvi,
	Lvmax,
	Lvmin,
	Lvw,
	Mm,
	Ms,
	Pc,
	#[cfg_attr(feature = "serde", serde(rename = "%"))]
	Percent,
	Pt,
	Px,
	Q,
	Rad,
	Rcap,
	Rch,
	Rem,
	Rex,
	Ric,
	Rlh,
	S,
	Svb,
	Svh,
	Svi,
	Svmax,
	Svmin,
	Svw,
	Turn,
	Vb,
	Vh,
	Vi,
	Vmax,
	Vmin,
	Vw,
	X,
}

impl DimensionUnit {
	pub fn is_empty(&self) -> bool {
		self == &Self::Unknown
	}

	pub const fn len(&self) -> u32 {
		match self {
			Self::Unknown => 0,
			Self::Percent | Self::Q | Self::S | Self::X => 1,
			Self::Db
			| Self::Ch
			| Self::Cm
			| Self::Em
			| Self::Ex
			| Self::Fr
			| Self::Hz
			| Self::Ic
			| Self::In
			| Self::Lh
			| Self::Mm
			| Self::Ms
			| Self::Pc
			| Self::Pt
			| Self::Px
			| Self::Vb
			| Self::Vh
			| Self::Vi
			| Self::Vw => 2,
			Self::Cap
			| Self::Cqb
			| Self::Cqh
			| Self::Cqi
			| Self::Cqw
			| Self::Deg
			| Self::Dpi
			| Self::Dvb
			| Self::Dvi
			| Self::Dvh
			| Self::Dvw
			| Self::Khz
			| Self::Lvb
			| Self::Lvi
			| Self::Lvh
			| Self::Lvw
			| Self::Rad
			| Self::Rch
			| Self::Rem
			| Self::Rex
			| Self::Ric
			| Self::Rlh
			| Self::Svb
			| Self::Svi
			| Self::Svh
			| Self::Svw => 3,
			Self::Dpcm | Self::Dppx | Self::Grad | Self::Rcap | Self::Turn | Self::Vmax | Self::Vmin => 4,
			Self::Dvmax
			| Self::Dvmin
			| Self::Lvmax
			| Self::Lvmin
			| Self::Svmax
			| Self::Svmin
			| Self::Cqmax
			| Self::Cqmin => 5,
		}
	}

	pub(crate) const fn from_u8(value: u8) -> Self {
		let unit = match value {
			1 => Self::Cap,
			2 => Self::Ch,
			3 => Self::Cm,
			4 => Self::Cqb,
			5 => Self::Cqh,
			6 => Self::Cqi,
			7 => Self::Cqmax,
			8 => Self::Cqmin,
			9 => Self::Cqw,
			10 => Self::Db,
			11 => Self::Deg,
			12 => Self::Dpcm,
			13 => Self::Dpi,
			14 => Self::Dppx,
			15 => Self::Dvb,
			16 => Self::Dvh,
			17 => Self::Dvi,
			18 => Self::Dvmax,
			19 => Self::Dvmin,
			20 => Self::Dvw,
			21 => Self::Em,
			22 => Self::Ex,
			23 => Self::Fr,
			24 => Self::Grad,
			25 => Self::Hz,
			26 => Self::Ic,
			27 => Self::In,
			28 => Self::Khz,
			29 => Self::Lh,
			30 => Self::Lvb,
			31 => Self::Lvh,
			32 => Self::Lvi,
			33 => Self::Lvmax,
			34 => Self::Lvmin,
			35 => Self::Lvw,
			36 => Self::Mm,
			37 => Self::Ms,
			38 => Self::Pc,
			39 => Self::Percent,
			40 => Self::Pt,
			41 => Self::Px,
			42 => Self::Q,
			43 => Self::Rad,
			44 => Self::Rcap,
			45 => Self::Rch,
			46 => Self::Rem,
			47 => Self::Rex,
			48 => Self::Ric,
			49 => Self::Rlh,
			50 => Self::S,
			51 => Self::Svb,
			52 => Self::Svh,
			53 => Self::Svi,
			54 => Self::Svmax,
			55 => Self::Svmin,
			56 => Self::Svw,
			57 => Self::Turn,
			58 => Self::Vb,
			59 => Self::Vh,
			60 => Self::Vi,
			61 => Self::Vmax,
			62 => Self::Vmin,
			63 => Self::Vw,
			64 => Self::X,
			_ => Self::Unknown,
		};
		debug_assert!(unit as u8 == value, "DimensionUnit::from_u8 failed as unit was not equal to value");
		unit
	}
}

impl From<u8> for DimensionUnit {
	fn from(value: u8) -> Self {
		Self::from_u8(value)
	}
}

impl From<&str> for DimensionUnit {
	fn from(value: &str) -> Self {
		match value {
			"cap" => Self::Cap,
			"ch" => Self::Ch,
			"cm" => Self::Cm,
			"cqb" => Self::Cqb,
			"cqh" => Self::Cqh,
			"cqi" => Self::Cqi,
			"cqmax" => Self::Cqmax,
			"cqmin" => Self::Cqmin,
			"cqw" => Self::Cqw,
			"deg" => Self::Deg,
			"dpcm" => Self::Dpcm,
			"dpi" => Self::Dpi,
			"dppx" => Self::Dppx,
			"dvb" => Self::Dvb,
			"dvh" => Self::Dvh,
			"dvi" => Self::Dvi,
			"dvmax" => Self::Dvmax,
			"dvmin" => Self::Dvmin,
			"dvw" => Self::Dvw,
			"em" => Self::Em,
			"ex" => Self::Ex,
			"fr" => Self::Fr,
			"grad" => Self::Grad,
			"hz" => Self::Hz,
			"ic" => Self::Ic,
			"in" => Self::In,
			"khz" => Self::Khz,
			"lh" => Self::Lh,
			"lvb" => Self::Lvb,
			"lvh" => Self::Lvh,
			"lvi" => Self::Lvi,
			"lvmax" => Self::Lvmax,
			"lvmin" => Self::Lvmin,
			"lvw" => Self::Lvw,
			"mm" => Self::Mm,
			"ms" => Self::Ms,
			"pc" => Self::Pc,
			"percent" => Self::Percent,
			"pt" => Self::Pt,
			"px" => Self::Px,
			"q" => Self::Q,
			"rad" => Self::Rad,
			"rcap" => Self::Rcap,
			"rch" => Self::Rch,
			"rem" => Self::Rem,
			"rex" => Self::Rex,
			"ric" => Self::Ric,
			"rlh" => Self::Rlh,
			"s" => Self::S,
			"svb" => Self::Svb,
			"svh" => Self::Svh,
			"svi" => Self::Svi,
			"svmax" => Self::Svmax,
			"svmin" => Self::Svmin,
			"svw" => Self::Svw,
			"turn" => Self::Turn,
			"vb" => Self::Vb,
			"vh" => Self::Vh,
			"vi" => Self::Vi,
			"vmax" => Self::Vmax,
			"vmin" => Self::Vmin,
			"vw" => Self::Vw,
			"x" => Self::X,
			_ => Self::Unknown,
		}
	}
}

impl From<DimensionUnit> for &'static str {
	fn from(value: DimensionUnit) -> &'static str {
		match value {
			DimensionUnit::Unknown => "",
			DimensionUnit::Cap => "cap",
			DimensionUnit::Ch => "ch",
			DimensionUnit::Cm => "cm",
			DimensionUnit::Cqb => "cqb",
			DimensionUnit::Cqh => "cqh",
			DimensionUnit::Cqi => "cqi",
			DimensionUnit::Cqmax => "cqmax",
			DimensionUnit::Cqmin => "cqmin",
			DimensionUnit::Cqw => "cqw",
			DimensionUnit::Db => "db",
			DimensionUnit::Deg => "deg",
			DimensionUnit::Dpcm => "dpcm",
			DimensionUnit::Dpi => "dpi",
			DimensionUnit::Dppx => "dppx",
			DimensionUnit::Dvb => "dvb",
			DimensionUnit::Dvh => "dvh",
			DimensionUnit::Dvi => "dvi",
			DimensionUnit::Dvmax => "dvmax",
			DimensionUnit::Dvmin => "dvmin",
			DimensionUnit::Dvw => "dvw",
			DimensionUnit::Em => "em",
			DimensionUnit::Ex => "ex",
			DimensionUnit::Fr => "fr",
			DimensionUnit::Grad => "grad",
			DimensionUnit::Hz => "hz",
			DimensionUnit::Ic => "ic",
			DimensionUnit::In => "in",
			DimensionUnit::Khz => "khz",
			DimensionUnit::Lh => "lh",
			DimensionUnit::Lvb => "lvb",
			DimensionUnit::Lvh => "lvh",
			DimensionUnit::Lvi => "lvi",
			DimensionUnit::Lvmax => "lvmax",
			DimensionUnit::Lvmin => "lvmin",
			DimensionUnit::Lvw => "lvw",
			DimensionUnit::Mm => "mm",
			DimensionUnit::Ms => "ms",
			DimensionUnit::Pc => "pc",
			DimensionUnit::Percent => "%",
			DimensionUnit::Pt => "pt",
			DimensionUnit::Px => "px",
			DimensionUnit::Q => "q",
			DimensionUnit::Rad => "rad",
			DimensionUnit::Rcap => "rcap",
			DimensionUnit::Rch => "rch",
			DimensionUnit::Rem => "rem",
			DimensionUnit::Rex => "rex",
			DimensionUnit::Ric => "ric",
			DimensionUnit::Rlh => "rlh",
			DimensionUnit::S => "s",
			DimensionUnit::Svb => "svb",
			DimensionUnit::Svh => "svh",
			DimensionUnit::Svi => "svi",
			DimensionUnit::Svmax => "svmin",
			DimensionUnit::Svmin => "svmax",
			DimensionUnit::Svw => "svw",
			DimensionUnit::Turn => "turn",
			DimensionUnit::Vb => "vb",
			DimensionUnit::Vh => "vh",
			DimensionUnit::Vi => "vi",
			DimensionUnit::Vmax => "vmax",
			DimensionUnit::Vmin => "vmin",
			DimensionUnit::Vw => "vw",
			DimensionUnit::X => "x",
		}
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<DimensionUnit>(), 1);
}
