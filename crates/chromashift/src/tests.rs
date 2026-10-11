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

/// Asserts that an RGB-like type preserves values outside [0,1].
macro_rules! assert_oog_rgb_preserved {
	($ty:ident) => {
		let oog = $ty::new(-0.5, 1.5, -0.3, 100.0);
		assert_eq!(oog.red, -0.5, "{} red was clamped", stringify!($ty));
		assert_eq!(oog.green, 1.5, "{} green was clamped", stringify!($ty));
		assert_eq!(oog.blue, -0.3, "{} blue was clamped", stringify!($ty));
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
		Srgb::from_u8(102, 51, 153, 100.0),
		LinearRgb::new(0.13286832, 0.03310476, 0.31854683, 100.0),
		Hex::new(0x663399FF),
		Hsl::new(270.0, 50.0, 40.0, 100.0),
		Hsv::new(270.0, 66.666_664, 60.0000004, 100.0),
		Hwb::new(270.0, 19.9999996, 39.9999996, 100.0),
		Lab::new(32.39271642, 38.42299376, -47.69112551, 100.0),
		Lch::new(32.39271642, 61.24352947, 308.85712105, 100.0),
		Oklab::new(0.44027179, 0.08817676, -0.13386435, 100.0),
		Oklch::new(0.44027179, 0.16029599, 303.37298848, 100.0),
		A98Rgb::new(0.35800372, 0.21231662, 0.58434310, 100.0),
		XyzD50::new(11.62668443, 7.26049173, 23.25379520, 100.0),
		XyzD65::new(12.412, 7.493, 30.930, 100.0),
	);
}

#[test]
fn cornflower_blue() {
	test_combos(
		Srgb::from_u8(100, 149, 237, 100.0),
		LinearRgb::new(0.12743768, 0.30054379, 0.84687323, 100.0),
		Hex::new(0x6495EDFF),
		Hsl::new(218.54015, 79.19075, 66.07843, 100.0),
		Hsv::new(218.54015, 57.80591, 92.94118, 100.0),
		Hwb::new(218.54015, 39.215687, 7.058823, 100.0),
		Lab::new(61.23323695, 3.04679608, -50.18821475, 100.0),
		Lch::new(61.23323695, 50.28061123, 273.47401435, 100.0),
		Oklab::new(0.67462201, -0.02128901, -0.13974453, 100.0),
		Oklch::new(0.67462201, 0.14135683, 261.33802289, 100.0),
		A98Rgb::new(0.45474762, 0.57889667, 0.91593325, 100.0),
		XyzD50::new(29.24953872, 29.51472517, 63.57032517, 100.0),
		XyzD65::new(31.28682295, 30.31754694, 84.32669615, 100.0),
	);
}

#[test]
fn hex_123() {
	test_combos(
		Srgb::from_u8(17, 34, 51, 100.0),
		LinearRgb::new(0.00560539, 0.01599629, 0.03310477, 100.0),
		Hex::new(0x112233FF),
		Hsl::new(210.0, 50.0, 13.333333, 100.0),
		Hsv::new(210.0, 66.7, 20.0, 100.0),
		Hwb::new(210.0, 6.66, 80.0, 100.0),
		Lab::new(12.42990141, -2.50827059, -13.55803689, 100.0),
		Lch::new(12.42990141, 13.78810305, 259.51864388, 100.0),
		Oklab::new(0.24619, -0.01380, -0.03738, 100.0),
		Oklch::new(0.24619, 0.03985, 249.73162, 100.0),
		A98Rgb::new(0.11481753, 0.15252996, 0.21025089, 100.0),
		XyzD50::new(1.334189, 1.472150, 2.527108, 100.0),
		XyzD65::new(1.400641, 1.502188, 3.348217, 100.0),
	);
}

#[test]
fn text_hex_display() {
	assert_eq!(format!("{}", Hex::new(0x663399FF)), "#639");
	assert_eq!(format!("{}", Hex::new(0x66339900)), "#6390");
	assert_eq!(format!("{}", Hex::new(0x112233FF)), "#123");
	assert_eq!(format!("{}", Hex::new(0xFF00FFED)), "#ff00ffed");
	assert_eq!(format!("{}", Hex::new(0xFF00FFEE)), "#f0fe");
}

#[test]
fn display_serialises_per_css_color_4() {
	assert_eq!(format!("{}", Srgb::from_u8(218, 165, 32, 100.0)), "rgb(218, 165, 32)");
	assert_eq!(format!("{}", Srgb::from_u8(29, 164, 192, 95.0)), "rgba(29, 164, 192, 0.95)");
	assert_eq!(format!("{}", Srgb::new(1.2, -0.1, 0.5, 50.0)), "color(srgb 1.2 -0.1 0.5 / 0.5)");
	assert_eq!(format!("{}", Hsl::new(270.0, 50.0, 40.0, 50.0)), "hsl(270 50% 40% / 0.5)");
	assert_eq!(format!("{}", Hwb::new(20.0, 20.0, 30.0, 50.0)), "hwb(20 20% 30% / 0.5)");
	assert_eq!(format!("{}", Hsv::new(210.0, 66.7, 20.0, 50.0)), "hsv(210 66.7 20 / 0.5)");
	assert_eq!(format!("{}", Lab::new(56.2, 0.0, 83.6, 100.0)), "lab(56.2 0 83.6)");
	assert_eq!(format!("{}", Lch::new(56.2, 83.6, 357.4, 93.0)), "lch(56.2 83.6 357.4 / 0.93)");
	assert_eq!(format!("{}", Oklab::new(0.54, -0.1, -0.02, 100.0)), "oklab(0.54 -0.1 -0.02)");
	assert_eq!(format!("{}", Oklch::new(0.5385, 0.1725, 320.67, 70.0)), "oklch(0.5385 0.1725 320.67 / 0.7)");
	assert_eq!(
		format!("{}", ProphotoRgb::new(0.2804, 0.40283, 0.42259, 85.0)),
		"color(prophoto-rgb 0.2804 0.40283 0.42259 / 0.85)"
	);
	assert_eq!(format!("{}", XyzD50::new(10.0, 20.0, 30.0, 50.0)), "color(xyz-d50 0.1 0.2 0.3 / 0.5)");
}

#[test]
fn display_rounds_to_six_decimals_with_halves_towards_positive_infinity() {
	assert_eq!(
		format!("{}", Lab::new(50.0, -0.0078125, 0.0078125, 12.345679)),
		"lab(50 -0.007812 0.007813 / 0.123457)"
	);
}

#[test]
fn display_never_serialises_negative_zero() {
	assert_eq!(format!("{}", Oklab::new(1.0, -0.0, -0.0000001, 100.0)), "oklab(1 0 0)");
}

#[test]
fn display_serialises_f32_channels_without_float_noise() {
	assert_eq!(format!("{}", Hsl::new(300.1, 33.3, 66.7, 100.0)), "hsl(300.1 33.3% 66.7%)");
}

#[test]
fn text_hex_alpha_conversion() {
	assert_eq!(format!("{}", Hex::from(Srgb::from_u8(255, 255, 255, 0.0))), "#fff0");
	assert_eq!(Hex::from(Srgb::from_u8(255, 255, 255, 0.0)), Hex::new(0xFFFFFF00));
	assert_eq!(format!("{}", Hex::from(Srgb::from_u8(255, 255, 255, 50.0))), "#ffffff80");
	assert_eq!(Hex::from(Srgb::from_u8(255, 255, 255, 50.0)), Hex::new(0xFFFFFF80));
	let original = Srgb::from_u8(255, 255, 255, 50.0);
	let hex = Hex::from(original);
	let round_tripped = Srgb::from(hex);
	assert!(
		round_tripped.close_to(original, COLOR_EPSILON),
		"Round-trip failed: original={:?}, round_tripped={:?}",
		original,
		round_tripped
	);
	assert_eq!(Hex::from(Srgb::from_u8(0, 0, 0, 25.0)), Hex::new(0x00000040));
	assert_eq!(Hex::from(Srgb::from_u8(0, 0, 0, 75.0)), Hex::new(0x000000BF));
	assert_eq!(Hex::from(Srgb::from_u8(0, 0, 0, 100.0)), Hex::new(0x000000FF));
}

#[test]
fn rgb_types_preserve_oog_values() {
	assert_oog_rgb_preserved!(LinearRgb);
	assert_oog_rgb_preserved!(DisplayP3);
	assert_oog_rgb_preserved!(A98Rgb);
	assert_oog_rgb_preserved!(ProphotoRgb);
	assert_oog_rgb_preserved!(Rec2020);
}

#[test]
fn lab_oklab_preserve_oog_values() {
	let lab = Lab::new(110.0, 200.0, -200.0, 100.0);
	assert_eq!(lab.lightness, 110.0);
	assert_eq!(lab.a, 200.0);
	assert_eq!(lab.b, -200.0);

	let oklab = Oklab::new(1.5, 0.5, -0.5, 100.0);
	assert_eq!(oklab.lightness, 1.5);
	assert_eq!(oklab.a, 0.5);
	assert_eq!(oklab.b, -0.5);
}

#[test]
fn hsl_preserves_oog_values() {
	// color(display-p3 0 1 0) in HSL is approximately hsl(127.879 301.946 25.334)
	let oog = Hsl::new(127.0, 301.0, 25.0, 100.0);
	assert_eq!(oog.saturation, 301.0);
	assert_eq!(oog.lightness, 25.0);
}

/// Per CSS Color 4 §4.2, alpha is always clamped to [0,100].
#[test]
fn alpha_still_clamped() {
	macro_rules! assert_alpha_clamped {
		($color:expr, $expected:expr) => {
			assert_eq!($color.alpha, $expected);
		};
	}
	assert_alpha_clamped!(LinearRgb::new(0.0, 0.0, 0.0, 150.0), 100.0);
	assert_alpha_clamped!(LinearRgb::new(0.0, 0.0, 0.0, -10.0), 0.0);
	assert_alpha_clamped!(DisplayP3::new(0.0, 0.0, 0.0, 150.0), 100.0);
	assert_alpha_clamped!(Lab::new(0.0, 0.0, 0.0, -10.0), 0.0);
	assert_alpha_clamped!(Oklch::new(0.0, 0.0, 0.0, 200.0), 100.0);
}

#[test]
fn display_p3_green_to_linear_rgb_is_oog() {
	let p3_green = DisplayP3::new(0.0, 1.0, 0.0, 100.0);
	let linear: LinearRgb = p3_green.into();
	assert!(linear.red < 0.0, "Expected negative linear red, got {}", linear.red);
	assert!(linear.blue < 0.0, "Expected negative linear blue, got {}", linear.blue);
	assert!(linear.green > 1.0, "Expected green > 1.0, got {}", linear.green);
}

#[test]
fn display_p3_round_trips_through_xyz() {
	let original = DisplayP3::new(0.5, 0.7, 0.3, 100.0);
	let xyz: XyzD65 = original.into();
	let back: DisplayP3 = xyz.into();
	assert!(back.close_to(original, COLOR_EPSILON), "Round-trip failed: {:?} vs {:?}", original, back);
}

/// WPT: color-mix-out-of-gamut.html
/// color-mix(in hsl, color(display-p3 0 1 0) 100%, rgb(0,0,0) 0%) → color(srgb -0.511814 1.01832 -0.310726)
///
/// Tests the full OOG chain: DisplayP3 → LinearRgb → Hsl (OOG) → LinearRgb → DisplayP3
/// The key assertion is that the OOG sRGB float values survive the round-trip through HSL.
#[test]
fn wpt_display_p3_green_through_hsl() {
	let p3_green = DisplayP3::new(0.0, 1.0, 0.0, 100.0);
	let hsl: Hsl = p3_green.into();
	// Mix 100%/0% — result is just the first color
	let mixed = Hsl::mix(hsl, Hsl::new(0.0, 0.0, 0.0, 100.0), 0.0);
	// Convert to DisplayP3 to check the round-trip
	let back: DisplayP3 = mixed.into();
	assert!(back.close_to(p3_green, 0.001), "Expected {:?}, got {:?}", p3_green, back);
}

#[test]
fn in_gamut_rgb_types() {
	// In-gamut values
	assert!(LinearRgb::new(0.0, 0.5, 1.0, 100.0).in_gamut());
	assert!(DisplayP3::new(0.0, 1.0, 0.0, 100.0).in_gamut());
	assert!(A98Rgb::new(0.5, 0.5, 0.5, 100.0).in_gamut());
	assert!(ProphotoRgb::new(0.0, 0.0, 0.0, 100.0).in_gamut());
	assert!(Rec2020::new(1.0, 1.0, 1.0, 100.0).in_gamut());

	// OOG values
	assert!(!LinearRgb::new(-0.1, 0.5, 1.0, 100.0).in_gamut());
	assert!(!DisplayP3::new(0.0, 1.1, 0.0, 100.0).in_gamut());
	assert!(!A98Rgb::new(0.5, 0.5, -0.01, 100.0).in_gamut());
}

#[test]
fn in_gamut_lab_oklab() {
	assert!(Lab::new(50.0, 0.0, 0.0, 100.0).in_gamut());
	assert!(!Lab::new(110.0, 0.0, 0.0, 100.0).in_gamut());
	assert!(!Lab::new(50.0, 200.0, 0.0, 100.0).in_gamut());

	assert!(Oklab::new(0.5, 0.0, 0.0, 100.0).in_gamut());
	assert!(!Oklab::new(1.5, 0.0, 0.0, 100.0).in_gamut());
	assert!(!Oklab::new(0.5, 0.5, 0.0, 100.0).in_gamut());
}

#[test]
fn in_gamut_lch_oklch() {
	assert!(Lch::new(50.0, 75.0, 180.0, 100.0).in_gamut());
	assert!(!Lch::new(-1.0, 75.0, 180.0, 100.0).in_gamut());
	assert!(!Lch::new(50.0, 200.0, 180.0, 100.0).in_gamut());

	assert!(Oklch::new(0.5, 0.2, 180.0, 100.0).in_gamut());
	assert!(!Oklch::new(0.5, 0.5, 180.0, 100.0).in_gamut());
}

#[test]
fn in_gamut_hsl_hwb() {
	assert!(Hsl::new(180.0, 50.0, 50.0, 100.0).in_gamut());
	assert!(!Hsl::new(180.0, 301.0, 50.0, 100.0).in_gamut());

	assert!(Hwb::new(180.0, 20.0, 20.0, 100.0).in_gamut());
	assert!(!Hwb::new(180.0, -10.0, 20.0, 100.0).in_gamut());
}

#[test]
fn in_gamut_always_in_gamut_types() {
	// Srgb (u8), Hex (u32), and Hsv (clamps in constructor) are always in gamut
	assert!(Srgb::from_u8(255, 0, 128, 100.0).in_gamut());
	assert!(Hex::new(0xFF00FFFF).in_gamut());
	assert!(Hsv::new(180.0, 100.0, 100.0, 100.0).in_gamut());
}

#[test]
fn clamp_to_gamut_rgb_types() {
	let oog = LinearRgb::new(-0.5, 1.5, 0.5, 100.0);
	let clamped = oog.clamp_to_gamut();
	assert!(clamped.in_gamut());
	assert_eq!(clamped.red, 0.0);
	assert_eq!(clamped.green, 1.0);
	assert_eq!(clamped.blue, 0.5);
	assert_eq!(clamped.alpha, 100.0);
}

#[test]
fn clamp_to_gamut_lab() {
	let oog = Lab::new(110.0, 200.0, -200.0, 100.0);
	let clamped = oog.clamp_to_gamut();
	assert!(clamped.in_gamut());
	assert_eq!(clamped.lightness, 100.0);
	assert_eq!(clamped.a, 125.0);
	assert_eq!(clamped.b, -125.0);
}

#[test]
fn clamp_to_gamut_hsl() {
	let oog = Hsl::new(127.0, 301.0, -10.0, 100.0);
	let clamped = oog.clamp_to_gamut();
	assert!(clamped.in_gamut());
	assert_eq!(clamped.saturation, 100.0);
	assert_eq!(clamped.lightness, 0.0);
}

#[test]
fn clamp_to_gamut_preserves_already_in_gamut() {
	let color = DisplayP3::new(0.5, 0.7, 0.3, 80.0);
	let clamped = color.clamp_to_gamut();
	assert_eq!(clamped, color);
}

/// Display P3 green is OOG when viewed as sRGB linear.
/// clamp_to_gamut in LinearRgb should clip the negative channels.
#[test]
fn display_p3_green_clamped_in_linear_rgb() {
	let p3_green = DisplayP3::new(0.0, 1.0, 0.0, 100.0);
	let linear: LinearRgb = p3_green.into();
	assert!(!linear.in_gamut());
	let clamped = linear.clamp_to_gamut();
	assert!(clamped.in_gamut());
	assert_eq!(clamped.red, 0.0);
	assert_eq!(clamped.blue, 0.0);
}

#[test]
fn map_to_gamut_already_in_gamut() {
	let color = DisplayP3::new(0.5, 0.3, 0.7, 100.0);
	let mapped = color.map_to_gamut();
	assert!(mapped.in_gamut());
	assert!(mapped.close_to(color, COLOR_EPSILON));
}

#[test]
fn map_to_gamut_outside_gamut_oklch() {
	let mapped_311 = Oklch::new(0.8, 0.436, 311.0, 100.0).map_to_gamut();
	assert!(
		mapped_311.close_to(Oklch::new(0.8, 0.13820, 311.0, 100.0), COLOR_EPSILON),
		"oklch(80% 109% 311) mapped: {:?}",
		mapped_311,
	);
	let mapped_87 = Oklch::new(0.8, 0.436, 87.0, 100.0).map_to_gamut();
	assert!(
		mapped_87.close_to(Oklch::new(0.8, 0.16362, 87.0, 100.0), COLOR_EPSILON),
		"oklch(80% 109% 87) mapped: {:?}",
		mapped_87,
	);
}

#[test]
fn test_map_to_gamut_okclh_hex() {
	macro_rules! assert_maps_to_hex {
		($input:expr, $hex:expr) => {
			assert_eq!(Hex::from($input.map_to_gamut()), Hex::new($hex), "{:?}", $input);
		};
	}
	assert_maps_to_hex!(Oklch::new(0.8, 0.436, 87.0, 100.0), 0xebb500ff);
	assert_maps_to_hex!(Oklch::new(0.8, 1.5, 113.0, 100.0), 0xbfc800ff);
	assert_maps_to_hex!(Oklch::new(0.95, 0.4, 150.0, 100.0), 0xc7ffd1ff);
	assert_maps_to_hex!(Oklch::new(0.95, 0.35, 30.0, 100.0), 0xffe9e5ff);
	assert_maps_to_hex!(Oklch::new(0.85, 0.4, 270.0, 100.0), 0xbbccffff);
	assert_maps_to_hex!(Oklch::new(0.7, 0.45, 330.0, 100.0), 0xff12f7ff);
	assert_maps_to_hex!(Oklch::new(0.8, 0.38, 90.0, 100.0), 0xe6b700ff);
	assert_maps_to_hex!(Oklch::new(0.6, 0.42, 300.0, 100.0), 0x9c44ffff);
	assert_maps_to_hex!(Oklch::new(0.75, 0.40, 180.0, 100.0), 0x00c9b1ff);
}

#[test]
fn named_try_from_other_spaces() {
	let named = Named::Rebeccapurple;
	let srgb = Srgb::from_u8(102, 51, 153, 100.0);
	assert_eq!(Named::try_from(srgb).unwrap(), named);
	assert_eq!(Named::try_from(Hex::new(0x663399FF)).unwrap(), named);
	assert_eq!(Named::try_from(Hsl::new(270.0, 50.0, 40.0, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Hsv::new(270.0, 66.666_664, 60.0000004, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Hwb::new(270.0, 19.9999996, 39.9999996, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Lab::new(32.39271642, 38.42299376, -47.69112551, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Lch::new(32.39271642, 61.24352947, 308.85712105, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(LinearRgb::new(0.13286832, 0.03310476, 0.31854683, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Oklab::new(0.44027179, 0.08817676, -0.13386435, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Oklch::new(0.44027179, 0.16029599, 303.37298848, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(A98Rgb::new(0.35800372, 0.21231662, 0.58434310, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(XyzD50::new(11.62668443, 7.26049173, 23.25379520, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(XyzD65::new(12.412, 7.493, 30.930, 100.0)).unwrap(), named);
	assert_eq!(Named::try_from(Color::Srgb(srgb)).unwrap(), named);

	let translucent = Hex::new(0x66339980);
	assert!(matches!(Named::try_from(translucent), Err(ToNamedError::NotOpaque)));
	assert!(matches!(Named::try_from(Color::Hex(translucent)), Err(ToNamedError::NotOpaque)));
}

#[test]
fn color_space_partial_ord() {
	// Every space equals itself.
	assert_eq!(ColorSpace::Srgb.partial_cmp(&ColorSpace::Srgb), Some(core::cmp::Ordering::Equal));

	// sRGB < every wider gamut.
	assert!(ColorSpace::Srgb < ColorSpace::DisplayP3);
	assert!(ColorSpace::Srgb < ColorSpace::A98Rgb);
	assert!(ColorSpace::Srgb < ColorSpace::Rec2020);
	assert!(ColorSpace::Srgb < ColorSpace::ProphotoRgb);

	// Display P3 < Rec2020 < ProPhoto RGB
	assert!(ColorSpace::DisplayP3 < ColorSpace::Rec2020);
	assert!(ColorSpace::Rec2020 < ColorSpace::ProphotoRgb);
	assert!(ColorSpace::DisplayP3 < ColorSpace::ProphotoRgb);

	// A98 RGB < ProPhoto RGB
	assert!(ColorSpace::A98Rgb < ColorSpace::ProphotoRgb);

	// Display P3 and A98 RGB are incomparable.
	assert_eq!(ColorSpace::DisplayP3.partial_cmp(&ColorSpace::A98Rgb), None);
	assert_eq!(ColorSpace::A98Rgb.partial_cmp(&ColorSpace::DisplayP3), None);

	// A98 RGB and Rec2020 are incomparable.
	assert_eq!(ColorSpace::A98Rgb.partial_cmp(&ColorSpace::Rec2020), None);
}

#[test]
fn color_space_contains() {
	// contains() is equivalent to >=
	assert!(ColorSpace::DisplayP3.contains(ColorSpace::Srgb));
	assert!(ColorSpace::DisplayP3.contains(ColorSpace::DisplayP3));
	assert!(!ColorSpace::Srgb.contains(ColorSpace::DisplayP3));
	assert!(!ColorSpace::DisplayP3.contains(ColorSpace::A98Rgb));
	assert!(ColorSpace::ProphotoRgb.contains(ColorSpace::Rec2020));
}

#[test]
fn in_gamut_of_srgb_for_srgb_native_colors() {
	// sRGB-native Color variants should be in the sRGB gamut.
	assert!(Color::Srgb(Srgb::from_u8(255, 0, 0, 100.0)).in_gamut_of(ColorSpace::Srgb));
	assert!(Color::Hex(Hex::new(0xFF0000FF)).in_gamut_of(ColorSpace::Srgb));
	assert!(Color::Named(Named::Red).in_gamut_of(ColorSpace::Srgb));
	assert!(Color::Hsl(Hsl::new(0.0, 100.0, 50.0, 100.0)).in_gamut_of(ColorSpace::Srgb));
	assert!(Color::Hwb(Hwb::new(0.0, 0.0, 0.0, 100.0)).in_gamut_of(ColorSpace::Srgb));
}

#[test]
fn in_gamut_of_display_p3_wider_than_srgb() {
	// Display P3 red (1,0,0) is out of sRGB but in Display P3.
	let p3_red = Color::DisplayP3(DisplayP3::new(1.0, 0.0, 0.0, 100.0));
	assert!(!p3_red.in_gamut_of(ColorSpace::Srgb));
	assert!(p3_red.in_gamut_of(ColorSpace::DisplayP3));

	// Display P3 green (0,1,0) is out of sRGB but in Display P3.
	let p3_green = Color::DisplayP3(DisplayP3::new(0.0, 1.0, 0.0, 100.0));
	assert!(!p3_green.in_gamut_of(ColorSpace::Srgb));
	assert!(p3_green.in_gamut_of(ColorSpace::DisplayP3));
}

#[test]
fn in_gamut_of_srgb_color_fits_all_wider_gamuts() {
	// Plain sRGB red should be in gamut for all wider RGB spaces.
	let red = Color::Srgb(Srgb::from_u8(255, 0, 0, 100.0));
	assert!(red.in_gamut_of(ColorSpace::Srgb));
	assert!(red.in_gamut_of(ColorSpace::DisplayP3));
	assert!(red.in_gamut_of(ColorSpace::A98Rgb));
	assert!(red.in_gamut_of(ColorSpace::ProphotoRgb));
	assert!(red.in_gamut_of(ColorSpace::Rec2020));
}

#[test]
fn in_gamut_of_perceptual_space_checks_target_gamut() {
	// An Oklch colour with very high chroma should be out of sRGB gamut.
	let vivid = Color::Oklch(Oklch::new(0.7, 0.35, 150.0, 100.0));
	assert!(!vivid.in_gamut_of(ColorSpace::Srgb));

	// But it may still be in Display P3 gamut (P3 is wider).
	let moderate = Color::Oklch(Oklch::new(0.7, 0.2, 150.0, 100.0));
	// P3 should accept at least as many colours as sRGB.
	if moderate.in_gamut_of(ColorSpace::Srgb) {
		assert!(moderate.in_gamut_of(ColorSpace::DisplayP3));
	}
}

fn assert_matrix(actual: Matrix3, expected: [[f64; 3]; 3], tolerance: f64) {
	for (row, expected_row) in expected.iter().enumerate() {
		for (col, want) in expected_row.iter().enumerate() {
			let got = actual.0[row][col];
			assert!((got - want).abs() < tolerance, "[{row}][{col}]: expected {want}, got {got}");
		}
	}
}

#[test]
fn every_space_matrix_pair_is_an_inverse() {
	for space in
		[RgbSpace::SRGB_LINEAR, RgbSpace::DISPLAY_P3, RgbSpace::A98_RGB, RgbSpace::PROPHOTO_RGB, RgbSpace::REC2020]
	{
		let identity = space.to_xyz.then(&space.from_xyz);
		assert_matrix(identity, [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], 1e-12);
	}
}

#[test]
fn space_matrices_match_css_color_4_chromaticities() {
	for (space, [red, green, blue]) in [
		(RgbSpace::SRGB_LINEAR, [(0.64, 0.33), (0.30, 0.60), (0.15, 0.06)]),
		(RgbSpace::DISPLAY_P3, [(0.680, 0.320), (0.265, 0.690), (0.150, 0.060)]),
		(RgbSpace::A98_RGB, [(0.64, 0.33), (0.21, 0.71), (0.15, 0.06)]),
		(RgbSpace::PROPHOTO_RGB, [(0.734699, 0.265301), (0.159597, 0.840403), (0.036598, 0.000105)]),
		(RgbSpace::REC2020, [(0.708, 0.292), (0.170, 0.797), (0.131, 0.046)]),
	] {
		for (rgb, (x, y)) in [
			([1.0, 0.0, 0.0], red),
			([0.0, 1.0, 0.0], green),
			([0.0, 0.0, 1.0], blue),
			([1.0, 1.0, 1.0], (space.white.x, space.white.y)),
		] {
			let [cx, cy, cz] = space.xyz_from_linear(rgb);
			let sum = cx + cy + cz;
			assert!((cx / sum - x).abs() < 1e-12 && (cy / sum - y).abs() < 1e-12, "{space:?} {rgb:?}");
		}
	}
}

#[test]
fn transfer_functions_round_trip_through_zero() {
	for transfer in [Transfer::Linear, Transfer::Srgb, Transfer::A98Rgb, Transfer::ProphotoRgb, Transfer::Rec2020] {
		for value in [-1.5, -0.5, -0.001, 0.0, 0.001, 0.5, 1.0, 2.5] {
			let round_tripped = transfer.decode(transfer.encode(value));
			assert!((round_tripped - value).abs() < 1e-12, "{transfer:?} {value}: got {round_tripped}");
			assert_eq!(transfer.encode(-value), -transfer.encode(value), "{transfer:?} {value} not mirrored");
		}
	}
}

#[test]
fn wide_gamut_greys_stay_grey_in_srgb() {
	assert_eq!(Srgb::from(DisplayP3::new(0.5, 0.5, 0.5, 100.0)).to_u8(), [128, 128, 128]);
	assert_eq!(Srgb::from(DisplayP3::new(1.0, 1.0, 1.0, 100.0)).to_u8(), [255, 255, 255]);
	assert_eq!(Srgb::from(Rec2020::new(1.0, 1.0, 1.0, 100.0)).to_u8(), [255, 255, 255]);
	assert_eq!(Srgb::from(ProphotoRgb::new(1.0, 1.0, 1.0, 100.0)).to_u8(), [255, 255, 255]);
}

#[test]
#[cfg(not(feature = "rec2020-bt1886"))]
fn rec2020_decodes_with_the_bt2020_oetf_by_default() {
	assert_eq!(Srgb::from(Rec2020::new(0.5, 0.5, 0.5, 100.0)).to_u8(), [139, 139, 139]);
}

#[test]
#[cfg(feature = "rec2020-bt1886")]
fn rec2020_decodes_with_the_bt1886_gamma_when_enabled() {
	assert_eq!(Srgb::from(Rec2020::new(0.5, 0.5, 0.5, 100.0)).to_u8(), [120, 120, 120]);
}

#[test]
fn map_to_gamut_maps_into_its_own_gamut_not_srgb() {
	let p3 = DisplayP3::new(1.1, 0.0, 0.0, 100.0).map_to_gamut();
	assert!(p3.in_gamut() && !Color::DisplayP3(p3).in_gamut_of(ColorSpace::Srgb), "{p3:?}");
	let prophoto = ProphotoRgb::new(1.1, 0.0, 0.0, 100.0).map_to_gamut();
	assert!(prophoto.in_gamut() && !Color::ProphotoRgb(prophoto).in_gamut_of(ColorSpace::Srgb), "{prophoto:?}");
}

#[test]
fn srgb_transfer_matches_wcag_formula_for_every_8_bit_channel() {
	for channel in 0..=255u8 {
		let value = channel as f64 / 255.0;
		let wcag = if value <= 0.03928 { value / 12.92 } else { ((value + 0.055) / 1.055).powf(2.4) };
		assert_eq!(Transfer::Srgb.decode(value), wcag, "channel {channel}");
	}
}

#[test]
fn hex_alpha_round_trips_every_byte_through_color() {
	for alpha in 0..=255u32 {
		let hex = Hex::new(0xff00ff00 | alpha);
		assert_eq!(Hex::from(Color::from(hex)), hex, "alpha byte {alpha}");
	}
}

#[test]
fn srgb_serialises_8_bit_alpha_with_legacy_rule() {
	assert_eq!(Srgb::from(Hex::new(0xff00ffed)).to_string(), "rgba(255, 0, 255, 0.93)");
	assert_eq!(Srgb::from(Hex::new(0xff00ffee)).to_string(), "rgba(255, 0, 255, 0.933)");
	assert_eq!(Srgb::from_u8(255, 0, 255, 12.3456).to_string(), "rgba(255, 0, 255, 0.123456)");
}

#[test]
fn achromatic_colours_have_zero_hue_after_conversion() {
	for color in [Color::from(Named::White), Color::from(Hex::new(0x777777ff)), Color::from(Named::Black)] {
		assert_eq!(Hsl::from(color).hue, 0.0, "{color} hsl");
		assert_eq!(Hsl::from(color).saturation, 0.0, "{color} hsl");
		assert_eq!(Hwb::from(color).hue, 0.0, "{color} hwb");
		assert_eq!(Hsv::from(color).hue, 0.0, "{color} hsv");
		assert_eq!(Lch::from(color).hue, 0.0, "{color} lch");
		assert_eq!(Oklch::from(color).hue, 0.0, "{color} oklch");
	}
	assert_eq!(Hsl::from(Oklch::new(1.0, 0.0, 0.0, 100.0)).to_string(), "hsl(0 0% 100%)");
}

#[test]
fn out_of_range_srgb_round_trips_through_every_unbounded_space() {
	macro_rules! assert_round_trips {
		($($ty:ident),+) => {$(
			for (red, green, blue) in [(2.2, 1.5, 1.1), (1.2, -0.1, 0.3), (-0.2, -0.05, -0.4), (1.3, 1.1, 1.3), (-0.5, 0.2, 0.2)] {
				let via = $ty::from(Srgb::new(red, green, blue, 100.0));
				let back = Srgb::from(via);
				for (got, want) in [(back.red, red), (back.green, green), (back.blue, blue)] {
					assert!(
						(got - want).abs() < 1e-9,
						"{red} {green} {blue} via {} {via:?} came back {back:?}",
						stringify!($ty)
					);
				}
			}
		)+};
	}
	assert_round_trips!(
		A98Rgb,
		DisplayP3,
		Hsl,
		Hwb,
		Lab,
		Lch,
		LinearRgb,
		Oklab,
		Oklch,
		ProphotoRgb,
		Rec2020,
		XyzD50,
		XyzD65
	);
}
