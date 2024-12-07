use bitmask_enum::bitmask;
use hdx_atom::{atom, Atomizable};
use hdx_derive::{Atomizable, Writable};
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::{
	types::Position,
	units::{Angle, Length, LengthPercentage},
};

use super::Color;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(To, atom!("to"));
	custom_keyword!(At, atom!("at"));
}

mod func {
	use hdx_parser::custom_function;
	custom_function!(LinearGradient, atom!("linear-gradient"));
	custom_function!(RadialGradient, atom!("radial-gradient"));
	custom_function!(RepeatingLinearGradient, atom!("repeating-linear-gradient"));
	custom_function!(RepeatingRadialGradient, atom!("repeating-radial-gradient"));
}

// https://drafts.csswg.org/css-images-3/#typedef-gradient
#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Gradient {
	Linear(LinearDirection, SmallVec<[ColorStopOrHint; 0]>),
	RepeatingLinear(LinearDirection, SmallVec<[ColorStopOrHint; 0]>),
	Radial(RadialSize, RadialShape, Option<Position>, SmallVec<[ColorStopOrHint; 0]>),
	RepeatingRadial(RadialSize, RadialShape, Option<Position>, SmallVec<[ColorStopOrHint; 0]>),
}

impl<'a> Gradient {
	fn parse_stops(p: &mut Parser<'a>) -> ParserResult<SmallVec<[ColorStopOrHint; 0]>> {
		let mut stops = smallvec![];
		let mut allow_hint = false;
		loop {
			if allow_hint && p.peek::<LengthPercentage>().is_some() {
				let hint = p.parse::<LengthPercentage>()?;
				stops.push(ColorStopOrHint::Hint(hint));
				p.parse::<T![,]>()?;
			}
			let color = p.parse::<Color>()?;
			let hint = if p.peek::<LengthPercentage>().is_some() {
				let hint = p.parse::<LengthPercentage>()?;
				allow_hint = true;
				Some(hint)
			} else {
				None
			};
			stops.push(ColorStopOrHint::Stop(color, hint));
			if p.parse_if_peek::<T![,]>()?.is_none() {
				break;
			}
		}
		Ok(stops)
	}
}

impl<'a> Peek<'a> for Gradient {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<func::LinearGradient>()
			.or_else(|| p.peek::<func::RadialGradient>())
			.or_else(|| p.peek::<func::RepeatingLinearGradient>())
			.or_else(|| p.peek::<func::RepeatingRadialGradient>())
	}
}

impl<'a> Parse<'a> for Gradient {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let gradient = if let Some(token) = p.peek::<func::LinearGradient>() {
			p.hop(token);
			let dir = p
				.parse_if_peek::<LinearDirection>()
				.and_then(|f| {
					if f.is_some() {
						p.parse::<T![,]>()?;
					}
					Ok(f)
				})?
				.unwrap_or_default();
			Self::Linear(dir, Self::parse_stops(p)?)
		} else if let Some(token) = p.peek::<func::RepeatingLinearGradient>() {
			p.hop(token);
			let dir = p
				.parse_if_peek::<LinearDirection>()
				.and_then(|f| {
					if f.is_some() {
						p.parse::<T![Comma]>()?;
					}
					Ok(f)
				})?
				.unwrap_or_default();
			Self::RepeatingLinear(dir, Self::parse_stops(p)?)
		} else if let Some(token) = p.peek::<func::RadialGradient>() {
			p.hop(token);
			let mut size = p.parse_if_peek::<RadialSize>()?;
			let shape = p.parse_if_peek::<RadialShape>()?;
			if size.is_none() && shape.is_none() {
				size = Some(p.parse::<RadialSize>()?);
			}
			let position = if let Some(token) = p.peek::<kw::At>() {
				p.hop(token);
				Some(p.parse::<Position>()?)
			} else {
				None
			};
			if size.is_some() || shape.is_some() {
				p.parse::<T![Comma]>()?;
			}
			Self::Radial(size.unwrap_or_default(), shape.unwrap_or_default(), position, Self::parse_stops(p)?)
		} else {
			p.parse::<func::RepeatingRadialGradient>()?;
			let mut size = p.parse_if_peek::<RadialSize>()?;
			let shape = p.parse_if_peek::<RadialShape>()?;
			if size.is_none() && shape.is_none() {
				size = Some(p.parse::<RadialSize>()?);
			}
			let position = if let Some(token) = p.peek::<kw::At>() {
				p.hop(token);
				Some(p.parse::<Position>()?)
			} else {
				None
			};
			if size.is_some() || shape.is_some() {
				p.parse::<T![Comma]>()?;
			}
			Self::Radial(size.unwrap_or_default(), shape.unwrap_or_default(), position, Self::parse_stops(p)?)
		};
		p.parse::<T![RightParen]>()?;
		Ok(gradient)
	}
}

impl<'a> WriteCss<'a> for Gradient {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Linear(dir, hints) => {
				atom!("linear-gradient").write_css(sink)?;
				sink.write_char('(')?;
				if dir != &LinearDirection::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
					dir.write_css(sink)?;
					sink.write_char(',')?;
					sink.write_whitespace()?;
				}
				hints.write_css(sink)?;
				sink.write_char(')')
			}
			Self::RepeatingLinear(dir, hints) => {
				atom!("repeating-linear-gradient").write_css(sink)?;
				sink.write_char('(')?;
				if dir != &LinearDirection::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
					dir.write_css(sink)?;
					sink.write_char(',')?;
					sink.write_whitespace()?;
				}
				hints.write_css(sink)?;
				sink.write_char(')')
			}
			Self::Radial(size, shape, pos, hints) => {
				atom!("radial-gradient").write_css(sink)?;
				sink.write_char('(')?;
				let mut wrote = false;
				if size != &RadialSize::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
					size.write_css(sink)?;
					sink.write_char(' ')?;
					wrote = true;
				}
				if shape != &RadialShape::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
					shape.to_atom().write_css(sink)?;
					wrote = true;
				}
				if pos.is_some() {
					sink.write_char(' ')?;
					atom!("at").write_css(sink)?;
					sink.write_char(' ')?;
					pos.write_css(sink)?;
					wrote = true;
				}
				if wrote {
					sink.write_char(',')?;
					sink.write_whitespace()?;
				}
				hints.write_css(sink)?;
				sink.write_char(')')
			}
			Self::RepeatingRadial(size, shape, pos, hints) => {
				atom!("repeating-radial-gradient").write_css(sink)?;
				sink.write_char('(')?;
				let mut wrote = false;
				if size != &RadialSize::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
					size.write_css(sink)?;
					sink.write_char(' ')?;
					wrote = true;
				}
				if shape != &RadialShape::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
					shape.to_atom().write_css(sink)?;
					wrote = true;
				}
				if pos.is_some() {
					sink.write_char(' ')?;
					atom!("at").write_css(sink)?;
					sink.write_char(' ')?;
					pos.write_css(sink)?;
					wrote = true;
				}
				if wrote {
					sink.write_char(',')?;
					sink.write_whitespace()?;
				}
				hints.write_css(sink)?;
				sink.write_char(')')
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LinearDirection {
	Angle(Angle),
	Named(NamedDirection),
}

impl Default for LinearDirection {
	fn default() -> Self {
		Self::Named(NamedDirection::Bottom)
	}
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NamedDirection {
	Bottom,
	Top,
	Left,
	Right,
}

impl<'a> Peek<'a> for LinearDirection {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<Angle>().or_else(|| p.peek::<kw::To>())
	}
}

impl<'a> Parse<'a> for LinearDirection {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Angle>().is_some() {
			if let Ok(angle) = p.try_parse::<Angle>() {
				return Ok(Self::Angle(angle));
			}
		}
		p.parse::<kw::To>()?;
		let mut dir = NamedDirection::none();
		let token = p.parse::<T![Ident]>()?;
		dir |= match p.parse_atom_lower(*token) {
			atom!("top") => NamedDirection::Top,
			atom!("left") => NamedDirection::Left,
			atom!("right") => NamedDirection::Right,
			atom!("bottom") => NamedDirection::Bottom,
			atom => Err(diagnostics::UnexpectedIdent(atom, token.span()))?,
		};
		if let Some(token) = p.peek::<T![Ident]>() {
			p.hop(token);
			dir |= match p.parse_atom_lower(token) {
				atom @ atom!("top") => {
					if dir.contains(NamedDirection::Top) || dir.contains(NamedDirection::Bottom) {
						Err(diagnostics::UnexpectedIdent(atom, token.span()))?
					}
					NamedDirection::Top
				}
				atom @ atom!("left") => {
					if dir.contains(NamedDirection::Left) || dir.contains(NamedDirection::Right) {
						Err(diagnostics::UnexpectedIdent(atom, token.span()))?
					}
					NamedDirection::Left
				}
				atom @ atom!("right") => {
					if dir.contains(NamedDirection::Right) || dir.contains(NamedDirection::Left) {
						Err(diagnostics::UnexpectedIdent(atom, token.span()))?
					}
					NamedDirection::Right
				}
				atom @ atom!("bottom") => {
					if dir.contains(NamedDirection::Bottom) || dir.contains(NamedDirection::Top) {
						Err(diagnostics::UnexpectedIdent(atom, token.span()))?
					}
					NamedDirection::Bottom
				}
				atom => Err(diagnostics::UnexpectedIdent(atom, token.span()))?,
			};
		}
		Ok(Self::Named(dir))
	}
}

impl<'a> WriteCss<'a> for LinearDirection {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Angle(a) => a.write_css(sink),
			Self::Named(dir) => {
				atom!("to").write_css(sink)?;
				sink.write_char(' ')?;
				if dir.contains(NamedDirection::Top) {
					atom!("top").write_css(sink)?;
					if dir != &NamedDirection::Top {
						sink.write_char(' ')?;
					}
				}
				if dir.contains(NamedDirection::Bottom) {
					atom!("bottom").write_css(sink)?;
					if dir != &NamedDirection::Bottom {
						sink.write_char(' ')?;
					}
				}
				if dir.contains(NamedDirection::Left) {
					atom!("left").write_css(sink)?;
				}
				if dir.contains(NamedDirection::Right) {
					atom!("right").write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

// https://drafts.csswg.org/css-images-3/#typedef-rg-size
#[derive(Writable, Default, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum RadialSize {
	#[default]
	ClosestCorner, // atom!("closest-corner")
	ClosestSide,    // atom!("closest-side")
	FarthestCorner, // atom!("farthest-corner")
	FarthestSide,   // atom!("farthest-side")
	Circular(Length),
	Elliptical(LengthPercentage, LengthPercentage),
}

impl<'a> Peek<'a> for RadialSize {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<LengthPercentage>().or_else(|| {
			p.peek::<T![Ident]>().filter(|t| {
				matches!(
					p.parse_atom_lower(*t),
					atom!("closest-corner") | atom!("closest-side") | atom!("farthest-corner") | atom!("farthest-side")
				)
			})
		})
	}
}

impl<'a> Parse<'a> for RadialSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = p.peek::<T![Ident]>() {
			p.hop(token);
			return Ok(match p.parse_atom_lower(token) {
				atom!("closest-corner") => RadialSize::ClosestCorner,
				atom!("closest-side") => RadialSize::ClosestSide,
				atom!("farthest-corner") => RadialSize::FarthestCorner,
				atom!("farthest-side") => RadialSize::FarthestSide,
				atom => Err(diagnostics::UnexpectedIdent(atom, token.span()))?,
			});
		}
		if p.peek::<T![Number]>().is_some() {
			let first_len = p.parse::<LengthPercentage>()?;
			if p.peek::<T![Number]>().is_none() {
				return p.parse::<Length>().map(Self::Circular);
			}
			let second_len = p.parse::<LengthPercentage>()?;
			return Ok(Self::Elliptical(first_len, second_len));
		}
		if let Some(token) = p.peek::<T![Dimension]>() {
			let atom = p.parse_atom(token);
			if atom == atom!("%") {
				let first_len = p.parse::<LengthPercentage>()?;
				if p.peek::<T![Dimension]>().is_none() {
					let token = p.peek::<T![Any]>().unwrap();
					Err(diagnostics::ExpectedDimension(token, token.span()))?
				}
				let second_len = p.parse::<LengthPercentage>()?;
				return Ok(Self::Elliptical(first_len, second_len));
			} else {
				Err(diagnostics::UnexpectedDimension(atom, token.span()))?
			}
		}
		let token = p.peek::<T![Any]>().unwrap();
		Err(diagnostics::ExpectedDimension(token, token.span()))?
	}
}

// https://drafts.csswg.org/css-images-3/#typedef-radial-shape
#[derive(Atomizable, Default, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum RadialShape {
	#[default]
	Circle, // atom!("circle")
	Ellipse, // atom!("ellipse")
}

impl<'a> Peek<'a> for RadialShape {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Ident]>().filter(|token| matches!(p.parse_atom_lower(*token), atom!("circle") | atom!("ellipse")))
	}
}

impl<'a> Parse<'a> for RadialShape {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = p.parse::<T![Ident]>()?;
		match p.parse_atom_lower(*token) {
			atom!("circle") => Ok(Self::Circle),
			atom!("ellipse") => Ok(Self::Ellipse),
			atom => Err(diagnostics::UnexpectedIdent(atom, token.span()))?,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorStopOrHint {
	Stop(Color, Option<LengthPercentage>),
	Hint(LengthPercentage),
}

impl<'a> WriteCss<'a> for ColorStopOrHint {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Stop(col, len) => {
				col.write_css(sink)?;
				if len.is_some() {
					sink.write_char(' ')?;
				}
				len.write_css(sink)
			}
			Self::Hint(len) => len.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Gradient, 64);
		assert_size!(LinearDirection, 8);
		assert_size!(RadialSize, 16);
		assert_size!(ColorStopOrHint, 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Gradient, "linear-gradient(to bottom, yellow, blue)");
		assert_parse!(Gradient, "linear-gradient(yellow, blue)", "linear-gradient(to bottom, yellow, blue)");
		assert_parse!(Gradient, "linear-gradient(to bottom, #fff, #fff 85%, #e6e6e6)");
		assert_parse!(Gradient, "linear-gradient(45deg, #808080 25%, transparent 25%)");
		assert_parse!(Gradient, "linear-gradient(to right, transparent, red 20%, red 80%, transparent)");
		assert_parse!(Gradient, "radial-gradient(closest-corner circle, rgba(1, 65, 255, 0.4), rgba(1, 65, 255, 0))");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Gradient, "linear-gradient(to bottom, red, blue)", "linear-gradient(red,blue)");
		assert_minify!(Gradient, "radial-gradient(closest-corner circle, red, blue)", "radial-gradient(red,blue)");
	}
}
