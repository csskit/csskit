use crate::{LinearRgb, ToAlpha, XyzD65, round_dp};
use core::fmt;

/// A colour in the Display P3 colour space.
/// The components are:
/// - Red - a number between 0.0 and 1.0
/// - Green - a number between 0.0 and 1.0
/// - Blue - a number between 0.0 and 1.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisplayP3 {
	pub red: f64,
	pub green: f64,
	pub blue: f64,
	pub alpha: f32,
}

impl DisplayP3 {
	pub fn new(red: f64, green: f64, blue: f64, alpha: f32) -> Self {
		Self { red, green, blue, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for DisplayP3 {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for DisplayP3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { red, green, blue, alpha } = self;
		write!(f, "color(display-p3 {} {} {}", round_dp(*red, 2), round_dp(*green, 2), round_dp(*blue, 2))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

/// sRGB transfer function: linear to gamma-encoded
fn gamma(u: f64) -> f64 {
	let abs = u.abs();
	if abs <= 0.0031308 { u * 12.92 } else { u.signum() * (1.055 * abs.powf(1.0 / 2.4) - 0.055) }
}

/// sRGB transfer function: gamma-encoded to linear
fn linear(c: f64) -> f64 {
	let abs = c.abs();
	if abs > 0.04045 { c.signum() * ((abs + 0.055) / 1.055).powf(2.4) } else { c / 12.92 }
}

impl From<XyzD65> for DisplayP3 {
	fn from(value: XyzD65) -> Self {
		let XyzD65 { x, y, z, alpha } = value;
		let x = x / 100.0;
		let y = y / 100.0;
		let z = z / 100.0;
		// XYZ D65 -> Linear Display P3 (see XYZ_to_lin_P3 in CSS Color 4)
		let lr = x * (446124.0 / 178915.0) + y * (-333277.0 / 357830.0) + z * (-72051.0 / 178915.0);
		let lg = x * (-14852.0 / 17905.0) + y * (63121.0 / 35810.0) + z * (423.0 / 17905.0);
		let lb = x * (11844.0 / 330415.0) + y * (-50337.0 / 660830.0) + z * (316169.0 / 330415.0);
		// Apply sRGB gamma
		DisplayP3::new(gamma(lr), gamma(lg), gamma(lb), alpha)
	}
}

impl From<DisplayP3> for XyzD65 {
	fn from(value: DisplayP3) -> Self {
		let DisplayP3 { red, green, blue, alpha } = value;
		// Linearize with sRGB gamma
		let lr = linear(red);
		let lg = linear(green);
		let lb = linear(blue);
		// Linear Display P3 -> XYZ D65 (see lin_d3_to_XYZ in CSS Color 4)
		let x = lr * (608311.0 / 1250200.0) + lg * (189793.0 / 714400.0) + lb * (198249.0 / 1000160.0);
		let y = lr * (35783.0 / 156275.0) + lg * (247089.0 / 357200.0) + lb * (198249.0 / 2500400.0);
		let z = lg * (32229.0 / 714400.0) + lb * (5220557.0 / 5000800.0);
		XyzD65::new(x * 100.0, y * 100.0, z * 100.0, alpha)
	}
}

impl From<DisplayP3> for LinearRgb {
	fn from(value: DisplayP3) -> Self {
		let DisplayP3 { red, green, blue, alpha } = value;
		// Linearize with sRGB gamma
		let lr = linear(red);
		let lg = linear(green);
		let lb = linear(blue);
		// Linear Display P3 -> Linear sRGB
		let red = lr * (3685649.0 / 3008840.0) + lg * (-676809.0 / 3008840.0);
		let green = lr * (-5617931.0 / 133579120.0) + lg * (139197051.0 / 133579120.0);
		let blue = lr * (-1323971.0 / 67420360.0) + lg * (-1514763.0 / 19262960.0) + lb * (148092003.0 / 134840720.0);
		LinearRgb::new(red, green, blue, alpha)
	}
}

impl From<LinearRgb> for DisplayP3 {
	fn from(value: LinearRgb) -> Self {
		let LinearRgb { red, green, blue, alpha } = value;
		// Linear sRGB -> Linear Display P3
		let lr = red * (2442703.0 / 2969989.0) + green * (527286.0 / 2969989.0);
		let lg = red * (621563.0 / 18725049.0) + green * (18103486.0 / 18725049.0);
		let lb =
			red * (281089.0 / 16454667.0) + green * (10721482.0 / 148092003.0) + blue * (134840720.0 / 148092003.0);
		// Apply sRGB gamma
		DisplayP3::new(gamma(lr), gamma(lg), gamma(lb), alpha)
	}
}
