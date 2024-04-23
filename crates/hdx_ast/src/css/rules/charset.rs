use crate::Atomizable;
use hdx_atom::atom;
use hdx_lexer::{Include, QuoteStyle, Token};
use hdx_parser::{
	diagnostics::{self},
	expect, unexpected, Parse, Parser, Result as ParserResult,
};
use hdx_writer::{write_css, CssWriter, OutputOption, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-syntax-3/#charset-rule
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum Charset {
	#[atomizable("utf-8")]
	Utf8,
	#[atomizable("us-ascii")]
	UsAscii,
	#[atomizable("iso-8859-1")]
	Iso88591,
	#[atomizable("iso-8859-2")]
	Iso88592,
	#[atomizable("iso-8859-3")]
	Iso88593,
	#[atomizable("iso-8859-4")]
	Iso88594,
	#[atomizable("iso-8859-5")]
	Iso88595,
	#[atomizable("iso-8859-6")]
	Iso88596,
	#[atomizable("iso-8859-7")]
	Iso88597,
	#[atomizable("iso-8859-8")]
	Iso88598,
	#[atomizable("iso-8859-9")]
	Iso88599,
	#[atomizable("iso-8859-10")]
	Iso885910,
	#[atomizable("shift_jis")]
	ShiftJis,
	#[atomizable("euc-jp")]
	EucJp,
	#[atomizable("iso-2022-kr")]
	Iso2022Kr,
	#[atomizable("euc-kr")]
	EucKr,
	#[atomizable("iso-2022-jp")]
	Iso2022Jp,
	#[atomizable("iso-2022-jp-2")]
	Iso2022Jp2,
	#[atomizable("iso-8859-6-e")]
	Iso88596E,
	#[atomizable("iso-8859-6-i")]
	Iso88596I,
	#[atomizable("iso-8859-8-e")]
	Iso88598E,
	#[atomizable("iso-8859-8-i")]
	Iso88598I,
	#[atomizable("gb2312")]
	Gb2312,
	#[atomizable("big5")]
	Big5,
	#[atomizable("koi8-r")]
	Koi8R,
}

impl<'a> Parse<'a> for Charset {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser.next(), Token::AtKeyword(atom!("charset")));
		expect!(parser.next_with(Include::Whitespace), Token::Whitespace);
		match parser.next_with(Include::Whitespace) {
			Token::String(atom, QuoteStyle::Double) => {
				if let Some(rule) = Self::from_atom(atom) {
					expect!(parser.next_with(Include::Whitespace), Token::Semicolon);
					Ok(rule)
				} else {
					Err(diagnostics::UnexpectedCharset(atom.clone(), parser.span()))?
				}
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Charset {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if !matches!(self, Charset::Utf8) || sink.can_output(OutputOption::RedundantRules) {
			write_css!(sink, '@', atom!("charset"), ' ', '"', self.to_atom(), '"', ';');
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
		assert_size!(Charset, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Charset, "@charset \"utf-8\";", "@charset \"utf-8\";");
		assert_parse!(Charset, "@charset \"UTF-8\";", "@charset \"utf-8\";");
	}

	#[test]
	fn test_minify() {
		// utf-8 is assumed, so we can drop the rule.
		assert_minify!(Charset, "@charset \"utf-8\";", "");
	}
}
