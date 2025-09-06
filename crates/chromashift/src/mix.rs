use crate::{Hsl, Lab, Lch, LinearRgb, Oklab, Oklch, Srgb, XyzD50, XyzD65};

/// A direction to interopolate hue values between, when mixing colours.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HueInterpolation {
	#[default]
	Shorter,
	Longer,
	Increasing,
	Decreasing,
}

/// Trait for calculating mixing two colors together.
///
/// This trait provides a static method which will receive two colours, and can output a Self which should be the result
/// of both colours mixed by the given percentage (the percentage pertains to how much the second colour should apply to
/// the first).
pub trait ColorMix<T, U>: Sized
where
	T: Into<Self>,
	U: Into<Self>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self;
}

/// Trait for calculating mixing two colors together, with a hue direction for Polar colour spaces.
///
/// This trait provides a static method which will receive two colours, and can output a Self which should be the result
/// of both colours mixed by the given percentage (the percentage pertains to how much the second colour should apply to
/// the first). The Hue direction should be respected. If the colour space is not Polar then consider [ColorMix]
/// instead.
pub trait ColorMixPolar<T, U>: Sized
where
	T: Into<Self>,
	U: Into<Self>,
{
	fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self;
}

/// Given two hues (`h1`, `h2`), a percentage transform (`t`), and an interpolation direction, return a new Hue rotation
/// transformed by that amount.
pub fn interpolate_hue(h1: f64, h2: f64, t: f64, interpolation: HueInterpolation) -> f64 {
	let (h1, h2) = (h1.rem_euclid(360.0), h2.rem_euclid(360.0));

	let diff = match interpolation {
		HueInterpolation::Shorter => {
			let d = h2 - h1;
			if d.abs() <= 180.0 {
				d
			} else if d > 180.0 {
				d - 360.0
			} else {
				d + 360.0
			}
		}
		HueInterpolation::Longer => {
			let d = h2 - h1;
			if d.abs() > 180.0 {
				d
			} else if d > 0.0 {
				d - 360.0
			} else {
				d + 360.0
			}
		}
		HueInterpolation::Increasing => {
			let mut d = h2 - h1;
			if d < 0.0 {
				d += 360.0;
			}
			d
		}
		HueInterpolation::Decreasing => {
			let mut d = h2 - h1;
			if d > 0.0 {
				d -= 360.0;
			}
			d
		}
	};

	(h1 + diff * t).rem_euclid(360.0)
}

mod sealed {
	pub trait PolarColor {}
}

impl sealed::PolarColor for Hsl {}
impl sealed::PolarColor for Lch {}
impl sealed::PolarColor for Oklch {}

impl<T, U, V> ColorMix<T, U> for V
where
	V: ColorMixPolar<T, U> + sealed::PolarColor + Sized,
	T: Into<V>,
	U: Into<V>,
{
	fn mix(first: T, second: U, percentage: f64) -> V {
		ColorMixPolar::mix_polar(first, second, percentage, HueInterpolation::Shorter)
	}
}

impl<T, U> ColorMix<T, U> for Srgb
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let r = first.red as f64 * (1.0 - t) + second.red as f64 * t;
		let g = first.green as f64 * (1.0 - t) + second.green as f64 * t;
		let b = first.blue as f64 * (1.0 - t) + second.blue as f64 * t;
		let a = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		Srgb::new(r.round() as u8, g.round() as u8, b.round() as u8, a as f32)
	}
}

impl<T, U> ColorMix<T, U> for LinearRgb
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let r = first.red * (1.0 - t) + second.red * t;
		let g = first.green * (1.0 - t) + second.green * t;
		let b = first.blue * (1.0 - t) + second.blue * t;
		let a = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		LinearRgb::new(r, g, b, a as f32)
	}
}

impl<T, U> ColorMixPolar<T, U> for Hsl
where
	Self: From<T> + From<U>,
{
	fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let h = interpolate_hue(first.hue as f64, second.hue as f64, t, hue_interpolation);
		let s = first.saturation as f64 * (1.0 - t) + second.saturation as f64 * t;
		let l = first.lightness as f64 * (1.0 - t) + second.lightness as f64 * t;
		let a = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		Hsl::new(h as f32, s as f32, l as f32, a as f32)
	}
}

impl<T, U> ColorMix<T, U> for Lab
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let l = first.lightness * (1.0 - t) + second.lightness * t;
		let a = first.a * (1.0 - t) + second.a * t;
		let b = first.b * (1.0 - t) + second.b * t;
		let alpha = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		Lab::new(l, a, b, alpha as f32)
	}
}

impl<T, U> ColorMixPolar<T, U> for Lch
where
	Self: From<T> + From<U>,
{
	fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let l = first.lightness * (1.0 - t) + second.lightness * t;
		let c = first.chroma * (1.0 - t) + second.chroma * t;
		let h = interpolate_hue(first.hue, second.hue, t, hue_interpolation);
		let a = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		Lch::new(l, c, h, a as f32)
	}
}

impl<T, U> ColorMix<T, U> for Oklab
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let l = first.lightness * (1.0 - t) + second.lightness * t;
		let a = first.a * (1.0 - t) + second.a * t;
		let b = first.b * (1.0 - t) + second.b * t;
		let alpha = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		Oklab::new(l, a, b, alpha as f32)
	}
}

impl<T, U> ColorMixPolar<T, U> for Oklch
where
	Self: From<T> + From<U>,
{
	fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let l = first.lightness * (1.0 - t) + second.lightness * t;
		let c = first.chroma * (1.0 - t) + second.chroma * t;
		let h = interpolate_hue(first.hue, second.hue, t, hue_interpolation);
		let a = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		Oklch::new(l, c, h, a as f32)
	}
}

impl<T, U> ColorMix<T, U> for XyzD50
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let x = first.x * (1.0 - t) + second.x * t;
		let y = first.y * (1.0 - t) + second.y * t;
		let z = first.z * (1.0 - t) + second.z * t;
		let a = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		XyzD50::new(x, y, z, a as f32)
	}
}

impl<T, U> ColorMix<T, U> for XyzD65
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let x = first.x * (1.0 - t) + second.x * t;
		let y = first.y * (1.0 - t) + second.y * t;
		let z = first.z * (1.0 - t) + second.z * t;
		let a = first.alpha as f64 * (1.0 - t) + second.alpha as f64 * t;
		XyzD65::new(x, y, z, a as f32)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::*;

	macro_rules! assert_close_to {
		($a: expr, $b: expr) => {
			assert!($a.close_to($b, COLOR_EPSILON), "Expected {:?} to be (closely) equal to {:?}", $a, $b);
		};
	}

	#[test]
	fn test_basic_mix() {
		let red = Srgb::new(255, 0, 0, 100.0);
		let blue = Srgb::new(0, 0, 255, 100.0);
		assert_close_to!(Srgb::mix(red, blue, 50.0), Srgb::new(128, 0, 128, 100.0));
	}

	#[test]
	fn test_mix_named_in_oklab() {
		assert_close_to!(
			Oklch::mix(Named::Rebeccapurple, Named::Hotpink, 50.0),
			Oklch::new(0.5842845967725198, 0.17868573405015944, 327.6838446374328, 100.0)
		);
	}

	#[test]
	fn test_mix_named_in_hsl_polar() {
		assert_close_to!(Hsl::mix(Named::Rebeccapurple, Named::Hotpink, 50.0), Hsl::new(300.0, 75.0, 55.294117, 100.0));
		assert_close_to!(
			Hsl::mix_polar(Named::Rebeccapurple, Named::Hotpink, 50.0, HueInterpolation::Longer),
			Hsl::new(120.0, 75.0, 55.294117, 100.0)
		);
		assert_close_to!(
			Hsl::mix_polar(Named::Rebeccapurple, Named::Hotpink, 50.0, HueInterpolation::Decreasing),
			Hsl::new(120.0, 75.0, 55.294117, 100.0)
		);
		assert_close_to!(
			Hsl::mix_polar(Named::Rebeccapurple, Named::Hotpink, 50.0, HueInterpolation::Increasing),
			Hsl::new(300.0, 75.0, 55.294117, 100.0)
		);
	}

	#[test]
	fn test_alpha_mixing() {
		let color1 = Srgb::new(255, 0, 0, 80.0);
		let color2 = Srgb::new(0, 0, 255, 40.0);

		let mixed = Srgb::mix(color1, color2, 50.0);
		assert_eq!(mixed.red, 128);
		assert_eq!(mixed.green, 0);
		assert_eq!(mixed.blue, 128);
		assert_eq!(mixed.alpha, 60.0);
	}
}
