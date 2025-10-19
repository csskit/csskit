use crate::{Hsv, ToAlpha, round_dp};
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
		Self {
			hue: hue.rem_euclid(360.0),
			whiteness: whiteness.clamp(0.0, 100.0),
			blackness: blackness.clamp(0.0, 100.0),
			alpha: alpha.clamp(0.0, 100.0),
		}
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
