use crate::{diagnostics, CursorSink, Parse, Parser, Peek, Result, ToCursors, T};
use css_lexer::{Cursor, Kind};

/// Represents a two tokens, the first being [Kind::Delim] where the char is `!`, and the second being an `Ident` with
/// the value `important`. [CSS defines this as]:
///
/// ```md
/// <ws*>
///     ╭──────────────────────────╮
///  │├─╯─╭─ <whitespace-token> ─╮─╰─┤│
///       ╰──────────────────────╯
///
/// <!important>
///  │├─ "!" ─ <ws*> ─ <ident-token "important"> ─ <ws*> ─┤│
/// ```
///
/// `<ws*>` is any number of `<whitespace-token>`s, defined as [Kind::Whitespace][css_lexer::Kind::Whitespace]. This is
/// automatically skipped by default in the [Parser] anyway.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#!important-diagram
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BangImportant {
	pub bang: T![!],
	pub important: T![Ident],
}

impl<'a> Peek<'a> for BangImportant {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		if c == Kind::Delim && c == '!' {
			let c = p.peek_n(2);
			c == Kind::Ident && p.eq_ignore_ascii_case(c, "important")
		} else {
			false
		}
	}
}

impl<'a> Parse<'a> for BangImportant {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let bang = p.parse::<T![!]>()?;
		let important = p.parse::<T![Ident]>()?;
		if !p.eq_ignore_ascii_case(important.into(), "important") {
			Err(diagnostics::ExpectedIdentOf("important", p.parse_str(important.into()).into(), important.into()))?
		}
		Ok(Self { bang, important })
	}
}

impl<'a> ToCursors for BangImportant {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.bang.into());
		s.append(self.important.into());
	}
}
