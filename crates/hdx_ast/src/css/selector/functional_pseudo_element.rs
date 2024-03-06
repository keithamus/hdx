use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{
	expect, unexpected, unexpected_function, Parse, Parser, Result as ParserResult,
	SelectorComponent as SelectorComponentTrait, Vec,
};
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
		match parser.cur() {
			Token::Function(ident) => match ident.to_ascii_lowercase() {
				atom!("highlight") => {
					parser.advance();
					match parser.cur() {
						Token::Ident(name) => {
							parser.advance();
							expect!(parser, Token::RightParen);
							parser.advance_including_whitespace();
							Ok(Self::Highlight(name))
						}
						token => unexpected!(parser, token),
					}
				}
				atom!("part") => {
					parser.advance();
					let mut parts = smallvec![];
					loop {
						match parser.cur() {
							Token::Ident(name) => {
								parser.advance();
								parts.push(name);
							}
							Token::RightParen => {
								parser.advance_including_whitespace();
								break;
							}
							token => unexpected!(parser, token),
						}
					}
					Ok(Self::Part(parts))
				}
				atom!("slotted") => {
					parser.advance();
					let mut selector = parser.new_vec();
					loop {
						if matches!(parser.cur(), Token::RightParen) {
							break;
						}
						let checkpoint = parser.checkpoint();
						let component = SelectorComponent::parse_selector_component(selector.iter().last(), parser)?;
						match component {
							SelectorComponent::Type(_)
							| SelectorComponent::NSPrefixedType(_)
							| SelectorComponent::NSPrefixedWildcard(_)
							| SelectorComponent::Wildcard
								if selector.is_empty() => {}
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
			},
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
