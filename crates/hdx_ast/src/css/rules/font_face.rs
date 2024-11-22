use hdx_atom::{atom, Atom};
use hdx_parser::{diagnostics, Declaration, Parse, Parser, Result as ParserResult, RuleList, Spanned, Vec, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use crate::css::properties::StyleValue;

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontFace<'a>(Vec<'a, Spanned<FontProperty<'a>>>);

impl<'a> Parse<'a> for FontFace<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![AtKeyword]>()?;
		let atom = p.parse_atom_lower(token);
		if atom != atom!("font-face") {
			Err(diagnostics::UnexpectedAtRule(atom, token.span()))?
		}
		Ok(Self(Self::parse_rule_list(p)?))
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
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = p.peek::<T![Ident]>() {
			let atom = p.parse_atom_lower(token);
			if !matches!(
				atom,
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
					| atom!("src") | atom!("unicode-range"),
			) {
				Err(diagnostics::UnexpectedIdent(atom, token.span()))?
			}
		} else {
			let token = p.peek::<T![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
		let (name, value, important) = Self::parse_declaration(p)?;
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
