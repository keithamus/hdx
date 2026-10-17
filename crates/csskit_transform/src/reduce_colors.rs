use crate::prelude::*;
use chromashift::{
	COLOR_EPSILON, ColorDistance, ColorSpace, Gamut, Hex, LinearRgb, Named, PerceptualRound, Srgb, ToAlpha, round_dp,
};
use css_ast::{
	CalcableValue, Color, ColorFunction, ColorMixFunction, CssTypes, HueInterpolationDirection,
	InterpolationColorSpace, ToChromashift, VisitNode, Visitable,
};
use css_parse::{Arena, format_in};

pub struct ReduceColors<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceColors<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.has_value_kinds(CssTypes::Color)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

trait Shortest<'a> {
	fn shortest(&self, arena: &'a Arena) -> Option<&'a str>;
}

impl<'a> Shortest<'a> for chromashift::Color {
	fn shortest(&self, arena: &'a Arena) -> Option<&'a str> {
		[
			Some(format_in!(in arena, "{}", Hex::from(*self)).into_str()),
			Named::try_from(*self).ok().map(|named| format_in!(in arena, "{named}").into_str()),
			Some(format_in!(in arena, "{}", Srgb::from(*self).round()).into_str()),
		]
		.into_iter()
		.flatten()
		.min_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)))
	}
}

fn fits_srgb(color: chromashift::Color) -> bool {
	color.in_gamut_of(ColorSpace::Srgb) || color.close_to(LinearRgb::from(color).clamp_to_gamut(), COLOR_EPSILON)
}

/// Formats a CSS alpha value (0–1) from chromashift's internal 0–100 representation.
/// Returns `None` when the alpha is fully opaque (100.0).
fn css_alpha(alpha: f32) -> Option<f64> {
	if alpha >= 100.0 {
		return None;
	}
	Some(round_dp(alpha as f64 / 100.0, 3))
}

/// Serialises a chromashift colour into valid CSS function syntax.
///
/// This produces the native-space representation (e.g. `lch(54.3 43.8 274.5 / 0.746)`) which the
/// compact writer will then minify (removing leading zeros, collapsing whitespace around `/`, etc.).
///
/// Returns `None` for colour types that don't have a non-sRGB CSS function syntax (Hex, Named, Srgb, Hsv).
trait ToCss<'a> {
	fn to_css(&self, arena: &'a Arena) -> Option<&'a str>;
}

macro_rules! impl_to_css_3ch {
	($ty:ident, $name:literal, $c1:ident, $c2:ident, $c3:ident) => {
		impl<'a> ToCss<'a> for chromashift::$ty {
			fn to_css(&self, arena: &'a Arena) -> Option<&'a str> {
				let alpha = css_alpha(self.alpha);
				if let Some(a) = alpha {
					Some(
						format_in!(in arena, concat!($name, "({} {} {} / {})"), self.$c1, self.$c2, self.$c3, a)
							.into_str(),
					)
				} else {
					Some(format_in!(in arena, concat!($name, "({} {} {})"), self.$c1, self.$c2, self.$c3).into_str())
				}
			}
		}
	};
	($ty:ident, $name:literal, $c1:ident, $c2:ident: $suf2:literal, $c3:ident: $suf3:literal) => {
		impl<'a> ToCss<'a> for chromashift::$ty {
			fn to_css(&self, arena: &'a Arena) -> Option<&'a str> {
				let alpha = css_alpha(self.alpha);
				if let Some(a) = alpha {
					Some(
						format_in!(
							in arena,
							concat!($name, "({} {}", $suf2, " {}", $suf3, " / {})"),
							self.$c1, self.$c2, self.$c3, a
						)
						.into_str(),
					)
				} else {
					Some(
						format_in!(
							in arena,
							concat!($name, "({} {}", $suf2, " {}", $suf3, ")"),
							self.$c1, self.$c2, self.$c3
						)
						.into_str(),
					)
				}
			}
		}
	};
}

macro_rules! impl_to_css_color_fn {
	($ty:ident, $space:literal) => {
		impl<'a> ToCss<'a> for chromashift::$ty {
			fn to_css(&self, arena: &'a Arena) -> Option<&'a str> {
				let alpha = css_alpha(self.alpha);
				if let Some(a) = alpha {
					Some(
						format_in!(
							in arena,
							concat!("color(", $space, " {} {} {} / {})"),
							self.red, self.green, self.blue, a
						)
						.into_str(),
					)
				} else {
					Some(
						format_in!(
							in arena,
							concat!("color(", $space, " {} {} {})"),
							self.red, self.green, self.blue
						)
						.into_str(),
					)
				}
			}
		}
	};
}

macro_rules! impl_to_css_xyz {
	($ty:ident, $space:literal) => {
		impl<'a> ToCss<'a> for chromashift::$ty {
			fn to_css(&self, arena: &'a Arena) -> Option<&'a str> {
				let alpha = css_alpha(self.alpha);
				// CSS color(xyz-*) uses 0–1 scale; chromashift stores 0–100 internally.
				// After dividing, re-apply round_dp(4) to eliminate float display artifacts
				// (e.g. 18.76 / 100.0 = 0.18760000000000002 without this).
				let x = round_dp(self.x / 100.0, 4);
				let y = round_dp(self.y / 100.0, 4);
				let z = round_dp(self.z / 100.0, 4);
				if let Some(a) = alpha {
					Some(format_in!(in arena, concat!("color(", $space, " {} {} {} / {})"), x, y, z, a).into_str())
				} else {
					Some(format_in!(in arena, concat!("color(", $space, " {} {} {})"), x, y, z).into_str())
				}
			}
		}
	};
}

impl_to_css_3ch!(Lab, "lab", lightness, a, b);
impl_to_css_3ch!(Lch, "lch", lightness, chroma, hue);
impl_to_css_3ch!(Oklab, "oklab", lightness, a, b);
impl_to_css_3ch!(Oklch, "oklch", lightness, chroma, hue);
impl_to_css_3ch!(Hsl, "hsl", hue, saturation: "%", lightness: "%");
impl_to_css_3ch!(Hwb, "hwb", hue, whiteness: "%", blackness: "%");

impl_to_css_color_fn!(DisplayP3, "display-p3");
impl_to_css_color_fn!(LinearRgb, "srgb-linear");
impl_to_css_color_fn!(A98Rgb, "a98-rgb");
impl_to_css_color_fn!(ProphotoRgb, "prophoto-rgb");
impl_to_css_color_fn!(Rec2020, "rec2020");

impl_to_css_xyz!(XyzD50, "xyz-d50");
impl_to_css_xyz!(XyzD65, "xyz-d65");

impl<'a> ToCss<'a> for chromashift::Color {
	fn to_css(&self, arena: &'a Arena) -> Option<&'a str> {
		match self {
			chromashift::Color::Lab(c) => c.to_css(arena),
			chromashift::Color::Lch(c) => c.to_css(arena),
			chromashift::Color::Oklab(c) => c.to_css(arena),
			chromashift::Color::Oklch(c) => c.to_css(arena),
			chromashift::Color::Hsl(c) => c.to_css(arena),
			chromashift::Color::Hwb(c) => c.to_css(arena),
			chromashift::Color::DisplayP3(c) => c.to_css(arena),
			chromashift::Color::LinearRgb(c) => c.to_css(arena),
			chromashift::Color::A98Rgb(c) => c.to_css(arena),
			chromashift::Color::ProphotoRgb(c) => c.to_css(arena),
			chromashift::Color::Rec2020(c) => c.to_css(arena),
			chromashift::Color::XyzD50(c) => c.to_css(arena),
			chromashift::Color::XyzD65(c) => c.to_css(arena),
			// sRGB types don't need native-space CSS — they use Shortest
			chromashift::Color::Hex(_)
			| chromashift::Color::Named(_)
			| chromashift::Color::Srgb(_)
			| chromashift::Color::Hsv(_) => None,
		}
	}
}

#[visitor]
impl<'a, 'ctx, N> Visit for ReduceColors<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.node_id.is_some() && Self::skips_subtree(&node.subtree_metadata()) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn visit_color(&mut self, color: &Color) {
		// color-mix() is handled by visit_color_mix_function
		if let Color::Function(colorfn) = color
			&& matches!(**colorfn, ColorFunction::ColorMix(_))
		{
			return;
		}
		let Some(chroma_color) = color.to_chromashift() else {
			return;
		};
		let arena = self.transformer.alloc();
		let len = color.to_span().len() as usize;

		if fits_srgb(chroma_color)
			&& let Some(candidate) = chroma_color.shortest(arena)
			&& candidate.len() < len
		{
			self.transformer.replace_parsed::<Color>(color.to_span(), candidate);
			return;
		}

		// Try the native-space rounded form. This preserves the original colour space
		// while reducing precision to perceptually safe levels.
		let rounded = chroma_color.round();
		if let Some(css) = rounded.to_css(arena)
			&& css.len() < len
		{
			self.transformer.replace_parsed::<Color>(color.to_span(), css);
		}
	}

	fn visit_color_mix_function<'b>(&mut self, mix: &ColorMixFunction<'b>) -> VisitFlow {
		let outer_span = mix.to_span();
		let outer_len = outer_span.len() as usize;
		let arena = self.transformer.alloc();

		let n = mix.parts.len();
		let default_pct = 100.0 / n as f32;

		// A percentage that is a substitution/math function (var()/calc()/...) can't be
		// resolved statically, so none of the numeric reductions below are safe. Bail if any
		// explicit percentage is not a plain literal.
		let literal_pct = |pct: &CalcableValue<'b, css_ast::Percentage>| match pct {
			CalcableValue::Literal(p) => Some(p.value()),
			_ => None,
		};
		if (&mix.parts).into_iter().any(|(p, _)| p.percentage.as_ref().is_some_and(|pct| literal_pct(pct).is_none())) {
			return VisitFlow::DESCEND;
		}

		// Resolve effective percentages
		let explicit_sum: f32 =
			(&mix.parts).into_iter().filter_map(|(p, _)| p.percentage.as_ref().and_then(&literal_pct)).sum();
		let implicit_count = (&mix.parts).into_iter().filter(|(p, _)| p.percentage.is_none()).count();
		let implicit_share = if implicit_count > 0 { (100.0 - explicit_sum) / implicit_count as f32 } else { 0.0 };
		let pcts: Vec<f32> = (&mix.parts)
			.into_iter()
			.map(|(p, _)| p.percentage.as_ref().and_then(&literal_pct).unwrap_or(implicit_share))
			.collect();
		let sum: f32 = pcts.iter().sum();

		// If exactly one part has all the weight (sum == 100 and all others are 0), collapse to it.
		if sum == 100.0
			&& let Some(dominant) = pcts.iter().position(|&p| p == 100.0)
		{
			{
				let (part, _) = &mix.parts[dominant];
				let chroma = part.color.to_chromashift();
				let str = chroma.and_then(|c| c.shortest(arena)).unwrap_or_else(|| {
					let span = part.color.to_span();
					&self.transformer.source_text[span.start().0 as usize..span.end().0 as usize]
				});
				self.transformer.clear_pending_edits(outer_span);
				self.transformer.replace_parsed::<Color>(outer_span, str);
				return VisitFlow::SKIP_CHILDREN;
			}
		}

		// All identical colors with alpha_mult == 1 (sum >= 100): collapse to first.
		if sum >= 100.0 {
			let chromata: Vec<_> = (&mix.parts).into_iter().map(|(p, _)| p.color.to_chromashift()).collect();
			if chromata.iter().all(Option::is_some) {
				let first = chromata[0].unwrap();
				if chromata[1..].iter().all(|c| c.unwrap().delta_e(first) < COLOR_EPSILON) {
					let (part, _) = &mix.parts[0];
					let str = first.shortest(arena).unwrap_or_else(|| {
						let span = part.color.to_span();
						&self.transformer.source_text[span.start().0 as usize..span.end().0 as usize]
					});
					self.transformer.clear_pending_edits(outer_span);
					self.transformer.replace_parsed::<Color>(outer_span, str);
					return VisitFlow::SKIP_CHILDREN;
				}
			}
		}

		// Try static mix to reduce colors:
		let all_known = (&mix.parts).into_iter().all(|(p, _)| p.color.to_chromashift().is_some());
		if all_known && let Some(mixed) = mix.to_chromashift() {
			let alpha_mult = (sum as f64 / 100.0).min(1.0);
			let mixed_alpha = (mixed.to_alpha() as f64 / 100.0 * alpha_mult * 100.0) as f32;
			let mixed = mixed.with_alpha(mixed_alpha);
			let rounded = mixed.round();
			let native_css = rounded.to_css(arena);
			let srgb_css = if fits_srgb(mixed) { mixed.shortest(arena) } else { None };
			let candidate =
				native_css.into_iter().chain(srgb_css).min_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
			if let Some(candidate) = candidate
				&& candidate.len() < outer_len
			{
				self.transformer.replace_parsed::<Color>(outer_span, candidate);
				return VisitFlow::SKIP_CHILDREN;
			}
		}

		// Remove redundant percentages on an equal N-way split (partial optimisation).
		if (sum - 100.0).abs() < 0.01 {
			for ((part, _), &effective) in (&mix.parts).into_iter().zip(&pcts) {
				if let Some(ref pct) = part.percentage
					&& (effective - default_pct).abs() < 0.01
				{
					self.transformer.delete(pct.to_span());
				}
			}
		}

		// Remove redundant "shorter hue" (shorter is the default hue interpolation direction)
		if let Some(ref interp) = mix.interpolation
			&& let InterpolationColorSpace::Polar(_, Some(ref hue_method)) = interp.color_space
			&& matches!(hue_method.direction, HueInterpolationDirection::Shorter(_))
		{
			self.transformer.delete(hue_method.to_span());
		}
		VisitFlow::DESCEND
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn reduces_full_length_hex() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: #ffffff; }",
			"body { color: #fff; }"
		);
	}

	#[test]
	fn prefers_shorthand_hex_over_keyword() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: #000000; }",
			"body { color: #000; }"
		);
	}

	#[test]
	fn ignores_invalid_hex_length() {
		assert_no_transform!(CssMinifierFeature::ReduceColors, CssAtomSet, StyleSheet, "body { color: #F0000; }");
	}

	#[test]
	fn prefers_named_over_rgb() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: rgb(210, 180, 140); }",
			"body { color: tan; }"
		);
	}

	#[test]
	fn shortens_alpha_hex() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"body { color: rgba(255, 0, 0, 0.5); }",
			"body { color: #ff000080; }"
		);
	}

	#[test]
	fn no_transform_when_already_short() {
		assert_no_transform!(CssMinifierFeature::ReduceColors, CssAtomSet, StyleSheet, "body { color: red; }");
	}

	#[test]
	fn no_transform_for_currentcolor() {
		assert_no_transform!(CssMinifierFeature::ReduceColors, CssAtomSet, StyleSheet, "body { color: currentcolor; }");
	}

	#[test]
	fn reduces_color_srgb_function() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color(srgb 1 0 0); }",
			"a { color: red; }"
		);
	}

	#[test]
	fn reduces_in_gamut_display_p3_to_shortest() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color(display-p3 0.5 0.5 0.5); }",
			"a { color: gray; }"
		);
	}

	#[test]
	fn no_transform_for_out_of_gamut_display_p3() {
		assert_no_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color(display-p3 1 0 0); }"
		);
	}
	#[test]

	fn color_mix_100_percent_first() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 100%, blue); }",
			"a { color: red; }"
		);
	}

	#[test]
	fn color_mix_0_percent_first() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 0%, blue); }",
			"a { color: #00f; }"
		);
	}

	#[test]
	fn color_mix_all_zero_percent() {
		// Per CSS Color 5 §3.3: sum of 0% → transparent black.
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 0%, green 0%, blue 0%); }",
			"a { color: #0000; }"
		);
	}

	#[test]
	fn color_mix_same_color_both_sides() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red, red); }",
			"a { color: red; }"
		);
	}

	#[test]
	fn color_mix_removes_redundant_50_50() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, currentcolor 50%, red 50%); }",
			"a { color: color-mix(in srgb, currentcolor, red); }"
		);
	}

	#[test]
	fn color_mix_removes_redundant_equal_n_way_split() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, currentcolor 33.33%, red 33.33%, blue 33.33%); }",
			"a { color: color-mix(in srgb, currentcolor, red, blue); }"
		);
	}

	#[test]
	fn color_mix_removes_single_redundant_50() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, currentcolor 50%, red); }",
			"a { color: color-mix(in srgb, currentcolor, red); }"
		);
	}

	#[test]
	fn color_mix_removes_shorter_hue() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch shorter hue, currentcolor, red); }",
			"a { color: color-mix(in oklch, currentcolor, red); }"
		);
	}

	#[test]
	fn color_mix_keeps_longer_hue() {
		assert_no_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch longer hue,currentcolor,red); }"
		);
	}

	#[test]
	fn color_mix_no_transform_when_already_compact() {
		assert_no_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch longer hue,currentcolor,red); }"
		);
	}

	#[test]
	fn color_mix_minifies_inner_colors() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch, rgba(255, 255, 255, 1), currentcolor); }",
			"a { color: color-mix(in oklch, #fff, currentcolor); }"
		);
	}

	#[test]
	fn color_mix_minifies_inner_rgb_to_named() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, hsl(0, 100%, 50%), currentcolor); }",
			"a { color: color-mix(in srgb, red, currentcolor); }"
		);
	}

	#[test]
	fn color_mix_mixes_static_colors() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red, blue); }",
			"a { color: purple; }"
		);
	}

	#[test]
	fn color_mix_normalizes_percentages_over_100() {
		// 80% + 40% = 120%, normalizes to 66.67%/33.33%, giving rgb(170, 0, 85) = #a05
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 80%, blue 40%); }",
			"a { color: #a05; }"
		);
	}

	#[test]
	fn color_mix_alpha_multiplier_under_100() {
		// 30% + 30% = 60%, alpha_mult = 0.6, result is semi-transparent purple
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 30%, blue 30%); }",
			"a { color: #80008099; }"
		);
	}

	#[test]
	fn color_mix_no_100_shortcircuit_when_both_explicit() {
		// red 100% + blue 50% sums to 150%, must normalize to 67/33 — not short-circuit to red
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, red 100%, blue 50%); }",
			"a { color: #a05; }"
		);
	}

	#[test]
	fn color_mix_oklch_out_of_gamut_uses_native_space() {
		// oklch mix of lime+blue is out of sRGB gamut — resolved to oklch(), not hex.
		// Perceptual rounding: L/C at 3dp, hue at 1dp.
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in oklch, lime, blue); }",
			"a { color: oklch(0.659 0.304 203.3); }"
		);
	}

	#[test]
	fn color_mix_none_channel_adopts_other() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: color-mix(in srgb, rgb(none 0 0) 50%, rgb(200 0 0) 50%); }",
			"a { color: #c80000; }"
		);
	}

	#[test]
	fn rgb_none_channel_resolves_to_zero() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: rgb(none 128 0); }",
			"a { color: green; }"
		);
	}

	#[test]
	fn relative_rgb_static_channels_minified() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: rgb(from red 200 g b); }",
			"a { color: #c80000; }"
		);
	}

	#[test]
	fn relative_rgb_all_keywords_passthrough() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: rgb(from red r g b); }",
			"a { color: red; }"
		);
	}

	#[test]
	fn relative_rgb_all_static_minified() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: rgb(from blue 255 255 0); }",
			"a { color: #ff0; }"
		);
	}

	#[test]
	fn relative_hsl_keywords_passthrough() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: hsl(from green h s l); }",
			"a { color: green; }"
		);
	}

	#[test]
	fn relative_oklch_static_produces_named() {
		assert_transform!(
			CssMinifierFeature::ReduceColors,
			CssAtomSet,
			StyleSheet,
			"a { color: oklch(from red l c h); }",
			"a { color: red; }"
		);
	}
}
