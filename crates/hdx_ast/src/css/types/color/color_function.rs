use crate::css::units::Angle;
use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, Build, Is, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(None, atom!("none"));
}

mod func {
	use hdx_parser::custom_function;
	custom_function!(Color, atom!("color"));
	custom_function!(Rgb, atom!("rgb"));
	custom_function!(Rgba, atom!("rgba"));
	custom_function!(Hsl, atom!("hsl"));
	custom_function!(Hsla, atom!("hsla"));
	custom_function!(Hwb, atom!("hwb"));
	custom_function!(Lab, atom!("lab"));
	custom_function!(Lch, atom!("lch"));
	custom_function!(Oklab, atom!("oklab"));
	custom_function!(Oklch, atom!("oklch"));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Hue {
	None(kw::None),
	Number(T![Number]),
	Angle(Angle),
}

impl<'a> Is<'a> for Hue {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		<T![Number]>::is(p, c) || Angle::is(p, c) || <kw::None>::is(p, c)
	}
}

impl<'a> Build<'a> for Hue {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		if <T![Number]>::is(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else if <kw::None>::is(p, c) {
			Self::None(<kw::None>::build(p, c))
		} else {
			Self::Angle(Angle::build(p, c))
		}
	}
}
impl From<Hue> for Cursor {
	fn from(value: Hue) -> Self {
		match value {
			Hue::None(c) => c.into(),
			Hue::Number(c) => c.into(),
			Hue::Angle(c) => c.into(),
		}
	}
}
impl From<&Hue> for Cursor {
	fn from(value: &Hue) -> Self {
		(*value).into()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Channel {
	None(kw::None),
	Number(T![Number]),
	Percent(T![Dimension::%]),
}

impl<'a> Is<'a> for Channel {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		<T![Number]>::is(p, c) || <T![Dimension::%]>::is(p, c) || <kw::None>::is(p, c)
	}
}

impl<'a> Build<'a> for Channel {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		if <T![Number]>::is(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else if <kw::None>::is(p, c) {
			Self::None(<kw::None>::build(p, c))
		} else {
			Self::Percent(<T![Dimension::%]>::build(p, c))
		}
	}
}

impl From<Channel> for Cursor {
	fn from(value: Channel) -> Self {
		match value {
			Channel::None(c) => c.into(),
			Channel::Number(c) => c.into(),
			Channel::Percent(c) => c.into(),
		}
	}
}

impl From<&Channel> for Cursor {
	fn from(value: &Channel) -> Self {
		(*value).into()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorSpace {
	Srgb(T![Ident]),
	SrgbLinear(T![Ident]),
	DisplayP3(T![Ident]),
	A98Rgb(T![Ident]),
	ProphotoRgb(T![Ident]),
	Rec2020(T![Ident]),
	Xyz(T![Ident]),
	XyzD50(T![Ident]),
	XyzD65(T![Ident]),
}

impl<'a> Is<'a> for ColorSpace {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		<T![Ident]>::is(p, c)
			&& matches!(
				p.parse_atom_lower(c),
				atom!("srgb")
					| atom!("srgb-linear")
					| atom!("display-p3")
					| atom!("a98-rgb")
					| atom!("prophoto-rgb")
					| atom!("rec2020")
					| atom!("xyz") | atom!("xyz-d50")
					| atom!("xyz-d65")
			)
	}
}

impl<'a> Build<'a> for ColorSpace {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		let ident = <T![Ident]>::build(p, c);
		match p.parse_atom_lower(c) {
			atom!("srgb") => Self::Srgb(ident),
			atom!("srgb-linear") => Self::SrgbLinear(ident),
			atom!("display-p3") => Self::DisplayP3(ident),
			atom!("a98-rgb") => Self::A98Rgb(ident),
			atom!("prophoto-rgb") => Self::ProphotoRgb(ident),
			atom!("rec2020") => Self::Rec2020(ident),
			atom!("xyz") => Self::Xyz(ident),
			atom!("xyz-d50") => Self::XyzD50(ident),
			atom!("xyz-d65") => Self::XyzD65(ident),
			_ => unreachable!(),
		}
	}
}

impl From<ColorSpace> for Cursor {
	fn from(value: ColorSpace) -> Self {
		match value {
			ColorSpace::Srgb(c) => c.into(),
			ColorSpace::SrgbLinear(c) => c.into(),
			ColorSpace::DisplayP3(c) => c.into(),
			ColorSpace::A98Rgb(c) => c.into(),
			ColorSpace::ProphotoRgb(c) => c.into(),
			ColorSpace::Rec2020(c) => c.into(),
			ColorSpace::Xyz(c) => c.into(),
			ColorSpace::XyzD50(c) => c.into(),
			ColorSpace::XyzD65(c) => c.into(),
		}
	}
}

impl From<&ColorSpace> for Cursor {
	fn from(value: &ColorSpace) -> Self {
		(*value).into()
	}
}

// https://drafts.csswg.org/css-color/#typedef-color-function
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorFunction {
	// https://drafts.csswg.org/css-color/#funcdef-color
	// color() = color( <colorspace-params> [ / [ <alpha-value> | none ] ]? )
	// <colorspace-params> = [ <predefined-rgb-params> | <xyz-params>]
	// <predefined-rgb-params> = <predefined-rgb> [ <number> | <percentage> | none ]{3}
	// <predefined-rgb> = srgb | srgb-linear | display-p3 | a98-rgb | prophoto-rgb | rec2020
	// <xyz-params> = <xyz-space> [ <number> | <percentage> | none ]{3}
	// <xyz-space> = xyz | xyz-d50 | xyz-d65
	Color(func::Color, ColorSpace, Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-rgb
	// rgb() = [ <legacy-rgb-syntax> | <modern-rgb-syntax> ]
	// rgba() = [ <legacy-rgba-syntax> | <modern-rgba-syntax> ]
	// <legacy-rgb-syntax> =   rgb( <percentage>#{3} , <alpha-value>? ) |
	//                   rgb( <number>#{3} , <alpha-value>? )
	// <legacy-rgba-syntax> = rgba( <percentage>#{3} , <alpha-value>? ) |
	//                   rgba( <number>#{3} , <alpha-value>? )
	// <modern-rgb-syntax> = rgb(
	//   [ <number> | <percentage> | none]{3}
	//   [ / [<alpha-value> | none] ]?  )
	// <modern-rgba-syntax> = rgba(
	//   [ <number> | <percentage> | none]{3}
	//   [ / [<alpha-value> | none] ]?  )
	Rgb(
		func::Rgb,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),
	Rgba(
		func::Rgba,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),

	// https://drafts.csswg.org/css-color/#funcdef-hsl
	// hsl() = [ <legacy-hsl-syntax> | <modern-hsl-syntax> ]
	// hsla() = [ <legacy-hsla-syntax> | <modern-hsla-syntax> ]
	// <modern-hsl-syntax> = hsl(
	//     [<hue> | none]
	//     [<percentage> | <number> | none]
	//     [<percentage> | <number> | none]
	//     [ / [<alpha-value> | none] ]? )
	// <modern-hsla-syntax> = hsla(
	//     [<hue> | none]
	//     [<percentage> | <number> | none]
	//     [<percentage> | <number> | none]
	//     [ / [<alpha-value> | none] ]? )
	// <legacy-hsl-syntax> = hsl( <hue>, <percentage>, <percentage>, <alpha-value>? )
	// <legacy-hsla-syntax> = hsla( <hue>, <percentage>, <percentage>, <alpha-value>? )
	Hsl(
		func::Hsl,
		Hue,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),
	Hsla(
		func::Hsla,
		Hue,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),

	// https://drafts.csswg.org/css-color/#funcdef-hwb
	// hwb() = hwb(
	//  [<hue> | none]
	//  [<percentage> | <number> | none]
	//  [<percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Hwb(func::Hwb, Hue, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-lab
	// lab() = lab( [<percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Lab(func::Lab, Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-lch
	// lch() = lch( [<percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <hue> | none]
	//  [ / [<alpha-value> | none] ]? )
	Lch(func::Lch, Channel, Channel, Hue, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-oklab
	// oklab() = oklab( [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Oklab(func::Oklab, Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-oklch
	// oklch() = oklch( [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <hue> | none]
	//  [ / [<alpha-value> | none] ]? )
	Oklch(func::Oklch, Channel, Channel, Hue, Option<T![/]>, Option<Channel>, Option<T![')']>),
}

impl<'a> Is<'a> for ColorFunction {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::is(p, c)
			&& matches!(
				p.parse_atom_lower(c),
				atom!("color")
					| atom!("rgb") | atom!("rgba")
					| atom!("hsl") | atom!("hsla")
					| atom!("hwb") | atom!("lab")
					| atom!("lch") | atom!("oklab")
					| atom!("oklch")
			)
	}
}

impl<'a> ColorFunction {
	fn parse_rgb(
		p: &mut Parser<'a>,
	) -> ParserResult<(
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
	)> {
		let a = p.parse::<Channel>()?;
		let b = p.parse_if_peek::<T![,]>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![,]>()?;
		let e = p.parse::<Channel>()?;
		let f = p.parse_if_peek::<T![,]>()?;
		let g = p.parse_if_peek::<T![/]>()?;
		let h = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e, f, g, h))
	}

	fn parse_hsl(
		p: &mut Parser<'a>,
	) -> ParserResult<(
		Hue,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
	)> {
		let a = p.parse::<Hue>()?;
		let b = p.parse_if_peek::<T![,]>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![,]>()?;
		let e = p.parse::<Channel>()?;
		let f = p.parse_if_peek::<T![,]>()?;
		let g = p.parse_if_peek::<T![/]>()?;
		let h = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e, f, g, h))
	}

	fn parse_hwb(p: &mut Parser<'a>) -> ParserResult<(Hue, Channel, Channel, Option<T![/]>, Option<Channel>)> {
		let a = p.parse::<Hue>()?;
		let b = p.parse::<Channel>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![/]>()?;
		let e = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e))
	}

	fn parse_lch(p: &mut Parser<'a>) -> ParserResult<(Channel, Channel, Hue, Option<T![/]>, Option<Channel>)> {
		let a = p.parse::<Channel>()?;
		let b = p.parse::<Channel>()?;
		let c = p.parse::<Hue>()?;
		let d = p.parse_if_peek::<T![/]>()?;
		let e = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e))
	}

	fn parse_three_channel(
		p: &mut Parser<'a>,
	) -> ParserResult<(Channel, Channel, Channel, Option<T![/]>, Option<Channel>)> {
		let a = p.parse::<Channel>()?;
		let b = p.parse::<Channel>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![/]>()?;
		let e = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e))
	}
}

impl<'a> Parse<'a> for ColorFunction {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if !Self::peek(p) {
			let c = p.next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		let cursor = p.next();
		match p.parse_atom_lower(cursor) {
			atom!("color") => {
				let space = p.parse::<ColorSpace>()?;
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Color(func::Color::build(p, cursor), space, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			atom!("rgb") => {
				let (a, b, c, d, e, f, g, h) = Self::parse_rgb(p)?;
				Ok(Self::Rgb(func::Rgb::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			atom!("rgba") => {
				let (a, b, c, d, e, f, g, h) = Self::parse_rgb(p)?;
				Ok(Self::Rgba(func::Rgba::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			atom!("hsl") => {
				let (a, b, c, d, e, f, g, h) = Self::parse_hsl(p)?;
				Ok(Self::Hsl(func::Hsl::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			atom!("hsla") => {
				let (a, b, c, d, e, f, g, h) = Self::parse_hsl(p)?;
				Ok(Self::Hsla(func::Hsla::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			atom!("hwb") => {
				let (a, b, c, d, e) = Self::parse_hwb(p)?;
				Ok(Self::Hwb(func::Hwb::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			atom!("lab") => {
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Lab(func::Lab::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			atom!("lch") => {
				let (a, b, c, d, e) = Self::parse_lch(p)?;
				Ok(Self::Lch(func::Lch::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			atom!("oklab") => {
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Oklab(func::Oklab::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			atom!("oklch") => {
				let (a, b, c, d, e) = Self::parse_lch(p)?;
				Ok(Self::Oklch(func::Oklch::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			atom => Err(diagnostics::UnexpectedFunction(atom, cursor.into()))?,
		}
	}
}

impl<'a> ToCursors for ColorFunction {
	fn to_cursors(&self, s: &mut impl hdx_parser::CursorSink) {
		match self {
			Self::Color(function, space, ch1, ch2, ch3, slash, alpha, close) => {
				s.append(function.into());
				s.append(space.into());
				s.append(ch1.into());
				s.append(ch2.into());
				s.append(ch3.into());
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Rgb(function, r, c1, g, c2, b, c3, slash, alpha, close) => {
				s.append(function.into());
				s.append(r.into());
				if let Some(c1) = c1 {
					s.append(c1.into());
				}
				s.append(g.into());
				if let Some(c2) = c2 {
					s.append(c2.into());
				}
				s.append(b.into());
				if let Some(c3) = c3 {
					s.append(c3.into());
				}
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Rgba(function, r, c1, g, c2, b, c3, slash, alpha, close) => {
				s.append(function.into());
				s.append(r.into());
				if let Some(c1) = c1 {
					s.append(c1.into());
				}
				s.append(g.into());
				if let Some(c2) = c2 {
					s.append(c2.into());
				}
				s.append(b.into());
				if let Some(c3) = c3 {
					s.append(c3.into());
				}
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Hsl(function, h, c1, sat, c2, l, c3, slash, alpha, close) => {
				s.append(function.into());
				s.append(h.into());
				if let Some(c1) = c1 {
					s.append(c1.into());
				}
				s.append(sat.into());
				if let Some(c2) = c2 {
					s.append(c2.into());
				}
				s.append(l.into());
				if let Some(c3) = c3 {
					s.append(c3.into());
				}
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Hsla(function, h, c1, sat, c2, l, c3, slash, alpha, close) => {
				s.append(function.into());
				s.append(h.into());
				if let Some(c1) = c1 {
					s.append(c1.into());
				}
				s.append(sat.into());
				if let Some(c2) = c2 {
					s.append(c2.into());
				}
				s.append(l.into());
				if let Some(c3) = c3 {
					s.append(c3.into());
				}
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Hwb(function, h, w, b, slash, alpha, close) => {
				s.append(function.into());
				s.append(h.into());
				s.append(w.into());
				s.append(b.into());
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Lab(function, l, a, b, slash, alpha, close) => {
				s.append(function.into());
				s.append(l.into());
				s.append(a.into());
				s.append(b.into());
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Lch(function, l, c, h, slash, alpha, close) => {
				s.append(function.into());
				s.append(l.into());
				s.append(c.into());
				s.append(h.into());
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Oklab(function, l, a, b, slash, alpha, close) => {
				s.append(function.into());
				s.append(l.into());
				s.append(a.into());
				s.append(b.into());
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Oklch(function, l, c, h, slash, alpha, close) => {
				s.append(function.into());
				s.append(l.into());
				s.append(c.into());
				s.append(h.into());
				if let Some(slash) = slash {
					s.append(slash.into());
				}
				if let Some(alpha) = alpha {
					s.append(alpha.into());
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
		}
	}
}
