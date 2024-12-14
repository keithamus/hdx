use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{
	diagnostics, keyword_typedef, Build, CursorSink, Is, Parse, Parser, Result as ParserResult, ToCursors, T,
};

use crate::css::{
	types::Position,
	units::{Angle, Length, LengthPercentage},
};

use super::Color;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(To, atom!("to"));
	custom_keyword!(At, atom!("at"));
	custom_keyword!(ClosestCorner, atom!("closest-corner"));
	custom_keyword!(ClosestSide, atom!("closest-side"));
	custom_keyword!(FarthestCorner, atom!("farthest-corner"));
	custom_keyword!(FarthestSide, atom!("farthest-side"));
}

mod func {
	use hdx_parser::custom_function;
	custom_function!(LinearGradient, atom!("linear-gradient"));
	custom_function!(RadialGradient, atom!("radial-gradient"));
	custom_function!(RepeatingLinearGradient, atom!("repeating-linear-gradient"));
	custom_function!(RepeatingRadialGradient, atom!("repeating-radial-gradient"));
}

// https://drafts.csswg.org/css-images-3/#typedef-gradient
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Gradient<'a> {
	Linear(func::LinearGradient, Option<LinearDirection>, Option<T![,]>, Vec<'a, ColorStopOrHint>, Option<T![')']>),
	RepeatingLinear(
		func::RepeatingLinearGradient,
		Option<LinearDirection>,
		Option<T![,]>,
		Vec<'a, ColorStopOrHint>,
		Option<T![')']>,
	),
	Radial(
		func::RadialGradient,
		Option<RadialSize>,
		Option<RadialShape>,
		Option<kw::At>,
		Option<Position>,
		Option<T![,]>,
		Vec<'a, ColorStopOrHint>,
		Option<T![')']>,
	),
	RepeatingRadial(
		func::RepeatingRadialGradient,
		Option<RadialSize>,
		Option<RadialShape>,
		Option<kw::At>,
		Option<Position>,
		Option<T![,]>,
		Vec<'a, ColorStopOrHint>,
		Option<T![')']>,
	),
}

impl<'a> Gradient<'a> {
	fn parse_stops(p: &mut Parser<'a>) -> ParserResult<Vec<'a, ColorStopOrHint>> {
		let mut stops = Vec::new_in(p.bump());
		let mut allow_hint = false;
		loop {
			if allow_hint && p.peek::<LengthPercentage>() {
				let hint = p.parse::<LengthPercentage>()?;
				let comma = p.parse::<T![,]>()?;
				stops.push(ColorStopOrHint::Hint(hint, comma));
			}
			let color = p.parse::<Color>()?;
			let hint = if p.peek::<LengthPercentage>() {
				let hint = p.parse::<LengthPercentage>()?;
				allow_hint = true;
				Some(hint)
			} else {
				None
			};
			let comma = p.parse_if_peek::<T![,]>()?;
			stops.push(ColorStopOrHint::Stop(color, hint, comma));
			if comma.is_none() {
				break;
			}
		}
		Ok(stops)
	}
}

impl<'a> Is<'a> for Gradient<'a> {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::is(p, c)
			&& matches!(
				p.parse_atom_lower(c),
				atom!("linear-gradient")
					| atom!("radial-gradient")
					| atom!("repeating-linear-gradient")
					| atom!("repeating-radial-gradient")
			)
	}
}

impl<'a> Parse<'a> for Gradient<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<func::LinearGradient>() {
			let function = p.parse::<func::LinearGradient>()?;
			let dir = p.parse_if_peek::<LinearDirection>()?;
			let comma = if dir.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
			let stops = Self::parse_stops(p)?;
			Ok(Self::Linear(function, dir, comma, stops, p.parse_if_peek::<T![')']>()?))
		} else if p.peek::<func::RepeatingLinearGradient>() {
			let function = p.parse::<func::RepeatingLinearGradient>()?;
			let dir = p.parse_if_peek::<LinearDirection>()?;
			let comma = if dir.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
			let stops = Self::parse_stops(p)?;
			Ok(Self::RepeatingLinear(function, dir, comma, stops, p.parse_if_peek::<T![')']>()?))
		} else if p.peek::<func::RadialGradient>() {
			let function = p.parse::<func::RadialGradient>()?;
			let mut size = p.parse_if_peek::<RadialSize>()?;
			let shape = p.parse_if_peek::<RadialShape>()?;
			if size.is_none() && shape.is_none() {
				size = Some(p.parse::<RadialSize>()?);
			}
			let at = p.parse_if_peek::<kw::At>()?;
			let position = if at.is_some() { p.parse_if_peek::<Position>()? } else { None };
			let comma = if size.is_some() || shape.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
			let stops = Self::parse_stops(p)?;
			Ok(Self::Radial(function, size, shape, at, position, comma, stops, p.parse_if_peek::<T![')']>()?))
		} else {
			let function = p.parse::<func::RepeatingRadialGradient>()?;
			let mut size = p.parse_if_peek::<RadialSize>()?;
			let shape = p.parse_if_peek::<RadialShape>()?;
			if size.is_none() && shape.is_none() {
				size = Some(p.parse::<RadialSize>()?);
			}
			let at = p.parse_if_peek::<kw::At>()?;
			let position = if at.is_some() { p.parse_if_peek::<Position>()? } else { None };
			let comma = if size.is_some() || shape.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
			let stops = Self::parse_stops(p)?;
			Ok(Self::RepeatingRadial(function, size, shape, at, position, comma, stops, p.parse_if_peek::<T![')']>()?))
		}
	}
}

impl<'a> ToCursors for Gradient<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Linear(func, direction, comma, stops, close) => {
				s.append(func.into());
				if let Some(direction) = direction {
					ToCursors::to_cursors(direction, s);
				}
				if let Some(comma) = comma {
					s.append(comma.into());
				}
				for stop in stops {
					ToCursors::to_cursors(stop, s);
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::RepeatingLinear(func, direction, comma, stops, close) => {
				s.append(func.into());
				if let Some(direction) = direction {
					ToCursors::to_cursors(direction, s);
				}
				if let Some(comma) = comma {
					s.append(comma.into());
				}
				for stop in stops {
					ToCursors::to_cursors(stop, s);
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::Radial(func, size, shape, at, position, comma, stops, close) => {
				s.append(func.into());
				if let Some(size) = size {
					ToCursors::to_cursors(size, s);
				}
				if let Some(shape) = shape {
					ToCursors::to_cursors(shape, s);
				}
				if let Some(at) = at {
					s.append(at.into());
				}
				if let Some(position) = position {
					ToCursors::to_cursors(position, s);
				}
				if let Some(comma) = comma {
					s.append(comma.into());
				}
				for stop in stops {
					ToCursors::to_cursors(stop, s);
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
			Self::RepeatingRadial(func, size, shape, at, position, comma, stops, close) => {
				s.append(func.into());
				if let Some(size) = size {
					ToCursors::to_cursors(size, s);
				}
				if let Some(shape) = shape {
					ToCursors::to_cursors(shape, s);
				}
				if let Some(at) = at {
					s.append(at.into());
				}
				if let Some(position) = position {
					ToCursors::to_cursors(position, s);
				}
				if let Some(comma) = comma {
					s.append(comma.into());
				}
				for stop in stops {
					ToCursors::to_cursors(stop, s);
				}
				if let Some(close) = close {
					s.append(close.into());
				}
			}
		}
	}
}

keyword_typedef!(NamedDirection {
	Bottom: atom!("bottom"),
	Top: atom!("top"),
	Left: atom!("left"),
	Right: atom!("right"),
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LinearDirection {
	Angle(Angle),
	Named(kw::To, NamedDirection, Option<NamedDirection>),
}

impl<'a> Is<'a> for LinearDirection {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		Angle::is(p, c) || kw::To::is(p, c)
	}
}

impl<'a> Parse<'a> for LinearDirection {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Angle>() {
			p.parse::<Angle>().map(Self::Angle)
		} else {
			let to = p.parse::<kw::To>()?;
			let first = p.parse::<NamedDirection>()?;
			let second = p.parse_if_peek::<NamedDirection>()?;
			Ok(Self::Named(to, first, second))
		}
	}
}

impl<'a> ToCursors for LinearDirection {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Angle(c) => s.append(c.into()),
			Self::Named(to, a, b) => {
				s.append(to.into());
				s.append(a.into());
				if let Some(b) = b {
					s.append(b.into());
				}
			}
		}
	}
}

// https://drafts.csswg.org/css-images-3/#typedef-rg-size
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum RadialSize {
	ClosestCorner(kw::ClosestCorner),
	ClosestSide(kw::ClosestSide),
	FarthestCorner(kw::FarthestCorner),
	FarthestSide(kw::FarthestSide),
	Circular(Length),
	Elliptical(LengthPercentage, LengthPercentage),
}

impl<'a> Is<'a> for RadialSize {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		LengthPercentage::is(p, c)
			|| (<T![Ident]>::is(p, c)
				&& matches!(
					p.parse_atom_lower(c),
					atom!("closest-corner") | atom!("closest-side") | atom!("farthest-corner") | atom!("farthest-side")
				))
	}
}

impl<'a> Parse<'a> for RadialSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Ident]>() {
			let c = p.next();
			return Ok(match p.parse_atom_lower(c) {
				atom!("closest-corner") => RadialSize::ClosestCorner(kw::ClosestCorner::build(p, c)),
				atom!("closest-side") => RadialSize::ClosestSide(kw::ClosestSide::build(p, c)),
				atom!("farthest-corner") => RadialSize::FarthestCorner(kw::FarthestCorner::build(p, c)),
				atom!("farthest-side") => RadialSize::FarthestSide(kw::FarthestSide::build(p, c)),
				atom => Err(diagnostics::UnexpectedIdent(atom, c.span()))?,
			});
		}
		if p.peek::<T![Number]>() {
			let first_len = p.parse::<LengthPercentage>()?;
			if !p.peek::<T![Number]>() {
				return p.parse::<Length>().map(Self::Circular);
			}
			let second_len = p.parse::<LengthPercentage>()?;
			return Ok(Self::Elliptical(first_len, second_len));
		}
		let first = p.parse::<LengthPercentage>()?;
		let second = p.parse::<LengthPercentage>()?;
		Ok(Self::Elliptical(first, second))
	}
}

impl<'a> ToCursors for RadialSize {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::ClosestCorner(c) => s.append(c.into()),
			Self::ClosestSide(c) => s.append(c.into()),
			Self::FarthestCorner(c) => s.append(c.into()),
			Self::FarthestSide(c) => s.append(c.into()),
			Self::Circular(c) => s.append(c.into()),
			Self::Elliptical(a, b) => {
				s.append(a.into());
				s.append(b.into());
			}
		}
	}
}

// https://drafts.csswg.org/css-images-3/#typedef-radial-shape
keyword_typedef!(RadialShape { Circle: atom!("circle"), Ellipse: atom!("ellipse") });

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorStopOrHint {
	Stop(Color, Option<LengthPercentage>, Option<T![,]>),
	Hint(LengthPercentage, T![,]),
}

impl<'a> ToCursors for ColorStopOrHint {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Stop(c, l, comma) => {
				ToCursors::to_cursors(c, s);
				if let Some(l) = l {
					s.append(l.into());
				}
				if let Some(comma) = comma {
					s.append(comma.into());
				}
			}
			Self::Hint(l, comma) => {
				s.append(l.into());
				s.append(comma.into());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Gradient, 208);
		assert_size!(LinearDirection, 44);
		assert_size!(RadialSize, 32);
		assert_size!(ColorStopOrHint, 192);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Gradient, "linear-gradient(to bottom,yellow,blue)");
		assert_parse!(Gradient, "linear-gradient(yellow,blue)");
		assert_parse!(Gradient, "linear-gradient(to bottom,#fff,#fff 85%,#e6e6e6)");
		assert_parse!(Gradient, "linear-gradient(45deg,#808080 25%,transparent 25%)");
		assert_parse!(Gradient, "linear-gradient(to right,transparent,red 20%,red 80%,transparent)");
		assert_parse!(Gradient, "radial-gradient(closest-corner circle,rgba(1,65,255,0.4),rgba(1,65,255,0))");
	}
}
