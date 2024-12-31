use crate::{
	diagnostics,
	syntax::{FunctionBlock, SimpleBlock},
	CursorSink, Parse, Parser, Peek, Result as ParserResult, State, ToCursors, T,
};
use css_lexer::{Cursor, Kind, KindSet};

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
// A compatible "Token" per CSS grammar, subsetted to the tokens possibly
// rendered by ComponentValue (so no pairwise, function tokens, etc).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(SimpleBlock<'a>),
	Function(FunctionBlock<'a>),
	Whitespace(T![Whitespace]),
	Number(T![Number]),
	Dimension(T![Dimension]),
	Ident(T![Ident]),
	AtKeyword(T![AtKeyword]),
	Hash(T![Hash]),
	String(T![String]),
	Url(T![Url]),
	Delim(T![Delim]),
	Colon(T![:]),
	Semicolon(T![;]),
	Comma(T![,]),
}

impl<'a> Peek<'a> for ComponentValue<'a> {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		let kindset = KindSet::new(&[
			Kind::Whitespace,
			Kind::Number,
			Kind::Dimension,
			Kind::Ident,
			Kind::AtKeyword,
			Kind::Hash,
			Kind::String,
			Kind::Url,
			Kind::Delim,
			Kind::Colon,
			Kind::Semicolon,
			Kind::Comma,
			Kind::Function,
			Kind::LeftCurly,
			Kind::LeftParen,
			Kind::LeftSquare,
		]);
		c == kindset
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![' ']>() {
			p.parse::<T![' ']>().map(Self::Whitespace)
		} else if p.peek::<T![PairWiseStart]>() {
			let old_state = p.set_state(State::Nested);
			let block = p.parse::<SimpleBlock>();
			p.set_state(old_state);
			Ok(Self::SimpleBlock(block?))
		} else if p.peek::<T![Function]>() {
			p.parse::<FunctionBlock>().map(Self::Function)
		} else if p.peek::<T![Number]>() {
			p.parse::<T![Number]>().map(Self::Number)
		} else if p.peek::<T![Dimension]>() {
			p.parse::<T![Dimension]>().map(Self::Dimension)
		} else if p.peek::<T![Ident]>() {
			p.parse::<T![Ident]>().map(Self::Ident)
		} else if p.peek::<T![AtKeyword]>() {
			p.parse::<T![AtKeyword]>().map(Self::AtKeyword)
		} else if p.peek::<T![Hash]>() {
			p.parse::<T![Hash]>().map(Self::Hash)
		} else if p.peek::<T![String]>() {
			p.parse::<T![String]>().map(Self::String)
		} else if p.peek::<T![Url]>() {
			p.parse::<T![Url]>().map(Self::Url)
		} else if p.peek::<T![Delim]>() {
			p.parse::<T![Delim]>().map(Self::Delim)
		} else if p.peek::<T![:]>() {
			p.parse::<T![:]>().map(Self::Colon)
		} else if p.peek::<T![;]>() {
			p.parse::<T![;]>().map(Self::Semicolon)
		} else if p.peek::<T![,]>() {
			p.parse::<T![,]>().map(Self::Comma)
		} else {
			let c = p.next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

impl<'a> ToCursors for ComponentValue<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::SimpleBlock(t) => ToCursors::to_cursors(t, s),
			Self::Function(t) => ToCursors::to_cursors(t, s),
			Self::Ident(t) => s.append(t.into()),
			Self::AtKeyword(t) => s.append(t.into()),
			Self::Hash(t) => s.append(t.into()),
			Self::String(t) => s.append(t.into()),
			Self::Url(t) => s.append(t.into()),
			Self::Delim(t) => s.append(t.into()),
			Self::Number(t) => s.append(t.into()),
			Self::Dimension(t) => s.append(t.into()),
			Self::Whitespace(t) => s.append(t.into()),
			Self::Colon(t) => s.append(t.into()),
			Self::Semicolon(t) => s.append(t.into()),
			Self::Comma(t) => s.append(t.into()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ComponentValue>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ComponentValue, "foo");
		assert_parse!(ComponentValue, " ");
		assert_parse!(ComponentValue, "{block}");
	}
}
