use hdx_atom::Atom;

use crate::{parser::Parser, Parse, Peek, Result, T};

mod kw {
	use crate::custom_keyword;
	custom_keyword!(Important, atom!("important"));
}

pub struct Important;
impl<'a> Peek<'a> for Important {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![!]>()
	}
}

impl<'a> Parse<'a> for Important {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		p.parse::<T![!]>()?;
		p.parse::<kw::Important>()?;
		Ok(Self)
	}
}

pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn parse_name(p: &mut Parser<'a>) -> Result<Atom> {
		let token = *p.parse::<T![Ident]>()?;
		p.parse::<T![:]>()?;
		Ok(p.parse_atom_lower(token))
	}

	fn parse_important(p: &mut Parser<'a>) -> Result<bool> {
		if p.peek::<Important>().is_some() {
			p.parse::<Important>()?;
			return Ok(true);
		}
		Ok(false)
	}

	fn parse_declaration(p: &mut Parser<'a>) -> Result<(Atom, Self::DeclarationValue, bool)> {
		let name = Self::parse_name(p)?;
		let value = Self::DeclarationValue::parse_declaration_value(&name, p)?;
		let important = Self::parse_important(p)?;
		p.parse::<T![;]>().ok();
		Ok((name, value, important))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(name: &Atom, p: &mut Parser<'a>) -> Result<Self>;
}
