use crate::{ToAlpha, XyzD65, round_dp};
use core::fmt;

/// A device independent expression of RGB. No exactly defined chromacities.
/// The components are:
/// - Red - a number between 0.0 and 1.0
/// - Blue - a number between 0.0 and 1.0
/// - Green - a number between 0.0 and 1.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgb {
	pub red: f64,
	pub green: f64,
	pub blue: f64,
	pub alpha: f32,
}

impl LinearRgb {
	pub fn new(red: f64, green: f64, blue: f64, alpha: f32) -> Self {
		Self { red, green, blue, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for LinearRgb {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for LinearRgb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { red, green, blue, alpha } = self;
		write!(f, "color(srgb-linear {} {}% {}%", round_dp(*red, 2), round_dp(*green, 2), round_dp(*blue, 2))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

impl From<XyzD65> for LinearRgb {
	fn from(value: XyzD65) -> Self {
		let XyzD65 { x, y, z, alpha } = value;
		let x = x / 100.0;
		let y = y / 100.0;
		let z = z / 100.0;
		let red = x * (12831.0 / 3959.0) + y * (-329.0 / 214.0) + z * (-1974.0 / 3959.0);
		let green = x * (-851781.0 / 878810.0) + y * (1648619.0 / 878810.0) + z * (36519.0 / 878810.0);
		let blue = x * (705.0 / 12673.0) + y * (-2585.0 / 12673.0) + z * (705.0 / 667.0);
		LinearRgb::new(red, green, blue, alpha)
	}
}

impl From<LinearRgb> for XyzD65 {
	fn from(value: LinearRgb) -> Self {
		let LinearRgb { red, green, blue, alpha } = value;
		let x = red * (506752.0 / 1228815.0) + green * (87881.0 / 245763.0) + blue * (12673.0 / 70218.0);
		let y = red * (87098.0 / 409605.0) + green * (175762.0 / 245763.0) + blue * (12673.0 / 175545.0);
		let z = red * (7918.0 / 409605.0) + green * (87881.0 / 737289.0) + blue * (1001167.0 / 1053270.0);
		XyzD65::new(x * 100.0, y * 100.0, z * 100.0, alpha)
	}
}
