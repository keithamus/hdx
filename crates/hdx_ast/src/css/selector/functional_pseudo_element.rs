use hdx_atom::{atom, Atom};
use hdx_parser::{diagnostics, todo, Parse, Parser, Result as ParserResult, Vec, T};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use super::SelectorComponent;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FunctionalPseudoElement<'a> {
	// https://drafts.csswg.org/css-highlight-api/#custom-highlight-pseudo
	Highlight(Atom),
	// https://drafts.csswg.org/css-shadow-parts/#part
	Part(SmallVec<[Atom; 1]>),
	// https://drafts.csswg.org/css-scoping/#slotted-pseudo
	Slotted(Vec<'a, SelectorComponent<'a>>),
}

impl<'a> Parse<'a> for FunctionalPseudoElement<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Function]>()?;
		match p.parse_atom_lower(token) {
			atom!("highlight") => {
				let name_token = *p.parse::<T![Ident]>()?;
				let name = p.parse_atom(name_token);
				p.parse::<T![RightParen]>()?;
				Ok(Self::Highlight(name))
			}
			atom!("part") => {
				let mut parts = smallvec![];
				loop {
					if p.parse::<T![RightParen]>().is_ok() {
						break;
					}
					let name_token = *p.parse::<T![Ident]>()?;
					let name = p.parse_atom(name_token);
					parts.push(name);
				}
				Ok(Self::Part(parts))
			}
			atom!("slotted") => {
				let selector = p.new_vec();
				loop {
					if p.parse::<T![RightParen]>().is_ok() {
						break;
					}
					let checkpoint = p.checkpoint();
					let component = p.parse::<SelectorComponent>()?;
					match component {
						SelectorComponent::Tag(_)
						| SelectorComponent::NSPrefixedTag(_)
						| SelectorComponent::NSPrefixedWildcard(_)
						| SelectorComponent::Wildcard
							if selector.is_empty() =>
						{
							todo!(p);
						}
						SelectorComponent::Id(_)
						| SelectorComponent::Class(_)
						| SelectorComponent::Attribute(_)
						| SelectorComponent::PseudoClass(_) => {}
						_ => {
							p.rewind(checkpoint);
							let token = p.peek::<T![Any]>().unwrap();
							Err(diagnostics::Unexpected(token, token.span()))?
						}
					}
				}
				Ok(Self::Slotted(selector))
			}
			ident => Err(diagnostics::UnexpectedFunction(ident, token.span()))?,
		}
	}
}

impl<'a> WriteCss<'a> for FunctionalPseudoElement<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Highlight(atom) => {
				atom!("highlight").write_css(sink)?;
				sink.write_char('(')?;
				atom.write_css(sink)?;
				sink.write_char(')')?;
			}
			Self::Part(parts) => {
				atom!("part").write_css(sink)?;
				sink.write_char('(')?;
				let mut iter = parts.iter().peekable();
				while let Some(part) = iter.next() {
					part.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_char(' ')?;
					}
				}
				sink.write_char(')')?;
			}
			Self::Slotted(selectors) => {
				atom!("slotted").write_css(sink)?;
				sink.write_char('(')?;
				for selector in selectors {
					selector.write_css(sink)?;
				}
				sink.write_char(')')?;
			}
		}
		Ok(())
	}
}
