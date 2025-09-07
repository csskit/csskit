use crate::{LinearRgb, round_dp};
use core::fmt;

/// An RGB colour space with defined chromacities.
/// The components are:
/// - Red - a number between 0.0 and 1.0
/// - Blue - a number between 0.0 and 1.0
/// - Green - a number between 0.0 and 1.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct A98Rgb {
	pub red: f64,
	pub green: f64,
	pub blue: f64,
	pub alpha: f32,
}

impl A98Rgb {
	pub fn new(red: f64, green: f64, blue: f64, alpha: f32) -> Self {
		Self {
			red: red.clamp(0.0, 1.0),
			green: green.clamp(0.0, 1.0),
			blue: blue.clamp(0.0, 1.0),
			alpha: alpha.clamp(0.0, 100.0),
		}
	}
}

impl fmt::Display for A98Rgb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { red, green, blue, alpha } = self;
		write!(f, "color(a98-rgb {} {}% {}%", round_dp(*red, 2), round_dp(*green, 2), round_dp(*blue, 2))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

impl From<LinearRgb> for A98Rgb {
	fn from(value: LinearRgb) -> Self {
		let LinearRgb { red, green, blue, alpha } = value;
		const INV_GAMMA: f64 = 256.0 / 563.0;
		let gamma_red = red.signum() * red.abs().powf(INV_GAMMA);
		let gamma_green = green.signum() * green.abs().powf(INV_GAMMA);
		let gamma_blue = blue.signum() * blue.abs().powf(INV_GAMMA);
		A98Rgb::new(gamma_red, gamma_green, gamma_blue, alpha)
	}
}

impl From<A98Rgb> for LinearRgb {
	fn from(value: A98Rgb) -> Self {
		let A98Rgb { red, green, blue, alpha } = value;
		const GAMMA: f64 = 563.0 / 256.0;
		let linear_red = red.signum() * red.abs().powf(GAMMA);
		let linear_green = green.signum() * green.abs().powf(GAMMA);
		let linear_blue = blue.signum() * blue.abs().powf(GAMMA);
		LinearRgb::new(linear_red, linear_green, linear_blue, alpha)
	}
}
