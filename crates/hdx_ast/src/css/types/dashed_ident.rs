use hdx_atom::Atom;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, Token};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DashedIdent(Atom);

impl<'a> Peek<'a> for DashedIdent {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<Token![Ident]>().filter(|token| token.is_dashed_ident())
	}
}

impl<'a> Parse<'a> for DashedIdent {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *parser.parse::<Token![Ident]>()?;
		let atom = parser.parse_atom(token);
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
