use hdx_atom::Atom;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DashedIdent(Atom);

impl<'a> Peek<'a> for DashedIdent {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Ident]>().filter(|token| token.is_dashed_ident())
	}
}

impl<'a> Parse<'a> for DashedIdent {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Ident]>()?;
		let atom = p.parse_atom(token);
		if !token.is_dashed_ident() {
			Err(diagnostics::UnexpectedIdent(atom.clone(), token.span()))?
		}
		Ok(Self(atom))
	}
}

impl<'a> WriteCss<'a> for DashedIdent {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}
