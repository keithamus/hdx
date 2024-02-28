#[cfg(feature = "serde")]
use serde::Serialize;

use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, expect, unexpected, unexpected_ident, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::WriteCss;

use crate::{css::values::units::Length, Atomizable, Value};

#[derive(Value, Debug, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Float {
	#[default]
	None,
	Left,
	Right,
	Top,
	Bottom,
	BlockStart,
	BlockEnd,
	InlineStart,
	InlineEnd,
	SnapBlock,
	SnapBlockFunction(Length, Option<SnapBlockDirection>),
	SnapInline,
	SnapInlineFunction(Length, Option<SnapInlineDirection>),
}

impl<'a> Parse<'a> for Float {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let value = match parser.cur() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("none") => {
					parser.advance();
					Float::None
				}
				atom!("left") => {
					parser.advance();
					Float::Left
				}
				atom!("right") => {
					parser.advance();
					Float::Right
				}
				atom!("top") => {
					parser.advance();
					Float::Top
				}
				atom!("bottom") => {
					parser.advance();
					Float::Bottom
				}
				atom!("block-start") => {
					parser.advance();
					Float::BlockStart
				}
				atom!("block-end") => {
					parser.advance();
					Float::BlockEnd
				}
				atom!("inline-start") => {
					parser.advance();
					Float::InlineStart
				}
				atom!("inline-end") => {
					parser.advance();
					Float::InlineEnd
				}
				atom!("snap-block") => {
					parser.advance();
					Float::SnapBlock
				}
				atom!("snap-inline") => {
					parser.advance();
					Float::SnapInline
				}
				atom => unexpected_ident!(parser, atom),
			},
			Token::Function(atom) => match atom.to_ascii_lowercase() {
				atom!("snap-block") => {
					parser.advance();
					let length =
						if let Some(length) = Length::from_token(parser.cur()) { length } else { unexpected!(parser) };
					parser.advance();
					let dir = match parser.cur() {
						Token::Comma => {
							parser.advance();
							match parser.cur() {
								Token::Ident(atom) => {
									if let Some(dir) = SnapBlockDirection::from_atom(atom.to_ascii_lowercase()) {
										parser.advance();
										Some(dir)
									} else {
										unexpected_ident!(parser, atom)
									}
								}
								token => unexpected!(parser, token),
							}
						}
						Token::RightParen => None,
						token => unexpected!(parser, token),
					};
					expect!(parser, Token::RightParen);
					parser.advance();
					Float::SnapBlockFunction(length, dir)
				}
				atom!("snap-inline") => {
					parser.advance();
					let length =
						if let Some(length) = Length::from_token(parser.cur()) { length } else { unexpected!(parser) };
					parser.advance();
					let dir = match parser.cur() {
						Token::Comma => {
							parser.advance();
							match parser.cur() {
								Token::Ident(atom) => {
									if let Some(dir) = SnapInlineDirection::from_atom(atom.to_ascii_lowercase()) {
										parser.advance();
										Some(dir)
									} else {
										unexpected_ident!(parser, atom)
									}
								}
								token => unexpected!(parser, token),
							}
						}
						Token::RightParen => None,
						token => unexpected!(parser, token),
					};
					expect!(parser, Token::RightParen);
					parser.advance();
					Float::SnapInlineFunction(length, dir)
				}
				atom => Err(diagnostics::UnexpectedFunction(atom, parser.span()))?,
			},
			token => unexpected!(parser, token),
		};
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for Float {
	fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Left => sink.write_str("left"),
			Self::Right => sink.write_str("right"),
			Self::Top => sink.write_str("top"),
			Self::Bottom => sink.write_str("bottom"),
			Self::BlockStart => sink.write_str("block-start"),
			Self::BlockEnd => sink.write_str("block-end"),
			Self::InlineStart => sink.write_str("inline-start"),
			Self::InlineEnd => sink.write_str("inline-end"),
			Self::SnapBlock => sink.write_str("snap-block"),
			Self::SnapBlockFunction(len, dir) => {
				sink.write_str("snap-block(")?;
				len.write_css(sink)?;
				if let Some(direction) = dir {
					sink.write_char(',')?;
					sink.write_trivia_char(' ')?;
					direction.to_atom().write_css(sink)?;
				}
				sink.write_char(')')
			}
			Self::SnapInline => sink.write_str("snap-inline"),
			Self::SnapInlineFunction(len, dir) => {
				sink.write_str("snap-inline(")?;
				len.write_css(sink)?;
				if let Some(direction) = dir {
					sink.write_char(',')?;
					sink.write_trivia_char(' ')?;
					direction.to_atom().write_css(sink)?;
				}
				sink.write_char(')')
			}
		}
	}
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum SnapBlockDirection {
	Start, // atom!("start")
	End,   // atom!("end")
	Near,  // atom!("near")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum SnapInlineDirection {
	Left,  // atom!("left")
	Right, // atom!("right")
	Near,  // atom!("near")
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Float>(), 12);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<Float>(&allocator, "none", "none");
		test_write::<Float>(&allocator, "left", "left");
		test_write::<Float>(&allocator, "right", "right");
		test_write::<Float>(&allocator, "block-end", "block-end");
		test_write::<Float>(&allocator, "snap-inline(20rem, left)", "snap-inline(20rem,left)");
		test_write::<Float>(&allocator, "snap-block(4px, end)", "snap-block(4px,end)");
	}
}
