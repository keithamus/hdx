use crate::{diagnostics, Parse, Parser, Result, ToCursors, T};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum Comparison {
	LessThan(T![<]),
	GreaterThan(T![>]),
	GreaterThanEqual(T![>=]),
	LessThanEqual(T![<=]),
	Equal(T![=]),
}

impl<'a> Parse<'a> for Comparison {
	fn parse(p: &mut Parser<'a>) -> Result<Comparison> {
		let c = p.peek_next();
		match c.token().char() {
			Some('=') => p.parse::<T![=]>().map(Comparison::Equal),
			Some('>') => {
				if p.peek::<T![>=]>() {
					p.parse::<T![>=]>().map(Comparison::GreaterThanEqual)
				} else {
					p.parse::<T![>]>().map(Comparison::GreaterThan)
				}
			}
			Some('<') => {
				if p.peek::<T![<=]>() {
					p.parse::<T![<=]>().map(Comparison::LessThanEqual)
				} else {
					p.parse::<T![<]>().map(Comparison::LessThan)
				}
			}
			Some(char) => Err(diagnostics::UnexpectedDelim(char, c.into()))?,
			_ => Err(diagnostics::Unexpected(c.into(), c.into()))?,
		}
	}
}

impl<'a> ToCursors<'a> for Comparison {
	fn to_cursors(&self, s: &mut crate::CursorStream<'a>) {
		match self {
			Self::LessThan(c) => s.append(c.into()),
			Self::GreaterThan(c) => s.append(c.into()),
			Self::GreaterThanEqual(c) => ToCursors::to_cursors(c, s),
			Self::LessThanEqual(c) => ToCursors::to_cursors(c, s),
			Self::Equal(c) => s.append(c.into()),
		}
	}
}
