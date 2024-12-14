use hdx_lexer::Kind;
use hdx_parser::{CursorSink, Important, Parse, Parser, Result as ParserResult, State, ToCursors, T};

use super::ComponentValues;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Declaration<'a> {
	pub name: T![Ident],
	pub colon: Option<T![:]>,
	pub value: ComponentValues<'a>,
	pub important: Option<Important>,
	pub semicolon: Option<T![;]>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-declaration
impl<'a> Parse<'a> for Declaration<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let name = p.parse::<T![Ident]>()?;
		let colon = p.parse::<T![:]>().ok();
		let old_stop = p.stop;
		p.stop = p.stop.add(Kind::Semicolon);
		let old_state = p.set_state(State::Nested);
		let value = p.parse::<ComponentValues>().inspect_err(|_| {
			p.set_state(old_state);
			p.stop = old_stop;
		})?;
		p.set_state(old_state);
		p.stop = old_stop;
		let important = None;
		// TODO: figure out a nice way to extract the last two ComponentValues
		// to check if they are !important, and push them into the important value.
		Ok(Self { name, colon, value, important, semicolon: p.parse_if_peek::<T![;]>()? })
	}
}

impl<'a> ToCursors for Declaration<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		if let Some(t) = self.colon {
			s.append(t.into());
		}
		ToCursors::to_cursors(&self.value, s);
		if let Some(t) = self.important {
			ToCursors::to_cursors(&t, s);
		}
		if let Some(t) = self.semicolon {
			s.append(t.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Declaration, 104);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Declaration, "color:black;");
	}
}
