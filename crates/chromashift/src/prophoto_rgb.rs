use crate::{ToAlpha, XyzD50, round_dp};
use core::fmt;

/// A colour in the ProPhoto RGB colour space (ROMM RGB).
/// The components are:
/// - Red - a number between 0.0 and 1.0
/// - Green - a number between 0.0 and 1.0
/// - Blue - a number between 0.0 and 1.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProphotoRgb {
	pub red: f64,
	pub green: f64,
	pub blue: f64,
	pub alpha: f32,
}

impl ProphotoRgb {
	pub fn new(red: f64, green: f64, blue: f64, alpha: f32) -> Self {
		Self { red, green, blue, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for ProphotoRgb {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for ProphotoRgb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { red, green, blue, alpha } = self;
		write!(f, "color(prophoto-rgb {} {} {}", round_dp(*red, 2), round_dp(*green, 2), round_dp(*blue, 2))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

/// ProPhoto RGB transfer function: linear to gamma-encoded
/// Uses gamma 1.8 with a linear segment.
fn gamma(u: f64) -> f64 {
	let abs = u.abs();
	if abs >= 1.0 / 512.0 { u.signum() * abs.powf(1.0 / 1.8) } else { u * 16.0 }
}

/// ProPhoto RGB transfer function: gamma-encoded to linear
fn linear(c: f64) -> f64 {
	let abs = c.abs();
	if abs >= 16.0 / 512.0 { c.signum() * abs.powf(1.8) } else { c / 16.0 }
}

impl From<XyzD50> for ProphotoRgb {
	fn from(value: XyzD50) -> Self {
		let XyzD50 { x, y, z, alpha } = value;
		let x = x / 100.0;
		let y = y / 100.0;
		let z = z / 100.0;
		// XYZ D50 -> Linear ProPhoto RGB
		let lr = x * 1.3457868816471583 + y * (-0.25557208737979464) + z * (-0.05110186497554526);
		let lg = x * (-0.5446307051249019) + y * 1.5082477428451468 + z * 0.02052744743642139;
		let lb = x * 0.0 + y * 0.0 + z * 1.2119675456389452;
		// Apply ProPhoto gamma
		ProphotoRgb::new(gamma(lr), gamma(lg), gamma(lb), alpha)
	}
}

impl From<ProphotoRgb> for XyzD50 {
	fn from(value: ProphotoRgb) -> Self {
		let ProphotoRgb { red, green, blue, alpha } = value;
		// Linearize with ProPhoto gamma
		let lr = linear(red);
		let lg = linear(green);
		let lb = linear(blue);
		// Linear ProPhoto RGB -> XYZ D50
		let x = lr * 0.7977666449006423 + lg * 0.13518129740053308 + lb * 0.0313477341283922;
		let y = lr * 0.2880748288194013 + lg * 0.711835234241873 + lb * 0.00008993693872564;
		let z = lr * 0.0 + lg * 0.0 + lb * 0.8251046025104602;
		XyzD50::new(x * 100.0, y * 100.0, z * 100.0, alpha)
	}
}
