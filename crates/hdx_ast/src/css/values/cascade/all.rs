use hdx_parser::{Parse, Parser, Spanned, Result as ParserResult, Span};
use hdx_writer::{WriteCss, CssWriter, Result as WriterResult};
#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-cascade-5/#propdef-all
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct All();

impl<'a> Parse<'a> for All {
	fn parse(_parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		// All has no defined grammay beyond the global keywords
		Ok(Self().spanned(Span::dummy()))
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
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<All>(), 0);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<All>(&allocator, "", "");
	}
}
