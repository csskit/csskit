/// The transfer function (a.k.a. "gamma") pairing a linear light value with its encoded form.
///
/// Every variant is defined for the whole real line by mirroring the curve about zero, as CSS Color 4 requires for
/// out-of-gamut channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transfer {
	Linear,
	Srgb,
	A98Rgb,
	ProphotoRgb,
	/// The ITU-R BT.2020 OETF, which is what browser engines currently render `rec2020` with. CSS Color 4 now specifies
	/// the BT.1886 EOTF, a plain 2.4 gamma, instead (<https://github.com/w3c/csswg-drafts/issues/12574>); the
	/// `rec2020-bt1886` feature switches to that.
	Rec2020,
}

impl Transfer {
	const REC2020_ALPHA: f64 = 1.09929682680944;
	const REC2020_BETA: f64 = 0.018053968510807;

	/// Encodes a linear light value, e.g. linear sRGB 0.2140 -> sRGB 0.5.
	pub fn encode(self, linear: f64) -> f64 {
		let abs = linear.abs();
		match self {
			Self::Linear => linear,
			Self::Srgb => {
				if abs <= 0.0031308 {
					linear * 12.92
				} else {
					linear.signum() * (1.055 * abs.powf(1.0 / 2.4) - 0.055)
				}
			}
			Self::A98Rgb => linear.signum() * abs.powf(256.0 / 563.0),
			Self::ProphotoRgb => {
				if abs >= 1.0 / 512.0 {
					linear.signum() * abs.powf(1.0 / 1.8)
				} else {
					linear * 16.0
				}
			}
			Self::Rec2020 => {
				if cfg!(feature = "rec2020-bt1886") {
					linear.signum() * abs.powf(1.0 / 2.4)
				} else if abs >= Self::REC2020_BETA {
					linear.signum() * (Self::REC2020_ALPHA * abs.powf(0.45) - (Self::REC2020_ALPHA - 1.0))
				} else {
					linear * 4.5
				}
			}
		}
	}

	/// Decodes an encoded value back to linear light, e.g. sRGB 0.5 -> linear sRGB 0.2140.
	pub fn decode(self, encoded: f64) -> f64 {
		let abs = encoded.abs();
		match self {
			Self::Linear => encoded,
			Self::Srgb => {
				if abs > 0.04045 {
					encoded.signum() * ((abs + 0.055) / 1.055).powf(2.4)
				} else {
					encoded / 12.92
				}
			}
			Self::A98Rgb => encoded.signum() * abs.powf(563.0 / 256.0),
			Self::ProphotoRgb => {
				if abs >= 16.0 / 512.0 {
					encoded.signum() * abs.powf(1.8)
				} else {
					encoded / 16.0
				}
			}
			Self::Rec2020 => {
				if cfg!(feature = "rec2020-bt1886") {
					encoded.signum() * abs.powf(2.4)
				} else if abs >= Self::REC2020_BETA * 4.5 {
					encoded.signum() * ((abs + (Self::REC2020_ALPHA - 1.0)) / Self::REC2020_ALPHA).powf(1.0 / 0.45)
				} else {
					encoded / 4.5
				}
			}
		}
	}

	/// Encodes three linear light channels, e.g. the red, green and blue of an RGB space.
	pub fn encode_all(self, linear: [f64; 3]) -> [f64; 3] {
		[self.encode(linear[0]), self.encode(linear[1]), self.encode(linear[2])]
	}

	/// Decodes three encoded channels back to linear light.
	pub fn decode_all(self, encoded: [f64; 3]) -> [f64; 3] {
		[self.decode(encoded[0]), self.decode(encoded[1]), self.decode(encoded[2])]
	}
}
