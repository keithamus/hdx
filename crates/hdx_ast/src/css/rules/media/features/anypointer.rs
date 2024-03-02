#[cfg(feature = "serde")]
use serde::Serialize;

use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, MediaFeature, Parse, Parser, Result as ParserResult, unexpected_ident};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(PartialEq, Default, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum AnyPointerMediaFeature {
	#[default]
	Any,
	None,
	Coarse,
	Fine,
}

impl<'a> Parse<'a> for AnyPointerMediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_media_feature(atom!("any-pointer"), parser)
	}
}

impl<'a> MediaFeature<'a> for AnyPointerMediaFeature {
	fn parse_media_feature_value(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("none") => {
					parser.advance();
					Ok(Self::None)
				}
				atom!("coarse") => {
					parser.advance();
					Ok(Self::Coarse)
				}
				atom!("fine") => {
					parser.advance();
					Ok(Self::Fine)
				}
				_ => unexpected_ident!(parser, ident),
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for AnyPointerMediaFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('(')?;
		atom!("any-pointer").write_css(sink)?;
		match self {
			Self::None => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("none").write_css(sink)?;
			},
			Self::Coarse => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("coarse").write_css(sink)?;
			}
			Self::Fine => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("fine").write_css(sink)?;
			}
			Self::Any => {}
		}
		sink.write_char(')')
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::{test_write, test_write_min};

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<AnyPointerMediaFeature>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<AnyPointerMediaFeature>(&allocator, "(any-pointer)", "(any-pointer)");
		test_write::<AnyPointerMediaFeature>(&allocator, "(any-pointer: none)", "(any-pointer: none)");
		test_write::<AnyPointerMediaFeature>(&allocator, "(any-pointer: coarse)", "(any-pointer: coarse)");
		test_write::<AnyPointerMediaFeature>(&allocator, "(any-pointer: fine)", "(any-pointer: fine)");
	}

	#[test]
	fn test_minify() {
		let allocator = Allocator::default();
		test_write_min::<AnyPointerMediaFeature>(&allocator, "(any-pointer)", "(any-pointer)");
		test_write_min::<AnyPointerMediaFeature>(&allocator, "(any-pointer: none)", "(any-pointer:none)");
		test_write_min::<AnyPointerMediaFeature>(&allocator, "(any-pointer: coarse)", "(any-pointer:coarse)");
		test_write_min::<AnyPointerMediaFeature>(&allocator, "(any-pointer: fine)", "(any-pointer:fine)");
	}
}
