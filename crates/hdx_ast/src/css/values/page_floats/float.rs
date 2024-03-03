use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, expect, unexpected, unexpected_ident, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::WriteCss;

use crate::{css::values::units::Length, Atomizable, Value};

// https://drafts.csswg.org/css-page-floats-3/#float-property
#[derive(Value, Debug, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
			Self::None => atom!("none").write_css(sink),
			Self::Left => atom!("left").write_css(sink),
			Self::Right => atom!("right").write_css(sink),
			Self::Top => atom!("top").write_css(sink),
			Self::Bottom => atom!("bottom").write_css(sink),
			Self::BlockStart => atom!("block-start").write_css(sink),
			Self::BlockEnd => atom!("block-end").write_css(sink),
			Self::InlineStart => atom!("inline-start").write_css(sink),
			Self::InlineEnd => atom!("inline-end").write_css(sink),
			Self::SnapBlock => atom!("snap-block").write_css(sink),
			Self::SnapBlockFunction(len, dir) => {
				atom!("snap-block").write_css(sink)?;
				sink.write_char('(')?;
				len.write_css(sink)?;
				if let Some(direction) = dir {
					sink.write_char(',')?;
					sink.write_whitespace()?;
					direction.to_atom().write_css(sink)?;
				}
				sink.write_char(')')
			}
			Self::SnapInline => atom!("snap-inline").write_css(sink),
			Self::SnapInlineFunction(len, dir) => {
				atom!("snap-inline").write_css(sink)?;
				sink.write_char('(')?;
				len.write_css(sink)?;
				if let Some(direction) = dir {
					sink.write_char(',')?;
					sink.write_whitespace()?;
					direction.to_atom().write_css(sink)?;
				}
				sink.write_char(')')
			}
		}
	}
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SnapBlockDirection {
	Start, // atom!("start")
	End,   // atom!("end")
	Near,  // atom!("near")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SnapInlineDirection {
	Left,  // atom!("left")
	Right, // atom!("right")
	Near,  // atom!("near")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Float, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Float, "none");
		assert_parse!(Float, "left");
		assert_parse!(Float, "right");
		assert_parse!(Float, "block-end");
		assert_parse!(Float, "snap-inline(20rem, left)");
		assert_parse!(Float, "snap-block(4px, end)");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Float, "snap-inline(20rem, left)", "snap-inline(20rem,left)");
		assert_minify!(Float, "snap-block(4.00px, end)", "snap-block(4px,end)");
	}
}
