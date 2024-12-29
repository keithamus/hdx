use crate::units::Angle;
use css_lexer::Cursor;
use css_parse::{function_set, keyword_set, Build, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

function_set!(ColorFunctionName {
	Color: "color",
	Rgb: "rgb",
	Rgba: "rgba",
	Hsl: "hsl",
	Hsla: "hsla",
	Hwb: "hwb",
	Lab: "lab",
	Lch: "lch",
	Oklab: "oklab",
	Oklch: "oklch",
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Hue {
	None(T![Ident]),
	Number(T![Number]),
	Angle(Angle),
}

impl<'a> Peek<'a> for Hue {
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		<T![Number]>::peek(p, c) || Angle::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "none"))
	}
}

impl<'a> Build<'a> for Hue {
	fn build(p: &Parser<'a>, c: css_lexer::Cursor) -> Self {
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else if Angle::peek(p, c) {
			Self::Angle(Angle::build(p, c))
		} else {
			Self::None(<T![Ident]>::build(p, c))
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
	None(T![Ident]),
	Number(T![Number]),
	Percent(T![Dimension::%]),
}

impl<'a> Peek<'a> for Channel {
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		<T![Number]>::peek(p, c)
			|| <T![Dimension::%]>::peek(p, c)
			|| (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "none"))
	}
}

impl<'a> Build<'a> for Channel {
	fn build(p: &Parser<'a>, c: css_lexer::Cursor) -> Self {
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else if <T![Dimension::%]>::peek(p, c) {
			Self::Percent(<T![Dimension::%]>::build(p, c))
		} else {
			Self::None(<T![Ident]>::build(p, c))
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

keyword_set!(ColorSpace {
	Srgb: "srgb",
	SrgbLinear: "srgb-linear",
	DisplayP3: "display-p3",
	A98Rgb: "a98-rgb",
	ProphotoRgb: "prophoto-rgb",
	Rec2020: "rec2020",
	Xyz: "xyz",
	XyzD50: "xyz-d50",
	XyzD65: "xyz-d65",
});

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
	Color(T![Function], ColorSpace, Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

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
		T![Function],
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
		T![Function],
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
		T![Function],
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
		T![Function],
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
	Hwb(T![Function], Hue, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-lab
	// lab() = lab( [<percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Lab(T![Function], Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-lch
	// lch() = lch( [<percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <hue> | none]
	//  [ / [<alpha-value> | none] ]? )
	Lch(T![Function], Channel, Channel, Hue, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-oklab
	// oklab() = oklab( [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Oklab(T![Function], Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-oklch
	// oklch() = oklch( [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <hue> | none]
	//  [ / [<alpha-value> | none] ]? )
	Oklch(T![Function], Channel, Channel, Hue, Option<T![/]>, Option<Channel>, Option<T![')']>),
}

impl<'a> Peek<'a> for ColorFunction {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		ColorFunctionName::peek(p, c)
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
		let function = ColorFunctionName::parse(p)?;
		match function {
			ColorFunctionName::Color(cursor) => {
				let space = p.parse::<ColorSpace>()?;
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Color(<T![Function]>::build(p, cursor), space, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Rgb(cursor) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_rgb(p)?;
				Ok(Self::Rgb(<T![Function]>::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Rgba(cursor) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_rgb(p)?;
				Ok(Self::Rgba(<T![Function]>::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Hsl(cursor) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_hsl(p)?;
				Ok(Self::Hsl(<T![Function]>::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Hsla(cursor) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_hsl(p)?;
				Ok(Self::Hsla(<T![Function]>::build(p, cursor), a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Hwb(cursor) => {
				let (a, b, c, d, e) = Self::parse_hwb(p)?;
				Ok(Self::Hwb(<T![Function]>::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Lab(cursor) => {
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Lab(<T![Function]>::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Lch(cursor) => {
				let (a, b, c, d, e) = Self::parse_lch(p)?;
				Ok(Self::Lch(<T![Function]>::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Oklab(cursor) => {
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Oklab(<T![Function]>::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Oklch(cursor) => {
				let (a, b, c, d, e) = Self::parse_lch(p)?;
				Ok(Self::Oklch(<T![Function]>::build(p, cursor), a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
		}
	}
}

impl<'a> ToCursors for ColorFunction {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Channel>(), 16);
		assert_eq!(std::mem::size_of::<ColorFunction>(), 160);
	}
}
