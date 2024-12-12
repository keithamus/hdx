use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, Build, CursorStream, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

mod func {
	use hdx_parser::custom_keyword;
	custom_keyword!(Normal, atom!("normal"));
	custom_keyword!(Bold, atom!("bold"));
	custom_keyword!(Bolder, atom!("bolder"));
	custom_keyword!(Lighter, atom!("lighter"));
}

// https://drafts.csswg.org/css-fonts-4/#propdef-font-weight
// <font-weight-absolute> = [normal | bold | <number [1,1000]>]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum FontWeightAbsolute {
	Normal(T![Ident]),
	Bold(T![Ident]),
	Bolder(T![Ident]),
	Lighter(T![Ident]),
}

impl<'a> Peek<'a> for FontWeightAbsolute {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<func::Normal>()
			|| p.peek::<func::Bold>()
			|| p.peek::<func::Bolder>()
			|| p.peek::<func::Lighter>()
			|| p.peek::<T![Number]>()
	}
}

impl<'a> Parse<'a> for FontWeightAbsolute {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		match p.parse_atom_lower(c) {
			atom!("normal") => Ok(Self::Normal(<T![Ident]>::build(p, c))),
			atom!("bold") => Ok(Self::Bold(<T![Ident]>::build(p, c))),
			atom!("bolder") => Ok(Self::Bolder(<T![Ident]>::build(p, c))),
			atom!("lighter") => Ok(Self::Lighter(<T![Ident]>::build(p, c))),
			atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
		}
	}
}

impl<'a> ToCursors<'a> for FontWeightAbsolute {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::Normal(c) => s.append(c.into()),
			Self::Bold(c) => s.append(c.into()),
			Self::Bolder(c) => s.append(c.into()),
			Self::Lighter(c) => s.append(c.into()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontWeightAbsolute, 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontWeightAbsolute, "normal");
		assert_parse!(FontWeightAbsolute, "bold");
		assert_parse!(FontWeightAbsolute, "bolder");
		assert_parse!(FontWeightAbsolute, "lighter");
		assert_parse!(FontWeightAbsolute, "100");
		assert_parse!(FontWeightAbsolute, "500");
		assert_parse!(FontWeightAbsolute, "900");
	}
}
