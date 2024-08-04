use hdx_atom::atom;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, Token};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(MinWidth, atom!("min-width"));
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum HackMediaFeature {
	IEBackslashZero,
}

impl<'a> Parse<'a> for HackMediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		parser.parse::<kw::MinWidth>()?;
		parser.parse::<Token![:]>()?;
		if let Some(token) = parser.peek::<Token![Dimension]>() {
			let str = parser.parse_raw_str(token);
			if str == "0\\0" {
				parser.hop(token);
				return Ok(Self::IEBackslashZero);
			}
		}
		let token = parser.peek::<Token![Any]>().unwrap();
		Err(diagnostics::Unexpected(token, token.span()))?
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
