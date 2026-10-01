/// Identifies a colour space for gamut containment checks.
///
/// Used with [`crate::Color::in_gamut_of`] to test whether a colour can be represented in a target space without
/// clamping.
///
/// Implements [`PartialOrd`] based on gamut containment - `a >= b` means every colour representable in `b` is also
/// representable in `a`. This is a *partial* order because some pairs (e.g. Display P3 and A98 RGB) overlap without
/// either being a strict superset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
	Srgb,
	DisplayP3,
	A98Rgb,
	ProphotoRgb,
	Rec2020,
}

impl PartialOrd for ColorSpace {
	fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
		if self == other {
			return Some(core::cmp::Ordering::Equal);
		}
		match (self.contains(*other), other.contains(*self)) {
			(true, false) => Some(core::cmp::Ordering::Greater),
			(false, true) => Some(core::cmp::Ordering::Less),
			_ => None,
		}
	}
}

impl ColorSpace {
	/// Returns `true` if every colour representable in `other` is also representable in `self`.
	///
	/// ```text
	/// sRGB ⊂ Display P3 ⊂ Rec 2020 ⊂ ProPhoto RGB
	/// sRGB ⊂ A98 RGB ⊂ ProPhoto RGB
	/// ```
	pub const fn contains(self, other: ColorSpace) -> bool {
		match (self, other) {
			(Self::Srgb, Self::Srgb) => true,
			(Self::Srgb, _) => false,
			(Self::DisplayP3, Self::Srgb | Self::DisplayP3) => true,
			(Self::DisplayP3, _) => false,
			(Self::A98Rgb, Self::Srgb | Self::A98Rgb) => true,
			(Self::A98Rgb, _) => false,
			(Self::ProphotoRgb, _) => true,
			(Self::Rec2020, Self::Srgb | Self::DisplayP3 | Self::Rec2020) => true,
			(Self::Rec2020, _) => false,
		}
	}
}
