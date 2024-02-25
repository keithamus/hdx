use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Atomizable, Value};

// https://drafts.csswg.org/css-box-4/#propdef-margin-trim
#[derive(Atomizable, Default)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MarginTrim {
	#[default]
	None,
	Block,
	Inline,
	BlockStart,
	BlockEnd,
	InlineStart,
	InlineEnd,
}

impl Value for MarginTrim {}

impl<'a> Parse<'a> for MarginTrim {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let mut value = Self::none();
		loop {
			if value.is_all() {
				break;
			}
			match parser.cur() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("block") => value |= MarginTrim::Block,
					atom!("inline") => value |= MarginTrim::Inline,
					atom!("block-start") => value |= MarginTrim::BlockStart,
					atom!("block-end") => value |= MarginTrim::BlockEnd,
					atom!("inline-start") => value |= MarginTrim::InlineStart,
					atom!("inline-end") => value |= MarginTrim::InlineEnd,
					atom => unexpected_ident!(parser, atom),
				},
				token => unexpected!(parser, token),
			}
			parser.advance();
			if value == Self::None || value == Self::Block || value == Self::Inline {
				break;
			}
		}
		if value.is_none() {
			unexpected!(parser);
		}
		Ok(value.spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for MarginTrim {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.contains(Self::BlockStart) {
			sink.write_str("block-start")?;
		}
		if self.contains(Self::BlockEnd) {
			if self.intersects(Self::BlockStart) {
				sink.write_char(' ')?;
			}
			sink.write_str("block-end")?;
		}
		if self.contains(Self::InlineStart) {
			if self.intersects(Self::BlockStart | Self::BlockEnd) {
				sink.write_char(' ')?;
			}
			sink.write_str("inline-start")?;
		}
		if self.contains(Self::InlineEnd) {
			if self.intersects(Self::BlockStart | Self::BlockEnd | Self::InlineStart) {
				sink.write_char(' ')?;
			}
			sink.write_str("inline-end")?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<MarginTrim>(), 1);
	}
}
