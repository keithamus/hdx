mod properties;
mod rules;
mod selector;
mod specificity;
mod stylerule;
mod stylesheet;
mod traits;
mod types;
mod units;
mod values;
mod visit;

pub use properties::*;
pub use rules::*;
pub use selector::*;
pub use stylerule::*;
pub use stylesheet::*;
pub use types::*;
pub use units::*;
pub use values::*;
pub use visit::*;

use css_lexer::Span;
use css_parse::{diagnostics, CursorSink, Parse, Parser, Result as ParserResult, ToCursors};

// TODO! - delete this when we're done ;)
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum Todo {
	#[default]
	Todo,
}

impl<'a> Parse<'a> for Todo {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Err(diagnostics::Unimplemented(Span::new(p.offset(), p.offset())))?
	}
}

impl ToCursors for Todo {
	fn to_cursors(&self, _: &mut impl CursorSink) {}
}

impl<'a> Visitable<'a> for Todo {
	fn accept<V: Visit<'a>>(&self, _: &mut V) {}
}
