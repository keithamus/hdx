use hdx_lexer::{SourceOffset, Token};
use hdx_parser::{CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::ComponentValues;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct SimpleBlock<'a> {
	start: SourceOffset,
	pub open: T![PairWiseStart],
	pub values: ComponentValues<'a>,
	pub close: Option<T![PairWiseEnd]>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let open = p.parse::<T![PairWiseStart]>()?;
		let values = p.parse::<ComponentValues>()?;
		if p.peek::<T![PairWiseEnd]>() {
			return Ok(Self { start, open, values, close: p.parse::<T![PairWiseEnd]>().ok() });
		}
		Ok(Self { start, open, values, close: None })
	}
}

impl<'a> ToCursors for SimpleBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(Into::<Token>::into(self.open).with_cursor(self.start));
		ToCursors::to_cursors(&self.values, s);
		if let Some(close) = self.close {
			s.append(close.into())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(SimpleBlock, 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SimpleBlock, "[foo]");
		assert_parse!(SimpleBlock, "(one two three)");
		assert_parse!(SimpleBlock, "{}");
		assert_parse!(SimpleBlock, "{foo}");
		assert_parse!(SimpleBlock, "{foo:bar}");
		assert_parse!(SimpleBlock, "{one(two)}");
		assert_parse!(SimpleBlock, "(one(two))");
		// Incomplete but recoverable
		assert_parse!(SimpleBlock, "[foo");
		assert_parse!(SimpleBlock, "{foo:bar");
		assert_parse!(SimpleBlock, "(one(two)");
		assert_parse!(SimpleBlock, "(one(two");
	}
}
