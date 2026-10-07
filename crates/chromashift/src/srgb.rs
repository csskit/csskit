use crate::{LinearRgb, RgbSpace, ToAlpha};
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
	/// The colour space these channels are encoded in.
	pub const SPACE: RgbSpace = RgbSpace::SRGB;

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

impl From<Srgb> for LinearRgb {
	fn from(value: Srgb) -> Self {
		let Srgb { red, green, blue, alpha } = value;
		let channels = [red as f64 / 255.0, green as f64 / 255.0, blue as f64 / 255.0];
		let [red, green, blue] = Srgb::SPACE.transfer.decode_all(channels);
		LinearRgb::new(red, green, blue, alpha)
	}
}

impl From<LinearRgb> for Srgb {
	fn from(value: LinearRgb) -> Self {
		let LinearRgb { red, green, blue, alpha } = value;
		let clamped = [red.clamp(0.0, 1.0), green.clamp(0.0, 1.0), blue.clamp(0.0, 1.0)];
		let [red, green, blue] = Srgb::SPACE.transfer.encode_all(clamped);
		Srgb::new((red * 255.0).round() as u8, (green * 255.0).round() as u8, (blue * 255.0).round() as u8, alpha)
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

#[cfg(feature = "owo-colors")]
impl From<Srgb> for owo_colors::Rgb {
	fn from(value: Srgb) -> Self {
		owo_colors::Rgb(value.red, value.green, value.blue)
	}
}

#[cfg(feature = "owo-colors")]
impl From<owo_colors::Rgb> for Srgb {
	fn from(value: owo_colors::Rgb) -> Self {
		Srgb::new(value.0, value.1, value.2, 100.0)
	}
}
