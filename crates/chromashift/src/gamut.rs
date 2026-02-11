use crate::*;

/// Whether a colour is within the natural bounds of its colour space, and the ability to produce a naively clamped version.
///
/// CSS Color 4 12.1 and 13.1 state that out-of-gamut values must be preserved through intermediate computations.
/// Gamut mapping (reducing to displayable range) only happens at "actual-value" / display time. This trait lets
/// callers query and clamp when appropriate.
///
/// Note: `clamp_to_gamut()` performs naive channel clamping. This is *not* the perceptual gamut-mapping algorithm from
/// CSS Color 4 13.2 (which reduces Oklch chroma iteratively).
pub trait Gamut: Sized {
	/// Returns `true` if all colour channels are within the natural bounds of this colour space.
	/// Alpha is not considered — it is always clamped on construction.
	fn in_gamut(&self) -> bool;

	/// Returns a copy with all colour channels clamped to the natural bounds of this colour space.
	fn clamp_to_gamut(&self) -> Self;
}

/// Helper: checks an f64 is in [0.0, 1.0]
fn in_unit(v: f64) -> bool {
	(0.0..=1.0).contains(&v)
}

/// Helper: checks an f32 is in [0.0, 100.0]
fn in_percent(v: f32) -> bool {
	(0.0..=100.0).contains(&v)
}

macro_rules! impl_gamut_rgb_f64 {
	($ty:ident) => {
		impl Gamut for $ty {
			fn in_gamut(&self) -> bool {
				in_unit(self.red) && in_unit(self.green) && in_unit(self.blue)
			}

			fn clamp_to_gamut(&self) -> Self {
				Self::new(self.red.clamp(0.0, 1.0), self.green.clamp(0.0, 1.0), self.blue.clamp(0.0, 1.0), self.alpha)
			}
		}
	};
}

impl_gamut_rgb_f64!(LinearRgb);
impl_gamut_rgb_f64!(A98Rgb);
impl_gamut_rgb_f64!(DisplayP3);
impl_gamut_rgb_f64!(ProphotoRgb);
impl_gamut_rgb_f64!(Rec2020);

impl Gamut for Srgb {
	fn in_gamut(&self) -> bool {
		true
	}

	fn clamp_to_gamut(&self) -> Self {
		*self
	}
}

impl Gamut for Hex {
	fn in_gamut(&self) -> bool {
		true
	}

	fn clamp_to_gamut(&self) -> Self {
		*self
	}
}

impl Gamut for Lab {
	fn in_gamut(&self) -> bool {
		(0.0..=100.0).contains(&self.lightness)
			&& (-125.0..=125.0).contains(&self.a)
			&& (-125.0..=125.0).contains(&self.b)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(
			self.lightness.clamp(0.0, 100.0),
			self.a.clamp(-125.0, 125.0),
			self.b.clamp(-125.0, 125.0),
			self.alpha,
		)
	}
}

impl Gamut for Oklab {
	fn in_gamut(&self) -> bool {
		(0.0..=1.0).contains(&self.lightness) && (-0.4..=0.4).contains(&self.a) && (-0.4..=0.4).contains(&self.b)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.lightness.clamp(0.0, 1.0), self.a.clamp(-0.4, 0.4), self.b.clamp(-0.4, 0.4), self.alpha)
	}
}

impl Gamut for Lch {
	fn in_gamut(&self) -> bool {
		(0.0..=100.0).contains(&self.lightness) && (0.0..=150.0).contains(&self.chroma)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.lightness.clamp(0.0, 100.0), self.chroma.clamp(0.0, 150.0), self.hue, self.alpha)
	}
}

impl Gamut for Oklch {
	fn in_gamut(&self) -> bool {
		(0.0..=1.0).contains(&self.lightness) && (0.0..=0.4).contains(&self.chroma)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.lightness.clamp(0.0, 1.0), self.chroma.clamp(0.0, 0.4), self.hue, self.alpha)
	}
}

impl Gamut for Hsl {
	fn in_gamut(&self) -> bool {
		in_percent(self.saturation) && in_percent(self.lightness)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.hue, self.saturation.clamp(0.0, 100.0), self.lightness.clamp(0.0, 100.0), self.alpha)
	}
}

impl Gamut for Hwb {
	fn in_gamut(&self) -> bool {
		in_percent(self.whiteness) && in_percent(self.blackness)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.hue, self.whiteness.clamp(0.0, 100.0), self.blackness.clamp(0.0, 100.0), self.alpha)
	}
}

// Hsv already clamps in its constructor, so it's always in gamut.
impl Gamut for Hsv {
	fn in_gamut(&self) -> bool {
		true
	}

	fn clamp_to_gamut(&self) -> Self {
		*self
	}
}

impl Gamut for XyzD50 {
	fn in_gamut(&self) -> bool {
		self.x >= 0.0 && self.y >= 0.0 && self.z >= 0.0 && self.y <= 100.0
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.x.max(0.0), self.y.clamp(0.0, 100.0), self.z.max(0.0), self.alpha)
	}
}

impl Gamut for XyzD65 {
	fn in_gamut(&self) -> bool {
		self.x >= 0.0 && self.y >= 0.0 && self.z >= 0.0 && self.y <= 100.0
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.x.max(0.0), self.y.clamp(0.0, 100.0), self.z.max(0.0), self.alpha)
	}
}

impl Gamut for Color {
	fn in_gamut(&self) -> bool {
		match self {
			Color::A98Rgb(c) => c.in_gamut(),
			Color::DisplayP3(c) => c.in_gamut(),
			Color::Hex(c) => c.in_gamut(),
			Color::Hsv(c) => c.in_gamut(),
			Color::Hsl(c) => c.in_gamut(),
			Color::Hwb(c) => c.in_gamut(),
			Color::Lab(c) => c.in_gamut(),
			Color::Lch(c) => c.in_gamut(),
			Color::LinearRgb(c) => c.in_gamut(),
			Color::Named(_) => true,
			Color::Oklab(c) => c.in_gamut(),
			Color::Oklch(c) => c.in_gamut(),
			Color::ProphotoRgb(c) => c.in_gamut(),
			Color::Rec2020(c) => c.in_gamut(),
			Color::Srgb(c) => c.in_gamut(),
			Color::XyzD50(c) => c.in_gamut(),
			Color::XyzD65(c) => c.in_gamut(),
		}
	}

	fn clamp_to_gamut(&self) -> Self {
		match self {
			Color::A98Rgb(c) => Color::A98Rgb(c.clamp_to_gamut()),
			Color::DisplayP3(c) => Color::DisplayP3(c.clamp_to_gamut()),
			Color::Hex(c) => Color::Hex(c.clamp_to_gamut()),
			Color::Hsv(c) => Color::Hsv(c.clamp_to_gamut()),
			Color::Hsl(c) => Color::Hsl(c.clamp_to_gamut()),
			Color::Hwb(c) => Color::Hwb(c.clamp_to_gamut()),
			Color::Lab(c) => Color::Lab(c.clamp_to_gamut()),
			Color::Lch(c) => Color::Lch(c.clamp_to_gamut()),
			Color::LinearRgb(c) => Color::LinearRgb(c.clamp_to_gamut()),
			Color::Named(n) => Color::Named(*n),
			Color::Oklab(c) => Color::Oklab(c.clamp_to_gamut()),
			Color::Oklch(c) => Color::Oklch(c.clamp_to_gamut()),
			Color::ProphotoRgb(c) => Color::ProphotoRgb(c.clamp_to_gamut()),
			Color::Rec2020(c) => Color::Rec2020(c.clamp_to_gamut()),
			Color::Srgb(c) => Color::Srgb(c.clamp_to_gamut()),
			Color::XyzD50(c) => Color::XyzD50(c.clamp_to_gamut()),
			Color::XyzD65(c) => Color::XyzD65(c.clamp_to_gamut()),
		}
	}
}
