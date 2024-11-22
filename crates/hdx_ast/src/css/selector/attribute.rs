use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, QuoteStyle};
use hdx_parser::{diagnostics, discard, Parse, Parser, Result as ParserResult, T};
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
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<T![LeftSquare]>()?;
		let mut attr = Self::default();
		if let Some(token) = p.peek::<T![Delim]>() {
			p.hop(token);
			if matches!(token.char(), Some('|')) {
				let token = *p.parse_with::<T![Ident]>(Include::Whitespace)?;
				attr.name = p.parse_atom(token);
			} else if matches!(token.char(), Some('*')) {
				p.parse_with::<T![|]>(Include::Whitespace)?;
				let token = *p.parse_with::<T![Ident]>(Include::Whitespace)?;
				attr.ns_prefix = NSPrefix::Wildcard;
				attr.name = p.parse_atom(token);
			} else {
				Err(diagnostics::UnexpectedDelim(token.char().unwrap(), token.span()))?
			}
		} else if let Some(token) = p.peek::<T![Ident]>() {
			p.hop(token);
			let first = p.parse_atom(token);
			if let Some(token) = p.peek_with::<T![|]>(Include::Whitespace) {
				let checkpoint = p.checkpoint();
				p.hop(token);
				if let Ok(token) = p.parse_with::<T![Ident]>(Include::Whitespace) {
					attr.ns_prefix = NSPrefix::Named(first);
					attr.name = p.parse_atom(*token);
				} else {
					p.rewind(checkpoint);
					attr.name = first
				}
			} else {
				attr.name = first;
			}
		} else {
			let token = p.peek::<T![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
		if p.parse::<T![RightSquare]>().is_ok() {
			return Ok(attr);
		}
		let token = *p.parse::<T![Delim]>()?;
		match token.char().unwrap() {
			'=' => attr.matcher = AttributeMatch::Exact,
			'~' => {
				p.parse_with::<T![=]>(Include::all_bits())?;
				attr.matcher = AttributeMatch::SpaceList
			}
			'|' => {
				p.parse_with::<T![=]>(Include::all_bits())?;
				attr.matcher = AttributeMatch::LangPrefix
			}
			'^' => {
				p.parse_with::<T![=]>(Include::all_bits())?;
				attr.matcher = AttributeMatch::Prefix
			}
			'$' => {
				p.parse_with::<T![=]>(Include::all_bits())?;
				attr.matcher = AttributeMatch::Suffix
			}
			'*' => {
				p.parse_with::<T![=]>(Include::all_bits())?;
				attr.matcher = AttributeMatch::Contains
			}
			c => Err(diagnostics::UnexpectedDelim(c, token.span()))?,
		}
		if let Some(token) = p.peek::<T![Ident]>() {
			p.hop(token);
			attr.value = p.parse_atom(token);
		} else {
			let token = *p.parse::<T![String]>()?;
			attr.quote = token.quote_style();
			attr.value = p.parse_atom(token);
		}
		if let Some(token) = p.peek::<T![Ident]>() {
			p.hop(token);
			attr.modifier = match p.parse_atom_lower(token) {
				atom!("i") => AttributeModifier::Insensitive,
				atom!("s") => AttributeModifier::Sensitive,
				atom => Err(diagnostics::UnexpectedIdent(atom, token.span()))?,
			};
		}
		p.parse::<T![RightSquare]>()?;
		Ok(attr)
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
		assert_parse!(Attribute, "[foo=\"bar\"]");
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
