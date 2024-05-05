use hdx_atom::{atom, Atomizable};
use hdx_derive::{Atomizable, Value};
use hdx_lexer::{Kind, Token};
use hdx_parser::{diagnostics, expect, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::Length;

// https://drafts.csswg.org/css-page-floats-3/#float-property
#[derive(Value, Debug, PartialEq, Default, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
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
		let value = match parser.next() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("none") => Self::None,
				atom!("left") => Self::Left,
				atom!("right") => Self::Right,
				atom!("top") => Self::Top,
				atom!("bottom") => Self::Bottom,
				atom!("block-start") => Self::BlockStart,
				atom!("block-end") => Self::BlockEnd,
				atom!("inline-start") => Self::InlineStart,
				atom!("inline-end") => Self::InlineEnd,
				atom!("snap-block") => Self::SnapBlock,
				atom!("snap-inline") => Self::SnapInline,
				atom => unexpected_ident!(parser, atom),
			},
			Token::Function(atom) => match atom.to_ascii_lowercase() {
				atom!("snap-block") => {
					let length = Length::parse(parser)?;
					match parser.next() {
						Token::Comma => match parser.next() {
							Token::Ident(atom) => {
								if let Some(dir) = SnapBlockDirection::from_atom(&atom.to_ascii_lowercase()) {
									expect!(parser.next(), Kind::RightParen);
									Self::SnapBlockFunction(length, Some(dir))
								} else {
									unexpected_ident!(parser, atom)
								}
							}
							token => unexpected!(parser, token),
						},
						Token::RightParen => Self::SnapBlockFunction(length, None),
						token => unexpected!(parser, token),
					}
				}
				atom!("snap-inline") => {
					let length = Length::parse(parser)?;
					match parser.next() {
						Token::Comma => match parser.next() {
							Token::Ident(atom) => {
								if let Some(dir) = SnapInlineDirection::from_atom(&atom.to_ascii_lowercase()) {
									expect!(parser.next(), Kind::RightParen);
									Self::SnapInlineFunction(length, Some(dir))
								} else {
									unexpected_ident!(parser, atom)
								}
							}
							token => unexpected!(parser, token),
						},
						Token::RightParen => Self::SnapInlineFunction(length, None),
						token => unexpected!(parser, token),
					}
				}
				atom => Err(diagnostics::UnexpectedFunction(atom, parser.span()))?,
			},
			token => unexpected!(parser, token),
		};
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for Float {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
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

#[derive(Atomizable, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SnapBlockDirection {
	Start, // atom!("start")
	End,   // atom!("end")
	Near,  // atom!("near")
}

#[derive(Atomizable, Debug, PartialEq, Clone, Hash)]
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
