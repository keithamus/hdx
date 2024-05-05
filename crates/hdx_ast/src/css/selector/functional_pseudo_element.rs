use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Token, Kind};
use hdx_parser::{expect, todo, unexpected, unexpected_function, Parse, Parser, Result as ParserResult, Vec};
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.next().clone() {
			Token::Function(ident) => match ident.to_ascii_lowercase() {
				atom!("highlight") => match parser.next().clone() {
					Token::Ident(name) => {
						expect!(parser.next(), Kind::RightParen);
						parser.advance_with(Include::Whitespace);
						Ok(Self::Highlight(name))
					}
					token => unexpected!(parser, token),
				},
				atom!("part") => {
					let mut parts = smallvec![];
					loop {
						match parser.next() {
							Token::Ident(name) => {
								parts.push(name.clone());
							}
							Token::RightParen => {
								parser.advance_with(Include::Whitespace);
								break;
							}
							token => unexpected!(parser, token),
						}
					}
					Ok(Self::Part(parts))
				}
				atom!("slotted") => {
					parser.advance();
					let selector = parser.new_vec();
					loop {
						if matches!(parser.cur(), Token::RightParen) {
							break;
						}
						let checkpoint = parser.checkpoint();
						let component = SelectorComponent::parse(parser)?;
						match component {
							SelectorComponent::Tag(_)
							| SelectorComponent::NSPrefixedTag(_)
							| SelectorComponent::NSPrefixedWildcard(_)
							| SelectorComponent::Wildcard
								if selector.is_empty() =>
							{
								todo!(parser);
							}
							SelectorComponent::Id(_)
							| SelectorComponent::Class(_)
							| SelectorComponent::Attribute(_)
							| SelectorComponent::PseudoClass(_) => {}
							_ => {
								parser.rewind(checkpoint);
								unexpected!(parser);
							}
						}
					}
					Ok(Self::Slotted(selector))
				}
				_ => unexpected_function!(parser, ident),
			},
			token => unexpected!(parser, token),
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
