/// A colour expressed as X, Y and Z values, expressed in the CIE XYZ tristimulus colour space, with an explicit D65
/// white point.
/// The components are:
/// - X - a number between 0.0 and 100.0
/// - Y - a number between 0.0 and 100.0
/// - Z - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyzD65 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub alpha: f32,
}

impl XyzD65 {
	pub fn new(x: f64, y: f64, z: f64, alpha: f32) -> Self {
		Self { x: x.clamp(0.0, 100.0), y: y.clamp(0.0, 100.0), z: z.clamp(0.0, 100.0), alpha: alpha.clamp(0.0, 100.0) }
	}
}
