use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(MinWidth, atom!("min-width"));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum HackMediaFeature {
	IEBackslashZero(kw::MinWidth, T![:], T![Dimension]),
}

impl<'a> Parse<'a> for HackMediaFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let keyword = p.parse::<kw::MinWidth>()?;
		let colon = p.parse::<T![:]>()?;
		let dimension = p.parse::<T![Dimension]>()?;
		let c: Cursor = dimension.into();
		let str = p.parse_raw_str(c);
		if str != "0\\0" {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		Ok(Self::IEBackslashZero(keyword, colon, dimension))
	}
}

impl<'a> ToCursors for HackMediaFeature {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::IEBackslashZero(keyword, colon, dimension) => {
				s.append(keyword.into());
				s.append(colon.into());
				s.append(dimension.into());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(HackMediaFeature, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HackMediaFeature, "min-width:0\\0");
	}
}
