use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Kind};

use crate::{discard, expect, expect_ignore_case, parser::Parser, unexpected, Result};

use super::Parse;

pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn parse_name(parser: &mut Parser<'a>) -> Result<Atom> {
		let token = parser.next();
		match token.kind() {
			Kind::Ident => {
				expect!(parser.next(), Kind::Colon);
				Ok(parser.parse_atom_lower(token))
			}
			_ => unexpected!(parser, token),
		}
	}

	fn parse_important(parser: &mut Parser<'a>) -> Result<bool> {
		let peeked = parser.peek();
		let nexted_peek = parser.peek_n(2);
		if matches!(peeked.kind(), Kind::Delim)
			&& matches!(peeked.char(), Some('!'))
			&& matches!(nexted_peek.kind(), Kind::Ident)
			&& matches!(parser.parse_atom_lower(*nexted_peek), atom!("important"))
		{
			parser.advance();
			expect_ignore_case!(parser.next_with(Include::all()), Kind::Ident, atom!("important"));
			Ok(true)
		} else {
			Ok(false)
		}
	}

	fn parse_declaration(parser: &mut Parser<'a>) -> Result<(Atom, Self::DeclarationValue, bool)> {
		let name = Self::parse_name(parser)?;
		let value = Self::DeclarationValue::parse_declaration_value(&name, parser)?;
		let important = Self::parse_important(parser)?;
		discard!(parser, Kind::Semicolon);
		Ok((name, value, important))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> Result<Self>;
}
