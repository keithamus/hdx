use hdx_atom::atom;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, T};
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
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<kw::MinWidth>()?;
		p.parse::<T![:]>()?;
		if let Some(token) = p.peek::<T![Dimension]>() {
			let str = p.parse_raw_str(token);
			if str == "0\\0" {
				p.hop(token);
				return Ok(Self::IEBackslashZero);
			}
		}
		let token = p.peek::<T![Any]>().unwrap();
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
