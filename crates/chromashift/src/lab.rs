use crate::{ToAlpha, XyzD50, round_dp};
use core::fmt;

const D50X: f64 = 96.4220;
const D50Y: f64 = 100.0;
const D50Z: f64 = 82.5210;

/// An CIE defined colour space representing L - perceptual lightness, and two axes A & B.
/// The components are:
/// - L - a number between 0.0 and 100.0
/// - A - a number between -125.0 and +125.0
/// - B - a number between -125.0 and +125.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lab {
	pub lightness: f64,
	pub a: f64,
	pub b: f64,
	pub alpha: f32,
}

impl Lab {
	pub fn new(lightness: f64, a: f64, b: f64, alpha: f32) -> Self {
		Self { lightness, a, b, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for Lab {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Lab {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { lightness, a, b, alpha } = self;
		write!(f, "lab({} {} {}", round_dp(*lightness, 2), round_dp(*a, 3), round_dp(*b, 3))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

impl From<XyzD50> for Lab {
	fn from(value: XyzD50) -> Self {
		let XyzD50 { x, y, z, alpha } = value;
		let x = x / D50X;
		let y = y / D50Y;
		let z = z / D50Z;
		let epsilon = 216.0 / 24389.0; // 6^3/29^3
		let kappa = 24389.0 / 27.0; // 29^3/3^3
		let fx = if x > epsilon { x.cbrt() } else { (kappa * x + 16.0) / 116.0 };
		let fy = if y > epsilon { y.cbrt() } else { (kappa * y + 16.0) / 116.0 };
		let fz = if z > epsilon { z.cbrt() } else { (kappa * z + 16.0) / 116.0 };
		let lightness = 116.0 * fy - 16.0;
		let a = 500.0 * (fx - fy);
		let b = 200.0 * (fy - fz);
		Lab::new(lightness, a, b, alpha)
	}
}

impl From<Lab> for XyzD50 {
	fn from(value: Lab) -> Self {
		let Lab { lightness, a, b, alpha } = value;
		let epsilon = 216.0 / 24389.0; // 6^3/29^3
		let kappa = 24389.0 / 27.0; // 29^3/3^3
		let fy = (lightness + 16.0) / 116.0;
		let fx = a / 500.0 + fy;
		let fz = fy - b / 200.0;
		let x = if fx.powi(3) > epsilon { fx.powi(3) } else { (116.0 * fx - 16.0) / kappa };
		let y = if lightness > kappa * epsilon { ((lightness + 16.0) / 116.0).powi(3) } else { lightness / kappa };
		let z = if fz.powi(3) > epsilon { fz.powi(3) } else { (116.0 * fz - 16.0) / kappa };
		XyzD50::new(x * D50X, y * D50Y, z * D50Z, alpha)
	}
}
