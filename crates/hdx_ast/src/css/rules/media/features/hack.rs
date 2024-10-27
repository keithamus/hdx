use hdx_atom::atom;
use hdx_lexer::Kind;
use hdx_parser::{expect, expect_ignore_case, peek, unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum HackMediaFeature {
	IEBackslashZero,
}

impl<'a> Parse<'a> for HackMediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case! { parser.next(), Token::Ident(_):
			atom!("min-width") => {
				expect!(parser.next(), Kind::Colon);
				let (a, b, c, d, e) = (
					parser.legacy_peek_next_char(0),
					parser.legacy_peek_next_char(1),
					parser.legacy_peek_next_char(2),
					parser.legacy_peek_next_char(3),
					parser.legacy_peek_next_char(4)
				);
				if peek!(parser, Kind::Dimension) &&
				(matches!((a, b, c, d), (Some('0'), Some('\\'), Some('0'), Some(' ') | Some(')') | None))) ||
				(matches!((a, b, c, d, e), (Some(' '), Some('0'), Some('\\'), Some('0'), Some(' ') | Some(')') | None)))
				{
					parser.next();
					return Ok(Self::IEBackslashZero);
				}
				unexpected!(parser, parser.peek())
			}
		}
	}
}

impl<'a> WriteCss<'a> for HackMediaFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::IEBackslashZero => write_css!(sink, atom!("min-width"), ':', (), '0', '\\', '0'),
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(HackMediaFeature, 0);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HackMediaFeature, "min-width: 0\\0");
		assert_parse!(HackMediaFeature, "min-width:0\\0", "min-width: 0\\0");
	}
}
