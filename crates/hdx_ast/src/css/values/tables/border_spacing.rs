use crate::css::values::units::Length;
use hdx_parser::{unexpected, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss, OutputOption};

use crate::Value;

#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderSpacing(Length, Length);

impl<'a> Parse<'a> for BorderSpacing {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(horiz) = Length::from_token(parser.cur()) {
			parser.advance();
			if let Some(vert) = Length::from_token(parser.cur()) {
				parser.advance();
				Ok(Self(horiz, vert))
			} else {
				Ok(Self(horiz, horiz))
			}
		} else {
			unexpected!(parser)
		}
	}
}

impl<'a> WriteCss<'a> for BorderSpacing {
    fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)?;
		if self.0 != self.1 || sink.can_output(OutputOption::RedundantRules) {
			sink.write_char(' ')?;
			self.1.write_css(sink)?;
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
		assert_size!(BorderSpacing, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderSpacing, "0px 0px");
		assert_parse!(BorderSpacing, "15px 11px");
	}

	#[test]
	fn test_minify() {
		assert_minify!(BorderSpacing, "0px 0px", "0px");
		assert_minify!(BorderSpacing, "12px 12px", "12px");
	}
}
