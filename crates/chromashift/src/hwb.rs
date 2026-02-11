use crate::{Hsv, LinearRgb, ToAlpha, round_dp};
use core::fmt;

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
		Self { hue: hue.rem_euclid(360.0), whiteness, blackness, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for Hwb {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Hwb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { hue, whiteness, blackness, alpha } = self;
		write!(
			f,
			"hwb({} {} {}",
			round_dp(*hue as f64, 2),
			round_dp(*whiteness as f64, 3),
			round_dp(*blackness as f64, 3)
		)?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

impl From<Hsv> for Hwb {
	fn from(value: Hsv) -> Self {
		let Hsv { hue, saturation, value, alpha } = value;
		let s = saturation / 100.0;
		let v = value / 100.0;
		let whiteness = (1.0 - s) * v;
		let blackness = 1.0 - v;
		Hwb::new(hue, whiteness * 100.0, blackness * 100.0, alpha)
	}
}

impl From<Hwb> for Hsv {
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
		Hsv::new(hue, s * 100.0, v * 100.0, alpha)
	}
}

/// sRGB gamma-encode: linear to gamma (handles negative/OOG values via signum)
fn gamma(u: f64) -> f64 {
	let abs = u.abs();
	if abs <= 0.0031308 { u * 12.92 } else { u.signum() * (1.055 * abs.powf(1.0 / 2.4) - 0.055) }
}

/// sRGB linearize: gamma to linear (handles negative/OOG values via signum)
fn linear(c: f64) -> f64 {
	let abs = c.abs();
	if abs > 0.04045 { c.signum() * ((abs + 0.055) / 1.055).powf(2.4) } else { c / 12.92 }
}

/// Convert float sRGB (r,g,b may be OOG) to HWB via HSV math.
fn srgb_float_to_hwb(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
	let max = r.max(g).max(b);
	let min = r.min(g).min(b);
	let delta = max - min;
	let v = max;
	let saturation = if max == 0.0 { 0.0 } else { delta / max };
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
	// HSV to HWB: w = (1-s)*v, b = 1-v
	let whiteness = (1.0 - saturation) * v;
	let blackness = 1.0 - v;
	(hue, whiteness * 100.0, blackness * 100.0)
}

/// Convert HWB to float sRGB (r,g,b may be OOG).
fn hwb_to_srgb_float(hue: f64, whiteness: f64, blackness: f64) -> (f64, f64, f64) {
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
	// HSV to RGB
	let h = hue / 60.0;
	let c = v * s;
	let x = c * (1.0 - (h % 2.0 - 1.0).abs());
	let m = v - c;
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
	(r_prime + m, g_prime + m, b_prime + m)
}

impl From<LinearRgb> for Hwb {
	fn from(value: LinearRgb) -> Self {
		let r = gamma(value.red);
		let g = gamma(value.green);
		let b_val = gamma(value.blue);
		let (hue, whiteness, blackness) = srgb_float_to_hwb(r, g, b_val);
		Hwb::new(hue as f32, whiteness as f32, blackness as f32, value.alpha)
	}
}

impl From<Hwb> for LinearRgb {
	fn from(value: Hwb) -> Self {
		let (r, g, b) = hwb_to_srgb_float(value.hue as f64, value.whiteness as f64, value.blackness as f64);
		LinearRgb::new(linear(r), linear(g), linear(b), value.alpha)
	}
}
