use std::{fmt::Debug, hash::Hash};

use hdx_atom::{atom, Atom};
use hdx_derive::Visitable;
use hdx_parser::{Declaration, DeclarationValue, Parse, Parser, Peek, Result as ParserResult, State, T};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::{css::values, syntax::ComponentValues};

// The build.rs generates a list of CSS properties from the value mods
include!(concat!(env!("OUT_DIR"), "/css_apply_properties.rs"));

#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Custom<'a>(pub ComponentValues<'a>);

impl<'a> WriteCss<'a> for Custom<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}

impl<'a> Parse<'a> for Custom<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let old_state = parser.set_state(State::StopOnSemicolon | State::Nested);
		parser
			.parse::<ComponentValues>()
			.inspect_err(|_| {
				parser.set_state(old_state);
			})
			.map(Self)
	}
}

#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Computed<'a>(pub ComponentValues<'a>);

impl<'a> WriteCss<'a> for Computed<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}

impl<'a> Peek<'a> for Computed<'a> {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		if let Some(token) = parser.peek::<T![Function]>() {
			if matches!(
				parser.parse_atom_lower(token),
				atom!("var")
					| atom!("calc") | atom!("min")
					| atom!("max") | atom!("clamp")
					| atom!("round")
					| atom!("mod") | atom!("rem")
					| atom!("sin") | atom!("cos")
					| atom!("tan") | atom!("asin")
					| atom!("atan") | atom!("atan2")
					| atom!("pow") | atom!("sqrt")
					| atom!("hypot")
					| atom!("log") | atom!("exp")
					| atom!("abs") | atom!("sign")
			) {
				return Some(token);
			}
		}
		None
	}
}

impl<'a> Parse<'a> for Computed<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let old_state = parser.set_state(State::StopOnSemicolon | State::Nested);
		parser
			.parse::<ComponentValues>()
			.inspect_err(|_| {
				parser.set_state(old_state);
			})
			.map(Self)
	}
}

#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Unknown<'a>(pub ComponentValues<'a>);

impl<'a> Parse<'a> for Unknown<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let old_state = parser.set_state(State::StopOnSemicolon | State::Nested);
		parser
			.parse::<ComponentValues>()
			.inspect_err(|_| {
				parser.set_state(old_state);
			})
			.map(Self)
	}
}

impl<'a> WriteCss<'a> for Unknown<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}

#[derive(Visitable, PartialEq, Debug, Hash)]
#[visitable(call)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
pub struct Property<'a> {
	#[visitable(skip)]
	pub name: Atom,
	#[visitable(skip)]
	pub value: StyleValue<'a>,
	#[visitable(skip)]
	pub important: bool,
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

macro_rules! style_value {
    ( $(
        $name: ident$(<$a: lifetime>)?: $atom: pat,
    )+ ) => {
		#[derive(PartialEq, Debug, Clone, Hash)]
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

impl<'a> StyleValue<'a> {
	pub fn default_for(name: &Atom) -> Option<Self> {
		macro_rules! default_value {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match name {
					$(
						&$atom => Some(Self::$name(values::$name::default())),
					)+
					_ => None,
				}
			}
		}
		apply_properties!(default_value)
	}
}

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
		if let Some(token) = parser.peek::<T![Ident]>() {
			match parser.parse_atom_lower(token) {
				atom!("initial") => {
					parser.hop(token);
					return Ok(Self::Initial);
				}
				atom!("inherit") => {
					parser.hop(token);
					return Ok(Self::Inherit);
				}
				atom!("unset") => {
					parser.hop(token);
					return Ok(Self::Unset);
				}
				atom!("revert") => {
					parser.hop(token);
					return Ok(Self::Revert);
				}
				atom!("revert-layer") => {
					parser.hop(token);
					return Ok(Self::RevertLayer);
				}
				_ => {}
			}
		}
		if parser.peek::<Computed>().is_some() {
			return parser.parse::<Computed>().map(Self::Computed);
		}
		let checkpoint = parser.checkpoint();
		macro_rules! parse_declaration_value {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match name {
					$(
						&$atom => {
							if let Ok(val) = parser.parse::<values::$name>() {
								if parser.at_end() {
									return Ok(Self::$name(val))
								} else if let Some(token) = parser.peek::<T![Any]>() {
									if matches!(token.char(), Some(';' | '}' | '!')) {
										return Ok(Self::$name(val))
									}
								}
							}
						},
					)+
					_ => {}
				}
			}
		}
		apply_properties!(parse_declaration_value);
		if parser.peek::<Computed>().is_some() {
			parser.rewind(checkpoint);
			Ok(Self::Computed(Computed::parse(parser)?))
		} else {
			parser.rewind(checkpoint);
			Ok(Self::Unknown(Unknown::parse(parser)?))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Property, 136);
		assert_size!(StyleValue, 120);
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
