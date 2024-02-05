use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{expect, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{parse_wq_name, NSPrefix};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Attribute {
	pub ns_prefix: NSPrefix,
	pub name: Atom,
	pub value: Atom,
	pub matcher: AttributeMatch,
	pub modifier: AttributeModifier,
}

impl<'a> Parse<'a> for Attribute {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::LeftSquare => {
				parser.advance();
				let (ns_prefix, name) = parse_wq_name(parser)?;
				let matcher = match parser.cur() {
					Token::RightSquare => {
						parser.advance_including_whitespace_and_comments();
						return Ok(Self {
							ns_prefix,
							name,
							value: atom!(""),
							modifier: AttributeModifier::None,
							matcher: AttributeMatch::Any,
						}
						.spanned(span.end(parser.pos())));
					}
					Token::Delim('=') => {
						parser.advance();
						AttributeMatch::Exact
					}
					Token::Delim('~') => {
						parser.advance_including_whitespace_and_comments();
						expect!(parser, Token::Delim('='));
						parser.advance();
						AttributeMatch::SpaceList
					}
					Token::Delim('|') => {
						parser.advance_including_whitespace_and_comments();
						expect!(parser, Token::Delim('='));
						parser.advance();
						AttributeMatch::LangPrefix
					}
					Token::Delim('^') => {
						parser.advance_including_whitespace_and_comments();
						expect!(parser, Token::Delim('='));
						parser.advance();
						AttributeMatch::Prefix
					}
					Token::Delim('$') => {
						parser.advance_including_whitespace_and_comments();
						expect!(parser, Token::Delim('='));
						parser.advance();
						AttributeMatch::Suffix
					}
					Token::Delim('*') => {
						parser.advance_including_whitespace_and_comments();
						expect!(parser, Token::Delim('='));
						parser.advance();
						AttributeMatch::Contains
					}
					token => unexpected!(parser, token),
				};
				let value = match parser.cur() {
					Token::Ident(value) | Token::String(value) => {
						parser.advance();
						value
					}
					token => unexpected!(parser, token),
				};
				match parser.cur() {
					Token::RightSquare => {
						parser.advance_including_whitespace_and_comments();
						Ok(Self { ns_prefix, name, value, modifier: AttributeModifier::None, matcher }
							.spanned(span.end(parser.pos())))
					}
					Token::Ident(ident) => {
						let modifier = match ident.to_ascii_lowercase() {
							atom!("i") => AttributeModifier::Insensitive,
							atom!("s") => AttributeModifier::Sensitive,
							atom => unexpected_ident!(parser, atom),
						};
						parser.advance();
						expect!(parser, Token::RightSquare);
						parser.advance_including_whitespace_and_comments();
						Ok(Self { ns_prefix, name, value, modifier, matcher }.spanned(span.end(parser.pos())))
					}
					token => unexpected!(parser, token),
				}
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for Attribute {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('[')?;
		match &self.ns_prefix {
			NSPrefix::None => {}
			NSPrefix::Named(ns) => {
				sink.write_str(ns.as_ref())?;
				sink.write_char('|')?;
			}
			NSPrefix::Wildcard => {
				sink.write_char('*')?;
				sink.write_char('|')?;
			}
		}
		sink.write_str(self.name.as_ref())?;
		match &self.matcher {
			AttributeMatch::Any => {}
			AttributeMatch::Exact => {
				sink.write_char('=')?;
			}
			AttributeMatch::SpaceList => {
				sink.write_char('~')?;
				sink.write_char('=')?;
			}
			AttributeMatch::LangPrefix => {
				sink.write_char('|')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Prefix => {
				sink.write_char('^')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Suffix => {
				sink.write_char('$')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Contains => {
				sink.write_char('*')?;
				sink.write_char('=')?;
			}
		}
		if &self.matcher != &AttributeMatch::Any {
			sink.write_char('"')?;
			sink.write_str(self.value.as_ref())?;
			sink.write_char('"')?;
		}

		sink.write_char(']')?;
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeMatch {
	Any,        // [attr]
	Exact,      // [attr=val]
	SpaceList,  // [attr~=val]
	LangPrefix, // [attr|=val]
	Prefix,     // [attr^=val]
	Suffix,     // [attr$=val]
	Contains,   // [attr*=val]
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum AttributeModifier {
	None,
	Sensitive,
	Insensitive,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Attribute>(), 40);
		assert_eq!(::std::mem::size_of::<AttributeMatch>(), 1);
		assert_eq!(::std::mem::size_of::<AttributeMatch>(), 1);
	}
}
