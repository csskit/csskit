#![deny(warnings)]

use core::fmt;
mod a98_rgb;
mod channels;
mod conversion;
mod distance;
mod hex;
mod hsb;
mod hsl;
mod hwb;
mod lab;
mod lch;
mod linear_rgb;
mod mix;
mod named;
mod oklab;
mod oklch;
mod srgb;
#[cfg(test)]
mod tests;
mod wcag;
mod xyzd50;
mod xyzd65;

pub use a98_rgb::A98Rgb;
pub use channels::ToAlpha;
pub use distance::ColorDistance;
pub use hex::Hex;
pub use hsb::Hsv;
pub use hsl::Hsl;
pub use hwb::Hwb;
pub use lab::Lab;
pub use lch::Lch;
pub use linear_rgb::LinearRgb;
pub use mix::{ColorMix, ColorMixPolar, HueInterpolation};
pub use named::{Named, ToNamedError};
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use srgb::Srgb;
pub use wcag::{WcagColorContrast, WcagLevel};
pub use xyzd50::XyzD50;
pub use xyzd65::XyzD65;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
	A98Rgb(A98Rgb),
	Hsv(Hsv),
	Hsl(Hsl),
	Hex(Hex),
	Hwb(Hwb),
	Lab(Lab),
	Lch(Lch),
	LinearRgb(LinearRgb),
	Named(Named),
	Oklab(Oklab),
	Oklch(Oklch),
	Srgb(Srgb),
	XyzD50(XyzD50),
	XyzD65(XyzD65),
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::A98Rgb(a) => fmt::Display::fmt(a, f),
			Self::Hex(h) => fmt::Display::fmt(h, f),
			Self::Hsv(h) => fmt::Display::fmt(h, f),
			Self::Hsl(h) => fmt::Display::fmt(h, f),
			Self::Hwb(h) => fmt::Display::fmt(h, f),
			Self::Lab(l) => fmt::Display::fmt(l, f),
			Self::Lch(l) => fmt::Display::fmt(l, f),
			Self::LinearRgb(l) => fmt::Display::fmt(l, f),
			Self::Named(n) => fmt::Display::fmt(n, f),
			Self::Oklab(o) => fmt::Display::fmt(o, f),
			Self::Oklch(o) => fmt::Display::fmt(o, f),
			Self::Srgb(s) => fmt::Display::fmt(s, f),
			Self::XyzD50(x) => fmt::Display::fmt(x, f),
			Self::XyzD65(x) => fmt::Display::fmt(x, f),
		}
	}
}

impl ToAlpha for Color {
	fn to_alpha(&self) -> f32 {
		match self {
			Color::A98Rgb(a) => a.to_alpha(),
			Color::Hex(h) => h.to_alpha(),
			Color::Hsv(h) => h.to_alpha(),
			Color::Hsl(h) => h.to_alpha(),
			Color::Hwb(h) => h.to_alpha(),
			Color::Lab(l) => l.to_alpha(),
			Color::Lch(l) => l.to_alpha(),
			Color::LinearRgb(l) => l.to_alpha(),
			Color::Named(n) => n.to_alpha(),
			Color::Oklab(o) => o.to_alpha(),
			Color::Oklch(o) => o.to_alpha(),
			Color::Srgb(s) => s.to_alpha(),
			Color::XyzD50(x) => x.to_alpha(),
			Color::XyzD65(x) => x.to_alpha(),
		}
	}
}

impl From<Color> for XyzD65 {
	fn from(value: Color) -> Self {
		match value {
			Color::A98Rgb(a) => a.into(),
			Color::Hex(h) => h.into(),
			Color::Hsv(h) => h.into(),
			Color::Hsl(h) => h.into(),
			Color::Hwb(h) => h.into(),
			Color::Lab(l) => l.into(),
			Color::Lch(l) => l.into(),
			Color::LinearRgb(l) => l.into(),
			Color::Named(n) => n.into(),
			Color::Oklab(o) => o.into(),
			Color::Oklch(o) => o.into(),
			Color::Srgb(s) => s.into(),
			Color::XyzD50(x) => x.into(),
			Color::XyzD65(x) => x,
		}
	}
}

pub const COLOR_EPSILON: f64 = 0.0072;

pub(crate) fn round_dp(f: f64, d: u32) -> f64 {
	let factor = 10u32.pow(d) as f64;
	(f * factor).round() / factor
}
