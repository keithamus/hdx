use bitmask_enum::bitmask;
use hdx_atom::{atom, Atom, Atomizable};
use hdx_derive::{Atomizable, Writable};
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{
	discard, expect, expect_ignore_case, peek, unexpected, unexpected_ident, FromToken, Parse, Parser,
	Result as ParserResult,
};
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
		Ok(match parser.peek().clone() {
			Token::Url(atom, style) => {
				parser.advance();
				Self::Url(atom.clone(), style)
			}
			Token::Function(atom) => match atom.to_ascii_lowercase() {
				atom!("url") => {
					parser.advance();
					match parser.next().clone() {
						Token::String(atom, style) => {
							expect!(parser.next(), Token::RightParen);
							Self::Url(atom, style)
						}
						token => unexpected!(parser, token),
					}
				}
				_ => Self::Gradient(Gradient::parse(parser)?),
			},
			token => unexpected!(parser, token),
		})
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
			if let Some(hint) = LengthPercentage::from_token(parser.peek()) {
				if allow_hint {
					parser.advance();
					stops.push(ColorStopOrHint::Hint(hint));
					expect!(parser.next(), Token::Comma);
				} else {
					unexpected!(parser);
				}
			}
			let color = Color::parse(parser)?;
			let hint = LengthPercentage::from_token(parser.peek());
			if hint.is_some() {
				parser.advance();
			}
			stops.push(ColorStopOrHint::Stop(color, hint));
			allow_hint = hint.is_some();
			if !discard!(parser, Token::Comma) {
				break;
			}
		}
		Ok(stops)
	}
}

impl<'a> Parse<'a> for Gradient {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let gradient = expect_ignore_case! { parser.next(), Token::Function(_):
			atom @ atom!("linear-gradient") | atom @ atom!("repeating-linear-gradient") => {
				let dir = if let Ok(dir) = LinearDirection::try_parse(parser) {
					expect!(parser.next(), Token::Comma);
					dir
				} else {
					LinearDirection::default()
				};
				match atom {
					atom!("linear-gradient") => Self::Linear(dir, Self::parse_stops(parser)?),
					atom!("repeating-linear-gradient") => {
						Self::RepeatingLinear(dir, Self::parse_stops(parser)?)
					}
					_ => unexpected_ident!(parser, atom),
				}
			},
			atom @ atom!("radial-gradient") | atom @ atom!("repeating-linear-gradient") => {
				let mut size = RadialSize::parse(parser).ok();
				let shape = RadialShape::parse(parser).ok();
				if size.is_none() && shape.is_some() {
					size = RadialSize::parse(parser).ok();
				}
				let position = if matches!(parser.cur(), Token::Ident(atom) if atom.to_ascii_lowercase() == atom!("at"))
				{
					parser.advance();
					Some(Position::parse(parser)?)
				} else {
					None
				};
				if size.is_some() || shape.is_some() {
					expect!(parser.next(), Token::Comma);
				}
				match atom {
					atom!("radial-gradient") => Self::Radial(
						size.unwrap_or(RadialSize::default()),
						shape.unwrap_or(RadialShape::default()),
						position,
						Self::parse_stops(parser)?,
					),
					atom!("repeating-radial-gradient") => Self::RepeatingRadial(
						size.unwrap_or(RadialSize::default()),
						shape.unwrap_or(RadialShape::default()),
						position,
						Self::parse_stops(parser)?,
					),
					_ => unexpected_ident!(parser, atom),
				}
			},
		};
		expect!(parser.next(), Token::RightParen);
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
		match parser.peek() {
			Token::Dimension(_, _, _) => return Ok(Self::Angle(Angle::parse(parser)?)),
			Token::Ident(_) => {}
			token => unexpected!(parser, token.clone()),
		};
		expect_ignore_case!(parser.next(), Token::Ident(atom!("to")));
		let mut dir = NamedDirection::none();
		dir |= expect_ignore_case! { parser.next(), Token::Ident(_):
			atom!("top") => NamedDirection::Top,
			atom!("left") => NamedDirection::Left,
			atom!("right") => NamedDirection::Right,
			atom!("bottom") => NamedDirection::Bottom,
		};
		if peek!(parser, Token::Ident(_)) {
			dir |= expect_ignore_case! { parser.next(), Token::Ident(_):
				atom @ atom!("top") => {
					if dir.contains(NamedDirection::Top) || dir.contains(NamedDirection::Bottom) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Top
				},
				atom @ atom!("left") => {
					if dir.contains(NamedDirection::Left) || dir.contains(NamedDirection::Right) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Left
				},
				atom @ atom!("right") => {
					if dir.contains(NamedDirection::Right) || dir.contains(NamedDirection::Left) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Right
				},
				atom @ atom!("bottom") => {
					if dir.contains(NamedDirection::Bottom) || dir.contains(NamedDirection::Top) {
						unexpected_ident!(parser, atom)
					}
					NamedDirection::Bottom
				}
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
		Ok(match parser.next().clone() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("closest-corner") => RadialSize::ClosestCorner,
				atom!("closest-side") => RadialSize::ClosestSide,
				atom!("farthest-corner") => RadialSize::FarthestCorner,
				atom!("farthest-side") => RadialSize::FarthestSide,
				_ => unexpected_ident!(parser, atom),
			},
			first @ Token::Number(_, _) | first @ Token::Dimension(_, atom!("%"), _) => match parser.peek().clone() {
				second @ Token::Number(_, _) | second @ Token::Dimension(_, atom!("%"), _) => {
					if matches!(first, Token::Number(_, _)) != matches!(second, Token::Number(_, _)) {
						unexpected!(parser);
					}
					parser.advance();
					Self::Elliptical(
						LengthPercentage::from_token(&first).unwrap(),
						LengthPercentage::from_token(&second).unwrap(),
					)
				}
				_ => {
					if matches!(first, Token::Dimension(_, _, _)) {
						let token = first.clone();
						unexpected!(parser, token);
					}
					Self::Circular(Length::from_token(&first).unwrap())
				}
			},
			token => unexpected!(parser, token),
		})
	}
}

// https://drafts.csswg.org/css-images-3/#typedef-rg-ending-shape
#[derive(Atomizable, Default, Debug, Clone, PartialEq, Hash)]
#[atomizable(FromToken)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum RadialShape {
	#[default]
	Circle, // atom!("circle")
	Ellipse, // atom!("ellipse")
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
