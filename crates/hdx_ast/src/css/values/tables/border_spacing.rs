use crate::css::units::Length;
use hdx_derive::Value;
use hdx_parser::{Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};

#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderSpacing(Length, Length);

impl<'a> Parse<'a> for BorderSpacing {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let horiz = Length::parse(parser)?;
		if let Ok(vert) = Length::try_parse(parser) {
			Ok(Self(horiz, vert))
		} else {
			Ok(Self(horiz, horiz))
		}
	}
}

impl<'a> WriteCss<'a> for BorderSpacing {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)?;
		if self.0 != self.1 || sink.can_output(OutputOption::RedundantShorthandValues) {
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
