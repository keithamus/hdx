use crate::*;

/// Whether a colour is within the natural bounds of its colour space, and the ability to produce a naively clamped
/// version or a perceptually gamut-mapped version.
///
/// CSS Color 4 12.1 and 13.1 state that out-of-gamut values must be preserved through intermediate computations. Gamut
/// mapping (reducing to displayable range) only happens at "actual-value" / display time. This trait lets callers query
/// and map when appropriate.
pub trait Gamut: Sized {
	/// Returns `true` if all colour channels are within the natural bounds of this colour space.
	/// Alpha is not considered — it is always clamped on construction.
	fn in_gamut(&self) -> bool;

	/// Returns a copy with all colour channels naively clamped to the natural bounds.
	///
	/// This is simple per-channel clipping for fast but less perceptually pleasing results.
	fn clamp_to_gamut(&self) -> Self;

	/// Perceptually maps this colour into gamut.
	///
	/// For RGB-based colour spaces this should use the ray trace algorithm in CSS Color 4 13.2, to casting a ray from an
	/// achromatic anchor toward the out-of-gamut colour and finding the intersection with the gamut's RGB cube via the
	/// slab method.
	///
	/// Reference: <https://facelessuser.github.io/coloraide/gamut/#ray-tracing-chroma-reduction>
	/// CSS Color 4 spec: <https://drafts.csswg.org/css-color-4/#pseudo-raytrace>
	fn map_to_gamut(self) -> Self;
}

/// CSS Color 4 13.2.6 ray trace gamut mapping, steps 3–14.
///
/// Caller has already performed steps 1–2 (in-gamut check and conversion to OkLCh).
///
/// <https://drafts.csswg.org/css-color-4/#pseudo-raytrace>
fn raytrace(oklch: Oklch, destination: &RgbSpace) -> [f64; 3] {
	let alpha = oklch.alpha;
	let to_destination = XyzD65::WHITE.adapt_to(destination.white).then(&destination.from_xyz);
	let from_destination = destination.to_xyz.then(&destination.white.adapt_to(XyzD65::WHITE));
	let to_linear = |oklch: Oklch| {
		let XyzD65 { x, y, z, .. } = XyzD65::from(Oklab::from(oklch));
		to_destination.transform([x / 100.0, y / 100.0, z / 100.0])
	};

	// 3. if the Lightness of |origin_OkLCh| is >= 100%, return white.
	if oklch.lightness >= 1.0 {
		return [1.0, 1.0, 1.0];
	}
	// 4. if the Lightness of |origin_OkLCh| is <= 0%, return black.
	if oklch.lightness <= 0.0 {
		return [0.0, 0.0, 0.0];
	}

	// 5. let |l_origin| be the OkLCh lightness of |origin_OkLCh|.
	let l_origin = oklch.lightness;

	// 6. let |h_origin| be the OkLCh hue of |origin_OkLCh|.
	let h_origin = oklch.hue;

	// 7. let |anchor| be an achromatic OkLCh color (l_origin, 0, h_origin),
	//    converted to the linear-light form of |destination|.
	let mut anchor = to_linear(Oklch::new(l_origin, 0.0, h_origin, alpha));

	// 8. let |origin_rgb| be |origin_OkLCh| converted to the linear-light form of |destination|.
	let mut origin_rgb = to_linear(oklch);

	// 9. let |low| be 1E-6.
	let low = 1e-6;

	// 10. let |high| be 1.0 - |low|.
	let high = 1.0 - low;

	// 11. let |last| be |origin_rgb|.
	let mut last = origin_rgb;

	// 12. for (i=0; i<4; i++)
	for i in 0..4 {
		// 12.1. if (i > 0)
		if i > 0 {
			// 12.1.1. let |current_OkLCh| be |origin_rgb| converted to OkLCh.
			let [x, y, z] = from_destination.transform(origin_rgb);
			let mut current_oklch = Oklch::from(Oklab::from(XyzD65::new(x * 100.0, y * 100.0, z * 100.0, alpha)));

			// 12.1.2. let the lightness of |current_OkLCh| be |l_origin|.
			current_oklch.lightness = l_origin;

			// 12.1.3. let the hue of |current_OkLCh| be |h_origin|.
			current_oklch.hue = h_origin;

			// 12.1.4. let |origin_rgb| be |current_OkLCh| converted to the linear-light
			//         form of |destination|.
			origin_rgb = to_linear(current_oklch);
		}

		// 12.2. Cast a ray from |anchor| to |origin_rgb| and let |intersection| be
		//       the intersection of this ray with the gamut boundary.
		let intersection = raytrace_box(&anchor, &origin_rgb);

		match intersection {
			// 12.3. if an intersection was not found, let |origin_rgb| be |last|
			//       and exit the loop.
			None => {
				origin_rgb = last;
				break;
			}
			Some(hit) => {
				// 12.4. if (i > 0) AND (each component of |origin_rgb| is between
				//       |low| and |high|) then let |anchor| be |origin_rgb|.
				if i > 0 && origin_rgb.iter().all(|&x| low < x && x < high) {
					anchor = origin_rgb;
				}

				// 12.5. let |origin_rgb| be |intersection|.
				// 12.6. let |last| be |intersection|.
				origin_rgb = hit;
				last = hit;
			}
		}
	}

	// 13. let |clipped| be |origin_rgb| clipped to gamut (components in range 0 to 1),
	//     trimming off any noise due to floating point inaccuracy.
	// 14. return |clipped|, converted to |destination| as the gamut mapped color.
	origin_rgb.map(|channel| channel.clamp(0.0, 1.0))
}

fn raytrace_to_linear_rgb(oklch: Oklch) -> LinearRgb {
	let [red, green, blue] = raytrace(oklch, &LinearRgb::SPACE);
	LinearRgb::new(red, green, blue, oklch.alpha)
}

/// Implements `map_to_gamut` for an RGB colour type, mapping into the gamut of its own `SPACE`.
///
/// Steps 1–2 of the spec algorithm live here; steps 3–14 are in [`raytrace`].
macro_rules! impl_map_to_gamut_raytrace {
	($($ty:ident),+ $(,)?) => {
		$(impl $ty {
			fn raytrace_map_to_gamut(self) -> Self {
				// 1. if |origin| is in gamut for |destination|, return it.
				if self.in_gamut() {
					return self;
				}

				// 2. let |origin_OkLCh| be |origin| converted to the OkLCh color space.
				let oklch = Oklch::from(self);

				// Steps 3–14.
				let [red, green, blue] = Self::SPACE.transfer.encode_all(raytrace(oklch, &Self::SPACE));
				Self::new(red, green, blue, self.alpha).clamp_to_gamut()
			}
		})+
	};
}

impl_map_to_gamut_raytrace!(Srgb, LinearRgb, DisplayP3, A98Rgb, ProphotoRgb, Rec2020);

/// CSS Color 4 13.2.6 "cast a ray" — ray–box intersection using the slab method.
///
/// Given a ray from `start` through `end`, finds where it intersects the unit cube [0,1]³.
/// Returns `None` if no valid intersection exists (parallel miss, behind ray, or degenerate).
///
/// <https://drafts.csswg.org/css-color-4/#pseudo-raytrace>
/// <https://en.wikipedia.org/wiki/Slab_method>
fn raytrace_box(start: &[f64; 3], end: &[f64; 3]) -> Option<[f64; 3]> {
	// 1. (bmin and bmax are [0,0,0] and [1,1,1] for unit-range RGB gamuts.)

	// 2. let |tfar| be infinity.
	let mut tfar = f64::INFINITY;

	// 3. let |tnear| be -infinity.
	let mut tnear = f64::NEG_INFINITY;

	// 4. let |direction| be a 3-element array.
	let mut direction = [0.0_f64; 3];

	// 5. for (i = 0; i < 3; i++):
	for i in 0..3 {
		let a = start[i]; //     let |a| be |start|[i].
		let b = end[i]; //       let |b| be |end|[i].
		let d = b - a; //        let |d| be |b| - |a|.
		direction[i] = d; //     let |direction|[i] be |d|.

		// if abs(|d|) > 1E-12:
		// (Corrected per https://github.com/w3c/csswg-drafts/pull/13416 — using an
		// epsilon to prevent numerical instability when d approaches zero.)
		if d.abs() > 1e-12 {
			let inv_d = 1.0 / d; //          let |inv_d| be 1 / |d|.
			let t1 = (0.0 - a) * inv_d; //   let |t1| be (|bmin|[i] - |a|) * |inv_d|.
			let t2 = (1.0 - a) * inv_d; //   let |t2| be (|bmax|[i] - |a|) * |inv_d|.
			tnear = tnear.max(t1.min(t2)); // let |tnear| be max(min(|t1|, |t2|), |tnear|).
			tfar = tfar.min(t1.max(t2)); //   let |tfar| be min(max(|t1|, |t2|), |tfar|).
		}
		// else if (|a| < |bmin|[i] or |a| > |bmax|[i]): return INTERSECTION NOT FOUND.
		else if !(0.0..=1.0).contains(&a) {
			return None;
		}
	}

	// 6. if (|tnear| > |tfar| or |tfar| < 0): return INTERSECTION NOT FOUND.
	if tnear > tfar || tfar < 0.0 {
		return None;
	}

	// 7. if |tnear| < 0: let |tnear| be |tfar|.
	//    (Favoring the first intersection in the direction |start| -> |end|.)
	if tnear < 0.0 {
		tnear = tfar;
	}

	// 8. if |tnear| is infinite: return INTERSECTION NOT FOUND.
	//    (Corrected per https://github.com/w3c/csswg-drafts/pull/13416 — checking
	//    for infinite rather than an arbitrary threshold.)
	if !tnear.is_finite() {
		return None;
	}

	// 9. for (i = 0; i < 3; i++): let |result|[i] be |start|[i] + |direction|[i] * |tnear|.
	// 10. return |result|.
	Some([start[0] + direction[0] * tnear, start[1] + direction[1] * tnear, start[2] + direction[2] * tnear])
}

const GAMUT_EPSILON: f64 = 1e-6;

/// Helper: checks an f64 is in [0.0, 1.0] with [`GAMUT_EPSILON`] tolerance.
fn in_unit(v: f64) -> bool {
	(-GAMUT_EPSILON..=1.0 + GAMUT_EPSILON).contains(&v)
}

/// Helper: checks an f64 is in [0.0, 100.0]
fn in_percent(v: f64) -> bool {
	(0.0..=100.0).contains(&v)
}

macro_rules! impl_gamut_rgb_f64 {
	($ty:ident) => {
		impl Gamut for $ty {
			fn in_gamut(&self) -> bool {
				in_unit(self.red) && in_unit(self.green) && in_unit(self.blue)
			}

			fn clamp_to_gamut(&self) -> Self {
				Self::new(self.red.clamp(0.0, 1.0), self.green.clamp(0.0, 1.0), self.blue.clamp(0.0, 1.0), self.alpha)
			}

			fn map_to_gamut(self) -> Self {
				self.raytrace_map_to_gamut()
			}
		}
	};
}

impl_gamut_rgb_f64!(Srgb);
impl_gamut_rgb_f64!(LinearRgb);
impl_gamut_rgb_f64!(A98Rgb);
impl_gamut_rgb_f64!(DisplayP3);
impl_gamut_rgb_f64!(ProphotoRgb);
impl_gamut_rgb_f64!(Rec2020);

impl Gamut for Hex {
	fn in_gamut(&self) -> bool {
		true
	}

	fn clamp_to_gamut(&self) -> Self {
		*self
	}

	fn map_to_gamut(self) -> Self {
		self.clamp_to_gamut()
	}
}

impl Gamut for Lab {
	fn in_gamut(&self) -> bool {
		(0.0..=100.0).contains(&self.lightness)
			&& (-125.0..=125.0).contains(&self.a)
			&& (-125.0..=125.0).contains(&self.b)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(
			self.lightness.clamp(0.0, 100.0),
			self.a.clamp(-125.0, 125.0),
			self.b.clamp(-125.0, 125.0),
			self.alpha,
		)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(XyzD65::from(XyzD50::from(self)));
		if rgb.in_gamut() {
			return self;
		}
		let oklch = Oklch::from(Oklab::from(XyzD65::from(XyzD50::from(self))));
		let mapped_rgb = raytrace_to_linear_rgb(oklch);
		Lab::from(XyzD50::from(XyzD65::from(mapped_rgb))).clamp_to_gamut()
	}
}

impl Gamut for Oklab {
	fn in_gamut(&self) -> bool {
		(0.0..=1.0).contains(&self.lightness) && (-0.4..=0.4).contains(&self.a) && (-0.4..=0.4).contains(&self.b)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.lightness.clamp(0.0, 1.0), self.a.clamp(-0.4, 0.4), self.b.clamp(-0.4, 0.4), self.alpha)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(XyzD65::from(self));
		if rgb.in_gamut() {
			return self;
		}
		let oklch = Oklch::from(self);
		let mapped_rgb = raytrace_to_linear_rgb(oklch);
		Oklab::from(XyzD65::from(mapped_rgb)).clamp_to_gamut()
	}
}

impl Gamut for Lch {
	fn in_gamut(&self) -> bool {
		(0.0..=100.0).contains(&self.lightness) && (0.0..=150.0).contains(&self.chroma)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.lightness.clamp(0.0, 100.0), self.chroma.clamp(0.0, 150.0), self.hue, self.alpha)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(XyzD65::from(XyzD50::from(Lab::from(self))));
		if rgb.in_gamut() {
			return self;
		}
		let oklch = Oklch::from(Oklab::from(XyzD65::from(XyzD50::from(Lab::from(self)))));
		let mapped_rgb = raytrace_to_linear_rgb(oklch);
		Lch::from(Lab::from(XyzD50::from(XyzD65::from(mapped_rgb)))).clamp_to_gamut()
	}
}

impl Gamut for Oklch {
	fn in_gamut(&self) -> bool {
		(0.0..=1.0).contains(&self.lightness) && (0.0..=0.4).contains(&self.chroma)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.lightness.clamp(0.0, 1.0), self.chroma.clamp(0.0, 0.4), self.hue, self.alpha)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(XyzD65::from(Oklab::from(self)));
		if rgb.in_gamut() {
			return self;
		}
		let mapped_rgb = raytrace_to_linear_rgb(self);
		Oklch::from(Oklab::from(XyzD65::from(mapped_rgb))).clamp_to_gamut()
	}
}

impl Gamut for Hsl {
	fn in_gamut(&self) -> bool {
		in_percent(self.saturation) && in_percent(self.lightness)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.hue, self.saturation.clamp(0.0, 100.0), self.lightness.clamp(0.0, 100.0), self.alpha)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(self);
		if rgb.in_gamut() {
			return self;
		}
		let oklch = Oklch::from(Oklab::from(XyzD65::from(rgb)));
		let mapped_rgb = raytrace_to_linear_rgb(oklch);
		Hsl::from(mapped_rgb).clamp_to_gamut()
	}
}

impl Gamut for Hwb {
	fn in_gamut(&self) -> bool {
		in_percent(self.whiteness) && in_percent(self.blackness)
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.hue, self.whiteness.clamp(0.0, 100.0), self.blackness.clamp(0.0, 100.0), self.alpha)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(self);
		if rgb.in_gamut() {
			return self;
		}
		let oklch = Oklch::from(Oklab::from(XyzD65::from(rgb)));
		let mapped_rgb = raytrace_to_linear_rgb(oklch);
		Hwb::from(mapped_rgb).clamp_to_gamut()
	}
}

// Hsv already clamps in its constructor, so it's always in gamut.
impl Gamut for Hsv {
	fn in_gamut(&self) -> bool {
		true
	}

	fn clamp_to_gamut(&self) -> Self {
		*self
	}

	fn map_to_gamut(self) -> Self {
		self.clamp_to_gamut()
	}
}

impl Gamut for XyzD50 {
	fn in_gamut(&self) -> bool {
		self.x >= 0.0 && self.y >= 0.0 && self.z >= 0.0 && self.y <= 100.0
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.x.max(0.0), self.y.clamp(0.0, 100.0), self.z.max(0.0), self.alpha)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(XyzD65::from(self));
		if rgb.in_gamut() {
			return self;
		}
		let oklch = Oklch::from(Oklab::from(XyzD65::from(self)));
		let mapped_rgb = raytrace_to_linear_rgb(oklch);
		XyzD50::from(XyzD65::from(mapped_rgb)).clamp_to_gamut()
	}
}

impl Gamut for XyzD65 {
	fn in_gamut(&self) -> bool {
		self.x >= 0.0 && self.y >= 0.0 && self.z >= 0.0 && self.y <= 100.0
	}

	fn clamp_to_gamut(&self) -> Self {
		Self::new(self.x.max(0.0), self.y.clamp(0.0, 100.0), self.z.max(0.0), self.alpha)
	}

	fn map_to_gamut(self) -> Self {
		let rgb = LinearRgb::from(self);
		if rgb.in_gamut() {
			return self;
		}
		let oklch = Oklch::from(Oklab::from(self));
		let mapped_rgb = raytrace_to_linear_rgb(oklch);
		XyzD65::from(mapped_rgb).clamp_to_gamut()
	}
}

impl Color {
	/// Returns the [`ColorSpace`] of this colour, if it maps to a bounded RGB gamut.
	///
	/// Perceptual and CIE spaces (`Lab`, `Lch`, `Oklab`, `Oklch`, `XyzD50`, `XyzD65`) return `None` — they can represent
	/// colours outside any single RGB gamut.
	pub fn color_space(&self) -> Option<ColorSpace> {
		match self {
			Color::Srgb(_)
			| Color::Hex(_)
			| Color::Named(_)
			| Color::Hsl(_)
			| Color::Hwb(_)
			| Color::Hsv(_)
			| Color::LinearRgb(_) => Some(ColorSpace::Srgb),
			Color::DisplayP3(_) => Some(ColorSpace::DisplayP3),
			Color::A98Rgb(_) => Some(ColorSpace::A98Rgb),
			Color::ProphotoRgb(_) => Some(ColorSpace::ProphotoRgb),
			Color::Rec2020(_) => Some(ColorSpace::Rec2020),
			Color::Lab(_) | Color::Lch(_) | Color::Oklab(_) | Color::Oklch(_) | Color::XyzD50(_) | Color::XyzD65(_) => {
				None
			}
		}
	}

	/// Returns `true` if this colour can be represented in `space` without clamping.
	///
	/// If the colour's own space is a subset of `space` and the colour is in gamut of its own space, this returns `true`
	/// without conversion.  Otherwise the colour is converted to the target space (via `XyzD65`) and the RGB channels are
	/// checked against `[0,1]`.
	pub fn in_gamut_of(&self, space: ColorSpace) -> bool {
		if let Some(src) = self.color_space()
			&& space.contains(src)
			&& self.in_gamut()
		{
			return true;
		}
		match space {
			ColorSpace::Srgb => LinearRgb::from(XyzD65::from(*self)).in_gamut(),
			ColorSpace::DisplayP3 => DisplayP3::from(XyzD65::from(*self)).in_gamut(),
			ColorSpace::A98Rgb => A98Rgb::from(XyzD65::from(*self)).in_gamut(),
			ColorSpace::ProphotoRgb => ProphotoRgb::from(XyzD65::from(*self)).in_gamut(),
			ColorSpace::Rec2020 => Rec2020::from(XyzD65::from(*self)).in_gamut(),
		}
	}
}

impl Gamut for Color {
	fn in_gamut(&self) -> bool {
		match self {
			Color::A98Rgb(c) => c.in_gamut(),
			Color::DisplayP3(c) => c.in_gamut(),
			Color::Hex(c) => c.in_gamut(),
			Color::Hsv(c) => c.in_gamut(),
			Color::Hsl(c) => c.in_gamut(),
			Color::Hwb(c) => c.in_gamut(),
			Color::Lab(c) => c.in_gamut(),
			Color::Lch(c) => c.in_gamut(),
			Color::LinearRgb(c) => c.in_gamut(),
			Color::Named(_) => true,
			Color::Oklab(c) => c.in_gamut(),
			Color::Oklch(c) => c.in_gamut(),
			Color::ProphotoRgb(c) => c.in_gamut(),
			Color::Rec2020(c) => c.in_gamut(),
			Color::Srgb(c) => c.in_gamut(),
			Color::XyzD50(c) => c.in_gamut(),
			Color::XyzD65(c) => c.in_gamut(),
		}
	}

	fn clamp_to_gamut(&self) -> Self {
		match self {
			Color::A98Rgb(c) => Color::A98Rgb(c.clamp_to_gamut()),
			Color::DisplayP3(c) => Color::DisplayP3(c.clamp_to_gamut()),
			Color::Hex(c) => Color::Hex(c.clamp_to_gamut()),
			Color::Hsv(c) => Color::Hsv(c.clamp_to_gamut()),
			Color::Hsl(c) => Color::Hsl(c.clamp_to_gamut()),
			Color::Hwb(c) => Color::Hwb(c.clamp_to_gamut()),
			Color::Lab(c) => Color::Lab(c.clamp_to_gamut()),
			Color::Lch(c) => Color::Lch(c.clamp_to_gamut()),
			Color::LinearRgb(c) => Color::LinearRgb(c.clamp_to_gamut()),
			Color::Named(n) => Color::Named(*n),
			Color::Oklab(c) => Color::Oklab(c.clamp_to_gamut()),
			Color::Oklch(c) => Color::Oklch(c.clamp_to_gamut()),
			Color::ProphotoRgb(c) => Color::ProphotoRgb(c.clamp_to_gamut()),
			Color::Rec2020(c) => Color::Rec2020(c.clamp_to_gamut()),
			Color::Srgb(c) => Color::Srgb(c.clamp_to_gamut()),
			Color::XyzD50(c) => Color::XyzD50(c.clamp_to_gamut()),
			Color::XyzD65(c) => Color::XyzD65(c.clamp_to_gamut()),
		}
	}

	fn map_to_gamut(self) -> Self {
		match self {
			Color::A98Rgb(c) => Color::A98Rgb(c.map_to_gamut()),
			Color::DisplayP3(c) => Color::DisplayP3(c.map_to_gamut()),
			Color::Hex(c) => Color::Hex(c.map_to_gamut()),
			Color::Hsv(c) => Color::Hsv(c.map_to_gamut()),
			Color::Hsl(c) => Color::Hsl(c.map_to_gamut()),
			Color::Hwb(c) => Color::Hwb(c.map_to_gamut()),
			Color::Lab(c) => Color::Lab(c.map_to_gamut()),
			Color::Lch(c) => Color::Lch(c.map_to_gamut()),
			Color::LinearRgb(c) => Color::LinearRgb(c.map_to_gamut()),
			Color::Named(n) => Color::Named(n),
			Color::Oklab(c) => Color::Oklab(c.map_to_gamut()),
			Color::Oklch(c) => Color::Oklch(c.map_to_gamut()),
			Color::ProphotoRgb(c) => Color::ProphotoRgb(c.map_to_gamut()),
			Color::Rec2020(c) => Color::Rec2020(c.map_to_gamut()),
			Color::Srgb(c) => Color::Srgb(c.map_to_gamut()),
			Color::XyzD50(c) => Color::XyzD50(c.map_to_gamut()),
			Color::XyzD65(c) => Color::XyzD65(c.map_to_gamut()),
		}
	}
}
