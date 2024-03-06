use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, discard, expect, peek, unexpected, unexpected_function, unexpected_ident, FromToken, Parse, Parser,
	Result as ParserResult, SelectorComponent as SelectorComponentTrait, SelectorList as SelectorListTrait, Span,
	Spanned,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::Atomizable;

use super::{ANBEvenOdd, ForgivingSelector, RelativeSelector, SelectorList, ANB};

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum FunctionalPseudoClass<'a> {
	Dir(DirValue),                 // atom!("dir")
	Has(RelativeSelector<'a>),     // atom!("has")
	Host(SelectorList<'a>),        // atom!("host")
	HostContext(SelectorList<'a>), // atom!("host-context")
	Is(ForgivingSelector<'a>),     // atom!("is")
	Lang(SmallVec<[Atom; 3]>),     // atom!("lang")
	Not(SelectorList<'a>),         // atom!("not")
	NthChild(ANBEvenOdd),          // atom!("nth-child")
	NthCol(ANB),                   // atom!("nth-col")
	NthLastChild(ANBEvenOdd),      // atom!("nth-last-child")
	NthLastCol(ANB),               // atom!("nth-last-col")
	NthLastOfType(ANBEvenOdd),     // atom!("nth-last-of-type")
	NthOfType(ANBEvenOdd),         // atom!("nth-of-type")
	Where(ForgivingSelector<'a>),  // atom!("where")
	State(Atom),                   // atom!("state")
}

impl<'a> Parse<'a> for FunctionalPseudoClass<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Function(ident) => match ident.to_ascii_lowercase() {
				atom!("dir") => {
					todo!()
				}
				atom!("has") => {
					todo!()
				}
				atom!("host") => {
					todo!()
				}
				atom!("host-context") => {
					todo!()
				}
				atom!("is") => {
					todo!()
				}
				atom!("lang") => {
					todo!()
				}
				atom!("not") => {
					todo!()
				}
				atom!("nth-child") => {
					todo!()
				}
				atom!("nth-col") => {
					todo!()
				}
				atom!("nth-last-child") => {
					todo!()
				}
				atom!("nth-last-col") => {
					todo!()
				}
				atom!("nth-last-of-type") => {
					todo!()
				}
				atom!("nth-of-type") => {
					todo!()
				}
				atom!("where") => {
					todo!()
				}
				atom!("state") => {
					todo!()
				}
				_ => unexpected_function!(parser, ident),
			},
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for FunctionalPseudoClass<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		todo!()
	}
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum DirValue {
	Rtl, // atom!("rtl")
	Ltr, // atom!("ltr")
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FunctionalPseudoClass, 40);
		assert_size!(DirValue, 1);
	}
}
