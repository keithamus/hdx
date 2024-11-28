use core::fmt;

use crate::{diagnostics, parser::Parser, CursorStream, Is, Parse, Peek, Result, ToCursors, T};
use hdx_atom::Atom;
use hdx_lexer::{Cursor, SourceOffset, Token};

mod kw {
	use crate::custom_keyword;
	custom_keyword!(Important, atom!("important"));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Important {
	pub bang: T![!],
	pub important: kw::Important,
}

impl<'a> Peek<'a> for Important {
	fn peek(p: &Parser<'a>) -> bool {
		<T![!]>::is(p, p.peek_n(1)) && <kw::Important>::is(p, p.peek_n(2))
	}
}

impl<'a> Parse<'a> for Important {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let bang = p.parse::<T![!]>()?;
		let important = p.parse::<kw::Important>()?;
		Ok(Self { bang, important })
	}
}

impl<'a> ToCursors<'a> for Important {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.bang.into());
		s.append(self.important.into());
	}
}

impl fmt::Display for Important {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&"!important", f)
	}
}

pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn valid_property(_p: &Parser, _c: Cursor) -> bool {
		true
	}

	fn parse_declaration(
		p: &mut Parser<'a>,
	) -> Result<(T![Ident], Option<T![:]>, Self::DeclarationValue, Option<Important>, Option<T![;]>)> {
		let name = p.parse::<T![Ident]>()?;
		let c: Cursor = name.into();
		if !Self::valid_property(p, c) {
			Err(diagnostics::UnknownDeclaration(c.into()))?;
		}
		let colon = p.parse_if_peek::<T![:]>()?;
		let value = Self::DeclarationValue::parse_declaration_value(c, p)?;
		let important = p.parse_if_peek::<Important>()?;
		let semi = p.parse_if_peek::<T![;]>()?;
		Ok((name, colon, value, important, semi))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(name: Cursor, p: &mut Parser<'a>) -> Result<Self>;
}
