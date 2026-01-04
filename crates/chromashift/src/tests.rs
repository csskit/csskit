use super::*;

macro_rules! each_color {
		($macro:ident, $($tt:tt)*) => {
			$macro!(A98Rgb, $($tt)*);
			$macro!(Hsv, $($tt)*);
			$macro!(Hex, $($tt)*);
			$macro!(Hsl, $($tt)*);
			$macro!(Hwb, $($tt)*);
			$macro!(Lab, $($tt)*);
			$macro!(Lch, $($tt)*);
			$macro!(LinearRgb, $($tt)*);
			$macro!(Oklab, $($tt)*);
			$macro!(Oklch, $($tt)*);
			$macro!(Srgb, $($tt)*);
			$macro!(XyzD50, $($tt)*);
			$macro!(XyzD65, $($tt)*);
		};
}

macro_rules! each_color_pairs {
	($macro:ident, $srgb:ident, $linear:ident, $hex:ident, $hsl:ident, $hsv:ident, $hwb:ident, $lab:ident, $lch:ident, $oklab:ident, $oklch:ident, $a98_rgb:ident, $xyzd50:ident, $xyzd65:ident) => {
		$macro!(Srgb, $srgb);
		$macro!(LinearRgb, $linear);
		$macro!(Hex, $hex);
		$macro!(Hsl, $hsl);
		$macro!(Hsv, $hsv);
		$macro!(Hwb, $hwb);
		$macro!(Lab, $lab);
		$macro!(Lch, $lch);
		$macro!(Oklab, $oklab);
		$macro!(Oklch, $oklch);
		$macro!(A98Rgb, $a98_rgb);
		$macro!(XyzD50, $xyzd50);
		$macro!(XyzD65, $xyzd65);
	};
}

macro_rules! assert_all_conversions {
	($srgb:ident, $linear:ident, $hex:ident, $hsl:ident, $hsv:ident, $hwb:ident, $lab:ident, $lch:ident, $oklab:ident, $oklch:ident, $a98_rgb:ident, $xyzd50:ident, $xyzd65:ident) => {
		macro_rules! for_each_from {
			($from_ty:ident, $from_val:ident) => {
				macro_rules! for_each_to {
					($to_ty:ident, $to_val:ident) => {
						let converted = $to_ty::from($from_val);
						assert!(
							converted.close_to($to_val, COLOR_EPSILON),
							"Conversion {} -> {} mismatch:\nexpected {:?}\nactual   {:?}",
							stringify!($from_ty),
							stringify!($to_ty),
							$to_val,
							converted
						);
					};
				}
				each_color_pairs!(
					for_each_to,
					$srgb,
					$linear,
					$hex,
					$hsl,
					$hsv,
					$hwb,
					$lab,
					$lch,
					$oklab,
					$oklch,
					$a98_rgb,
					$xyzd50,
					$xyzd65
				);
			};
		}

		each_color_pairs!(
			for_each_from,
			$srgb,
			$linear,
			$hex,
			$hsl,
			$hsv,
			$hwb,
			$lab,
			$lch,
			$oklab,
			$oklch,
			$a98_rgb,
			$xyzd50,
			$xyzd65
		);
	};
}

#[allow(clippy::too_many_arguments)]
fn test_combos(
	srgb: Srgb,
	linear: LinearRgb,
	hex: Hex,
	hsl: Hsl,
	hsv: Hsv,
	hwb: Hwb,
	lab: Lab,
	lch: Lch,
	oklab: Oklab,
	oklch: Oklch,
	a98_rgb: A98Rgb,
	xyzd50: XyzD50,
	xyzd65: XyzD65,
) {
	macro_rules! assert_for_each_color {
		($to_ty:ident, $from_ty:ty, $val: ident) => {
			let from: $to_ty = $val.into();
			let back: $from_ty = from.into();
			assert!(
				back.close_to($val, COLOR_EPSILON),
				"Conversion {} -> {} mismatch:\nexpected {:?}\nactual   {:?}",
				stringify!($to_ty),
				stringify!($from_ty),
				$val,
				back
			);
		};
	}
	each_color!(assert_for_each_color, Srgb, srgb);
	each_color!(assert_for_each_color, LinearRgb, linear);
	each_color!(assert_for_each_color, Hex, hex);
	each_color!(assert_for_each_color, Hsl, hsl);
	each_color!(assert_for_each_color, Hsv, hsv);
	each_color!(assert_for_each_color, Hwb, hwb);
	each_color!(assert_for_each_color, Lab, lab);
	each_color!(assert_for_each_color, Lch, lch);
	each_color!(assert_for_each_color, Oklab, oklab);
	each_color!(assert_for_each_color, Oklch, oklch);
	each_color!(assert_for_each_color, A98Rgb, a98_rgb);
	each_color!(assert_for_each_color, XyzD50, xyzd50);
	each_color!(assert_for_each_color, XyzD65, xyzd65);
	assert_all_conversions!(srgb, linear, hex, hsl, hsv, hwb, lab, lch, oklab, oklch, a98_rgb, xyzd50, xyzd65);
}

#[test]
fn rebeccapurple() {
	test_combos(
		Srgb::new(102, 51, 153, 100.0),
		LinearRgb::new(0.13286832, 0.03310476, 0.31854683, 100.0),
		Hex::new(0x663399FF),
		Hsl::new(270.0, 50.0, 40.0, 100.0),
		Hsv::new(270.0, 66.666_664, 60.0000004, 100.0),
		Hwb::new(270.0, 19.9999996, 39.9999996, 100.0),
		Lab::new(32.39271642, 38.42945581, -47.68554267, 100.0),
		Lch::new(32.39271642, 61.24323680, 308.86510559, 100.0),
		Oklab::new(0.44027179, 0.08817676, -0.13386435, 100.0),
		Oklch::new(0.44027179, 0.16029599, 303.37298848, 100.0),
		A98Rgb::new(0.39940515, 0.21231660, 0.59441553, 100.0),
		XyzD50::new(11.62668443, 7.26049173, 23.25379520, 100.0),
		XyzD65::new(12.412, 7.493, 30.930, 100.0),
	);
}

#[test]
fn cornflower_blue() {
	test_combos(
		Srgb::new(100, 149, 237, 100.0),
		LinearRgb::new(0.12743768, 0.30054379, 0.84687323, 100.0),
		Hex::new(0x6495EDFF),
		Hsl::new(218.5, 79.2, 66.1, 100.0),
		Hsv::new(218.54015, 57.80591, 92.94118, 100.0),
		Hwb::new(218.54015, 39.215687, 7.058823, 100.0),
		Lab::new(61.23323694, 3.05558478, -50.18040851, 100.0),
		Lch::new(61.23323694, 50.27335275, 273.48455139, 100.0),
		Oklab::new(0.67462201, -0.02128901, -0.13974453, 100.0),
		Oklch::new(0.67462201, 0.14135683, 261.33802289, 100.0),
		A98Rgb::new(0.39189772, 0.57889666, 0.92721090, 100.0),
		XyzD50::new(29.24953872, 29.51472517, 63.57032517, 100.0),
		XyzD65::new(31.28682295, 30.31754694, 84.32669615, 100.0),
	);
}

#[test]
fn hex_123() {
	test_combos(
		Srgb::new(17, 34, 51, 100.0),
		LinearRgb::new(0.00560539, 0.01599629, 0.03310477, 100.0),
		Hex::new(0x112233FF),
		Hsl::new(210.0, 50.0, 13.3, 100.0),
		Hsv::new(210.0, 66.7, 20.0, 100.0),
		Hwb::new(210.0, 6.67, 80.0, 100.0),
		Lab::new(12.42990, -2.50513, -13.55537, 100.0),
		Lch::new(12.42990, 13.78491, 259.52946, 100.0),
		Oklab::new(0.24619, -0.01380, -0.03738, 100.0),
		Oklch::new(0.24619, 0.03985, 249.73162, 100.0),
		A98Rgb::new(0.094684, 0.152530, 0.212317, 100.0),
		XyzD50::new(1.334189, 1.472150, 2.527108, 100.0),
		XyzD65::new(1.400641, 1.502188, 3.348217, 100.0),
	);
}

#[test]
fn text_hex_display() {
	assert_eq!(format!("{}", Hex::new(0x663399FF)), "#639");
	assert_eq!(format!("{}", Hex::new(0x66339900)), "#6390");
	assert_eq!(format!("{}", Hex::new(0x112233FF)), "#123");
}

#[test]
fn text_hex_alpha_conversion() {
	assert_eq!(format!("{}", Hex::from(Srgb::new(255, 255, 255, 0.0))), "#fff0");
	assert_eq!(Hex::from(Srgb::new(255, 255, 255, 0.0)), Hex::new(0xFFFFFF00));
	assert_eq!(format!("{}", Hex::from(Srgb::new(255, 255, 255, 50.0))), "#ffffff80");
	assert_eq!(Hex::from(Srgb::new(255, 255, 255, 50.0)), Hex::new(0xFFFFFF80));
	let original = Srgb::new(255, 255, 255, 50.0);
	let hex = Hex::from(original);
	let round_tripped = Srgb::from(hex);
	assert!(
		round_tripped.close_to(original, COLOR_EPSILON),
		"Round-trip failed: original={:?}, round_tripped={:?}",
		original,
		round_tripped
	);
	assert_eq!(Hex::from(Srgb::new(0, 0, 0, 25.0)), Hex::new(0x00000040));
	assert_eq!(Hex::from(Srgb::new(0, 0, 0, 75.0)), Hex::new(0x000000BF));
	assert_eq!(Hex::from(Srgb::new(0, 0, 0, 100.0)), Hex::new(0x000000FF));
}

#[test]
fn named_try_from_other_spaces() {
	let named = Named::Rebeccapurple;
	let srgb = Srgb::new(102, 51, 153, 100.0);
	assert_eq!(Named::try_from(srgb).unwrap(), named);
	assert_eq!(Named::try_from(Hex::new(0x663399FF)).unwrap(), named);
	assert_eq!(Named::try_from(Hsl::new(270.0, 50.0, 40.0, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Hsv::new(270.0, 66.666_664, 60.0000004, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Hwb::new(270.0, 19.9999996, 39.9999996, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Lab::new(32.39271642, 38.42945581, -47.68554267, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Lch::new(32.39271642, 61.24323680, 308.86510559, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(LinearRgb::new(0.13286832, 0.03310476, 0.31854683, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Oklab::new(0.44027179, 0.08817676, -0.13386435, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Oklch::new(0.44027179, 0.16029599, 303.37298848, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(A98Rgb::new(0.39940515, 0.21231660, 0.59441553, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(XyzD50::new(11.62668443, 7.26049173, 23.25379520, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(XyzD65::new(12.412, 7.493, 30.930, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Color::Srgb(srgb)).unwrap(), named);

	let translucent = Hex::new(0x66339980);
	assert!(matches!(Named::try_from(translucent), Err(ToNamedError::NotOpaque)));
	assert!(matches!(Named::try_from(Color::Hex(translucent)), Err(ToNamedError::NotOpaque)));
}
