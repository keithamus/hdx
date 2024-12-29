use css_lexer::Cursor;
use css_parse::{diagnostics, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::{Visit, Visitable};

// https://drafts.csswg.org/css-syntax-3/#charset-rule
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CharsetRule {
	at_keyword: T![AtKeyword],
	space: T![' '],
	string: T![String],
	semicolon: Option<T![;]>,
}

// CharsetRule is a special rule which means it cannot use standard AtRule parsing... comments below
// https://drafts.csswg.org/css-syntax-3/#determine-the-fallback-encoding
impl<'a> Parse<'a> for CharsetRule {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let at_keyword = p.parse::<T![AtKeyword]>()?;
		let c: Cursor = at_keyword.into();
		// CharsetRule MUST be all lowercase, alt cases such as CHARSET or charSet aren't
		// valid here, compares to other at-rules which are case insensitive.
		if !p.eq_ignore_ascii_case(c, "charset") {
			Err(diagnostics::UnexpectedAtRule(p.parse_str(c).into(), c.into()))?;
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

impl<'a> ToCursors for CharsetRule {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		s.append(self.at_keyword.into());
		s.append(self.space.into());
		s.append(self.string.into());
		if let Some(semicolon) = self.semicolon {
			s.append(semicolon.into());
		}
	}
}

impl<'a> Visitable<'a> for CharsetRule {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_charset_rule(self);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CharsetRule>(), 52);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CharsetRule, "@charset \"utf-8\";", "@charset \"utf-8\";");
		assert_parse!(CharsetRule, "@charset \"UTF-8\";", "@charset \"UTF-8\";");
	}
}
