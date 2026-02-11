use crate::{
	A98Rgb, DisplayP3, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, ProphotoRgb, Rec2020, Srgb, XyzD50, XyzD65,
};

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
///
/// Per CSS Color (4 12.3 & 5 3.5), interpolation uses premultiplied alpha:
/// 1. Premultiply each component by its alpha
/// 2. Linearly interpolate premultiplied values and alpha independently
/// 3. Un-premultiply by dividing by the interpolated alpha
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
///
/// Per CSS Color 4 12.3, premultiplied alpha is used for non-hue components. The hue component
/// is NOT premultiplied - it is interpolated directly using the specified hue interpolation method.
pub trait ColorMixPolar<T, U>: Sized
where
	T: Into<Self>,
	U: Into<Self>,
{
	fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self;
}

/// Interpolate a single component using premultiplied alpha.
///
/// CSS Color 4 12.3:
///   premultiplied1 = component1 * alpha1
///   premultiplied2 = component2 * alpha2
///   result_premultiplied = premultiplied1 * (1 - t) + premultiplied2 * t
///   result = result_premultiplied / result_alpha
fn premultiply_lerp(c1: f64, a1: f64, c2: f64, a2: f64, t: f64, result_alpha: f64) -> f64 {
	if result_alpha == 0.0 {
		return c1 * (1.0 - t) + c2 * t;
	}
	let pm1 = c1 * a1;
	let pm2 = c2 * a2;
	(pm1 * (1.0 - t) + pm2 * t) / result_alpha
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
impl sealed::PolarColor for Hwb {}
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let r = premultiply_lerp(first.red as f64, a1, second.red as f64, a2, t, a);
		let g = premultiply_lerp(first.green as f64, a1, second.green as f64, a2, t, a);
		let b = premultiply_lerp(first.blue as f64, a1, second.blue as f64, a2, t, a);
		Srgb::new(r.round() as u8, g.round() as u8, b.round() as u8, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let r = premultiply_lerp(first.red, a1, second.red, a2, t, a);
		let g = premultiply_lerp(first.green, a1, second.green, a2, t, a);
		let b = premultiply_lerp(first.blue, a1, second.blue, a2, t, a);
		LinearRgb::new(r, g, b, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let h = interpolate_hue(first.hue as f64, second.hue as f64, t, hue_interpolation);
		let s = premultiply_lerp(first.saturation as f64, a1, second.saturation as f64, a2, t, a);
		let l = premultiply_lerp(first.lightness as f64, a1, second.lightness as f64, a2, t, a);
		Hsl::new(h as f32, s as f32, l as f32, (a * 100.0) as f32)
	}
}

impl<T, U> ColorMixPolar<T, U> for Hwb
where
	Self: From<T> + From<U>,
{
	fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let h = interpolate_hue(first.hue as f64, second.hue as f64, t, hue_interpolation);
		let w = premultiply_lerp(first.whiteness as f64, a1, second.whiteness as f64, a2, t, a);
		let b = premultiply_lerp(first.blackness as f64, a1, second.blackness as f64, a2, t, a);
		Hwb::new(h as f32, w as f32, b as f32, (a * 100.0) as f32)
	}
}

impl<T, U> ColorMix<T, U> for A98Rgb
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let r = premultiply_lerp(first.red, a1, second.red, a2, t, a);
		let g = premultiply_lerp(first.green, a1, second.green, a2, t, a);
		let b = premultiply_lerp(first.blue, a1, second.blue, a2, t, a);
		A98Rgb::new(r, g, b, (a * 100.0) as f32)
	}
}

impl<T, U> ColorMix<T, U> for DisplayP3
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let r = premultiply_lerp(first.red, a1, second.red, a2, t, a);
		let g = premultiply_lerp(first.green, a1, second.green, a2, t, a);
		let b = premultiply_lerp(first.blue, a1, second.blue, a2, t, a);
		DisplayP3::new(r, g, b, (a * 100.0) as f32)
	}
}

impl<T, U> ColorMix<T, U> for ProphotoRgb
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let r = premultiply_lerp(first.red, a1, second.red, a2, t, a);
		let g = premultiply_lerp(first.green, a1, second.green, a2, t, a);
		let b = premultiply_lerp(first.blue, a1, second.blue, a2, t, a);
		ProphotoRgb::new(r, g, b, (a * 100.0) as f32)
	}
}

impl<T, U> ColorMix<T, U> for Rec2020
where
	Self: From<T> + From<U>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self {
		let first: Self = first.into();
		let second: Self = second.into();
		let t = percentage / 100.0;
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let r = premultiply_lerp(first.red, a1, second.red, a2, t, a);
		let g = premultiply_lerp(first.green, a1, second.green, a2, t, a);
		let b = premultiply_lerp(first.blue, a1, second.blue, a2, t, a);
		Rec2020::new(r, g, b, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let l = premultiply_lerp(first.lightness, a1, second.lightness, a2, t, a);
		let ab_a = premultiply_lerp(first.a, a1, second.a, a2, t, a);
		let ab_b = premultiply_lerp(first.b, a1, second.b, a2, t, a);
		Lab::new(l, ab_a, ab_b, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let l = premultiply_lerp(first.lightness, a1, second.lightness, a2, t, a);
		let c = premultiply_lerp(first.chroma, a1, second.chroma, a2, t, a);
		let h = interpolate_hue(first.hue, second.hue, t, hue_interpolation);
		Lch::new(l, c, h, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let l = premultiply_lerp(first.lightness, a1, second.lightness, a2, t, a);
		let ab_a = premultiply_lerp(first.a, a1, second.a, a2, t, a);
		let ab_b = premultiply_lerp(first.b, a1, second.b, a2, t, a);
		Oklab::new(l, ab_a, ab_b, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let l = premultiply_lerp(first.lightness, a1, second.lightness, a2, t, a);
		let c = premultiply_lerp(first.chroma, a1, second.chroma, a2, t, a);
		let h = interpolate_hue(first.hue, second.hue, t, hue_interpolation);
		Oklch::new(l, c, h, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let x = premultiply_lerp(first.x, a1, second.x, a2, t, a);
		let y = premultiply_lerp(first.y, a1, second.y, a2, t, a);
		let z = premultiply_lerp(first.z, a1, second.z, a2, t, a);
		XyzD50::new(x, y, z, (a * 100.0) as f32)
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
		let a1 = first.alpha as f64 / 100.0;
		let a2 = second.alpha as f64 / 100.0;
		let a = a1 * (1.0 - t) + a2 * t;
		let x = premultiply_lerp(first.x, a1, second.x, a2, t, a);
		let y = premultiply_lerp(first.y, a1, second.y, a2, t, a);
		let z = premultiply_lerp(first.z, a1, second.z, a2, t, a);
		XyzD65::new(x, y, z, (a * 100.0) as f32)
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
		assert_eq!(mixed.red, 170);
		assert_eq!(mixed.green, 0);
		assert_eq!(mixed.blue, 85);
		assert_eq!(mixed.alpha, 60.0);
	}

	#[test]
	fn test_hwb_mix() {
		// Hwb is polar - default mix uses Shorter hue interpolation
		// 0° to 240°: diff=240 > 180, so shorter wraps via 360°, midpoint is 300°
		let red = Hwb::new(0.0, 0.0, 0.0, 100.0);
		let blue = Hwb::new(240.0, 0.0, 0.0, 100.0);
		let mixed = Hwb::mix(red, blue, 50.0);
		assert_close_to!(mixed, Hwb::new(300.0, 0.0, 0.0, 100.0));
	}

	#[test]
	fn test_hwb_mix_polar() {
		// 0° to 240°: longer arc goes through 120°
		let red = Hwb::new(0.0, 0.0, 0.0, 100.0);
		let blue = Hwb::new(240.0, 0.0, 0.0, 100.0);
		assert_close_to!(Hwb::mix_polar(red, blue, 50.0, HueInterpolation::Longer), Hwb::new(120.0, 0.0, 0.0, 100.0));
	}

	#[test]
	fn test_a98_rgb_mix() {
		let c1 = A98Rgb::new(1.0, 0.0, 0.0, 100.0);
		let c2 = A98Rgb::new(0.0, 0.0, 1.0, 100.0);
		let mixed = A98Rgb::mix(c1, c2, 50.0);
		assert_close_to!(mixed, A98Rgb::new(0.5, 0.0, 0.5, 100.0));
	}

	/// WPT: color-mix(in lab, lab(10 20 30 / .4), lab(50 60 70 / .8)) → lab(36.666664 46.666664 56.666664 / 0.6)
	#[test]
	fn test_premultiplied_alpha_lab() {
		let c1 = Lab::new(10.0, 20.0, 30.0, 40.0);
		let c2 = Lab::new(50.0, 60.0, 70.0, 80.0);
		let mixed = Lab::mix(c1, c2, 50.0);
		assert_close_to!(mixed, Lab::new(36.666664, 46.666664, 56.666664, 60.0));
	}

	/// WPT: color-mix(in lab, lab(10 20 30 / .4) 25%, lab(50 60 70 / .8)) → lab(44.285713 54.285717 64.28571 / 0.7)
	#[test]
	fn test_premultiplied_alpha_lab_25_75() {
		let c1 = Lab::new(10.0, 20.0, 30.0, 40.0);
		let c2 = Lab::new(50.0, 60.0, 70.0, 80.0);
		// 25% first, 75% second: mix_percentage = 75
		let mixed = Lab::mix(c1, c2, 75.0);
		assert_close_to!(mixed, Lab::new(44.285713, 54.285717, 64.28571, 70.0));
	}

	/// WPT: color-mix(in oklch, oklch(0.1 0.2 30deg / .4), oklch(0.5 0.6 70deg / .8)) → oklch(0.36666664 0.46666664 50 / 0.6)
	#[test]
	fn test_premultiplied_alpha_oklch() {
		let c1 = Oklch::new(0.1, 0.2, 30.0, 40.0);
		let c2 = Oklch::new(0.5, 0.6, 70.0, 80.0);
		let mixed = Oklch::mix(c1, c2, 50.0);
		assert_close_to!(mixed, Oklch::new(0.36666664, 0.46666664, 50.0, 60.0));
	}

	/// When both alphas are 100%, premultiplied interpolation == simple interpolation.
	#[test]
	fn test_premultiplied_alpha_opaque_same_as_simple() {
		let c1 = Lab::new(10.0, 20.0, 30.0, 100.0);
		let c2 = Lab::new(50.0, 60.0, 70.0, 100.0);
		let mixed = Lab::mix(c1, c2, 50.0);
		assert_close_to!(mixed, Lab::new(30.0, 40.0, 50.0, 100.0));
	}
}
