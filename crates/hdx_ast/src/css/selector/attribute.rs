use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Kind, QuoteStyle, Token};
use hdx_parser::{expect, expect_delim, peek, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use super::NSPrefix;

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Attribute {
	pub ns_prefix: NSPrefix,
	pub name: Atom,
	pub value: Atom,
	pub quote: QuoteStyle,
	pub matcher: AttributeMatch,
	pub modifier: AttributeModifier,
}

impl<'a> Parse<'a> for Attribute {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser.next(), Kind::LeftSquare);
		let mut attr = Self::default();
		let token = parser.next();
		match token.kind() {
			Kind::Delim if matches!(token.char(), Some('|')) => {
				let token = parser.next_with(Include::Whitespace);
				match token.kind() {
					Token::Ident => {
						attr.name = parser.parse_atom(token);
					}
					_ => unexpected!(parser, token),
				}
			},
			Kind::Delim if matches!(token.char(), Some('*')) => {
				let token = parser.next_with(Include::Whitespace);
				match token.kind() {
					Token::Delim if matches!(token.char(), Some('|')) => {
						let token = parser.next_with(Include::Whitespace);
						match token.kind() {
							Token::Ident => {
								attr.ns_prefix = NSPrefix::Wildcard;
								attr.name = parser.parse_atom(token);
							}
							_ => unexpected!(parser, token),
						}
					},
					_ => unexpected!(parser, token),
				}
			},
			Kind::Ident => {
				let ns = parser.parse_atom(token);
				let token = parser.peek_with(Include::Whitespace);
				match token.kind() {
					Kind::Delim if matches!(token.char(), Some('|')) && peek!(parser, 2, Kind::Ident) => {
						expect_delim!(parser.next_with(Include::Whitespace), '|');
						let token = parser.next_with(Include::Whitespace);
						match token.kind() {
							Kind::Ident => {
								attr.ns_prefix = NSPrefix::Named(ns);
								attr.name = parser.parse_atom(token);
							}
							_ => unexpected!(parser, token),
						}
					},
					_ => {
						attr.name = ns;
					}
				}
			},
			_ => unexpected!(parser, token),
		};
		let token = parser.next();
		match token.kind() {
			Kind::Delim => match token.char().unwrap() {
				'=' => attr.matcher = AttributeMatch::Exact,
				'~' => {
					expect_delim!(parser.next_with(Include::all_bits()), '=');
					attr.matcher = AttributeMatch::SpaceList
				}
				'|' => {
					expect_delim!(parser.next_with(Include::all_bits()), '=');
					attr.matcher = AttributeMatch::LangPrefix
				}
				'^' => {
					expect_delim!(parser.next_with(Include::all_bits()), '=');
					attr.matcher = AttributeMatch::Prefix
				}
				'$' => {
					expect_delim!(parser.next_with(Include::all_bits()), '=');
					attr.matcher = AttributeMatch::Suffix
				}
				'*' => {
					expect_delim!(parser.next_with(Include::all_bits()), '=');
					attr.matcher = AttributeMatch::Contains
				}
			},
			Kind::RightSquare => {
				return Ok(attr);
			}
			_ => unexpected!(parser, token),
		}
		let token = parser.next();
		match token.kind() {
			Kind::Ident => attr.value = parser.parse_atom(token),
			Kind::String => {
				attr.quote = token.quote_style();
				attr.value = parser.parse_atom(token);
			}
			_ => unexpected!(parser, token),
		};
		let token = parser.next();
		match token.kind() {
			Kind::Ident => {
				attr.modifier = match parser.parse_atom_lower(token) {
					atom!("i") => AttributeModifier::Insensitive,
					atom!("s") => AttributeModifier::Sensitive,
					atom => unexpected_ident!(parser, atom),
				};
				expect!(parser.next(), Kind::RightSquare);
				Ok(attr)
			}
			Kind::RightSquare => Ok(attr),
			_ => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Attribute {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		write_css!(sink, '[', self.ns_prefix, self.name, self.matcher);
		if self.matcher != AttributeMatch::Any {
			sink.write_with_quotes(self.value.as_ref(), self.quote, true)?;
		}
		if self.modifier != AttributeModifier::None {
			write_css!(sink, ' ', self.modifier);
		}
		write_css!(sink, ']');
		Ok(())
	}
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeMatch {
	#[default]
	Any, // [attr]
	Exact,      // [attr=val]
	SpaceList,  // [attr~=val]
	LangPrefix, // [attr|=val]
	Prefix,     // [attr^=val]
	Suffix,     // [attr$=val]
	Contains,   // [attr*=val]
}

impl<'a> WriteCss<'a> for AttributeMatch {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			AttributeMatch::Any => {}
			AttributeMatch::Exact => write_css!(sink, '='),
			AttributeMatch::SpaceList => write_css!(sink, '~', '='),
			AttributeMatch::LangPrefix => write_css!(sink, '|', '='),
			AttributeMatch::Prefix => write_css!(sink, '^', '='),
			AttributeMatch::Suffix => write_css!(sink, '$', '='),
			AttributeMatch::Contains => write_css!(sink, '*', '='),
		}
		Ok(())
	}
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum AttributeModifier {
	#[default]
	None,
	Sensitive,
	Insensitive,
}

impl<'a> WriteCss<'a> for AttributeModifier {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Sensitive => write_css!(sink, 's'),
			Self::Insensitive => write_css!(sink, 'i'),
			Self::None => {}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Attribute, 40);
		assert_size!(AttributeMatch, 1);
		assert_size!(AttributeMatch, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Attribute, "[foo]");
		assert_parse!(Attribute, "[foo='bar']");
		assert_parse!(Attribute, "[foo='bar']");
		assert_parse!(Attribute, "[attr*='foo']");
		assert_parse!(Attribute, "[attr='foo']");
		assert_parse!(Attribute, "[*|attr='foo']");
		assert_parse!(Attribute, "[x|attr='foo']");
		assert_parse!(Attribute, "[attr|='foo']");
		assert_parse!(Attribute, "[attr|='foo' i]");
		assert_parse!(Attribute, "[attr|='foo' s]");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Attribute, "[ foo ]", "[foo]");
		assert_minify!(Attribute, "[foo='bar']", "[foo=bar]");
		assert_minify!(Attribute, "[foo|='bar']", "[foo|=bar]");
		assert_minify!(Attribute, "[foo|='bar' s]", "[foo|=bar s]");
		assert_minify!(Attribute, "[foo='value with spaces']", "[foo=\"value with spaces\"]");
		assert_minify!(Attribute, "[attr|='foo' s]", "[attr|=foo s]");
	}
}
