#[cfg(feature = "serde")]
use serde::Serialize;

use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, MediaFeature, Parse, Parser, Result as ParserResult, unexpected_ident};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(PartialEq, Default, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum AnyHoverMediaFeature {
	#[default]
	Any,
	None,
	Hover,
}

impl<'a> Parse<'a> for AnyHoverMediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_media_feature(atom!("any-hover"), parser)
	}
}

impl<'a> MediaFeature<'a> for AnyHoverMediaFeature {
	fn parse_media_feature_value(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("none") => {
					parser.advance();
					Ok(Self::None)
				}
				atom!("hover") => {
					parser.advance();
					Ok(Self::Hover)
				}
				_ => unexpected_ident!(parser, ident),
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for AnyHoverMediaFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('(')?;
		atom!("any-hover").write_css(sink)?;
		match self {
			Self::None => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("none").write_css(sink)?;
			},
			Self::Hover => {
				sink.write_char(':')?;
				sink.write_whitespace()?;
				atom!("hover").write_css(sink)?;
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
		assert_eq!(size_of::<AnyHoverMediaFeature>(), 1);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<AnyHoverMediaFeature>(&allocator, "(any-hover)", "(any-hover)");
		test_write::<AnyHoverMediaFeature>(&allocator, "(any-hover: hover)", "(any-hover: hover)");
		test_write::<AnyHoverMediaFeature>(&allocator, "(any-hover: none)", "(any-hover: none)");
	}

	#[test]
	fn test_minify() {
		let allocator = Allocator::default();
		test_write_min::<AnyHoverMediaFeature>(&allocator, "(any-hover: hover)", "(any-hover:hover)");
		test_write_min::<AnyHoverMediaFeature>(&allocator, "(any-hover: none)", "(any-hover:none)");
	}
}
