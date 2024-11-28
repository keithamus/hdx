use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, ToCursors, T};

// https://drafts.csswg.org/css-syntax-3/#charset-rule
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Charset {
	at_keyword: T![AtKeyword],
	space: T![' '],
	string: T![String],
	semicolon: Option<T![;]>,
}

// Charset is a special rule which means it cannot use standard AtRule parsing... comments below
// https://drafts.csswg.org/css-syntax-3/#determine-the-fallback-encoding
impl<'a> Parse<'a> for Charset {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let at_keyword = p.parse::<T![AtKeyword]>()?;
		let c: Cursor = at_keyword.into();
		// Charset MUST be all lowercase, alt cases such as CHARSET or charSet aren't
		// valid here, compares to other at-rules which are case insensitive.
		let atom = p.parse_atom(c);
		if atom != atom!("charset") {
			Err(diagnostics::UnexpectedAtRule(atom, c.into()))?;
		}
		// Charsets MUST have a space between the at keyword and the string. This
		// isn't necessary in other at rules where an at keyword can align with other
		// delims (e.g. `(`) or unambinguous tokens like strings.
		let space = p.parse::<T![' ']>()?;
		let string = p.parse::<T![String]>()?;
		// TODO: check quote style as it should be "
		let semicolon = p.parse::<T![;]>().ok();
		Ok(Self { at_keyword, space, string, semicolon })
	}
}

impl<'a> ToCursors<'a> for Charset {
	fn to_cursors(&self, s: &mut hdx_parser::CursorStream<'a>) {
		s.append(self.at_keyword.into());
		s.append(self.space.into());
		s.append(self.string.into());
		if let Some(semicolon) = self.semicolon {
			s.append(semicolon.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Charset, 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Charset, "@charset \"utf-8\";", "@charset \"utf-8\";");
		assert_parse!(Charset, "@charset \"UTF-8\";", "@charset \"UTF-8\";");
	}
}
