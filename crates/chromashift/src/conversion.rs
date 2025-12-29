use super::*;

macro_rules! simple_from {
	($from: ident to $to: ty, via $int: ty) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				let intermediary: $int = value.into();
				intermediary.into()
			}
		}
	};
}

simple_from!(Hsv to A98Rgb, via Srgb);
simple_from!(Hex to A98Rgb, via Srgb);
simple_from!(Hsl to A98Rgb, via Srgb);
simple_from!(Hwb to A98Rgb, via Hsv);
simple_from!(Lab to A98Rgb, via XyzD50);
simple_from!(Lch to A98Rgb, via Lab);
simple_from!(Named to A98Rgb, via Srgb);
simple_from!(Oklab to A98Rgb, via XyzD65);
simple_from!(Oklch to A98Rgb, via Oklab);
simple_from!(Srgb to A98Rgb, via LinearRgb);
simple_from!(XyzD50 to A98Rgb, via XyzD65);
simple_from!(XyzD65 to A98Rgb, via LinearRgb);

simple_from!(A98Rgb to Hsv, via LinearRgb);
simple_from!(A98Rgb to Hex, via Srgb);
simple_from!(A98Rgb to Hsl, via LinearRgb);
simple_from!(A98Rgb to Hwb, via LinearRgb);
simple_from!(A98Rgb to Lab, via LinearRgb);
simple_from!(A98Rgb to Lch, via LinearRgb);
simple_from!(A98Rgb to Oklab, via LinearRgb);
simple_from!(A98Rgb to Oklch, via LinearRgb);
simple_from!(A98Rgb to Srgb, via LinearRgb);
simple_from!(A98Rgb to XyzD50, via LinearRgb);
simple_from!(A98Rgb to XyzD65, via LinearRgb);

simple_from!(Hsv to Hex, via Srgb);
simple_from!(Hsl to Hex, via Srgb);
simple_from!(Hwb to Hex, via Srgb);
simple_from!(Lab to Hex, via Srgb);
simple_from!(Lch to Hex, via Srgb);
simple_from!(LinearRgb to Hex, via Srgb);
simple_from!(Named to Hex, via Srgb);
simple_from!(Oklab to Hex, via XyzD65);
simple_from!(Oklch to Hex, via Oklab);
simple_from!(XyzD50 to Hex, via XyzD65);
simple_from!(XyzD65 to Hex, via Srgb);

simple_from!(Hex to Hsv, via Srgb);
simple_from!(Hex to Hsl, via Srgb);
simple_from!(Hex to Hwb, via Srgb);
simple_from!(Hex to Lab, via Srgb);
simple_from!(Hex to Lch, via Srgb);
simple_from!(Hex to LinearRgb, via Srgb);
simple_from!(Hex to Oklab, via XyzD65);
simple_from!(Hex to Oklch, via Oklab);
simple_from!(Hex to XyzD50, via XyzD65);
simple_from!(Hex to XyzD65, via Srgb);

simple_from!(Hsl to Hsv, via Srgb);
simple_from!(Lab to Hsv, via Srgb);
simple_from!(Lch to Hsv, via Srgb);
simple_from!(Named to Hsv, via Srgb);
simple_from!(Oklab to Hsv, via XyzD65);
simple_from!(Oklch to Hsv, via Oklab);
simple_from!(XyzD50 to Hsv, via XyzD65);
simple_from!(XyzD65 to Hsv, via Srgb);

simple_from!(Hsv to Hsl, via Srgb);
simple_from!(Hsv to Lab, via Srgb);
simple_from!(Hsv to Lch, via Srgb);
simple_from!(Hsv to Oklab, via Srgb);
simple_from!(Hsv to Oklch, via Srgb);
simple_from!(Hsv to XyzD50, via Srgb);
simple_from!(Hsv to XyzD65, via Srgb);

simple_from!(Hwb to Hsl, via Hsv);
simple_from!(Lab to Hsl, via Srgb);
simple_from!(Lch to Hsl, via Srgb);
simple_from!(Named to Hsl, via Srgb);
simple_from!(Oklab to Hsl, via Srgb);
simple_from!(Oklch to Hsl, via Srgb);
simple_from!(XyzD50 to Hsl, via Srgb);
simple_from!(XyzD65 to Hsl, via Srgb);

simple_from!(Hsl to Hwb, via Srgb);
simple_from!(Hsl to Lab, via Srgb);
simple_from!(Hsl to Lch, via Srgb);
simple_from!(Hsl to Oklab, via Srgb);
simple_from!(Hsl to Oklch, via Srgb);
simple_from!(Hsl to XyzD50, via Srgb);
simple_from!(Hsl to XyzD65, via Srgb);

simple_from!(Lab to Hwb, via Srgb);
simple_from!(Lch to Hwb, via Srgb);
simple_from!(Named to Hwb, via Srgb);
simple_from!(Oklab to Hwb, via Srgb);
simple_from!(Oklch to Hwb, via Srgb);
simple_from!(XyzD50 to Hwb, via Srgb);
simple_from!(XyzD65 to Hwb, via Srgb);

simple_from!(Hwb to Lab, via Srgb);
simple_from!(Hwb to Lch, via Srgb);
simple_from!(Hwb to Oklab, via Srgb);
simple_from!(Hwb to Oklch, via Srgb);
simple_from!(Hwb to XyzD50, via Srgb);
simple_from!(Hwb to XyzD65, via Srgb);

simple_from!(Named to Lab, via Srgb);
simple_from!(Oklab to Lab, via Srgb);
simple_from!(Oklch to Lab, via Srgb);
simple_from!(XyzD65 to Lab, via Srgb);

simple_from!(Lab to Oklab, via XyzD50);
simple_from!(Lab to Oklch, via XyzD50);
simple_from!(Lab to XyzD65, via XyzD50);

simple_from!(Named to Lch, via Srgb);
simple_from!(Oklab to Lch, via Srgb);
simple_from!(Oklch to Lch, via Srgb);
simple_from!(XyzD50 to Lch, via Srgb);
simple_from!(XyzD65 to Lch, via Srgb);

simple_from!(Lch to Oklab, via Srgb);
simple_from!(Lch to Oklch, via Srgb);
simple_from!(Lch to XyzD50, via Srgb);
simple_from!(Lch to XyzD65, via Srgb);

simple_from!(Hsv to LinearRgb, via Srgb);
simple_from!(Hsl to LinearRgb, via Srgb);
simple_from!(Hwb to LinearRgb, via Srgb);
simple_from!(Lab to LinearRgb, via XyzD50);
simple_from!(Lch to LinearRgb, via Lab);
simple_from!(Named to LinearRgb, via Srgb);
simple_from!(Oklab to LinearRgb, via XyzD65);
simple_from!(Oklch to LinearRgb, via Oklab);
simple_from!(XyzD50 to LinearRgb, via XyzD65);

simple_from!(LinearRgb to Hsv, via Srgb);
simple_from!(LinearRgb to Hsl, via Srgb);
simple_from!(LinearRgb to Hwb, via Srgb);
simple_from!(LinearRgb to Lab, via XyzD50);
simple_from!(LinearRgb to Lch, via Lab);
simple_from!(LinearRgb to Oklab, via XyzD65);
simple_from!(LinearRgb to Oklch, via Oklab);
simple_from!(LinearRgb to XyzD50, via XyzD65);

simple_from!(Named to Oklab, via Srgb);
simple_from!(XyzD50 to Oklab, via XyzD65);

simple_from!(Oklab to XyzD50, via XyzD65);

simple_from!(Named to Oklch, via Srgb);
simple_from!(XyzD50 to Oklch, via Oklab);
simple_from!(XyzD65 to Oklch, via Oklab);

simple_from!(Oklch to XyzD50, via Oklab);
simple_from!(Oklch to XyzD65, via Oklab);

simple_from!(Named to XyzD50, via Srgb);
simple_from!(Named to XyzD65, via Srgb);

simple_from!(Srgb to Lab, via XyzD50);
simple_from!(Srgb to Lch, via Lab);
simple_from!(Srgb to Oklab, via XyzD65);
simple_from!(Srgb to Oklch, via Oklab);
simple_from!(Srgb to XyzD50, via LinearRgb);
simple_from!(Srgb to XyzD65, via LinearRgb);

simple_from!(XyzD50 to Srgb, via XyzD65);
simple_from!(XyzD65 to Srgb, via LinearRgb);
simple_from!(Lab to Srgb, via XyzD50);
simple_from!(Lch to Srgb, via Lab);
simple_from!(Oklab to Srgb, via XyzD65);
simple_from!(Oklch to Srgb, via Oklab);
simple_from!(Hwb to Srgb, via Hsv);
simple_from!(Srgb to Hwb, via Hsv);

simple_from!(Color to A98Rgb, via XyzD65);
simple_from!(Color to Hsv, via XyzD65);
simple_from!(Color to Hex, via XyzD65);
simple_from!(Color to Hsl, via XyzD65);
simple_from!(Color to Hwb, via XyzD65);
simple_from!(Color to Lab, via XyzD65);
simple_from!(Color to Lch, via XyzD65);
simple_from!(Color to LinearRgb, via XyzD65);
simple_from!(Color to Oklab, via XyzD65);
simple_from!(Color to Oklch, via XyzD65);
simple_from!(Color to Srgb, via XyzD65);
simple_from!(Color to XyzD50, via XyzD65);

macro_rules! impl_named_try_from_via_srgb {
	($($ty:path),+ $(,)?) => {
		$(
			impl TryFrom<$ty> for Named {
				type Error = ToNamedError;

				fn try_from(value: $ty) -> Result<Self, Self::Error> {
					Self::try_from(Srgb::from(value))
				}
			}
		)+
	};
}

impl_named_try_from_via_srgb!(
	crate::A98Rgb,
	crate::Hex,
	crate::Hsv,
	crate::Hsl,
	crate::Hwb,
	crate::Lab,
	crate::Lch,
	crate::LinearRgb,
	crate::Oklab,
	crate::Oklch,
	crate::XyzD50,
	crate::XyzD65,
	crate::Color,
);

#[cfg(feature = "anstyle")]
simple_from!(Color to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(A98Rgb to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Hsv to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Hex to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Hsl to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Hwb to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Lab to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Lch to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(LinearRgb to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Named to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Oklab to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Oklch to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(XyzD50 to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(XyzD65 to anstyle::RgbColor, via Srgb);

#[cfg(feature = "anstyle")]
simple_from!(Color to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(A98Rgb to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Hsv to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Hex to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Hsl to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Hwb to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Lab to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Lch to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(LinearRgb to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Named to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Oklab to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Oklch to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Srgb to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(XyzD50 to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(XyzD65 to anstyle::Color, via anstyle::RgbColor);

#[cfg(feature = "owo-colors")]
simple_from!(Color to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(A98Rgb to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Hsv to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Hex to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Hsl to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Hwb to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Lab to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Lch to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(LinearRgb to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Named to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Oklab to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Oklch to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(XyzD50 to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(XyzD65 to owo_colors::Rgb, via Srgb);
