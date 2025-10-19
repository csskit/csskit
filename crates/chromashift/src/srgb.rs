use crate::{LinearRgb, ToAlpha};
use core::fmt;

/// An RGB colour space with defined chromacities.
/// The components are:
/// - Red - a number between 0 and 255
/// - Blue - a number between 0 and 255
/// - Green - a number between 0 and 255
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: f32,
}

impl Srgb {
	pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
		Self { red, green, blue, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for Srgb {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Srgb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { red, green, blue, alpha } = self;
		write!(f, "rgb({red} {green} {blue}")?;
		if *alpha < 100.0 {
			write!(f, " / {alpha}%")?;
		}
		write!(f, ")")
	}
}

fn linear(c: f64) -> f64 {
	if c > 0.04045 { ((c + 0.055) / 1.055).powf(2.4) } else { c / 12.92 }
}

fn gamma(u: f64) -> f64 {
	if u <= 0.0031308 { u * 12.92 } else { 1.055 * u.powf(1.0 / 2.4) - 0.055 }
}

fn clamp01(value: f64) -> f64 {
	value.clamp(0.0, 1.0)
}

impl From<Srgb> for LinearRgb {
	fn from(value: Srgb) -> Self {
		let Srgb { red, green, blue, alpha } = value;
		LinearRgb::new(linear(red as f64 / 255.0), linear(green as f64 / 255.0), linear(blue as f64 / 255.0), alpha)
	}
}

impl From<LinearRgb> for Srgb {
	fn from(value: LinearRgb) -> Self {
		let LinearRgb { red, green, blue, alpha } = value;
		Srgb::new(
			(gamma(clamp01(red)) * 255.0).round() as u8,
			(gamma(clamp01(green)) * 255.0).round() as u8,
			(gamma(clamp01(blue)) * 255.0).round() as u8,
			alpha,
		)
	}
}

#[cfg(feature = "anstyle")]
impl From<Srgb> for anstyle::RgbColor {
	fn from(value: Srgb) -> Self {
		anstyle::RgbColor(value.red, value.green, value.blue)
	}
}

#[cfg(feature = "anstyle")]
impl From<anstyle::RgbColor> for Srgb {
	fn from(value: anstyle::RgbColor) -> Self {
		Srgb::new(value.0, value.1, value.2, 100.0)
	}
}
