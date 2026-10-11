use crate::Lab;

/// Trait for calculating perceptual color distance between colors.
///
/// This trait provides methods to determine if two colors are perceptually close using the CIE Delta E distance
/// calculation in Lab color space.
pub trait ColorDistance<T>: Sized + Copy
where
	Lab: From<T> + From<Self>,
{
	/// Compares `self` to `other` via [delta_e][ColorDistance::delta_e], checking that the result is less than or equal
	/// to `tolerance`.
	///
	/// `other` must implement [Into]&lt;[Lab]>
	///
	/// Tolerance values:
	///   - 0.0-1.0: Not perceptible by human eye
	///   - 1.0-2.0: Perceptible through close observation
	///   - 2.0-10.0: Perceptible at a glance
	///   - 11.0-49.0: Colors are more similar than opposite
	///   - 100.0: Colors are exact opposites
	///
	/// # Example
	///
	/// ```
	/// use chromashift::{Srgb, ColorDistance};
	///
	/// let red1 = Srgb::from_u8(255, 0, 0, 100.0);
	/// let red2 = Srgb::from_u8(254, 1, 1, 100.0);
	///
	/// assert!(red1.close_to(red2, 2.0));
	/// ```
	fn close_to(&self, other: T, tolerance: f64) -> bool {
		self.delta_e(other) <= tolerance
	}

	/// Compares `self` to `other` via [delta_e][ColorDistance::delta_e], checking that the result is greater than or
	/// equal to `tolerance`.
	///
	/// `other` must implement [Into]&lt;[Lab]>
	///
	/// Tolerance values:
	///   - 0.0-1.0: Not perceptible by human eye
	///   - 1.0-2.0: Perceptible through close observation
	///   - 2.0-10.0: Perceptible at a glance
	///   - 11.0-49.0: Colors are more similar than opposite
	///   - 100.0: Colors are exact opposites
	///
	/// # Example
	///
	/// ```
	/// use chromashift::{Srgb, ColorDistance};
	///
	/// let red = Srgb::from_u8(255, 0, 0, 100.0);
	/// let blue = Srgb::from_u8(0, 0, 255, 100.0);
	///
	/// assert!(red.far_from(blue, 10.0));
	/// ```
	fn far_from(&self, other: T, tolerance: f64) -> bool {
		self.delta_e(other) >= tolerance
	}

	/// This uses the CIEDE2000 Delta E formula difference between `self` and `other`.
	///
	/// `other` must implement [Into]&lt;[Lab]>
	///
	/// Tolerance values:
	///   - 0.0-1.0: Not perceptible by human eye
	///   - 1.0-2.0: Perceptible through close observation
	///   - 2.0-10.0: Perceptible at a glance
	///   - 11.0-49.0: Colors are more similar than opposite
	///   - 100.0: Colors are exact opposites
	///
	/// # Example
	///
	/// ```
	/// use chromashift::{Srgb, Named, ColorDistance};
	///
	/// let red = Srgb::from_u8(255, 0, 0, 100.0);
	/// let green = Srgb::from_u8(0, 255, 0, 100.0);
	///
	/// assert_eq!(red.delta_e(green).round(), 84.0);
	///
	/// assert_eq!(Named::Black.delta_e(Named::White).round(), 100.0);
	/// ```
	fn delta_e(&self, other: T) -> f64 {
		let lab1 = Lab::from(*self);
		let lab2 = Lab::from(other);
		// Extract Lab values
		let (l1, a1, b1) = (lab1.lightness, lab1.a, lab1.b);
		let (l2, a2, b2) = (lab2.lightness, lab2.a, lab2.b);

		// Initial chroma and mean chroma
		let c1 = (a1 * a1 + b1 * b1).sqrt();
		let c2 = (a2 * a2 + b2 * b2).sqrt();
		let c_bar = (c1 + c2) / 2.0;

		// G compensation for a* non-uniformity
		let c_bar_7 = c_bar.powf(7.0);
		let g = 0.5 * (1.0 - (c_bar_7 / (c_bar_7 + 25.0_f64.powf(7.0))).sqrt());

		// Corrected a* values and chroma
		let a1_prime = (1.0 + g) * a1;
		let a2_prime = (1.0 + g) * a2;
		let c1_prime = (a1_prime * a1_prime + b1 * b1).sqrt();
		let c2_prime = (a2_prime * a2_prime + b2 * b2).sqrt();

		// Hue angles (in degrees)
		let h1_prime =
			if a1_prime == 0.0 && b1 == 0.0 { 0.0 } else { b1.atan2(a1_prime).to_degrees().rem_euclid(360.0) };
		let h2_prime =
			if a2_prime == 0.0 && b2 == 0.0 { 0.0 } else { b2.atan2(a2_prime).to_degrees().rem_euclid(360.0) };

		// Differences
		let delta_l = l2 - l1;
		let delta_c = c2_prime - c1_prime;
		let delta_h = if c1_prime * c2_prime == 0.0 {
			0.0
		} else {
			let diff = h2_prime - h1_prime;
			if diff.abs() <= 180.0 {
				diff
			} else if diff > 180.0 {
				diff - 360.0
			} else {
				diff + 360.0
			}
		};
		let delta_h_big = 2.0 * (c1_prime * c2_prime).sqrt() * (delta_h.to_radians() / 2.0).sin();

		// Mean values
		let l_bar = (l1 + l2) / 2.0;
		let c_prime_bar = (c1_prime + c2_prime) / 2.0;
		let h_prime_bar = if c1_prime * c2_prime == 0.0 {
			h1_prime + h2_prime
		} else {
			let sum = h1_prime + h2_prime;
			let diff = (h1_prime - h2_prime).abs();
			if diff <= 180.0 {
				sum / 2.0
			} else if sum < 360.0 {
				(sum + 360.0) / 2.0
			} else {
				(sum - 360.0) / 2.0
			}
		};

		// Weighting functions (T factor for hue)
		let t = 1.0 - 0.17 * (h_prime_bar - 30.0).to_radians().cos()
			+ 0.24 * (2.0 * h_prime_bar).to_radians().cos()
			+ 0.32 * (3.0 * h_prime_bar + 6.0).to_radians().cos()
			- 0.20 * (4.0 * h_prime_bar - 63.0).to_radians().cos();

		let l_offset = l_bar - 50.0;
		let sl = 1.0 + (0.015 * l_offset * l_offset) / (20.0 + l_offset * l_offset).sqrt();
		let sc = 1.0 + 0.045 * c_prime_bar;
		let sh = 1.0 + 0.015 * c_prime_bar * t;

		// Rotation term for blue region
		let delta_theta = 30.0 * (-((h_prime_bar - 275.0) / 25.0).powf(2.0)).exp();
		let c_prime_bar_7 = c_prime_bar.powf(7.0);
		let rc = 2.0 * (c_prime_bar_7 / (c_prime_bar_7 + 25.0_f64.powf(7.0))).sqrt();
		let rt = -rc * (2.0 * delta_theta).to_radians().sin();

		// Final calculation (kL = kC = kH = 1.0, so omitted)
		let l_term = delta_l / sl;
		let c_term = delta_c / sc;
		let h_term = delta_h_big / sh;

		(l_term * l_term + c_term * c_term + h_term * h_term + rt * c_term * h_term).sqrt()
	}
}

impl<C, T> ColorDistance<T> for C
where
	C: Copy,
	Lab: From<C> + From<T>,
{
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Hsl, Srgb};

	#[test]
	fn test_identical_colors() {
		let red = Srgb::from_u8(255, 0, 0, 100.0);
		assert!(red.close_to(red, 0.0));
		assert_eq!(red.delta_e(red), 0.0);
	}

	#[test]
	fn test_very_similar_colors() {
		let red1 = Srgb::from_u8(255, 0, 0, 100.0);
		let red2 = Srgb::from_u8(254, 1, 1, 100.0);

		assert!(red1.close_to(red2, 2.0));
		assert!(red1.delta_e(red2) < 2.0);
	}

	#[test]
	fn test_different_colors() {
		let red = Srgb::from_u8(255, 0, 0, 100.0);
		let blue = Srgb::from_u8(0, 0, 255, 100.0);

		assert!(!red.close_to(blue, 10.0));
		assert!(red.delta_e(blue) > 40.0);
	}

	#[test]
	fn test_cross_color_space_comparison() {
		let red_srgb = Srgb::from_u8(255, 0, 0, 100.0);
		let red_hsl = Hsl::new(0.0, 100.0, 50.0, 100.0);

		// These should be very close (same red color in different spaces)
		assert!(red_srgb.close_to(red_hsl, 2.0));
	}

	#[test]
	fn test_tolerance_levels() {
		let color1 = Srgb::from_u8(100, 100, 100, 100.0);
		let color2 = Srgb::from_u8(110, 105, 95, 100.0);

		let distance = color1.delta_e(color2);

		// Test that tolerance works correctly
		assert!(!color1.close_to(color2, distance - 0.1));
		assert!(color1.close_to(color2, distance + 0.1));
	}

	#[test]
	fn test_alpha_ignored_in_distance() {
		let color1 = Srgb::from_u8(255, 0, 0, 100.0);
		let color2 = Srgb::from_u8(255, 0, 0, 50.0);

		// Alpha should not affect color distance calculation
		assert!(color1.close_to(color2, 0.1));
	}
}
