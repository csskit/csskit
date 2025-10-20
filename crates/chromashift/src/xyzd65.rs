use crate::{ToAlpha, round_dp};
use core::fmt;

/// A colour expressed as X, Y and Z values, expressed in the CIE XYZ tristimulus colour space, with an explicit D65
/// white point.
/// The components are:
/// - X - a number between 0.0 and 100.0
/// - Y - a number between 0.0 and 100.0
/// - Z - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyzD65 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub alpha: f32,
}

impl XyzD65 {
	pub fn new(x: f64, y: f64, z: f64, alpha: f32) -> Self {
		Self { x, y, z, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for XyzD65 {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for XyzD65 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { x, y, z, alpha } = self;
		write!(f, "color(xyz-d65 {} {}% {}%", round_dp(*x, 4), round_dp(*y, 4), round_dp(*z, 4))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}
