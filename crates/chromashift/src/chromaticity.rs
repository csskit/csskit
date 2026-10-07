use crate::Matrix3;

/// A CIE 1931 xy chromaticity: a colour's hue and purity with its luminance factored out.
///
/// Used for the white point of an [`RgbSpace`](crate::RgbSpace).
///
/// ```
/// # use chromashift::Chromaticity;
/// let [x, y, z] = Chromaticity::D65.xyz();
/// assert_eq!(y, 1.0);
/// assert!((x - 0.9504559).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chromaticity {
	pub x: f64,
	pub y: f64,
}

impl Chromaticity {
	/// The D65 white point, as used by CSS Color 4: <https://drafts.csswg.org/css-color-4/#d65>
	pub const D65: Self = Self::new(0.3127, 0.3290);

	/// The D50 white point, as used by CSS Color 4: <https://drafts.csswg.org/css-color-4/#d50>
	pub const D50: Self = Self::new(0.3457, 0.3585);

	const BRADFORD: Matrix3 =
		Matrix3([[0.8951, 0.2664, -0.1614], [-0.7502, 1.7135, 0.0367], [0.0389, -0.0685, 1.0296]]);

	pub const fn new(x: f64, y: f64) -> Self {
		Self { x, y }
	}

	/// This chromaticity as CIE XYZ, normalised to `Y = 1`.
	pub const fn xyz(self) -> [f64; 3] {
		[self.x / self.y, 1.0, (1.0 - self.x - self.y) / self.y]
	}

	/// The Bradford chromatic adaptation matrix taking CIE XYZ relative to `self` to XYZ relative to `other`.
	///
	/// <https://en.wikipedia.org/wiki/LMS_color_space#Bradford's_spectrally_sharpened_matrix_(LLAB,_CIECAM97s)>
	///
	/// ```
	/// # use chromashift::Chromaticity;
	/// let m = Chromaticity::D65.adapt_to(Chromaticity::D50);
	/// assert!((m.0[0][0] - 1.0479297925449969).abs() < 1e-12);
	/// ```
	pub const fn adapt_to(self, other: Self) -> Matrix3 {
		if self.x == other.x && self.y == other.y {
			return Matrix3::IDENTITY;
		}
		let source = Self::BRADFORD.transform(self.xyz());
		let dest = Self::BRADFORD.transform(other.xyz());
		let scale = Matrix3::diagonal([dest[0] / source[0], dest[1] / source[1], dest[2] / source[2]]);
		Self::BRADFORD.then(&scale).then(&Self::BRADFORD.inverse())
	}
}
