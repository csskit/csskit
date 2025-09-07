use crate::{Lab, round_dp};
use core::fmt;

/// A cylindrical colour space representing within the CIE colour space.
/// The components are:
/// - Lightness / Luminance - a number between 0.0 and 100.0
/// - Chroma - a number between 0.0 and 150.0
/// - Hue - a number between 0.0 and 360.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lch {
	pub lightness: f64,
	pub chroma: f64,
	pub hue: f64,
	pub alpha: f32,
}

impl Lch {
	pub fn new(lightness: f64, chroma: f64, hue: f64, alpha: f32) -> Self {
		Self {
			lightness: lightness.clamp(0.0, 100.0),
			chroma: chroma.clamp(0.0, 150.0),
			hue: hue.rem_euclid(360.0),
			alpha: alpha.clamp(0.0, 100.0),
		}
	}
}

impl fmt::Display for Lch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { lightness, chroma, hue, alpha } = self;
		write!(f, "lch({}% {} {}", round_dp(*lightness, 2), round_dp(*chroma, 5), round_dp(*hue, 2))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

impl From<Lab> for Lch {
	fn from(value: Lab) -> Self {
		let Lab { lightness, a, b, alpha } = value;
		let chroma = (a * a + b * b).sqrt();
		let hue = b.atan2(a).to_degrees();
		let hue = if hue < 0.0 { hue + 360.0 } else { hue }; // Normalize to [0, 360)
		Lch::new(lightness, chroma, hue, alpha)
	}
}

impl From<Lch> for Lab {
	fn from(value: Lch) -> Self {
		let Lch { lightness, chroma, hue, alpha } = value;
		let h_rad = hue.to_radians();
		Lab::new(lightness, chroma * h_rad.cos(), chroma * h_rad.sin(), alpha)
	}
}
