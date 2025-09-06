use crate::Hsb;

/// An colour represented as Hue, Whiteness, and Blackness expressed in the sRGB colour space.
/// The components are:
/// - Hue - a number between 0.0 and 360.0
/// - Whiteness - a number between 0.0 and 100.0
/// - Blackness - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hwb {
	pub hue: f32,
	pub whiteness: f32,
	pub blackness: f32,
	pub alpha: f32,
}

impl Hwb {
	pub fn new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) -> Self {
		Self {
			hue: hue.rem_euclid(360.0),
			whiteness: whiteness.clamp(0.0, 100.0),
			blackness: blackness.clamp(0.0, 100.0),
			alpha: alpha.clamp(0.0, 100.0),
		}
	}
}

impl From<Hsb> for Hwb {
	fn from(value: Hsb) -> Self {
		let Hsb { hue, saturation, brightness, alpha } = value;
		let s = saturation / 100.0;
		let v = brightness / 100.0;
		let whiteness = (1.0 - s) * v;
		let blackness = 1.0 - v;
		Hwb::new(hue, whiteness * 100.0, blackness * 100.0, alpha)
	}
}

impl From<Hwb> for Hsb {
	fn from(value: Hwb) -> Self {
		let Hwb { hue, whiteness, blackness, alpha } = value;
		let w = whiteness / 100.0;
		let b = blackness / 100.0;
		let sum = w + b;
		let (s, v) = if sum >= 1.0 {
			(0.0, w / sum)
		} else {
			let v = 1.0 - b;
			let s = if v == 0.0 { 0.0 } else { 1.0 - w / v };
			(s, v)
		};
		Hsb::new(hue, s * 100.0, v * 100.0, alpha)
	}
}
