use hdx_atom::{atom, Atom};
use hdx_derive::{Atomizable, Parsable, Writable};
use hdx_lexer::Token;
use hdx_parser::{discard, expect, unexpected, unexpected_function, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use super::{ForgivingSelector, Nth, RelativeSelector, SelectorList};

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum FunctionalPseudoClass<'a> {
	Dir(DirValue),                 // atom!("dir")
	Has(RelativeSelector<'a>),     // atom!("has")
	Host(SelectorList<'a>),        // atom!("host")
	HostContext(SelectorList<'a>), // atom!("host-context")
	Is(ForgivingSelector<'a>),     // atom!("is")
	Lang(SmallVec<[Atom; 1]>),     // atom!("lang")
	Not(SelectorList<'a>),         // atom!("not")
	NthChild(Nth),                 // atom!("nth-child")
	NthCol(Nth),                   // atom!("nth-col")
	NthLastChild(Nth),             // atom!("nth-last-child")
	NthLastCol(Nth),               // atom!("nth-last-col")
	NthLastOfType(Nth),            // atom!("nth-last-of-type")
	NthOfType(Nth),                // atom!("nth-of-type")
	Where(ForgivingSelector<'a>),  // atom!("where")
	State(Atom),                   // atom!("state")
}

impl<'a> Parse<'a> for FunctionalPseudoClass<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let val = match parser.next() {
			Token::Function(ident) => match ident.to_ascii_lowercase() {
				atom!("dir") => Self::Dir(DirValue::parse(parser)?),
				atom!("has") => Self::Has(RelativeSelector::parse(parser)?),
				atom!("host") => Self::Host(SelectorList::parse(parser)?),
				atom!("host-context") => Self::HostContext(SelectorList::parse(parser)?),
				atom!("is") => Self::Is(ForgivingSelector::parse(parser)?),
				atom!("lang") => {
					let mut langs = smallvec![];
					loop {
						match parser.next() {
							Token::Ident(atom) | Token::String(atom, _) => langs.push(atom.clone()),
							token => unexpected!(parser, token),
						}
						if !discard!(parser, Token::Comma) {
							break;
						}
					}
					Self::Lang(langs)
				}
				atom!("not") => Self::Not(SelectorList::parse(parser)?),
				atom!("nth-child") => Self::NthChild(Nth::parse(parser)?),
				atom!("nth-col") => Self::NthCol(Nth::parse(parser)?),
				atom!("nth-last-child") => Self::NthLastChild(Nth::parse(parser)?),
				atom!("nth-last-col") => Self::NthLastCol(Nth::parse(parser)?),
				atom!("nth-last-of-type") => Self::NthLastOfType(Nth::parse(parser)?),
				atom!("nth-of-type") => Self::NthOfType(Nth::parse(parser)?),
				atom!("where") => Self::Where(ForgivingSelector::parse(parser)?),
				atom!("state") => {
					if let Token::Ident(atom) = parser.next().clone() {
						Self::State(atom)
					} else {
						unexpected!(parser)
					}
				}
				_ => unexpected_function!(parser, ident),
			},
			token => unexpected!(parser, token),
		};
		expect!(parser.next(), Token::RightParen);
		Ok(val)
	}
}

impl<'a> WriteCss<'a> for FunctionalPseudoClass<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Dir(dir) => {
				atom!("dir").write_css(sink)?;
				sink.write_char('(')?;
				dir.write_css(sink)?;
			}
			Self::Has(sel) => {
				atom!("has").write_css(sink)?;
				sink.write_char('(')?;
				sel.write_css(sink)?;
			}
			Self::Host(sel) => {
				atom!("host").write_css(sink)?;
				sink.write_char('(')?;
				sel.write_css(sink)?;
			}
			Self::HostContext(sel) => {
				atom!("host-context").write_css(sink)?;
				sink.write_char('(')?;
				sel.write_css(sink)?;
			}
			Self::Is(sel) => {
				atom!("is").write_css(sink)?;
				sink.write_char('(')?;
				sel.write_css(sink)?;
			}
			Self::Lang(langs) => {
				atom!("lang").write_css(sink)?;
				sink.write_char('(')?;
				langs.write_css(sink)?;
			}
			Self::Not(sel) => {
				atom!("not").write_css(sink)?;
				sink.write_char('(')?;
				sel.write_css(sink)?;
			}
			Self::NthChild(nth) => {
				atom!("nth-child").write_css(sink)?;
				sink.write_char('(')?;
				nth.write_css(sink)?;
			}
			Self::NthCol(nth) => {
				atom!("nth-col").write_css(sink)?;
				sink.write_char('(')?;
				nth.write_css(sink)?;
			}
			Self::NthLastChild(nth) => {
				atom!("nth-last-child").write_css(sink)?;
				sink.write_char('(')?;
				nth.write_css(sink)?;
			}
			Self::NthLastCol(nth) => {
				atom!("nth-last-col").write_css(sink)?;
				sink.write_char('(')?;
				nth.write_css(sink)?;
			}
			Self::NthLastOfType(nth) => {
				atom!("nth-last-of-type").write_css(sink)?;
				sink.write_char('(')?;
				nth.write_css(sink)?;
			}
			Self::NthOfType(nth) => {
				atom!("nth-of-type").write_css(sink)?;
				sink.write_char('(')?;
				nth.write_css(sink)?;
			}
			Self::Where(sel) => {
				atom!("where").write_css(sink)?;
				sink.write_char('(')?;
				sel.write_css(sink)?;
			}
			Self::State(atom) => {
				atom!("state").write_css(sink)?;
				sink.write_char('(')?;
				atom.write_css(sink)?;
			}
		}
		sink.write_char(')')
	}
}

#[derive(Writable, Parsable, Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum DirValue {
	Rtl, // atom!("rtl")
	Ltr, // atom!("ltr")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FunctionalPseudoClass, 40);
		assert_size!(DirValue, 1);
	}
}
