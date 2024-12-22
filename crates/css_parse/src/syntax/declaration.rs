use css_lexer::Kind;

use crate::{
	syntax::{BangImportant, ComponentValues},
	CursorSink, Declaration as DeclarationTrait, Parse, Parser, Peek, Result, ToCursors, T,
};

/// Represents a generic "Declaration".
///
/// This [consumes a declaration][1] using the [Declaration][crate::Declaration] trait implementation.
/// The name is any ident, and the value is a list of component values. It's better to define discrete structs for each
/// known declaration, but this can serve as an unknown declaration for unrecognised declarations or as the generic
/// Declaration in an unknown rule.
///
/// ```md
/// <declaration>
///  │├─ <ident> ─ ":" ─ <component-values> ──╮─────────────────────────────╭─┤│
///                                           ╰─ "!" ─ <ident "important"> ─╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#consume-a-declaration
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Declaration<'a> {
	pub name: T![Ident],
	pub colon: T![:],
	pub value: ComponentValues<'a>,
	pub important: Option<BangImportant>,
}

impl<'a> DeclarationTrait<'a> for Declaration<'a> {
	type DeclarationValue = ComponentValues<'a>;
}

impl<'a> Peek<'a> for Declaration<'a> {
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		c == Kind::Ident && p.peek_n(2) == Kind::Colon
	}
}

impl<'a> Parse<'a> for Declaration<'a> {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let (name, colon, value, important) = Self::parse_declaration(p)?;
		Ok(Self { name, colon, value, important })
	}
}

impl<'a> ToCursors for Declaration<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		s.append(self.colon.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(t) = self.important {
			ToCursors::to_cursors(&t, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Declaration>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Declaration, "color:black;");
	}
}
