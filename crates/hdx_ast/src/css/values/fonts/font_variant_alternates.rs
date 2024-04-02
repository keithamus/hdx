use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, expect, unexpected, unexpected_function, unexpected_ident, Parse, Parser, Result as ParserResult,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::{Parsable, Writable, Value};

// https://drafts.csswg.org/css-fonts/#font-variant-alternates-prop
#[derive(Value, Writable, Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FontVariantAlternates {
	#[default]
	Normal,
	HistoricalForms,

	#[writable(as_function = "stylistic")]
	Stylistic(Atom),
	#[writable(as_function = "swash")]
	Swash(Atom),
	#[writable(as_function = "ornaments")]
	Ornaments(Atom),
	#[writable(as_function = "annotation")]
	Annotation(Atom),

	#[writable(as_function = "styleset")]
	Styleset(SmallVec<[Atom; 1]>),
	#[writable(as_function = "character-variant")]
	CharacterVariant(SmallVec<[Atom; 1]>),
}

impl<'a> Parse<'a> for FontVariantAlternates {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.cur() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("normal") => {
					parser.advance();
					Self::Normal
				}
				atom!("historical-forms") => {
					parser.advance();
					Self::HistoricalForms
				}
				_ => unexpected_ident!(parser, atom),
			},
			Token::Function(atom) => match atom.to_ascii_lowercase() {
				atom!("stylistic") => {
					parser.advance();
					match parser.cur() {
						Token::Ident(atom) => Self::Stylistic(atom),
						token => unexpected!(parser, token),
					}
				}
				atom!("swash") => {
					parser.advance();
					match parser.cur() {
						Token::Ident(atom) => Self::Swash(atom),
						token => unexpected!(parser, token),
					}
				}
				atom!("ornaments") => {
					parser.advance();
					match parser.cur() {
						Token::Ident(atom) => Self::Ornaments(atom),
						token => unexpected!(parser, token),
					}
				}
				atom!("annotation") => {
					parser.advance();
					match parser.cur() {
						Token::Ident(atom) => Self::Annotation(atom),
						token => unexpected!(parser, token),
					}
				}
				atom!("styleset") => {
					parser.advance();
					let mut idents = smallvec![];
					loop {
						match parser.cur() {
							Token::Ident(atom) => {
								idents.push(atom);
								parser.advance();
								expect!(parser, Token::Comma | Token::RightParen);
								if matches!(parser.cur(), Token::Comma) {
									parser.advance();
								}
							}
							_ => break,
						}
					}
					expect!(parser, Token::RightParen);
					parser.advance();
					Self::Styleset(idents)
				}
				atom!("character-variant") => {
					parser.advance();
					let mut idents = smallvec![];
					loop {
						match parser.cur() {
							Token::Ident(atom) => {
								idents.push(atom);
								parser.advance();
								expect!(parser, Token::Comma | Token::RightParen);
								if matches!(parser.cur(), Token::Comma) {
									parser.advance();
								}
							}
							_ => break,
						}
					}
					expect!(parser, Token::RightParen);
					parser.advance();
					Self::CharacterVariant(idents)
				}
				_ => unexpected_function!(parser, atom),
			},
			token => unexpected!(parser, token),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontVariantAlternates, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariantAlternates, "normal");
		assert_parse!(FontVariantAlternates, "historical-forms");
		assert_parse!(FontVariantAlternates, "styleset(dots)");
		assert_parse!(FontVariantAlternates, "styleset(dots, chomp)");
		assert_parse!(FontVariantAlternates, "character-variant(a, b, c, d)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(FontVariantAlternates, "swash");
		assert_parse_error!(FontVariantAlternates, "swash(a, b)");
		assert_parse_error!(FontVariantAlternates, "swash(2)");
		assert_parse_error!(FontVariantAlternates, "swash(a");
		assert_parse_error!(FontVariantAlternates, "styleset(dots chomp)");
		assert_parse_error!(FontVariantAlternates, "styleset(a");
		assert_parse_error!(FontVariantAlternates, "character-variant(a b c d)");
		assert_parse_error!(FontVariantAlternates, "character-variant(a");
	}
}
