use super::*;

macro_rules! simple_from {
	($from: ident to $to: ident via $int: ident) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				let intermediary: $int = value.into();
				intermediary.into()
			}
		}
	};
}

simple_from!(Hsb to A98Rgb via Srgb);
simple_from!(Hex to A98Rgb via Srgb);
simple_from!(Hsl to A98Rgb via Srgb);
simple_from!(Hwb to A98Rgb via Hsb);
simple_from!(Lab to A98Rgb via XyzD50);
simple_from!(Lch to A98Rgb via Lab);
simple_from!(Named to A98Rgb via Srgb);
simple_from!(Oklab to A98Rgb via XyzD65);
simple_from!(Oklch to A98Rgb via Oklab);
simple_from!(Srgb to A98Rgb via LinearRgb);
simple_from!(XyzD50 to A98Rgb via XyzD65);
simple_from!(XyzD65 to A98Rgb via LinearRgb);

simple_from!(A98Rgb to Hsb via LinearRgb);
simple_from!(A98Rgb to Hex via Srgb);
simple_from!(A98Rgb to Hsl via LinearRgb);
simple_from!(A98Rgb to Hwb via LinearRgb);
simple_from!(A98Rgb to Lab via LinearRgb);
simple_from!(A98Rgb to Lch via LinearRgb);
simple_from!(A98Rgb to Oklab via LinearRgb);
simple_from!(A98Rgb to Oklch via LinearRgb);
simple_from!(A98Rgb to Srgb via LinearRgb);
simple_from!(A98Rgb to XyzD50 via LinearRgb);
simple_from!(A98Rgb to XyzD65 via LinearRgb);

simple_from!(Hsb to Hex via Srgb);
simple_from!(Hsl to Hex via Srgb);
simple_from!(Hwb to Hex via Srgb);
simple_from!(Lab to Hex via Srgb);
simple_from!(Lch to Hex via Srgb);
simple_from!(LinearRgb to Hex via Srgb);
simple_from!(Named to Hex via Srgb);
simple_from!(Oklab to Hex via XyzD65);
simple_from!(Oklch to Hex via Oklab);
simple_from!(XyzD50 to Hex via XyzD65);
simple_from!(XyzD65 to Hex via Srgb);

simple_from!(Hex to Hsb via Srgb);
simple_from!(Hex to Hsl via Srgb);
simple_from!(Hex to Hwb via Srgb);
simple_from!(Hex to Lab via Srgb);
simple_from!(Hex to Lch via Srgb);
simple_from!(Hex to LinearRgb via Srgb);
simple_from!(Hex to Oklab via XyzD65);
simple_from!(Hex to Oklch via Oklab);
simple_from!(Hex to XyzD50 via XyzD65);
simple_from!(Hex to XyzD65 via Srgb);

simple_from!(Hsl to Hsb via Srgb);
simple_from!(Lab to Hsb via Srgb);
simple_from!(Lch to Hsb via Srgb);
simple_from!(Named to Hsb via Srgb);
simple_from!(Oklab to Hsb via XyzD65);
simple_from!(Oklch to Hsb via Oklab);
simple_from!(XyzD50 to Hsb via XyzD65);
simple_from!(XyzD65 to Hsb via Srgb);

simple_from!(Hsb to Hsl via Srgb);
simple_from!(Hsb to Lab via Srgb);
simple_from!(Hsb to Lch via Srgb);
simple_from!(Hsb to Oklab via Srgb);
simple_from!(Hsb to Oklch via Srgb);
simple_from!(Hsb to XyzD50 via Srgb);
simple_from!(Hsb to XyzD65 via Srgb);

simple_from!(Hwb to Hsl via Hsb);
simple_from!(Lab to Hsl via Srgb);
simple_from!(Lch to Hsl via Srgb);
simple_from!(Named to Hsl via Srgb);
simple_from!(Oklab to Hsl via Srgb);
simple_from!(Oklch to Hsl via Srgb);
simple_from!(XyzD50 to Hsl via Srgb);
simple_from!(XyzD65 to Hsl via Srgb);

simple_from!(Hsl to Hwb via Srgb);
simple_from!(Hsl to Lab via Srgb);
simple_from!(Hsl to Lch via Srgb);
simple_from!(Hsl to Oklab via Srgb);
simple_from!(Hsl to Oklch via Srgb);
simple_from!(Hsl to XyzD50 via Srgb);
simple_from!(Hsl to XyzD65 via Srgb);

simple_from!(Lab to Hwb via Srgb);
simple_from!(Lch to Hwb via Srgb);
simple_from!(Named to Hwb via Srgb);
simple_from!(Oklab to Hwb via Srgb);
simple_from!(Oklch to Hwb via Srgb);
simple_from!(XyzD50 to Hwb via Srgb);
simple_from!(XyzD65 to Hwb via Srgb);

simple_from!(Hwb to Lab via Srgb);
simple_from!(Hwb to Lch via Srgb);
simple_from!(Hwb to Oklab via Srgb);
simple_from!(Hwb to Oklch via Srgb);
simple_from!(Hwb to XyzD50 via Srgb);
simple_from!(Hwb to XyzD65 via Srgb);

simple_from!(Named to Lab via Srgb);
simple_from!(Oklab to Lab via Srgb);
simple_from!(Oklch to Lab via Srgb);
simple_from!(XyzD65 to Lab via Srgb);

simple_from!(Lab to Oklab via XyzD50);
simple_from!(Lab to Oklch via XyzD50);
simple_from!(Lab to XyzD65 via XyzD50);

simple_from!(Named to Lch via Srgb);
simple_from!(Oklab to Lch via Srgb);
simple_from!(Oklch to Lch via Srgb);
simple_from!(XyzD50 to Lch via Srgb);
simple_from!(XyzD65 to Lch via Srgb);

simple_from!(Lch to Oklab via Srgb);
simple_from!(Lch to Oklch via Srgb);
simple_from!(Lch to XyzD50 via Srgb);
simple_from!(Lch to XyzD65 via Srgb);

simple_from!(Hsb to LinearRgb via Srgb);
simple_from!(Hsl to LinearRgb via Srgb);
simple_from!(Hwb to LinearRgb via Srgb);
simple_from!(Lab to LinearRgb via XyzD50);
simple_from!(Lch to LinearRgb via Lab);
simple_from!(Named to LinearRgb via Srgb);
simple_from!(Oklab to LinearRgb via XyzD65);
simple_from!(Oklch to LinearRgb via Oklab);
simple_from!(XyzD50 to LinearRgb via XyzD65);

simple_from!(LinearRgb to Hsb via Srgb);
simple_from!(LinearRgb to Hsl via Srgb);
simple_from!(LinearRgb to Hwb via Srgb);
simple_from!(LinearRgb to Lab via XyzD50);
simple_from!(LinearRgb to Lch via Lab);
simple_from!(LinearRgb to Oklab via XyzD65);
simple_from!(LinearRgb to Oklch via Oklab);
simple_from!(LinearRgb to XyzD50 via XyzD65);

simple_from!(Named to Oklab via Srgb);
simple_from!(XyzD50 to Oklab via XyzD65);

simple_from!(Oklab to XyzD50 via XyzD65);

simple_from!(Named to Oklch via Srgb);
simple_from!(XyzD50 to Oklch via Oklab);
simple_from!(XyzD65 to Oklch via Oklab);

simple_from!(Oklch to XyzD50 via Oklab);
simple_from!(Oklch to XyzD65 via Oklab);

simple_from!(Named to XyzD50 via Srgb);
simple_from!(Named to XyzD65 via Srgb);

simple_from!(Srgb to Lab via XyzD50);
simple_from!(Srgb to Lch via Lab);
simple_from!(Srgb to Oklab via XyzD65);
simple_from!(Srgb to Oklch via Oklab);
simple_from!(Srgb to XyzD50 via LinearRgb);
simple_from!(Srgb to XyzD65 via LinearRgb);

simple_from!(XyzD50 to Srgb via XyzD65);
simple_from!(XyzD65 to Srgb via LinearRgb);
simple_from!(Lab to Srgb via XyzD50);
simple_from!(Lch to Srgb via Lab);
simple_from!(Oklab to Srgb via XyzD65);
simple_from!(Oklch to Srgb via Oklab);
simple_from!(Hwb to Srgb via Hsb);
simple_from!(Srgb to Hwb via Hsb);
