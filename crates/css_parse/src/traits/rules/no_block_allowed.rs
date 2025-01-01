use crate::{diagnostics, CursorSink, Parse, Parser, Peek, Result, ToCursors, T};

/// A struct to provide to [AtRule][crate::AtRule] to disallow blocks.
///
/// Sometimes [AtRules][crate::AtRule] do not have a block - for example `@charset`, `@import`. In those case, assigning
/// this struct to the [AtRule::Block] can be useful to ensure that the [AtRule] appropriately errors if it enters the
/// Block parsing context. This captures the `;` token that may optionally end a "statement-style" at-rule.
pub struct NoBlockAllowed(Option<T![;]>);
impl<'a> Parse<'a> for NoBlockAllowed {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.at_end() {
			Ok(Self(None))
		} else if let Some(semicolon) = p.parse_if_peek::<T![;]>()? {
			Ok(Self(Some(semicolon)))
		} else {
			let c = p.peek_next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

impl<'a> Peek<'a> for NoBlockAllowed {
	fn peek(_: &Parser<'a>, _: css_lexer::Cursor) -> bool {
		false
	}
}

impl ToCursors for NoBlockAllowed {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(semicolon) = self.0 {
			s.append(semicolon.into());
		}
	}
}
