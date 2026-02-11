use crate::{ToAlpha, XyzD65, round_dp};
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
		// XYZ D65 -> Linear Display P3
		let lr = x * 2.4934969119414263 + y * (-0.9313836179191239) + z * (-0.40271078445071684);
		let lg = x * (-0.8294889695615747) + y * 1.7626640603183463 + z * 0.023624685841943577;
		let lb = x * 0.03584583024378447 + y * (-0.07617238926804182) + z * 0.9568845240076872;
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
		// Linear Display P3 -> XYZ D65
		let x = lr * 0.4865709486482162 + lg * 0.26566769316909306 + lb * 0.1982172852343625;
		let y = lr * 0.22897456406974884 + lg * 0.6917385218365064 + lb * 0.079286914093745;
		let z = lr * 0.0 + lg * 0.04511338185890264 + lb * 1.043944368900976;
		XyzD65::new(x * 100.0, y * 100.0, z * 100.0, alpha)
	}
}
