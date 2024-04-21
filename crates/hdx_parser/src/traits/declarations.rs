use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Token};

use crate::{discard, expect, expect_ignore_case, match_ignore_case, parser::Parser, unexpected, Result};

use super::Parse;

pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn parse_name(parser: &mut Parser<'a>) -> Result<Atom> {
		match parser.next().clone() {
			Token::Ident(atom) => {
				expect!(parser.next(), Token::Colon);
				Ok(atom.to_ascii_lowercase())
			}
			token => unexpected!(parser, token),
		}
	}

	fn parse_important(parser: &mut Parser<'a>) -> Result<bool> {
		if matches!(parser.peek(), Token::Delim('!'))
			&& match_ignore_case!(parser.peek_n(2), Token::Ident(atom!("important")))
		{
			parser.advance();
			expect_ignore_case!(parser.next_with(Include::all()), Token::Ident(atom!("important")));
			Ok(true)
		} else {
			Ok(false)
		}
	}

	fn parse_declaration(parser: &mut Parser<'a>) -> Result<(Atom, Self::DeclarationValue, bool)> {
		let name = Self::parse_name(parser)?;
		let value = Self::DeclarationValue::parse_declaration_value(&name, parser)?;
		let important = Self::parse_important(parser)?;
		discard!(parser, Token::Semicolon);
		Ok((name, value, important))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> Result<Self>;
}
