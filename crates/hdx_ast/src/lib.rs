pub mod css;
pub mod specificity;
pub mod syntax;
pub mod traits;

#[cfg(test)]
pub mod test_helpers;

use hdx_parser::{CursorStream, Parse, Parser, Result as ParserResult, ToCursors};
pub use traits::StyleValue;

// TODO! - delete this when we're done ;)
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum Todo {
	#[default]
	Todo,
}

impl<'a> Parse<'a> for Todo {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		todo!("{p:?}")
	}
}

impl<'a> ToCursors<'a> for Todo {
	fn to_cursors(&self, _: &mut CursorStream<'a>) {
		todo!();
	}
}
