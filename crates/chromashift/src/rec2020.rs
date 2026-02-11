use crate::{ToAlpha, XyzD65, round_dp};
use core::fmt;

/// A colour in the Rec. 2020 colour space.
/// The components are:
/// - Red - a number between 0.0 and 1.0
/// - Green - a number between 0.0 and 1.0
/// - Blue - a number between 0.0 and 1.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rec2020 {
	pub red: f64,
	pub green: f64,
	pub blue: f64,
	pub alpha: f32,
}

impl Rec2020 {
	pub fn new(red: f64, green: f64, blue: f64, alpha: f32) -> Self {
		Self { red, green, blue, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for Rec2020 {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Rec2020 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { red, green, blue, alpha } = self;
		write!(f, "color(rec2020 {} {} {}", round_dp(*red, 2), round_dp(*green, 2), round_dp(*blue, 2))?;
		if *alpha < 100.0 {
			write!(f, " / {}", round_dp(*alpha as f64, 2))?;
		}
		write!(f, ")")
	}
}

const ALPHA: f64 = 1.09929682680944;
const BETA: f64 = 0.018053968510807;

/// BT.2020 transfer function: linear to gamma-encoded
fn gamma(u: f64) -> f64 {
	let abs = u.abs();
	if abs >= BETA { u.signum() * (ALPHA * abs.powf(0.45) - (ALPHA - 1.0)) } else { u * 4.5 }
}

/// BT.2020 transfer function: gamma-encoded to linear
fn linear(c: f64) -> f64 {
	let abs = c.abs();
	if abs >= BETA * 4.5 { c.signum() * ((abs + (ALPHA - 1.0)) / ALPHA).powf(1.0 / 0.45) } else { c / 4.5 }
}

impl From<XyzD65> for Rec2020 {
	fn from(value: XyzD65) -> Self {
		let XyzD65 { x, y, z, alpha } = value;
		let x = x / 100.0;
		let y = y / 100.0;
		let z = z / 100.0;
		// XYZ D65 -> Linear Rec. 2020
		let lr = x * 1.7166511879712674 + y * (-0.35567078377639233) + z * (-0.25336628137365974);
		let lg = x * (-0.666684351832489) + y * 1.616481236634939 + z * 0.01576854581391113;
		let lb = x * 0.017639857445310783 + y * (-0.042770613257808524) + z * 0.9421031212354738;
		// Apply BT.2020 gamma
		Rec2020::new(gamma(lr), gamma(lg), gamma(lb), alpha)
	}
}

impl From<Rec2020> for XyzD65 {
	fn from(value: Rec2020) -> Self {
		let Rec2020 { red, green, blue, alpha } = value;
		// Linearize with BT.2020 gamma
		let lr = linear(red);
		let lg = linear(green);
		let lb = linear(blue);
		// Linear Rec. 2020 -> XYZ D65
		let x = lr * 0.6369580483012914 + lg * 0.14461690358620832 + lb * 0.16888097516417205;
		let y = lr * 0.2627002120112671 + lg * 0.6779980715188708 + lb * 0.05930171646986196;
		let z = lr * 0.0 + lg * 0.028072693049087428 + lb * 1.0609850577107909;
		XyzD65::new(x * 100.0, y * 100.0, z * 100.0, alpha)
	}
}
