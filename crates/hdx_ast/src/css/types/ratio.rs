use crate::css::units::CSSInt;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-values-4/#ratios
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Ratio(pub CSSInt, pub CSSInt);

impl<'a> Peek<'a> for Ratio {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Number]>()
	}
}

impl<'a> Parse<'a> for Ratio {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Number]>()?;
		let a: CSSInt = p.parse_number(token).into();
		let b: CSSInt = if let Some(token) = p.peek::<T![/]>() {
			p.hop(token);
			let token = *p.parse::<T![Number]>()?;
			p.parse_number(token).into()
		} else {
			1.into()
		};
		Ok(Self(a, b))
	}
}

impl<'a> WriteCss<'a> for Ratio {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		Ok(write_css!(sink, self.0, (), '/', (), self.1))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Ratio, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Ratio, "1 / 1", "1 / 1");
		assert_parse!(Ratio, "5 / 3", "5 / 3");
		assert_parse!(Ratio, "5", "5 / 1");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Ratio, "5 : 3");
		assert_parse_error!(Ratio, "5 / 1 / 1");
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_serializes() {
		assert_json!(Ratio, "5 / 3", {
			"node": [5, 3],
			"start": 0,
			"end": 5
		});
	}
}
