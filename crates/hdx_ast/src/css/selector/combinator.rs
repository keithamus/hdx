use hdx_parser::{CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

// https://drafts.csswg.org/selectors/#combinators
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum Combinator {
	Child(T![>]),
	NextSibling(T![+]),
	SubsequentSibling(T![~]),
	Column(T![||]),
	Nesting(T![&]),
	Descendant(T![' ']),
}

impl<'a> Parse<'a> for Combinator {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![>]>() {
			Ok(Self::Child(p.parse::<T![>]>()?))
		} else if p.peek::<T![+]>() {
			Ok(Self::NextSibling(p.parse::<T![+]>()?))
		} else if p.peek::<T![~]>() {
			Ok(Self::SubsequentSibling(p.parse::<T![~]>()?))
		} else if p.peek::<T![&]>() {
			Ok(Self::Nesting(p.parse::<T![&]>()?))
		} else if p.peek::<T![||]>() {
			Ok(Self::Column(p.parse::<T![||]>()?))
		} else {
			Ok(Self::Descendant(p.parse::<T![' ']>()?))
		}
	}
}

impl<'a> ToCursors for Combinator {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Descendant(c) => s.append(c.into()),
			Self::Child(c) => s.append(c.into()),
			Self::NextSibling(c) => s.append(c.into()),
			Self::SubsequentSibling(c) => s.append(c.into()),
			Self::Column(c) => ToCursors::to_cursors(c, s),
			Self::Nesting(c) => s.append(c.into()),
		}
	}
}

impl<'a> Visitable<'a> for Combinator {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Combinator, 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Combinator, ">");
		assert_parse!(Combinator, "+");
		assert_parse!(Combinator, "~");
		assert_parse!(Combinator, "&");
		// Descendent combinator
		assert_parse!(Combinator, "     ");
		assert_parse!(Combinator, "     ");
		assert_parse!(Combinator, "  /**/   /**/   /**/ ", "  ");
		// Column
		assert_parse!(Combinator, "||");
	}
}
