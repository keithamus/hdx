use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{
	AtRule, CursorStream, Declaration, Important, NoPreludeAllowed, Parse, Parser, Result as ParserResult, RuleList,
	ToCursors, T,
};

use crate::css::properties::StyleValue;

// https://drafts.csswg.org/css-fonts/#font-face-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontFace<'a> {
	pub at_keyword: T![AtKeyword],
	pub block: FontFaceDeclaration<'a>,
}

impl<'a> AtRule<'a> for FontFace<'a> {
	type Prelude = NoPreludeAllowed;
	type Block = FontFaceDeclaration<'a>;
}

impl<'a> Parse<'a> for FontFace<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, _, block) = Self::parse_at_rule(p, Some(atom!("font-face")))?;
		Ok(Self { at_keyword, block })
	}
}

impl<'a> ToCursors<'a> for FontFace<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.block, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontFaceDeclaration<'a> {
	pub open: T!['{'],
	pub properties: Vec<'a, FontProperty<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> RuleList<'a> for FontFaceDeclaration<'a> {
	type Rule = FontProperty<'a>;
}

impl<'a> Parse<'a> for FontFaceDeclaration<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> ToCursors<'a> for FontFaceDeclaration<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.open.into());
		for property in &self.properties {
			ToCursors::to_cursors(property, s);
		}
		if let Some(close) = &self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
pub struct FontProperty<'a> {
	pub name: T![Ident],
	pub colon: Option<T![:]>,
	pub value: StyleValue<'a>,
	pub important: Option<Important>,
	pub semicolon: Option<T![;]>,
}

impl<'a> Declaration<'a> for FontProperty<'a> {
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

impl<'a> Parse<'a> for FontProperty<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important, semicolon) = Self::parse_declaration(p)?;
		Ok(Self { name, colon, value, important, semicolon })
	}
}

impl<'a> ToCursors<'a> for FontProperty<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.name.into());
		if let Some(colon) = self.colon {
			s.append(colon.into());
		}
		ToCursors::to_cursors(&self.value, s);
		if let Some(important) = &self.important {
			ToCursors::to_cursors(important, s);
		}
		if let Some(semicolon) = &self.semicolon {
			ToCursors::to_cursors(semicolon, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontFace, 72);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(FontFace, "@font-face {}");
	}
}
