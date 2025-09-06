use std::fmt;

use crate::Srgb;

/// An Hex representation of the sRGB colour space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hex(u32);

fn is_shorthand_byte(byte: u8) -> bool {
	(byte >> 4) == (byte & 0xF)
}

impl Hex {
	pub fn new(hex: u32) -> Self {
		Self(hex)
	}

	pub fn can_use_3_digit(self) -> bool {
		let Srgb { red, green, blue, .. } = self.into();
		!self.has_alpha() && is_shorthand_byte(red) && is_shorthand_byte(green) && is_shorthand_byte(blue)
	}

	pub fn can_use_4_digit(self) -> bool {
		let Srgb { red, green, blue, alpha } = self.into();
		is_shorthand_byte(red)
			&& is_shorthand_byte(green)
			&& is_shorthand_byte(blue)
			&& is_shorthand_byte(((alpha as u32 * 255) / 100) as u8)
	}

	pub fn has_alpha(&self) -> bool {
		self.0 & 0xFF != 0xFF
	}
}

impl fmt::Display for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Srgb { red, green, blue, alpha } = (*self).into();
		let alpha = (alpha as u32 * 255) / 100;
		if self.can_use_3_digit() {
			write!(f, "#{:x}{:x}{:x}", red & 0xF, green & 0xF, blue & 0xF)
		} else if self.can_use_4_digit() {
			write!(f, "#{:x}{:x}{:x}{:x}", red & 0xF, green & 0xF, blue & 0xF, alpha & 0xF)
		} else if self.has_alpha() {
			write!(f, "#{red:02x}{green:02x}{blue:02x}{alpha:02x}")
		} else {
			write!(f, "#{red:02x}{green:02x}{blue:02x}")
		}
	}
}

impl From<Hex> for Srgb {
	fn from(value: Hex) -> Self {
		let hex = value.0;
		let r = ((hex >> 24) & 0xFF) as u8;
		let g = ((hex >> 16) & 0xFF) as u8;
		let b = ((hex >> 8) & 0xFF) as u8;
		let alpha = (((hex & 0xFF) * 100) / 255) as f32;
		Srgb::new(r, g, b, alpha)
	}
}

impl From<Srgb> for Hex {
	fn from(value: Srgb) -> Self {
		let Srgb { red, green, blue, alpha } = value;
		Hex::new(((red as u32) << 24) | ((green as u32) << 16) | ((blue as u32) << 8) | ((alpha as u32 * 255) / 100))
	}
}
