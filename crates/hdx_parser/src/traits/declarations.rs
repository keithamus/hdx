use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Kind};

use crate::{discard, expect, expect_ignore_case, parser::Parser, peek_delim, peek_ignore_case, unexpected, Result};

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
		if peek_delim!(parser, '!') && peek_ignore_case!(parser, Kind::Ident, atom!("important")) {
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
