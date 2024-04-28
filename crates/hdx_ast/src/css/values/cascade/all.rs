use hdx_derive::Value;
use hdx_parser::{Parse, Parser, Result as ParserResult, Span, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-cascade-5/#propdef-all
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct All();

impl<'a> Parse<'a> for All {
	fn parse(_parser: &mut Parser<'a>) -> ParserResult<Self> {
		// All has no defined grammay beyond the global keywords
		Ok(Self())
	}
	fn parse_spanned(_parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		Ok(Spanned { node: Self(), span: Span::dummy() })
	}
}

impl<'a> WriteCss<'a> for All {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		// All has no defined keywords, other than the globals
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(All, 0);
	}

	#[test]
	fn test_writes() {
		assert_parse!(All, "");
	}
}
