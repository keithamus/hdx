use crate::{ColorMix, Srgb};
use core::fmt;

pub trait WcagRelativeLuminance: Sized + Copy
where
	Srgb: From<Self>,
{
	/// Calculate the relative luminance of a color
	/// [according to WCAG 2.1](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)
	fn relative_luminance(&self) -> f64 {
		let Srgb { red, green, blue, .. } = (*self).into();
		let [r, g, b] = Srgb::SPACE.transfer.decode_all([red, green, blue]);
		WCAG_WEIGHTS[0] * r + WCAG_WEIGHTS[1] * g + WCAG_WEIGHTS[2] * b
	}
}

const WCAG_WEIGHTS: [f64; 3] = [0.2126, 0.7152, 0.0722];

impl<C: Copy> WcagRelativeLuminance for C where Srgb: From<C> {}

/// Trait for calculating WCAG specified color contrast between colors.
///
/// This trait provides methods to determine how perceptually contrasting two colours are using the WCAG relative
/// luminance calculation in Srgb color space.
pub trait WcagColorContrast<T: WcagRelativeLuminance>: WcagRelativeLuminance + Sized
where
	Srgb: From<T> + From<Self>,
{
	/// Calculate the contrast ratio between `self` and `other`. colors according to WCAG 2.1. Returns a value between
	/// 1:1 and 21:1
	///
	/// `other` must implement [Into]&lt;[Srgb]>
	///
	fn wcag_contrast_ratio(&self, other: T) -> f64 {
		let lum1 = self.relative_luminance();
		let lum2 = other.relative_luminance();

		// Ensure the lighter color is in the numerator
		let (lighter, darker) = if lum1 > lum2 { (lum1, lum2) } else { (lum2, lum1) };

		(lighter + 0.05) / (darker + 0.05)
	}

	/// Get the WCAG level for the contrast between `self` and `other`.
	///
	/// `other` must implement [Into]&lt;[Srgb]>
	///
	fn wcag_level(&self, other: T) -> WcagLevel {
		WcagLevel::from_ratio(self.wcag_contrast_ratio(other))
	}

	/// Find the closest color that meets a specific WCAG level against a background.
	///
	/// This function uses [ColorMix] in the specified `Space` to find the minimum adjustment needed to meet the contrast
	/// requirement - mixing the colour at lower percentages to find the lowest percentage mix that would meet the
	/// [WcagLevel] requirement.
	///
	/// Returns `None` if it is impossible to find a
	fn find_minimum_contrast<Space>(&self, other: T, level: WcagLevel) -> Option<Space>
	where
		Self: WcagColorContrast<Space>,
		Space: ColorMix<Self, T> + From<Self> + From<T> + Copy + PartialEq + std::fmt::Debug,
		Srgb: From<Space>,
		crate::Hex: From<Space>,
	{
		let current_ratio = self.wcag_contrast_ratio(other);
		if current_ratio <= level.min_ratio() {
			return None;
		}
		let mut low = 0.0;
		let mut high = 100.0;
		let mut best_color: Option<Space> = None;
		let tolerance = 0.001; // 1% tolerance for floating point precision
		// A binary search from 0.0-100.0 of non-descrete numbers would be infinite, but
		// given we're dealing with f32s we have a practical max precision which is EPSILON.
		// This means we could have an upper bound of log2(H - L / EPSILON), or, in Rust parlance:
		// const MAX_ITERATIONS: usize = (100.0 / f32::EPSILON).log2().round() as usize;
		//
		// But this is not const safe (log2 & round aren't const), and the result of this is 30.
		// So this long comment is here to tell you why this number is 30:
		const MAX_ITERATIONS: usize = 30;
		for _ in 0..MAX_ITERATIONS {
			if high - low < tolerance {
				break;
			}
			let mid = (low + high) / 2.0;
			let candidate = Space::mix(*self, other, mid);
			let candidate_ratio = (self.wcag_contrast_ratio(candidate) * 100.0).round() / 100.0;

			if candidate_ratio > level.min_ratio() {
				best_color = Some(candidate);
				high = mid;
			} else {
				low = mid;
			}
		}
		best_color
	}
}

impl<C: Copy, T: Copy> WcagColorContrast<T> for C where Srgb: From<C> + From<T> {}

/// WCAG 2.1 color contrast levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WcagLevel {
	/// Fails all WCAG requirements (contrast < 3:1)
	Fail,
	/// Meets WCAG AA Large requirements (contrast >= 3:1)
	AALarge,
	/// Meets WCAG AA requirements (contrast >= 4.5:1)
	AA,
	/// Meets WCAG AAA requirements (contrast >= 7:1)
	AAA,
}

impl fmt::Display for WcagLevel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Fail => write!(f, "Fail"),
			Self::AALarge => write!(f, "AA Large"),
			Self::AA => write!(f, "AA"),
			Self::AAA => write!(f, "AAA"),
		}
	}
}

impl WcagLevel {
	/// Get the minimum contrast ratio for this level
	pub const fn min_ratio(self) -> f64 {
		match self {
			WcagLevel::Fail => 0.0,
			WcagLevel::AALarge => 3.0,
			WcagLevel::AA => 4.5,
			WcagLevel::AAA => 7.0,
		}
	}

	/// Get the level from a contrast ratio
	pub const fn from_ratio(ratio: f64) -> Self {
		if ratio >= 7.0 {
			WcagLevel::AAA
		} else if ratio >= 4.5 {
			WcagLevel::AA
		} else if ratio >= 3.0 {
			WcagLevel::AALarge
		} else {
			WcagLevel::Fail
		}
	}

	/// Human-readable description
	pub const fn description(self) -> &'static str {
		match self {
			WcagLevel::Fail => "Fails WCAG requirements",
			WcagLevel::AALarge => "WCAG AA Large (3:1)",
			WcagLevel::AA => "WCAG AA (4.5:1)",
			WcagLevel::AAA => "WCAG AAA (7:1)",
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::*;

	#[test]
	fn test_relative_luminance() {
		assert_eq!(Named::White.relative_luminance(), 1.0);
		assert_eq!(Named::Black.relative_luminance(), 0.0);
		assert_eq!(Srgb::from_u8(128, 128, 128, 100.0).relative_luminance(), 0.21586050011389923);
	}

	#[test]
	fn test_contrast_ratio() {
		assert_eq!(Named::White.wcag_contrast_ratio(Named::Black), 21.0);
		assert_eq!(Named::White.wcag_contrast_ratio(Named::White), 1.0);

		// Order doesn't matter
		let ratio1 = Named::White.wcag_contrast_ratio(Named::Black);
		let ratio2 = Named::Black.wcag_contrast_ratio(Named::White);
		assert_eq!(ratio1, ratio2);
	}

	#[test]
	fn test_wcag_levels() {
		assert_eq!(Named::White.wcag_level(Named::Black), WcagLevel::AAA);
		assert_eq!(Named::White.wcag_level(Named::White), WcagLevel::Fail);
		assert_eq!(Named::Rebeccapurple.wcag_level(Named::Gold), WcagLevel::AA);
	}

	#[test]
	fn test_find_minimum_contrast_aa() {
		// Should find a darker version that meets AA contrast (4.5:1)
		let min = Named::Rebeccapurple.find_minimum_contrast::<Srgb>(Named::White, WcagLevel::AA);
		assert!(min.is_some());
		let ratio = Named::Rebeccapurple.wcag_contrast_ratio(min.unwrap());
		assert_eq!((ratio * 10.0).round() / 10.0, 4.5, "Should hit the actual contrast");
	}

	#[test]
	fn test_find_minimum_contrast_aaa() {
		// Should find a darker version that meets AAA contrast (7.0:1)
		let min = Named::Rebeccapurple.find_minimum_contrast::<Srgb>(Named::White, WcagLevel::AAA);
		assert!(min.is_some());
		let ratio = Named::Rebeccapurple.wcag_contrast_ratio(min.unwrap());
		assert_eq!((ratio * 10.0).round() / 10.0, 7.0, "Should hit the actual contrast");
	}
}
