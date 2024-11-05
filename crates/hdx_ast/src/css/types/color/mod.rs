mod named;
mod syntax;
mod system;

use crate::css::units::{Angle, CSSFloat, Percent};
use hdx_atom::{atom, Atomizable};
use hdx_lexer::Span;
use hdx_parser::{diagnostics, discard, todo, Delim, Parse, Parser, Peek, Result as ParserResult, Token};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use std::str::Chars;

pub use named::*;
pub use system::*;
pub use syntax::*;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(None, atom!("none"));
	custom_keyword!(Currentcolor, atom!("currentcolor"));
	custom_keyword!(Transparent, atom!("transparent"));
}

mod func {
	use hdx_parser::custom_function;
	custom_function!(Color, atom!("color"));
	custom_function!(ColorMix, atom!("color-mix"));
}

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
		if let Some(token) = parser.peek::<kw::None>() {
			parser.hop(token);
			return Ok(Self::None);
		}
		if let Some(token) = parser.peek::<Token![Number]>() {
			parser.hop(token);
			return Ok(Self::Float(parser.parse_number(token).into()));
		}
		if let Some(token) = parser.peek::<Token![Dimension]>() {
			if parser.parse_atom(token) == atom!("%") {
				parser.hop(token);
				Ok(Self::Percent(parser.parse_number(token).into()))
			} else {
				Ok(Self::Hue(Angle::parse(parser)?))
			}
		} else {
			let token = parser.peek::<Token![Any]>().unwrap();
			Err(diagnostics::ExpectedDimension(token, token.span()))?
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

impl<'a> Peek<'a> for AbsoluteColorFunction {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		if let Some(token) = parser.peek::<Token![Function]>() {
			match parser.parse_atom_lower(token) {
				atom!("color") => return Some(token),
				named => {
					if ColorFunctionSyntax::from_named_function(&named).is_some() {
						return Some(token);
					}
				}
			}
		}
		None
	}
}

impl<'a> Parse<'a> for AbsoluteColorFunction {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut syntax = if let Some(token) = parser.peek::<func::Color>() {
			parser.hop(token);
			let token = *parser.parse::<Token![Ident]>()?;
			let atom = parser.parse_atom_lower(token);
			if let Some(space) = ColorFunctionSyntax::from_color_space(&atom) {
				space
			} else {
				Err(diagnostics::UnexpectedIdent(atom, token.span()))?
			}
		} else {
			let token = *parser.parse::<Token![Function]>()?;
			let named = parser.parse_atom_lower(token);
			if let Some(func) = ColorFunctionSyntax::from_named_function(&named) {
				func
			} else {
				Err(diagnostics::UnexpectedFunction(named, token.span()))?
			}
		};
		let start = parser.offset();
		let first = parser.parse::<Channel>()?;
		let percent = matches!(first, Channel::Percent(_));
		if matches!(first, Channel::Hue(_)) != syntax.first_is_hue() {
			if matches!(first, Channel::Hue(_)) {
				Err(diagnostics::ColorMustStartWithHue(Span::new(start, parser.offset())))?
			} else {
				Err(diagnostics::ColorMustNotStartWithHue(Span::new(start, parser.offset())))?
			}
		}
		if discard!(parser, Comma) {
			syntax |= ColorFunctionSyntax::Legacy;
		}
		let start = parser.offset();
		let second = parser.parse::<Channel>()?;
		if syntax.is_legacy() && matches!(second, Channel::Percent(_)) != percent {
			Err(diagnostics::ColorLegacyMustNotUsePercent(Span::new(start, parser.offset())))?
		}
		if matches!(second, Channel::Hue(_)) {
			Err(diagnostics::ColorMustNotHaveHueInMiddle(Span::new(start, parser.offset())))?
		}
		let start = parser.offset();
		if syntax.contains(ColorFunctionSyntax::Legacy) != discard!(parser, Comma) {
			Err(diagnostics::ColorLegacyMustIncludeComma(Span::new(start, start)))?
		}
		let start = parser.offset();
		let third = parser.parse::<Channel>()?;
		if syntax.is_legacy() && matches!(third, Channel::Percent(_)) != percent {
			Err(diagnostics::ColorLegacyMustNotUsePercent(Span::new(start, parser.offset())))?
		}
		if matches!(third, Channel::Hue(_)) != syntax.third_is_hue() {
			Err(diagnostics::ColorMustNotHaveHueInMiddle(Span::new(start, parser.offset())))?
		}
		if parser.parse_if_peek::<Token![RightParen]>()?.is_some() {
			return Ok(Self(syntax | ColorFunctionSyntax::OmitAlpha, first, second, third, Channel::None));
		}
		if syntax.contains(ColorFunctionSyntax::Legacy) {
			parser.parse::<Delim![,]>()?;
		} else {
			parser.parse::<Delim![/]>()?;
		}
		let token = parser.peek::<Token![Any]>().unwrap();
		let fourth = parser.parse::<Channel>()?;
		if matches!(fourth, Channel::None) {
			Err(diagnostics::ExpectedNumber(token, token.span()))?
		}
		parser.parse::<Token![RightParen]>()?;
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Color {
	Currentcolor,
	Transparent,
	System(SystemColor),
	Hex(u32),
	Named(NamedColor),
	Absolute(AbsoluteColorFunction),
	// TODO: need bumpalo::Box PartialEq, or bumpalo::Box serde
	// Relative(Box<'a, Color<'a>>, ColorFunction),
	// Mix(ColorMixSyntax, Box<'a, Color<'a>>, u8, Box<'a, Color<'a>>),
}

impl Color {
	// Alias CanvasText for #[initial()]
	#[allow(non_upper_case_globals)]
	pub const Canvastext: Color = Color::System(SystemColor::CanvasText);
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

impl<'a> Peek<'a> for Color {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser
			.peek::<func::Color>()
			.or_else(|| parser.peek::<func::ColorMix>())
			.or_else(|| parser.peek::<AbsoluteColorFunction>())
			.or_else(|| parser.peek::<Token![Hash]>())
			.or_else(|| {
				parser.peek::<Token![Ident]>().filter(|token| {
					let atom = parser.parse_atom_lower(*token);
					matches!(atom, atom!("currentcolor") | atom!("canvastext") | atom!("transparent"))
						|| NamedColor::from_atom(&atom).is_some()
				})
			})
	}
}

impl<'a> Parse<'a> for Color {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::Currentcolor>() {
			parser.hop(token);
			Ok(Color::Currentcolor)
		} else if let Some(token) = parser.peek::<kw::Transparent>() {
			parser.hop(token);
			Ok(Color::Transparent)
		} else if let Some(token) = parser.peek::<Token![Ident]>() {
			let name = parser.parse_atom_lower(token);
			if let Some(named) = NamedColor::from_atom(&name) {
				parser.hop(token);
				Ok(Color::Named(named))
			} else if let Some(named) = SystemColor::from_atom(&name) {
				parser.hop(token);
				Ok(Color::System(named))
			} else {
				Err(diagnostics::UnexpectedIdent(name, token.span()))?
			}
		} else if let Some(token) = parser.peek::<Token![Hash]>() {
			parser.hop(token);
			let str = parser.parse_str(token);
			let mut chars = str.chars();
			let (r, g, b, a) = match str.len() {
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
				l => Err(diagnostics::ColorHexWrongLength(l, token.span()))?,
			};
			Ok(Color::Hex(r << 24 | g << 16 | b << 8 | a))
		} else if let Some(token) = parser.peek::<func::Color>() {
			parser.hop(token);
			todo!(parser)
		} else if let Some(token) = parser.peek::<func::ColorMix>() {
			parser.hop(token);
			todo!(parser)
		} else {
			parser.parse::<AbsoluteColorFunction>().map(Color::Absolute)
		}
	}
}

impl<'a> WriteCss<'a> for Color {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Currentcolor => kw::Currentcolor::atom().write_css(sink),
			Self::Transparent => kw::Transparent::atom().write_css(sink),
			Self::System(name) => name.to_atom().write_css(sink),
			Self::Named(name) => name.to_atom().write_css(sink),
			Self::Absolute(func) => func.write_css(sink),
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
