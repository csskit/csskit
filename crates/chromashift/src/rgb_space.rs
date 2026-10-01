use crate::{Chromaticity, Matrix3, Transfer};

/// A description of an RGB colour space: its white point, and how to move between its encoded channels and CIE XYZ.
///
/// The matrices are relative to the space's own [`white`](RgbSpace::white) point, with `Y = 1` for that white.
///
/// The constants are the predefined RGB colour spaces of CSS Color 4, with the matrices from its sample code:
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
///
/// ```
/// # use chromashift::RgbSpace;
/// let white = RgbSpace::SRGB.xyz_from_encoded([1.0, 1.0, 1.0]);
/// assert!(white.iter().zip(RgbSpace::SRGB.white.xyz()).all(|(a, b)| (a - b).abs() < 1e-12));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbSpace {
	/// Chromaticity of the white point.
	pub white: Chromaticity,
	/// Linear RGB -> CIE XYZ, at this space's white point.
	pub to_xyz: Matrix3,
	/// CIE XYZ -> linear RGB, at this space's white point.
	pub from_xyz: Matrix3,
	/// The transfer function between linear light and this space's encoded channels.
	pub transfer: Transfer,
}

impl RgbSpace {
	/// sRGB (IEC 61966-2-1), D65.
	///
	/// <https://drafts.csswg.org/css-color-4/#predefined-sRGB>
	pub const SRGB: Self = Self { transfer: Transfer::Srgb, ..Self::SRGB_LINEAR };

	/// sRGB primaries with no transfer function ("srgb-linear"), D65.
	///
	/// <https://drafts.csswg.org/css-color-4/#predefined-sRGB-linear>
	pub const SRGB_LINEAR: Self = Self {
		white: Chromaticity::D65,
		to_xyz: Matrix3([
			[506752.0 / 1228815.0, 87881.0 / 245763.0, 12673.0 / 70218.0],
			[87098.0 / 409605.0, 175762.0 / 245763.0, 12673.0 / 175545.0],
			[7918.0 / 409605.0, 87881.0 / 737289.0, 1001167.0 / 1053270.0],
		]),
		from_xyz: Matrix3([
			[12831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0],
			[-851781.0 / 878810.0, 1648619.0 / 878810.0, 36519.0 / 878810.0],
			[705.0 / 12673.0, -2585.0 / 12673.0, 705.0 / 667.0],
		]),
		transfer: Transfer::Linear,
	};

	/// Display P3 (DCI-P3 primaries, sRGB transfer), D65.
	///
	/// <https://drafts.csswg.org/css-color-4/#predefined-display-p3>
	pub const DISPLAY_P3: Self = Self {
		white: Chromaticity::D65,
		to_xyz: Matrix3([
			[608311.0 / 1250200.0, 189793.0 / 714400.0, 198249.0 / 1000160.0],
			[35783.0 / 156275.0, 247089.0 / 357200.0, 198249.0 / 2500400.0],
			[0.0, 32229.0 / 714400.0, 5220557.0 / 5000800.0],
		]),
		from_xyz: Matrix3([
			[446124.0 / 178915.0, -333277.0 / 357830.0, -72051.0 / 178915.0],
			[-14852.0 / 17905.0, 63121.0 / 35810.0, 423.0 / 17905.0],
			[11844.0 / 330415.0, -50337.0 / 660830.0, 316169.0 / 330415.0],
		]),
		transfer: Transfer::Srgb,
	};

	/// Adobe RGB (1998), D65.
	///
	/// <https://drafts.csswg.org/css-color-4/#predefined-a98-rgb>
	pub const A98_RGB: Self = Self {
		white: Chromaticity::D65,
		to_xyz: Matrix3([
			[573536.0 / 994567.0, 263643.0 / 1420810.0, 187206.0 / 994567.0],
			[591459.0 / 1989134.0, 6239551.0 / 9945670.0, 374412.0 / 4972835.0],
			[53769.0 / 1989134.0, 351524.0 / 4972835.0, 4929758.0 / 4972835.0],
		]),
		from_xyz: Matrix3([
			[1829569.0 / 896150.0, -506331.0 / 896150.0, -308931.0 / 896150.0],
			[-851781.0 / 878810.0, 1648619.0 / 878810.0, 36519.0 / 878810.0],
			[16779.0 / 1248040.0, -147721.0 / 1248040.0, 1266979.0 / 1248040.0],
		]),
		transfer: Transfer::A98Rgb,
	};

	/// ProPhoto RGB (ROMM RGB), D50.
	///
	/// <https://drafts.csswg.org/css-color-4/#predefined-prophoto-rgb>
	pub const PROPHOTO_RGB: Self = Self {
		white: Chromaticity::D50,
		to_xyz: Matrix3([
			[0.7977666449006423, 0.13518129740053308, 0.0313477341283922],
			[0.2880748288194013, 0.711835234241873, 0.00008993693872564],
			[0.0, 0.0, 0.8251046025104602],
		]),
		from_xyz: Matrix3([
			[1.3457868816471583, -0.25557208737979464, -0.05110186497554526],
			[-0.5446307051249019, 1.5082477428451468, 0.02052744743642139],
			[0.0, 0.0, 1.2119675456389452],
		]),
		transfer: Transfer::ProphotoRgb,
	};

	/// ITU-R BT.2020 primaries, D65, with the transfer function described at [`Transfer::Rec2020`].
	///
	/// <https://drafts.csswg.org/css-color-4/#predefined-rec2020>
	pub const REC2020: Self = Self {
		white: Chromaticity::D65,
		to_xyz: Matrix3([
			[63426534.0 / 99577255.0, 20160776.0 / 139408157.0, 47086771.0 / 278816314.0],
			[26158966.0 / 99577255.0, 472592308.0 / 697040785.0, 8267143.0 / 139408157.0],
			[0.0, 19567812.0 / 697040785.0, 295819943.0 / 278816314.0],
		]),
		from_xyz: Matrix3([
			[30757411.0 / 17917100.0, -6372589.0 / 17917100.0, -4539589.0 / 17917100.0],
			[-19765991.0 / 29648200.0, 47925759.0 / 29648200.0, 467509.0 / 29648200.0],
			[792561.0 / 44930125.0, -1921689.0 / 44930125.0, 42328811.0 / 44930125.0],
		]),
		transfer: Transfer::Rec2020,
	};

	/// Converts linear channels of this space to CIE XYZ at this space's white point.
	pub const fn xyz_from_linear(&self, rgb: [f64; 3]) -> [f64; 3] {
		self.to_xyz.transform(rgb)
	}

	/// Converts CIE XYZ at this space's white point to this space's linear channels.
	pub const fn linear_from_xyz(&self, xyz: [f64; 3]) -> [f64; 3] {
		self.from_xyz.transform(xyz)
	}

	/// Converts this space's encoded channels to CIE XYZ at this space's white point.
	pub fn xyz_from_encoded(&self, rgb: [f64; 3]) -> [f64; 3] {
		self.xyz_from_linear(self.transfer.decode_all(rgb))
	}

	/// Converts CIE XYZ at this space's white point to this space's encoded channels.
	pub fn encoded_from_xyz(&self, xyz: [f64; 3]) -> [f64; 3] {
		self.transfer.encode_all(self.linear_from_xyz(xyz))
	}
}
