use bitmask_enum::bitmask;
use hdx_atom::{atom, Atom, Atomizable};
use hdx_derive::{Atomizable, Writable};
use hdx_lexer::QuoteStyle;
use hdx_parser::{unexpected, unexpected_function, unexpected_ident, Parse, Parser, Result as ParserResult, Token};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::{
	types::Position,
	units::{Angle, Length, LengthPercentage},
};

use super::Color;

// https://drafts.csswg.org/css-images-3/#typedef-image
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Image {
	Url(Atom, QuoteStyle),
	Gradient(Gradient),
}

impl<'a> Parse<'a> for Image {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<Token![Url]>() {
			parser.advance_to(token);
			return Ok(Self::Url(parser.parse_atom(token), token.quote_style()));
		}
		if let Some(token) = parser.peek::<Token![Function]>() {
			if parser.parse_atom_lower(token) == atom!("url") {
				parser.advance_to(token);
				let string_token = parser.parse::<Token![String]>()?;
				parser.parse::<Token![RightParen]>()?;
				return Ok(Self::Url(parser.parse_atom(*string_token), string_token.quote_style()));
			}
		}
		parser.parse::<Gradient>().map(Self::Gradient)
	}
}

impl<'a> WriteCss<'a> for Image {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Url(atom, style) => {
				atom!("url").write_css(sink)?;
				sink.write_char('(')?;
				sink.write_with_quotes(atom, *style, true)?;
				sink.write_char(')')
			}
			Self::Gradient(g) => g.write_css(sink),
		}
	}
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
	fn parse_stops(parser: &mut Parser<'a>) -> ParserResult<SmallVec<[ColorStopOrHint; 0]>> {
		let mut stops = smallvec![];
		let mut allow_hint = false;
		loop {
			if allow_hint {
				if let Some(hint) = parser.try_parse::<LengthPercentage>().ok() {
					stops.push(ColorStopOrHint::Hint(hint));
					parser.parse::<Token![Comma]>()?;
				}
			}
			let color = parser.parse::<Color>()?;
			let hint = parser.try_parse::<LengthPercentage>().ok();
			stops.push(ColorStopOrHint::Stop(color, hint));
			allow_hint = hint.is_some();
			if parser.peek::<Token![Comma]>().is_none() {
				break;
			}
			parser.parse::<Token![Comma]>()?;
		}
		Ok(stops)
	}
}

impl<'a> Parse<'a> for Gradient {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let func_token = parser.parse::<Token![Function]>()?;
		let gradient = match parser.parse_atom_lower(*func_token) {
			atom @ atom!("linear-gradient") | atom @ atom!("repeating-linear-gradient") => {
				let dir = if let Ok(dir) = parser.try_parse::<LinearDirection>() {
					parser.parse::<Token![Comma]>()?;
					dir
				} else {
					LinearDirection::default()
				};
				if atom == atom!("linear-gradient") {
					Self::Linear(dir, Self::parse_stops(parser)?)
				} else {
					Self::RepeatingLinear(dir, Self::parse_stops(parser)?)
				}
			}
			atom @ atom!("radial-gradient") | atom @ atom!("repeating-linear-gradient") => {
				let mut size = parser.try_parse::<RadialSize>().ok();
				let shape = parser.parse::<RadialShape>().ok();
				if size.is_none() && shape.is_some() {
					size = Some(RadialSize::parse(parser)?);
				}
				let mut position = None;
				if let Some(token) = parser.peek::<Token![Ident]>() {
					if parser.parse_atom_lower(token) == atom!("at") {
						parser.advance_to(token);
						position = Some(Position::parse(parser)?)
					}
				}
				if size.is_some() || shape.is_some() {
					parser.parse::<Token![Comma]>();
				}
				if atom == atom!("radial-gradient") {
					Self::Radial(
						size.unwrap_or(RadialSize::default()),
						shape.unwrap_or(RadialShape::default()),
						position,
						Self::parse_stops(parser)?,
					)
				} else {
					Self::RepeatingRadial(
						size.unwrap_or(RadialSize::default()),
						shape.unwrap_or(RadialShape::default()),
						position,
						Self::parse_stops(parser)?,
					)
				}
			}
			atom => unexpected_function!(parser, atom),
		};
		parser.parse::<Token![RightParen]>();
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

impl<'a> Parse<'a> for LinearDirection {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Ok(angle) = parser.try_parse::<Angle>() {
			return Ok(Self::Angle(angle));
		}
		let to_token = parser.parse::<Token![Ident]>()?;
		let to = parser.parse_atom_lower(*to_token);
		if to != atom!("to") {
			unexpected_ident!(parser, to);
		}
		let mut dir = NamedDirection::none();
		let token = parser.parse::<Token![Ident]>()?;
		dir |= match parser.parse_atom_lower(*token) {
			atom!("top") => NamedDirection::Top,
			atom!("left") => NamedDirection::Left,
			atom!("right") => NamedDirection::Right,
			atom!("bottom") => NamedDirection::Bottom,
			atom => unexpected_ident!(parser, atom),
		};
		if let Some(token) = parser.peek::<Token![Ident]>() {
			parser.advance_to(token);
			dir |= match parser.parse_atom_lower(token) {
				atom @ atom!("top") => {
					if dir.contains(NamedDirection::Top) || dir.contains(NamedDirection::Bottom) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Top
				}
				atom @ atom!("left") => {
					if dir.contains(NamedDirection::Left) || dir.contains(NamedDirection::Right) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Left
				}
				atom @ atom!("right") => {
					if dir.contains(NamedDirection::Right) || dir.contains(NamedDirection::Left) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Right
				}
				atom @ atom!("bottom") => {
					if dir.contains(NamedDirection::Bottom) || dir.contains(NamedDirection::Top) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Bottom
				}
				atom => unexpected_ident!(parser, atom),
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

impl<'a> Parse<'a> for RadialSize {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<Token![Ident]>() {
			parser.advance_to(token);
			return Ok(match parser.parse_atom_lower(token) {
				atom!("closest-corner") => RadialSize::ClosestCorner,
				atom!("closest-side") => RadialSize::ClosestSide,
				atom!("farthest-corner") => RadialSize::FarthestCorner,
				atom!("farthest-side") => RadialSize::FarthestSide,
				atom => unexpected_ident!(parser, atom),
			});
		}
		if let Some(_) = parser.peek::<Token![Number]>() {
			let first_len = parser.parse::<LengthPercentage>()?;
			if parser.peek::<Token![Number]>().is_none() {
				return parser.parse::<Length>().map(Self::Circular);
			}
			let second_len = parser.parse::<LengthPercentage>()?;
			return Ok(Self::Elliptical(first_len, second_len));
		}
		if let Some(token) = parser.peek::<Token![Dimension]>() {
			if parser.parse_atom(token) == atom!("%") {
				let first_len = parser.parse::<LengthPercentage>()?;
				if parser.peek::<Token![Dimension]>().is_none() {
					unexpected!(parser);
				}
				let second_len = parser.parse::<LengthPercentage>()?;
				return Ok(Self::Elliptical(first_len, second_len));
			} else {
				unexpected_dimension!(parser, token);
			}
		}
		unexpected!(parser)
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

impl<'a> Parse<'a> for RadialShape {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.parse::<Token![Ident]>()?;
		match parser.parse_atom_lower(*token) {
			atom!("circle") => Ok(Self::Circle),
			atom!("ellipse") => Ok(Self::Ellipse),
			atom => unexpected_ident!(parser, atom),
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
		assert_size!(Image, 72);
		assert_size!(Gradient, 72);
		assert_size!(LinearDirection, 8);
		assert_size!(RadialSize, 16);
		assert_size!(ColorStopOrHint, 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Image, "url('foo')");
		assert_parse!(Image, "url(\"foo\")");
		assert_parse!(Image, "url(foo)");
		assert_parse!(Image, "linear-gradient(to bottom, yellow, blue)");
		assert_parse!(Image, "linear-gradient(yellow, blue)", "linear-gradient(to bottom, yellow, blue)");
		assert_parse!(Image, "linear-gradient(to bottom, #fff, #fff 85%, #e6e6e6)");
		assert_parse!(Image, "linear-gradient(45deg, #808080 25%, transparent 25%)");
		assert_parse!(Image, "linear-gradient(to right, transparent, red 20%, red 80%, transparent)");
		assert_parse!(Image, "radial-gradient(closest-corner circle, rgba(1, 65, 255, 0.4), rgba(1, 65, 255, 0))");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Image, "url('foo')", "url(foo)");
		assert_minify!(Image, "linear-gradient(to bottom, red, blue)", "linear-gradient(red,blue)");
		assert_minify!(Image, "radial-gradient(closest-corner circle, red, blue)", "radial-gradient(red,blue)");
	}
}
