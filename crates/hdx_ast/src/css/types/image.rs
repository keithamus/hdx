use hdx_parser::{CursorStream, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use super::Gradient;

mod func {
	use hdx_parser::custom_function;
	custom_function!(Url, atom!("url"));
}

// https://drafts.csswg.org/css-images-3/#typedef-image
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Image<'a> {
	Url(T![Url]),
	UrlFunction(func::Url, T![String], T![')']),
	Gradient(Gradient<'a>),
}

impl<'a> Peek<'a> for Image<'a> {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<T![Url]>() || p.peek::<func::Url>() || p.peek::<Gradient>()
	}
}

impl<'a> Parse<'a> for Image<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Url]>() {
			return Ok(Self::Url(p.parse::<T![Url]>()?));
		}
		if p.peek::<func::Url>() {
			let func = p.parse::<func::Url>()?;
			let string = p.parse::<T![String]>()?;
			let close = p.parse::<T![')']>()?;
			return Ok(Self::UrlFunction(func, string, close));
		}
		p.parse::<Gradient>().map(Self::Gradient)
	}
}

impl<'a> ToCursors<'a> for Image<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Image, 184);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Image, "url('foo')");
		assert_parse!(Image, "url(\"foo\")");
		assert_parse!(Image, "url(foo)");
	}
}
