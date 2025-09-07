#![deny(warnings)]

mod a98_rgb;
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
pub use distance::ColorDistance;
pub use hex::Hex;
pub use hsb::Hsb;
pub use hsl::Hsl;
pub use hwb::Hwb;
pub use lab::Lab;
pub use lch::Lch;
pub use linear_rgb::LinearRgb;
pub use mix::{ColorMix, ColorMixPolar, HueInterpolation};
pub use named::Named;
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use srgb::Srgb;
pub use wcag::{WcagColorContrast, WcagLevel};
pub use xyzd50::XyzD50;
pub use xyzd65::XyzD65;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
	A98Rgb(A98Rgb),
	Hsb(Hsb),
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

impl From<Color> for XyzD65 {
	fn from(value: Color) -> Self {
		match value {
			Color::A98Rgb(a) => a.into(),
			Color::Hex(h) => h.into(),
			Color::Hsb(h) => h.into(),
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
