use crate::Value;
use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-color-adjust/#color-scheme-prop
#[derive(Value, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ColorScheme {
	#[default]
	Normal,
	Defined(SmallVec<[ColorSchemeKeyword; 1]>),
	Only(SmallVec<[ColorSchemeKeyword; 1]>),
}

impl<'a> Parse<'a> for ColorScheme {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut only = false;
		let mut keywords = smallvec![];
		while let Token::Ident(ident) = parser.cur() {
			match ident.to_ascii_lowercase() {
				atom!("normal") => return Ok(Self::Normal),
				atom!("only") => {
					if only {
						unexpected_ident!(parser, ident)
					}
					parser.advance();
					only = true;
				}
				atom!("light") => {
					parser.advance();
					keywords.push(ColorSchemeKeyword::Light);
				}
				atom!("dark") => {
					parser.advance();
					keywords.push(ColorSchemeKeyword::Dark);
				}
				_ => {
					parser.advance();
					keywords.push(ColorSchemeKeyword::Custom(ident));
				}
			}
		}
		if only && keywords.is_empty() {
			unexpected!(parser)
		}
		if only {
			Ok(Self::Only(keywords))
		} else {
			Ok(Self::Defined(keywords))
		}
	}
}

impl<'a> WriteCss<'a> for ColorScheme {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Normal => atom!("normal").write_css(sink),
			Self::Only(kw) | Self::Defined(kw) => {
				let mut iter = kw.iter().peekable();
				while let Some(selector) = iter.next() {
					selector.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_char(' ')?;
					}
				}
				if matches!(self, Self::Only(_)) {
					sink.write_char(' ')?;
					atom!("only").write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ColorSchemeKeyword {
	Light,
	Dark,
	Custom(Atom),
}

impl<'a> WriteCss<'a> for ColorSchemeKeyword {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Light => atom!("light").write_css(sink),
			Self::Dark => atom!("dark").write_css(sink),
			Self::Custom(kw) => kw.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<ColorScheme>(), 40);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<ColorScheme>(&allocator, "light", "light");
		test_write::<ColorScheme>(&allocator, "dark", "dark");
		test_write::<ColorScheme>(&allocator, "light dark", "light dark");
		test_write::<ColorScheme>(&allocator, "only dark", "dark only");
		test_write::<ColorScheme>(&allocator, "light dark magic", "light dark magic");
		test_write::<ColorScheme>(&allocator, "light only dark magic", "light dark magic only");
		test_write::<ColorScheme>(&allocator, "light only dark --other-custom", "light dark --other-custom only");
	}
}
