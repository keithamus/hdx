use hdx_atom::{atom, Atom};
use hdx_derive::{Atomizable, Parsable, Writable};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, T};
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
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Function]>()?;
		let value = match p.parse_atom_lower(token) {
			atom!("dir") => p.parse::<DirValue>().map(Self::Dir)?,
			atom!("has") => p.parse::<RelativeSelector>().map(Self::Has)?,
			atom!("host") => p.parse::<SelectorList>().map(Self::Host)?,
			atom!("host-context") => p.parse::<SelectorList>().map(Self::HostContext)?,
			atom!("is") => p.parse::<ForgivingSelector>().map(Self::Is)?,
			atom!("lang") => {
				let mut langs = smallvec![];
				loop {
					if let Some(token) = p.peek::<T![Ident]>() {
						p.hop(token);
						langs.push(p.parse_atom(token));
					} else {
						let token = *p.parse::<T![String]>()?;
						p.hop(token);
						langs.push(p.parse_atom(token));
					}
					if !p.parse::<T![,]>().is_ok() {
						break;
					}
				}
				Self::Lang(langs)
			}
			atom!("not") => p.parse::<SelectorList>().map(Self::Not)?,
			atom!("nth-child") => p.parse::<Nth>().map(Self::NthChild)?,
			atom!("nth-col") => p.parse::<Nth>().map(Self::NthCol)?,
			atom!("nth-last-child") => p.parse::<Nth>().map(Self::NthLastCol)?,
			atom!("nth-last-col") => p.parse::<Nth>().map(Self::NthLastCol)?,
			atom!("nth-last-of-type") => p.parse::<Nth>().map(Self::NthLastOfType)?,
			atom!("nth-of-type") => p.parse::<Nth>().map(Self::NthOfType)?,
			atom!("where") => p.parse::<ForgivingSelector>().map(Self::Where)?,
			atom!("state") => {
				let token = *p.parse::<T![Ident]>()?;
				Self::State(p.parse_atom(token))
			}
			ident => Err(diagnostics::UnexpectedFunction(ident, token.span()))?,
		};
		p.parse::<T![RightParen]>()?;
		Ok(value)
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
