use crate::{ToAlpha, XyzD65, round_dp};
use core::fmt;

/// A colour expressed as X, Y and Z values, expressed in the CIE XYZ tristimulus colour space, with an explicit D50
/// white point.
/// The components are:
/// - X - a number between 0.0 and 100.0
/// - Y - a number between 0.0 and 100.0
/// - Z - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyzD50 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub alpha: f32,
}

impl XyzD50 {
	pub fn new(x: f64, y: f64, z: f64, alpha: f32) -> Self {
		Self { x, y, z, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for XyzD50 {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for XyzD50 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { x, y, z, alpha } = self;
		write!(f, "color(xyz-d50 {} {}% {}%", round_dp(*x, 4), round_dp(*y, 4), round_dp(*z, 4))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

impl From<XyzD50> for XyzD65 {
	fn from(value: XyzD50) -> Self {
		let XyzD50 { x, y, z, alpha } = value;
		let x_d65 = x * 0.955473421488075 + y * (-0.02309845494876471) + z * 0.06325924320057072;
		let y_d65 = x * (-0.0283697093338637) + y * 1.0099953980813041 + z * 0.021041441191917323;
		let z_d65 = x * 0.012314014864481998 + y * (-0.020507649298898964) + z * 1.330365926242124;
		XyzD65::new(x_d65, y_d65, z_d65, alpha)
	}
}

impl From<XyzD65> for XyzD50 {
	fn from(value: XyzD65) -> Self {
		let XyzD65 { x, y, z, alpha } = value;
		let x_d50 = x * 1.0479297925449969 + y * 0.022946870601609652 + z * (-0.05019226628920524);
		let y_d50 = x * 0.02962780877005599 + y * 0.9904344267538799 + z * (-0.017073799063418826);
		let z_d50 = x * (-0.009243040646204504) + y * 0.015055191490298152 + z * 0.7518742814281371;
		XyzD50::new(x_d50, y_d50, z_d50, alpha)
	}
}
