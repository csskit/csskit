use crate::{Srgb, round_dp};
use core::fmt;

/// An colour represented as Hue, Saturation, and Lightness expressed in the sRGB colour space.
/// The components are:
/// - Hue - a number between 0.0 and 360.0
/// - Saturation - a number between 0.0 and 100.0
/// - Brightness - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
	pub hue: f32,
	pub saturation: f32,
	pub lightness: f32,
	pub alpha: f32,
}

impl Hsl {
	pub fn new(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
		Self {
			hue: hue.rem_euclid(360.0),
			saturation: saturation.clamp(0.0, 100.0),
			lightness: lightness.clamp(0.0, 100.0),
			alpha: alpha.clamp(0.0, 100.0),
		}
	}
}

impl fmt::Display for Hsl {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { hue, saturation, lightness, alpha } = self;
		write!(
			f,
			"hsl({} {}% {}%",
			round_dp(*hue as f64, 2),
			round_dp(*saturation as f64, 2),
			round_dp(*lightness as f64, 2)
		)?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

impl From<Srgb> for Hsl {
	fn from(value: Srgb) -> Self {
		let r = value.red as f32 / 255.0;
		let g = value.green as f32 / 255.0;
		let b = value.blue as f32 / 255.0;
		let max = r.max(g).max(b);
		let min = r.min(g).min(b);
		let delta = max - min;
		let lightness = (max + min) / 2.0;
		let saturation = if delta == 0.0 { 0.0 } else { delta / (1.0 - (2.0 * lightness - 1.0).abs()) };
		let hue = if delta == 0.0 {
			0.0
		} else if max == r {
			60.0 * (((g - b) / delta) % 6.0)
		} else if max == g {
			60.0 * ((b - r) / delta + 2.0)
		} else {
			60.0 * ((r - g) / delta + 4.0)
		};
		let hue = if hue < 0.0 { hue + 360.0 } else { hue };
		Hsl::new(hue, saturation * 100.0, lightness * 100.0, value.alpha)
	}
}

impl From<Hsl> for Srgb {
	fn from(value: Hsl) -> Self {
		let h = value.hue / 60.0;
		let s = value.saturation / 100.0;
		let l = value.lightness / 100.0;
		let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
		let x = c * (1.0 - (h % 2.0 - 1.0).abs());
		let m = l - c / 2.0;
		let (r_prime, g_prime, b_prime) = if h < 1.0 {
			(c, x, 0.0)
		} else if h < 2.0 {
			(x, c, 0.0)
		} else if h < 3.0 {
			(0.0, c, x)
		} else if h < 4.0 {
			(0.0, x, c)
		} else if h < 5.0 {
			(x, 0.0, c)
		} else {
			(c, 0.0, x)
		};
		Srgb::new(
			((r_prime + m) * 255.0).clamp(0.0, 255.0).round() as u8,
			((g_prime + m) * 255.0).clamp(0.0, 255.0).round() as u8,
			((b_prime + m) * 255.0).clamp(0.0, 255.0).round() as u8,
			value.alpha,
		)
	}
}
