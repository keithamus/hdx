use hdx_atom::{atom, Atom};
use hdx_lexer::Token;

use crate::{discard, expect, parser::Parser, peek, unexpected, unexpected_ident, Result};

use super::Parse;

pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn parse_name(parser: &mut Parser<'a>) -> Result<Atom> {
		match parser.cur() {
			Token::Ident(atom) => {
				parser.advance();
				expect!(parser, Token::Colon);
				parser.advance();
				Ok(atom.to_ascii_lowercase())
			}
			token => unexpected!(parser, token),
		}
	}

	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> Result<Self::DeclarationValue>;

	fn parse_important(parser: &mut Parser<'a>) -> Result<bool> {
		if matches!(parser.cur(), Token::Delim('!')) && peek!(parser, Token::Ident(_)) {
			parser.advance_including_whitespace_and_comments();
			match parser.cur() {
				Token::Ident(ident) => match ident.to_ascii_lowercase() {
					atom!("important") => {}
					_ => unexpected_ident!(parser, ident),
				},
				token => unexpected!(parser, token),
			}
			parser.advance();
			Ok(true)
		} else {
			Ok(false)
		}
	}

	fn parse_declaration(parser: &mut Parser<'a>) -> Result<(Atom, Self::DeclarationValue, bool)> {
		let name = Self::parse_name(parser)?;
		let value = Self::parse_declaration_value(&name, parser)?;
		let important = Self::parse_important(parser)?;
		discard!(parser, Token::Semicolon);
		Ok((name, value, important))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> Result<Self>;
}
