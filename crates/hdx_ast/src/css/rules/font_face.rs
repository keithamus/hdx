use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{
	AtRule, CursorSink, Declaration, Important, NoPreludeAllowed, Parse, Parser, Result as ParserResult, RuleList,
	ToCursors, T,
};
use hdx_proc_macro::visit;

use crate::css::{properties::StyleValue, Visit, Visitable};

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FontFaceRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub block: FontFaceRuleBlock<'a>,
}

impl<'a> AtRule<'a> for FontFaceRule<'a> {
	type Prelude = NoPreludeAllowed;
	type Block = FontFaceRuleBlock<'a>;
}

impl<'a> Parse<'a> for FontFaceRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, _, block) = Self::parse_at_rule(p, Some(atom!("font-face")))?;
		Ok(Self { at_keyword, block })
	}
}

impl<'a> ToCursors for FontFaceRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for FontFaceRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_font_face_rule(self);
		Visitable::accept(&self.block, v);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontFaceRuleBlock<'a> {
	pub open: T!['{'],
	pub properties: Vec<'a, FontFaceRuleProperty<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> RuleList<'a> for FontFaceRuleBlock<'a> {
	type Rule = FontFaceRuleProperty<'a>;
}

impl<'a> Parse<'a> for FontFaceRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> ToCursors for FontFaceRuleBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for property in &self.properties {
			ToCursors::to_cursors(property, s);
		}
		if let Some(close) = &self.close {
			s.append(close.into());
		}
	}
}

impl<'a> Visitable<'a> for FontFaceRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for property in &self.properties {
			Visitable::accept(property, v);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
#[visit]
pub struct FontFaceRuleProperty<'a> {
	pub name: T![Ident],
	pub colon: T![:],
	pub value: StyleValue<'a>,
	pub important: Option<Important>,
	pub semicolon: Option<T![;]>,
}

impl<'a> Declaration<'a> for FontFaceRuleProperty<'a> {
	type DeclarationValue = StyleValue<'a>;
	fn valid_property(p: &Parser, c: Cursor) -> bool {
		matches!(
			p.parse_atom_lower(c),
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
				| atom!("src")
				| atom!("unicode-range")
		)
	}
}

impl<'a> Parse<'a> for FontFaceRuleProperty<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important, semicolon) = Self::parse_declaration(p)?;
		Ok(Self { name, colon, value, important, semicolon })
	}
}

impl<'a> ToCursors for FontFaceRuleProperty<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		s.append(self.colon.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(important) = &self.important {
			ToCursors::to_cursors(important, s);
		}
		if let Some(semicolon) = &self.semicolon {
			ToCursors::to_cursors(semicolon, s);
		}
	}
}

impl<'a> Visitable<'a> for FontFaceRuleProperty<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_font_face_rule_property(self);
		Visitable::accept(&self.value, v);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontFaceRule, 80);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFaceRule, "@font-face {}");
	}
}
