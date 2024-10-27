use hdx_atom::{atom, Atom};
use hdx_derive::{Value, Writable};
use hdx_lexer::{Kind, Token};
use hdx_parser::{
	discard, expect, unexpected, unexpected_function, unexpected_ident, Parse, Parser, Result as ParserResult,
};
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-fonts/#font-variant-alternates-prop
#[derive(Value, Writable, Debug, Default, PartialEq, Clone, Hash)]
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
		let token = parser.next();
		Ok(match token.kind() {
			Kind::Ident => match parser.parse_atom_lower(token) {
				atom!("normal") => Self::Normal,
				atom!("historical-forms") => Self::HistoricalForms,
				atom => unexpected_ident!(parser, atom),
			},
			Kind::Function => match parser.parse_atom_lower(token) {
				atom!("stylistic") => {
					let token = parser.next();
					match token.kind() {
						Kind::Ident => {
							expect!(parser.next(), Kind::RightParen);
							Self::Stylistic(parser.parse_atom(token))
						}
						_ => unexpected!(parser, token),
					}
				},
				atom!("swash") => {
					let token = parser.next();
					match token.kind() {
						Kind::Ident => {
							expect!(parser.next(), Kind::RightParen);
							Self::Swash(parser.parse_atom(token))
						}
						_ => unexpected!(parser, token),
					}
				},
				atom!("ornaments") => {
					let token = parser.next();
					match token.kind() {
						Kind::Ident => {
							expect!(parser.next(), Kind::RightParen);
							Self::Ornaments(parser.parse_atom(token))
						}
						_ => unexpected!(parser, token),
					}
				},
				atom!("annotation") => {
					let token = parser.next();
					match token.kind() {
						Kind::Ident => {
							expect!(parser.next(), Kind::RightParen);
							Self::Annotation(parser.parse_atom(token))
						}
						_ => unexpected!(parser, token),
					}
				},
				atom!("styleset") => {
					let mut idents = smallvec![];
					loop {
						let token = parser.next();
						if token.kind() != Kind::Ident {
							break;
						}
						idents.push(parser.parse_atom(token));
						if !discard!(parser, Kind::Comma) {
							break;
						}
					}
					expect!(parser.next(), Kind::RightParen);
					Self::Styleset(idents)
				}
				atom!("character-variant") => {
					let mut idents = smallvec![];
					loop {
						let token = parser.next();
						if token.kind() != Kind::Ident {
							break;
						}
						idents.push(parser.parse_atom(token));
						if !discard!(parser, Kind::Comma) {
							break;
						}
					}
					expect!(parser.next(), Kind::RightParen);
					Self::CharacterVariant(idents)
				}
				atom => unexpected_function!(parser, atom),
			},
			_ => unexpected!(parser, token),
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
		assert_parse!(FontVariantAlternates, "swash(dots)");
		assert_parse!(FontVariantAlternates, "stylistic(foo)");
		assert_parse!(FontVariantAlternates, "swash(foo)");
		assert_parse!(FontVariantAlternates, "ornaments(foo)");
		assert_parse!(FontVariantAlternates, "annotation(foo)");
		assert_parse!(FontVariantAlternates, "styleset(foo)");
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
