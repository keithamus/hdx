use std::{fmt::Debug, hash::Hash};

use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{peek, Declaration, DeclarationValue, Parse, Parser, Result as ParserResult, State};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::{css::values, syntax::ComponentValues};

mod property_list;
use property_list::apply_properties;

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Custom<'a>(pub ComponentValues<'a>);

impl<'a> WriteCss<'a> for Custom<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}

impl<'a> Parse<'a> for Custom<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(ComponentValues::parse_with_state(parser, State::StopOnSemicolon)?))
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Computed<'a>(pub ComponentValues<'a>);

impl<'a> WriteCss<'a> for Computed<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}

impl<'a> Parse<'a> for Computed<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(ComponentValues::parse_with_state(parser, State::StopOnSemicolon | State::Nested)?))
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Unknown<'a>(pub ComponentValues<'a>);

impl<'a> Parse<'a> for Unknown<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(ComponentValues::parse_with_state(parser, State::StopOnSemicolon | State::Nested)?))
	}
}

impl<'a> WriteCss<'a> for Unknown<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
pub struct Property<'a> {
	name: Atom,
	value: StyleValue<'a>,
	important: bool,
}

impl<'a> Parse<'a> for Property<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, value, important) = Self::parse_declaration(parser)?;
		Ok(Self { name, value, important })
	}
}

impl<'a> Declaration<'a> for Property<'a> {
	type DeclarationValue = StyleValue<'a>;
}

impl<'a> WriteCss<'a> for Property<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str(self.name.as_ref())?;
		sink.write_char(':')?;
		sink.write_whitespace()?;
		self.value.write_css(sink)?;
		if self.important {
			sink.write_whitespace()?;
			sink.write_char('!')?;
			atom!("important").write_css(sink)?;
		}
		Ok(())
	}
}

#[inline]
fn is_computed_token(token: &Token) -> bool {
	matches!(token, Token::Function(atom) if matches!(
		atom.to_ascii_lowercase(),
		atom!("var")
			| atom!("calc") | atom!("min")
			| atom!("max") | atom!("clamp")
			| atom!("round") | atom!("mod")
			| atom!("rem") | atom!("sin")
			| atom!("cos") | atom!("tan")
			| atom!("asin") | atom!("atan")
			| atom!("atan2") | atom!("pow")
			| atom!("sqrt") | atom!("hypot")
			| atom!("log") | atom!("exp")
			| atom!("abs") | atom!("sign")
	))
}

macro_rules! style_value {
    ( $(
        $name: ident$(<$a: lifetime>)?: $atom: pat,
    )+ ) => {
		#[derive(PartialEq, Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		pub enum StyleValue<'a> {
			Initial,
			Inherit,
			Unset,
			Revert,
			RevertLayer,
			#[cfg_attr(feature = "serde", serde(untagged))]
			Custom(Custom<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Computed(Computed<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Unknown(Unknown<'a>),
			$(
				#[cfg_attr(feature = "serde", serde(untagged))]
				$name(values::$name$(<$a>)?),
			)+
		}
	}
}

apply_properties!(style_value);

impl<'a> WriteCss<'a> for StyleValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		macro_rules! write_css {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match self {
					Self::Initial => atom!("initial").write_css(sink),
					Self::Inherit => atom!("inherit").write_css(sink),
					Self::Unset => atom!("unset").write_css(sink),
					Self::Revert => atom!("revert").write_css(sink),
					Self::RevertLayer => atom!("revert-layer").write_css(sink),
					Self::Custom(v) => v.write_css(sink),
					Self::Unknown(v) => v.write_css(sink),
					Self::Computed(v) => v.write_css(sink),
					$(
						Self::$name(v) => v.write_css(sink),
					)+
				}
			}
		}
		apply_properties!(write_css)
	}
}

impl<'a> DeclarationValue<'a> for StyleValue<'a> {
	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> ParserResult<Self> {
		if name.starts_with("--") {
			return Ok(Self::Custom(Custom::parse(parser)?));
		}
		if let Token::Ident(atom) = parser.peek() {
			match atom.to_ascii_lowercase() {
				atom!("initial") => {
					parser.advance();
					return Ok(Self::Initial);
				}
				atom!("inherit") => {
					parser.advance();
					return Ok(Self::Inherit);
				}
				atom!("unset") => {
					parser.advance();
					return Ok(Self::Unset);
				}
				atom!("revert") => {
					parser.advance();
					return Ok(Self::Revert);
				}
				atom!("revert-layer") => {
					parser.advance();
					return Ok(Self::RevertLayer);
				}
				_ => {}
			}
		}
		if is_computed_token(parser.peek()) {
			return Ok(Self::Computed(Computed::parse(parser)?));
		}
		macro_rules! parse_declaration_value {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match name {
					$(
						&$atom => {
							let checkpoint = parser.checkpoint();
							if let Ok(val) = values::$name::parse(parser) {
								if peek!(parser, Token::Semicolon | Token::RightCurly | Token::Eof | Token::Delim('!')) {
									return Ok(Self::$name(val))
								}
							}
							if is_computed_token(parser.peek()) {
								parser.rewind(checkpoint);
								Self::Computed(Computed::parse(parser)?)
							} else {
								parser.rewind(checkpoint);
								if is_computed_token(parser.peek()) {
									Self::Computed(Computed::parse(parser)?)
								} else {
									Self::Unknown(Unknown::parse(parser)?)
								}
							}
						},
					)+
					_ => Self::Unknown(Unknown::parse(parser)?)
				}
			}
		}
		Ok(apply_properties!(parse_declaration_value))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Property, 160);
		assert_size!(StyleValue, 144);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Property, "float: none !important");
		assert_parse!(Property, "width: 1px");
		assert_parse!(Property, "width: min(1px, 2px)");
		assert_parse!(Property, "border: 1px solid var(--red)");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Property, "float: none !important", "float:none!important");
		assert_minify!(Property, "width: 1px", "width:1px");
		assert_minify!(Property, "width: min(1px, 2px)", "width:min(1px, 2px)");
	}
}
