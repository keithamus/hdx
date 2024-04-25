use hdx_atom::{atom, Atom};
use hdx_parser::{expect_ignore_case, Declaration, Parse, Parser, Result as ParserResult, RuleList, Vec, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss, write_css};

use crate::css::properties::StyleValue;

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontFace<'a>(Vec<'a, Spanned<FontProperty<'a>>>);

impl<'a> Parse<'a> for FontFace<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case!(parser.next(), Token::AtKeyword(atom!("font-face")));
		let span = parser.span();
		Ok(Self(Self::parse_rule_list(parser)?))
	}
}

impl<'a> RuleList<'a> for FontFace<'a> {
	type Rule = FontProperty<'a>;
}

impl<'a> WriteCss<'a> for FontFace<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		write_css!(sink, '@', atom!("font-face"), (), '{');
		sink.indent();
		sink.write_newline()?;
		let mut rules = self.0.iter().peekable();
		while let Some(rule) = rules.next() {
			sink.write_indent()?;
			rule.write_css(sink)?;
			if rules.peek().is_some() {
				sink.write_char(';')?;
			} else {
				sink.write_trailing_char(';')?;
			}
			sink.write_newline()?;
		}
		sink.write_char('}')
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
pub struct FontProperty<'a> {
	name: Atom,
	value: StyleValue<'a>,
	important: bool,
}

impl<'a> Parse<'a> for FontProperty<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case!(
			parser.peek(),
			Token::Ident(
				atom!("ascent-override")
					| atom!("descent-override")
					| atom!("font-display")
					| atom!("font-family")
					| atom!("font-feature-settings")
					| atom!("font-language-override")
					| atom!("font-named-instance")
					| atom!("font-style")
					| atom!("font-variation-settings")
					| atom!("font-weight")
					| atom!("font-width")
					| atom!("line-gap-override")
					| atom!("src") | atom!("unicode-range")
			)
		);
		let (name, value, important) = Self::parse_declaration(parser)?;
		Ok(Self { name, value, important })
	}
}

impl<'a> Declaration<'a> for FontProperty<'a> {
	type DeclarationValue = StyleValue<'a>;
}

impl<'a> WriteCss<'a> for FontProperty<'a> {
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

// macro_rules! font_value {
//     ( $(
//         $name: ident$(<$a: lifetime>)?: $atom: pat,
//     )+ ) => {
// 		#[derive(PartialEq, Debug, Hash)]
// 		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
// 		pub enum StyleValue<'a> {
// 			Initial,
// 			Inherit,
// 			Unset,
// 			Revert,
// 			RevertLayer,
// 			#[cfg_attr(feature = "serde", serde(untagged))]
// 			Custom(Custom<'a>),
// 			#[cfg_attr(feature = "serde", serde(untagged))]
// 			Computed(Computed<'a>),
// 			#[cfg_attr(feature = "serde", serde(untagged))]
// 			Unknown(Unknown<'a>),
// 			$(
// 				#[cfg_attr(feature = "serde", serde(untagged))]
// 				$name(values::$name$(<$a>)?),
// 			)+
// 		}
// 	}
// }
//
// apply_properties!(style_value);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontFace, 32);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFace, "@font-face {}");
	}
}
