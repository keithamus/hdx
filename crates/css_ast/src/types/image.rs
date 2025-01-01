use css_lexer::Cursor;
use css_parse::{diagnostics, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use super::Gradient;

// https://drafts.csswg.org/css-images-3/#typedef-image
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Image<'a> {
	Url(T![Url]),
	UrlFunction(T![Function], T![String], T![')']),
	Gradient(Gradient<'a>),
}

impl<'a> Peek<'a> for Image<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Url]>::peek(p, c)
			|| <Gradient>::peek(p, c)
			|| (<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "url"))
	}
}

impl<'a> Parse<'a> for Image<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Url]>() {
			return Ok(Self::Url(p.parse::<T![Url]>()?));
		} else if p.peek::<Gradient>() {
			return Ok(Self::Gradient(p.parse::<Gradient>()?));
		} else {
			let func = p.parse::<T![Function]>()?;
			if !p.eq_ignore_ascii_case(func.into(), "url") {
				Err(diagnostics::UnexpectedFunction(p.parse_str(func.into()).into(), func.into()))?
			}
			let string = p.parse::<T![String]>()?;
			let close = p.parse::<T![')']>()?;
			return Ok(Self::UrlFunction(func, string, close));
		}
	}
}

impl<'a> ToCursors for Image<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Url(c) => s.append(c.into()),
			Self::UrlFunction(func, string, close) => {
				s.append(func.into());
				s.append(string.into());
				s.append(close.into());
			}
			Self::Gradient(c) => ToCursors::to_cursors(c, s),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Image>(), 208);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Image, "url('foo')");
		assert_parse!(Image, "url(\"foo\")");
		assert_parse!(Image, "url(foo)");
	}
}
