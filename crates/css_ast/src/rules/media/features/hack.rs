use css_lexer::Cursor;
use css_parse::{diagnostics, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum HackMediaFeature {
	IEBackslashZero(T!['('], T![Ident], T![:], T![Dimension], T![')']),
}

impl<'a> Parse<'a> for HackMediaFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse::<T!['(']>()?;
		let keyword = p.parse::<T![Ident]>()?;
		if !p.eq_ignore_ascii_case(keyword.into(), "min-width") {
			Err(diagnostics::UnexpectedIdent(p.parse_str(keyword.into()).into(), keyword.into()))?
		}
		let colon = p.parse::<T![:]>()?;
		let dimension = p.parse::<T![Dimension]>()?;
		let c: Cursor = dimension.into();
		let str = p.parse_raw_str(c);
		if str != "0\\0" {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		let close = p.parse::<T![')']>()?;
		Ok(Self::IEBackslashZero(open, keyword, colon, dimension, close))
	}
}

impl<'a> ToCursors for HackMediaFeature {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::IEBackslashZero(open, keyword, colon, dimension, close) => {
				s.append(open.into());
				s.append(keyword.into());
				s.append(colon.into());
				s.append(dimension.into());
				s.append(close.into());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HackMediaFeature>(), 60);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HackMediaFeature, "(min-width:0\\0)");
	}
}
