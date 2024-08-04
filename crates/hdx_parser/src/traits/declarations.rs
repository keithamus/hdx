use hdx_atom::Atom;

use crate::{discard, parser::Parser, Delim, Parse, Peek, Result, Token};

mod kw {
	use crate::custom_keyword;
	custom_keyword!(Important, atom!("important"));
}

pub struct Important;
impl<'a> Peek<'a> for Important {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<Delim![!]>()
	}
}

impl<'a> Parse<'a> for Important {
	fn parse(parser: &mut Parser<'a>) -> Result<Self> {
		parser.parse::<Delim![!]>()?;
		parser.parse::<kw::Important>()?;
		Ok(Self)
	}
}

pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn parse_name(parser: &mut Parser<'a>) -> Result<Atom> {
		let token = *parser.parse::<Token![Ident]>()?;
		parser.parse::<Delim![:]>()?;
		Ok(parser.parse_atom_lower(token))
	}

	fn parse_important(parser: &mut Parser<'a>) -> Result<bool> {
		if parser.peek::<Important>().is_some() {
			parser.parse::<Important>()?;
			return Ok(true);
		}
		Ok(false)
	}

	fn parse_declaration(parser: &mut Parser<'a>) -> Result<(Atom, Self::DeclarationValue, bool)> {
		let name = Self::parse_name(parser)?;
		let value = Self::DeclarationValue::parse_declaration_value(&name, parser)?;
		let important = Self::parse_important(parser)?;
		discard!(parser, Semicolon);
		Ok((name, value, important))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> Result<Self>;
}
