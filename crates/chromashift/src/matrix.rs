/// A 3x3 matrix of [`f64`], as used for linear colour space conversions.
///
/// Rows are the outer array: `Matrix3([row0, row1, row2])`.
///
/// ```
/// # use chromashift::Matrix3;
/// let m = Matrix3([[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]]);
/// assert_eq!(m.transform([1.0, 2.0, 3.0]), [2.0, 4.0, 6.0]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3(pub [[f64; 3]; 3]);

impl Matrix3 {
	/// The matrix that leaves a vector unchanged.
	pub const IDENTITY: Self = Self::diagonal([1.0, 1.0, 1.0]);

	/// A matrix scaling each component independently.
	pub const fn diagonal([x, y, z]: [f64; 3]) -> Self {
		Self([[x, 0.0, 0.0], [0.0, y, 0.0], [0.0, 0.0, z]])
	}

	/// Applies this matrix to a column vector.
	pub const fn transform(&self, [x, y, z]: [f64; 3]) -> [f64; 3] {
		let Self(m) = self;
		[
			m[0][0] * x + m[0][1] * y + m[0][2] * z,
			m[1][0] * x + m[1][1] * y + m[1][2] * z,
			m[2][0] * x + m[2][1] * y + m[2][2] * z,
		]
	}

	/// Multiplies two matrices, so that `a.then(b)` applies `a` first, then `b`.
	pub const fn then(&self, next: &Self) -> Self {
		let (Self(first), Self(second)) = (self, next);
		let mut out = [[0.0; 3]; 3];
		let mut row = 0;
		while row < 3 {
			let mut col = 0;
			while col < 3 {
				out[row][col] =
					second[row][0] * first[0][col] + second[row][1] * first[1][col] + second[row][2] * first[2][col];
				col += 1;
			}
			row += 1;
		}
		Self(out)
	}

	/// The inverse of this matrix.
	///
	/// Colour conversion matrices are always invertible; a singular matrix yields infinities rather than panicking.
	///
	/// ```
	/// # use chromashift::{Matrix3, RgbSpace};
	/// let roundtrip = RgbSpace::SRGB.to_xyz.inverse().then(&RgbSpace::SRGB.to_xyz);
	/// assert!((roundtrip.0[0][0] - 1.0).abs() < 1e-12);
	/// ```
	pub const fn inverse(&self) -> Self {
		let Self(m) = self;
		let c00 = m[1][1] * m[2][2] - m[1][2] * m[2][1];
		let c01 = m[1][2] * m[2][0] - m[1][0] * m[2][2];
		let c02 = m[1][0] * m[2][1] - m[1][1] * m[2][0];
		let det = m[0][0] * c00 + m[0][1] * c01 + m[0][2] * c02;
		Self([
			[c00 / det, (m[0][2] * m[2][1] - m[0][1] * m[2][2]) / det, (m[0][1] * m[1][2] - m[0][2] * m[1][1]) / det],
			[c01 / det, (m[0][0] * m[2][2] - m[0][2] * m[2][0]) / det, (m[0][2] * m[1][0] - m[0][0] * m[1][2]) / det],
			[c02 / det, (m[0][1] * m[2][0] - m[0][0] * m[2][1]) / det, (m[0][0] * m[1][1] - m[0][1] * m[1][0]) / det],
		])
	}
}
