/// Identifies a colour space for gamut containment checks.
///
/// Used with [`crate::Color::in_gamut_of`] to test whether a colour can be represented in a target space without clamping.
///
/// Implements [`PartialOrd`] based on gamut containment — `a >= b` means every colour representable in `b` is also
/// representable in `a`.  This is a *partial* order because some pairs (e.g. Display P3 and A98 RGB) overlap without
/// either being a strict superset.
///
/// ```text
/// sRGB ⊂ Display P3 ⊂ Rec 2020 ⊂ ProPhoto RGB
/// sRGB ⊂ A98 RGB ⊂ ProPhoto RGB
/// ```
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
		let self_contains_other = self.contains(*other);
		let other_contains_self = other.contains(*self);
		match (self_contains_other, other_contains_self) {
			(true, false) => Some(core::cmp::Ordering::Greater),
			(false, true) => Some(core::cmp::Ordering::Less),
			_ => None,
		}
	}
}

impl ColorSpace {
	/// Returns `true` if every colour representable in `other` is also representable in `self`.
	pub fn contains(self, other: ColorSpace) -> bool {
		if self == other {
			return true;
		}
		match self {
			ColorSpace::Srgb => false,
			ColorSpace::DisplayP3 => other == ColorSpace::Srgb,
			ColorSpace::A98Rgb => other == ColorSpace::Srgb,
			ColorSpace::ProphotoRgb => other != ColorSpace::ProphotoRgb,
			ColorSpace::Rec2020 => matches!(other, ColorSpace::Srgb | ColorSpace::DisplayP3),
		}
	}
}
