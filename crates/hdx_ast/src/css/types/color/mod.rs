mod named;
mod syntax;

use crate::css::units::{Angle, CSSFloat, Percent};
use hdx_atom::{atom, Atomizable};
use hdx_lexer::{Kind, Token};
use hdx_parser::{
	discard, expect, expect_delim, match_ignore_case, todo, unexpected, unexpected_function, unexpected_ident, Parse,
	Parser, Result as ParserResult,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use std::str::Chars;

pub use named::*;
pub use syntax::*;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Channel {
	None,
	Float(CSSFloat),
	Percent(Percent),
	Hue(Angle),
}

impl<'a> Parse<'a> for Channel {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.peek().clone() {
			Token::Ident(atom) if atom.to_ascii_lowercase() == atom!("none") => {
				parser.advance();
				Ok(Self::None)
			}
			Token::Number(n, _) => {
				parser.advance();
				Ok(Self::Float(n.into()))
			}
			Token::Dimension(n, unit, _) if unit.to_ascii_lowercase() == atom!("%") => {
				parser.advance();
				Ok(Self::Percent(n.into()))
			}
			Token::Dimension(_, _, _) => Ok(Self::Hue(Angle::parse(parser)?)),
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Channel {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::None => atom!("none").write_css(sink),
			Self::Float(n) => n.write_css(sink),
			Self::Percent(n) => n.write_css(sink),
			Self::Hue(n) => n.write_css(sink),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AbsoluteColorFunction(pub ColorFunctionSyntax, pub Channel, pub Channel, pub Channel, pub Channel);

impl<'a> Parse<'a> for AbsoluteColorFunction {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut syntax = match parser.next() {
			Token::Function(atom) => match atom.to_ascii_lowercase() {
				atom!("color") => match parser.next() {
					Token::Ident(atom) => {
						if let Some(space) = ColorFunctionSyntax::from_color_space(atom.clone()) {
							space
						} else {
							unexpected_ident!(parser, atom)
						}
					}
					token => unexpected!(parser, token),
				},
				named => {
					if let Some(func) = ColorFunctionSyntax::from_named_function(named) {
						func
					} else {
						unexpected_function!(parser, atom)
					}
				}
			},
			token => unexpected!(parser, token),
		};
		let first = Channel::parse(parser)?;
		let percent = matches!(first, Channel::Percent(_));
		if matches!(first, Channel::Hue(_)) != syntax.first_is_hue() {
			unexpected!(parser);
		}
		if discard!(parser, Kind::Comma) {
			syntax |= ColorFunctionSyntax::Legacy;
		}
		let second = Channel::parse(parser)?;
		if (syntax.is_legacy() && matches!(second, Channel::Percent(_)) != percent) || matches!(second, Channel::Hue(_))
		{
			unexpected!(parser)
		}
		if syntax.contains(ColorFunctionSyntax::Legacy) != discard!(parser, Kind::Comma) {
			unexpected!(parser)
		}
		let third = Channel::parse(parser)?;
		if syntax.is_legacy() && matches!(third, Channel::Percent(_)) != percent {
			unexpected!(parser)
		}
		if matches!(third, Channel::Hue(_)) != syntax.third_is_hue() {
			unexpected!(parser);
		}
		if discard!(parser, Kind::RightParen) {
			return Ok(Self(syntax | ColorFunctionSyntax::OmitAlpha, first, second, third, Channel::None));
		}
		if syntax.contains(ColorFunctionSyntax::Legacy) {
			expect!(parser.next(), Kind::Comma);
		} else {
			expect_delim!(parser.next(), '/');
		}
		let fourth = Channel::parse(parser)?;
		if matches!(fourth, Channel::None) {
			unexpected!(parser)
		}
		expect!(parser.next(), Kind::RightParen);
		Ok(Self(syntax, first, second, third, fourth))
	}
}

impl<'a> WriteCss<'a> for AbsoluteColorFunction {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.0.is_named() {
			self.0.named_function().unwrap().write_css(sink)?;
			sink.write_char('(')?;
			self.1.write_css(sink)?;
			if self.0.contains(ColorFunctionSyntax::Legacy) {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			} else {
				sink.write_char(' ')?;
			}
			self.2.write_css(sink)?;
			if self.0.contains(ColorFunctionSyntax::Legacy) {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			} else {
				sink.write_char(' ')?;
			}
			self.3.write_css(sink)?;
			if self.0.contains(ColorFunctionSyntax::Legacy) && !self.0.contains(ColorFunctionSyntax::OmitAlpha) {
				sink.write_char(',')?;
				sink.write_whitespace()?;
				self.4.write_css(sink)?;
			} else if !self.0.contains(ColorFunctionSyntax::OmitAlpha) {
				sink.write_whitespace()?;
				sink.write_char('/')?;
				sink.write_whitespace()?;
				self.4.write_css(sink)?;
			}
		}
		sink.write_char(')')
	}
}

#[derive(Debug, Default, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Color {
	#[default]
	CurrentColor,
	Transparent,
	Hex(u32),
	Named(NamedColor),
	Absolute(AbsoluteColorFunction),
	// TODO: need bumpalo::Box PartialEq, or bumpalo::Box serde
	// Relative(Box<'a, Color<'a>>, ColorFunction),
	// Mix(ColorMixSyntax, Box<'a, Color<'a>>, u8, Box<'a, Color<'a>>),
}

trait HexableChars {
	fn next_as_hex(&mut self) -> Option<u32>;
}

impl<'a> HexableChars for Chars<'a> {
	fn next_as_hex(&mut self) -> Option<u32> {
		match self.next() {
			Some(ch) => {
				let b = ch as u8;
				match b {
					b'A'..=b'F' => Some((b - b'A' + 10) as u32),
					b'a'..=b'f' => Some((b - b'a' + 10) as u32),
					b'0'..=b'9' => Some((b - b'0') as u32),
					_ => None,
				}
			}
			_ => None,
		}
	}
}

impl<'a> Parse<'a> for Color {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match_ignore_case! { parser.peek(), Token::Function(_):
			atom!("color") => todo!(parser),
			atom!("color-mix") => todo!(parser),
			_ => return Ok(Color::Absolute(AbsoluteColorFunction::parse(parser)?))
		};
		Ok(match parser.next() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("currentcolor") => Color::CurrentColor,
				atom!("transparent") => Color::Transparent,
				name => {
					if let Some(named) = NamedColor::from_atom(&name) {
						Color::Named(named)
					} else {
						unexpected_ident!(parser, atom)
					}
				}
			},
			Token::Hash(atom) | Token::HashId(atom) => {
				let mut chars = atom.chars();
				let (r, g, b, a) = match atom.len() {
					// <r><g><b> implied alpha
					3 => (
						chars.next_as_hex().unwrap() * 17,
						chars.next_as_hex().unwrap() * 17,
						chars.next_as_hex().unwrap() * 17,
						255,
					),
					// <r><g><b><a>
					4 => (
						chars.next_as_hex().unwrap() * 17,
						chars.next_as_hex().unwrap() * 17,
						chars.next_as_hex().unwrap() * 17,
						chars.next_as_hex().unwrap() * 17,
					),
					// <rr><gg><bb> implied alpha
					6 => (
						chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
						chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
						chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
						255,
					),
					// <rr><gg><bb><aa>
					8 => (
						chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
						chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
						chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
						chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
					),
					_ => unexpected!(parser),
				};
				Color::Hex(r << 24 | g << 16 | b << 8 | a)
			}
			token => unexpected!(parser, token),
		})
	}
}

impl<'a> WriteCss<'a> for Color {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::CurrentColor => atom!("currentcolor").write_css(sink),
			Self::Transparent => atom!("transparent").write_css(sink),
			Self::Hex(d) => {
				let compacted = ((d & 0x0FF00000) >> 12) | ((d & 0x00000FF0) >> 4);
				let expanded = ((compacted & 0xF000) << 16)
					| ((compacted & 0xFF00) << 12)
					| ((compacted & 0x0FF0) << 8)
					| ((compacted & 0x00FF) << 4)
					| (compacted & 0x000F);
				// Shorthand can be used
				if &expanded == d && d & 255 == 255 {
					sink.write_str(&format!("#{:03x}", compacted >> 4))
				} else if &expanded == d {
					sink.write_str(&format!("#{:04x}", compacted))
				} else if d & 255 == 255 {
					sink.write_str(&format!("#{:06x}", d >> 8))
				} else {
					sink.write_str(&format!("#{:08x}", d))
				}
			}
			Self::Named(name) => name.to_atom().write_css(sink),
			Self::Absolute(func) => func.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(AbsoluteColorFunction, 36);
		assert_size!(Color, 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Color, "currentcolor");
		assert_parse!(Color, "#fff");
		assert_parse!(Color, "red");
		assert_parse!(Color, "#ababab");
		assert_parse!(Color, "rgb(255 255 255)");
		assert_parse!(Color, "rgb(255, 255, 255)");
		assert_parse!(Color, "rgba(255, 255, 255, 0.5)");
		assert_parse!(Color, "rgb(29 164 192 / 95%)");
		assert_parse!(Color, "rgb(255 255 255 / 0.5)");
		assert_parse!(Color, "rgb(255 20% 12.2 / 0.5)");
		assert_parse!(Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(Color, "hwb(740deg 20% 30% / 50%)");
		assert_parse!(Color, "lch(20% 30% 740deg / 50%)");
	}

	#[test]
	fn test_errors() {
		// Missing /
		assert_parse_error!(Color, "rgb(255 20% 255 0.5)");
		// Mixed legacy values
		assert_parse_error!(Color, "rgba(255, 20%, 255, 0.5)");
		// Mixed legacy values
		assert_parse_error!(Color, "rgba(255, 20%, 255, 0.5)");
		// Using / for alpha
		assert_parse_error!(Color, "rgba(255, 255, 255 / 0.5)");
		// Using degrees for RGB
		assert_parse_error!(Color, "rgba(250deg, 255, 255 / 0.5)");
		// Using % for first component in hsl
		assert_parse_error!(Color, "hsl(250%, 255, 255)");
		// Using % for first component in lch
		assert_parse_error!(Color, "lch(250%, 255, 255)");
		// Using degrees for wrong component in hsl
		assert_parse_error!(Color, "hsl(250, 255deg, 255)");
		// Using degrees for wrong component in lch
		assert_parse_error!(Color, "lch(250, 255deg, 255)");
	}
}
