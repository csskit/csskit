use crate::{Alpha, Chromaticity, Matrix3, Number, ToAlpha};
use core::fmt;

macro_rules! xyz_color {
	($(#[$meta:meta])* $ty:ident, $css:literal, $white:ident) => {
		$(#[$meta])*
		/// The components are:
		/// - X - a number between 0.0 and 100.0
		/// - Y - a number between 0.0 and 100.0
		/// - Z - a number between 0.0 and 100.0
		/// - Alpha - a number between 0.0 and 100.0
		#[derive(Debug, Clone, Copy, PartialEq)]
		pub struct $ty {
			pub x: f64,
			pub y: f64,
			pub z: f64,
			pub alpha: f32,
		}

		impl $ty {
			/// The white point these tristimulus values are relative to.
			pub const WHITE: Chromaticity = Chromaticity::$white;

			pub fn new(x: f64, y: f64, z: f64, alpha: f32) -> Self {
				Self { x, y, z, alpha: alpha.clamp(0.0, 100.0) }
			}
		}

		impl ToAlpha for $ty {
			fn to_alpha(&self) -> f32 {
				self.alpha
			}
		}

		impl fmt::Display for $ty {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				let Self { x, y, z, alpha } = *self;
				write!(f, concat!("color(", $css, " {} {} {}{})"), Number(x / 100.0), Number(y / 100.0), Number(z / 100.0), Alpha(alpha))
			}
		}
	};
}

xyz_color!(
	/// A colour expressed as X, Y and Z values in the CIE XYZ tristimulus colour space, with an explicit D50 white
	/// point.
	XyzD50,
	"xyz-d50",
	D50
);

xyz_color!(
	/// A colour expressed as X, Y and Z values in the CIE XYZ tristimulus colour space, with an explicit D65 white
	/// point.
	XyzD65,
	"xyz-d65",
	D65
);

const D50_TO_D65: Matrix3 = XyzD50::WHITE.adapt_to(XyzD65::WHITE);
const D65_TO_D50: Matrix3 = XyzD65::WHITE.adapt_to(XyzD50::WHITE);

impl From<XyzD50> for XyzD65 {
	fn from(value: XyzD50) -> Self {
		let XyzD50 { x, y, z, alpha } = value;
		let [x, y, z] = D50_TO_D65.transform([x, y, z]);
		XyzD65::new(x, y, z, alpha)
	}
}

impl From<XyzD65> for XyzD50 {
	fn from(value: XyzD65) -> Self {
		let XyzD65 { x, y, z, alpha } = value;
		let [x, y, z] = D65_TO_D50.transform([x, y, z]);
		XyzD50::new(x, y, z, alpha)
	}
}
