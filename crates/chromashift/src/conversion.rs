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

simple_from!(A98Rgb to DisplayP3, via XyzD65);
simple_from!(A98Rgb to Hex, via XyzD65);
simple_from!(A98Rgb to Hsl, via XyzD65);
simple_from!(A98Rgb to Hsv, via XyzD65);
simple_from!(A98Rgb to Hwb, via XyzD65);
simple_from!(A98Rgb to Lab, via XyzD65);
simple_from!(A98Rgb to Lch, via XyzD65);
simple_from!(A98Rgb to LinearRgb, via XyzD65);
simple_from!(A98Rgb to Oklab, via XyzD65);
simple_from!(A98Rgb to Oklch, via XyzD65);
simple_from!(A98Rgb to ProphotoRgb, via XyzD65);
simple_from!(A98Rgb to Rec2020, via XyzD65);
simple_from!(A98Rgb to Srgb, via XyzD65);
simple_from!(A98Rgb to XyzD50, via XyzD65);

simple_from!(DisplayP3 to A98Rgb, via XyzD65);
simple_from!(DisplayP3 to Hex, via LinearRgb);
simple_from!(DisplayP3 to Hsl, via LinearRgb);
simple_from!(DisplayP3 to Hsv, via LinearRgb);
simple_from!(DisplayP3 to Hwb, via LinearRgb);
simple_from!(DisplayP3 to Lab, via XyzD65);
simple_from!(DisplayP3 to Lch, via XyzD65);
simple_from!(DisplayP3 to Oklab, via XyzD65);
simple_from!(DisplayP3 to Oklch, via XyzD65);
simple_from!(DisplayP3 to ProphotoRgb, via XyzD65);
simple_from!(DisplayP3 to Rec2020, via XyzD65);
simple_from!(DisplayP3 to Srgb, via LinearRgb);
simple_from!(DisplayP3 to XyzD50, via XyzD65);

simple_from!(Hex to A98Rgb, via Srgb);
simple_from!(Hex to DisplayP3, via Srgb);
simple_from!(Hex to Hsl, via Srgb);
simple_from!(Hex to Hsv, via Srgb);
simple_from!(Hex to Hwb, via Srgb);
simple_from!(Hex to Lab, via Srgb);
simple_from!(Hex to Lch, via Srgb);
simple_from!(Hex to LinearRgb, via Srgb);
simple_from!(Hex to Oklab, via Srgb);
simple_from!(Hex to Oklch, via Srgb);
simple_from!(Hex to ProphotoRgb, via Srgb);
simple_from!(Hex to Rec2020, via Srgb);
simple_from!(Hex to XyzD50, via Srgb);
simple_from!(Hex to XyzD65, via Srgb);

simple_from!(Hsl to A98Rgb, via Srgb);
simple_from!(Hsl to DisplayP3, via Srgb);
simple_from!(Hsl to Hex, via Srgb);
simple_from!(Hsl to Hsv, via Srgb);
simple_from!(Hsl to Hwb, via Srgb);
simple_from!(Hsl to Lab, via Srgb);
simple_from!(Hsl to Lch, via Srgb);
simple_from!(Hsl to LinearRgb, via Srgb);
simple_from!(Hsl to Oklab, via Srgb);
simple_from!(Hsl to Oklch, via Srgb);
simple_from!(Hsl to ProphotoRgb, via Srgb);
simple_from!(Hsl to Rec2020, via Srgb);
simple_from!(Hsl to XyzD50, via Srgb);
simple_from!(Hsl to XyzD65, via Srgb);

simple_from!(Hsv to A98Rgb, via Hwb);
simple_from!(Hsv to DisplayP3, via Hwb);
simple_from!(Hsv to Hex, via Srgb);
simple_from!(Hsv to Hsl, via Hwb);
simple_from!(Hsv to Lab, via Hwb);
simple_from!(Hsv to Lch, via Hwb);
simple_from!(Hsv to LinearRgb, via Hwb);
simple_from!(Hsv to Oklab, via Hwb);
simple_from!(Hsv to Oklch, via Hwb);
simple_from!(Hsv to ProphotoRgb, via Hwb);
simple_from!(Hsv to Rec2020, via Hwb);
simple_from!(Hsv to Srgb, via Hwb);
simple_from!(Hsv to XyzD50, via Hwb);
simple_from!(Hsv to XyzD65, via Hwb);

simple_from!(Hwb to A98Rgb, via Srgb);
simple_from!(Hwb to DisplayP3, via Srgb);
simple_from!(Hwb to Hex, via Srgb);
simple_from!(Hwb to Hsl, via Srgb);
simple_from!(Hwb to Lab, via Srgb);
simple_from!(Hwb to Lch, via Srgb);
simple_from!(Hwb to LinearRgb, via Srgb);
simple_from!(Hwb to Oklab, via Srgb);
simple_from!(Hwb to Oklch, via Srgb);
simple_from!(Hwb to ProphotoRgb, via Srgb);
simple_from!(Hwb to Rec2020, via Srgb);
simple_from!(Hwb to XyzD50, via Srgb);
simple_from!(Hwb to XyzD65, via Srgb);

simple_from!(Lab to A98Rgb, via XyzD50);
simple_from!(Lab to DisplayP3, via XyzD50);
simple_from!(Lab to Hex, via XyzD50);
simple_from!(Lab to Hsl, via XyzD50);
simple_from!(Lab to Hsv, via XyzD50);
simple_from!(Lab to Hwb, via XyzD50);
simple_from!(Lab to LinearRgb, via XyzD50);
simple_from!(Lab to Oklab, via XyzD50);
simple_from!(Lab to Oklch, via XyzD50);
simple_from!(Lab to ProphotoRgb, via XyzD50);
simple_from!(Lab to Rec2020, via XyzD50);
simple_from!(Lab to Srgb, via XyzD50);
simple_from!(Lab to XyzD65, via XyzD50);

simple_from!(Lch to A98Rgb, via Lab);
simple_from!(Lch to DisplayP3, via Lab);
simple_from!(Lch to Hex, via Lab);
simple_from!(Lch to Hsl, via Lab);
simple_from!(Lch to Hsv, via Lab);
simple_from!(Lch to Hwb, via Lab);
simple_from!(Lch to LinearRgb, via Lab);
simple_from!(Lch to Oklab, via Lab);
simple_from!(Lch to Oklch, via Lab);
simple_from!(Lch to ProphotoRgb, via Lab);
simple_from!(Lch to Rec2020, via Lab);
simple_from!(Lch to Srgb, via Lab);
simple_from!(Lch to XyzD50, via Lab);
simple_from!(Lch to XyzD65, via Lab);

simple_from!(LinearRgb to A98Rgb, via XyzD65);
simple_from!(LinearRgb to Hex, via Srgb);
simple_from!(LinearRgb to Hsl, via Srgb);
simple_from!(LinearRgb to Hsv, via Srgb);
simple_from!(LinearRgb to Hwb, via Srgb);
simple_from!(LinearRgb to Lab, via XyzD65);
simple_from!(LinearRgb to Lch, via XyzD65);
simple_from!(LinearRgb to Oklab, via XyzD65);
simple_from!(LinearRgb to Oklch, via XyzD65);
simple_from!(LinearRgb to ProphotoRgb, via XyzD65);
simple_from!(LinearRgb to Rec2020, via XyzD65);
simple_from!(LinearRgb to XyzD50, via XyzD65);

simple_from!(Named to A98Rgb, via Srgb);
simple_from!(Named to DisplayP3, via Srgb);
simple_from!(Named to Hex, via Srgb);
simple_from!(Named to Hsl, via Srgb);
simple_from!(Named to Hsv, via Srgb);
simple_from!(Named to Hwb, via Srgb);
simple_from!(Named to Lab, via Srgb);
simple_from!(Named to Lch, via Srgb);
simple_from!(Named to LinearRgb, via Srgb);
simple_from!(Named to Oklab, via Srgb);
simple_from!(Named to Oklch, via Srgb);
simple_from!(Named to ProphotoRgb, via Srgb);
simple_from!(Named to Rec2020, via Srgb);
simple_from!(Named to XyzD50, via Srgb);
simple_from!(Named to XyzD65, via Srgb);

simple_from!(Oklab to A98Rgb, via XyzD65);
simple_from!(Oklab to DisplayP3, via XyzD65);
simple_from!(Oklab to Hex, via XyzD65);
simple_from!(Oklab to Hsl, via XyzD65);
simple_from!(Oklab to Hsv, via XyzD65);
simple_from!(Oklab to Hwb, via XyzD65);
simple_from!(Oklab to Lab, via XyzD65);
simple_from!(Oklab to Lch, via XyzD65);
simple_from!(Oklab to LinearRgb, via XyzD65);
simple_from!(Oklab to ProphotoRgb, via XyzD65);
simple_from!(Oklab to Rec2020, via XyzD65);
simple_from!(Oklab to Srgb, via XyzD65);
simple_from!(Oklab to XyzD50, via XyzD65);

simple_from!(Oklch to A98Rgb, via Oklab);
simple_from!(Oklch to DisplayP3, via Oklab);
simple_from!(Oklch to Hex, via Oklab);
simple_from!(Oklch to Hsl, via Oklab);
simple_from!(Oklch to Hsv, via Oklab);
simple_from!(Oklch to Hwb, via Oklab);
simple_from!(Oklch to Lab, via Oklab);
simple_from!(Oklch to Lch, via Oklab);
simple_from!(Oklch to LinearRgb, via Oklab);
simple_from!(Oklch to ProphotoRgb, via Oklab);
simple_from!(Oklch to Rec2020, via Oklab);
simple_from!(Oklch to Srgb, via Oklab);
simple_from!(Oklch to XyzD50, via Oklab);
simple_from!(Oklch to XyzD65, via Oklab);

simple_from!(ProphotoRgb to A98Rgb, via XyzD50);
simple_from!(ProphotoRgb to DisplayP3, via XyzD50);
simple_from!(ProphotoRgb to Hex, via XyzD50);
simple_from!(ProphotoRgb to Hsl, via XyzD50);
simple_from!(ProphotoRgb to Hsv, via XyzD50);
simple_from!(ProphotoRgb to Hwb, via XyzD50);
simple_from!(ProphotoRgb to Lab, via XyzD50);
simple_from!(ProphotoRgb to Lch, via XyzD50);
simple_from!(ProphotoRgb to LinearRgb, via XyzD50);
simple_from!(ProphotoRgb to Oklab, via XyzD50);
simple_from!(ProphotoRgb to Oklch, via XyzD50);
simple_from!(ProphotoRgb to Rec2020, via XyzD50);
simple_from!(ProphotoRgb to Srgb, via XyzD50);
simple_from!(ProphotoRgb to XyzD65, via XyzD50);

simple_from!(Rec2020 to A98Rgb, via XyzD65);
simple_from!(Rec2020 to DisplayP3, via XyzD65);
simple_from!(Rec2020 to Hex, via XyzD65);
simple_from!(Rec2020 to Hsl, via XyzD65);
simple_from!(Rec2020 to Hsv, via XyzD65);
simple_from!(Rec2020 to Hwb, via XyzD65);
simple_from!(Rec2020 to Lab, via XyzD65);
simple_from!(Rec2020 to Lch, via XyzD65);
simple_from!(Rec2020 to LinearRgb, via XyzD65);
simple_from!(Rec2020 to Oklab, via XyzD65);
simple_from!(Rec2020 to Oklch, via XyzD65);
simple_from!(Rec2020 to ProphotoRgb, via XyzD65);
simple_from!(Rec2020 to Srgb, via XyzD65);
simple_from!(Rec2020 to XyzD50, via XyzD65);

simple_from!(Srgb to A98Rgb, via LinearRgb);
simple_from!(Srgb to DisplayP3, via LinearRgb);
simple_from!(Srgb to Hsv, via Hwb);
simple_from!(Srgb to Lab, via LinearRgb);
simple_from!(Srgb to Lch, via LinearRgb);
simple_from!(Srgb to Oklab, via LinearRgb);
simple_from!(Srgb to Oklch, via LinearRgb);
simple_from!(Srgb to ProphotoRgb, via LinearRgb);
simple_from!(Srgb to Rec2020, via LinearRgb);
simple_from!(Srgb to XyzD50, via LinearRgb);

simple_from!(XyzD50 to A98Rgb, via XyzD65);
simple_from!(XyzD50 to DisplayP3, via XyzD65);
simple_from!(XyzD50 to Hex, via XyzD65);
simple_from!(XyzD50 to Hsl, via XyzD65);
simple_from!(XyzD50 to Hsv, via XyzD65);
simple_from!(XyzD50 to Hwb, via XyzD65);
simple_from!(XyzD50 to Lch, via Lab);
simple_from!(XyzD50 to LinearRgb, via XyzD65);
simple_from!(XyzD50 to Oklab, via XyzD65);
simple_from!(XyzD50 to Oklch, via XyzD65);
simple_from!(XyzD50 to Rec2020, via XyzD65);
simple_from!(XyzD50 to Srgb, via XyzD65);

simple_from!(XyzD65 to Hex, via LinearRgb);
simple_from!(XyzD65 to Hsl, via Srgb);
simple_from!(XyzD65 to Hsv, via Srgb);
simple_from!(XyzD65 to Hwb, via Srgb);
simple_from!(XyzD65 to Lab, via XyzD50);
simple_from!(XyzD65 to Lch, via XyzD50);
simple_from!(XyzD65 to Oklch, via Oklab);
simple_from!(XyzD65 to ProphotoRgb, via XyzD50);

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
	crate::DisplayP3,
	crate::Hex,
	crate::Hsv,
	crate::Hsl,
	crate::Hwb,
	crate::Lab,
	crate::Lch,
	crate::LinearRgb,
	crate::Oklab,
	crate::Oklch,
	crate::ProphotoRgb,
	crate::Rec2020,
	crate::XyzD50,
	crate::XyzD65,
	crate::Color,
);

#[cfg(feature = "anstyle")]
simple_from!(Color to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(A98Rgb to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(DisplayP3 to anstyle::RgbColor, via Srgb);
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
simple_from!(ProphotoRgb to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(Rec2020 to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(XyzD50 to anstyle::RgbColor, via Srgb);
#[cfg(feature = "anstyle")]
simple_from!(XyzD65 to anstyle::RgbColor, via Srgb);

#[cfg(feature = "anstyle")]
simple_from!(Color to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(A98Rgb to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(DisplayP3 to anstyle::Color, via anstyle::RgbColor);
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
simple_from!(ProphotoRgb to anstyle::Color, via anstyle::RgbColor);
#[cfg(feature = "anstyle")]
simple_from!(Rec2020 to anstyle::Color, via anstyle::RgbColor);
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
simple_from!(DisplayP3 to owo_colors::Rgb, via Srgb);
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
simple_from!(ProphotoRgb to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(Rec2020 to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(XyzD50 to owo_colors::Rgb, via Srgb);
#[cfg(feature = "owo-colors")]
simple_from!(XyzD65 to owo_colors::Rgb, via Srgb);
