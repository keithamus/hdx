// https://drafts.css-houdini.org/css-properties-values-api/#at-ruledef-property
use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{
	diagnostics, AtRule, CursorStream, Declaration, DeclarationList, DeclarationValue, Parse, Parser,
	Result as ParserResult, ToCursors, T,
};

use crate::syntax::ComponentValues;

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PropertyRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub name: T![DashedIdent],
	pub block: PropertyRuleBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for PropertyRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, name, block) = Self::parse_at_rule(p, Some(atom!("property")))?;
		if let Some(name) = name {
			Ok(Self { at_keyword, name, block })
		} else {
			let c: Cursor = at_keyword.into();
			Err(diagnostics::MissingAtRulePrelude(c.into()))?
		}
	}
}

impl<'a> AtRule<'a> for PropertyRule<'a> {
	type Prelude = T![DashedIdent];
	type Block = PropertyRuleBlock<'a>;
}

impl<'a> ToCursors<'a> for PropertyRule<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		s.append(self.name.into());
		ToCursors::to_cursors(&self.block, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PropertyRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, PropertyRuleProperty<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for PropertyRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_declaration_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> DeclarationList<'a> for PropertyRuleBlock<'a> {
	type Declaration = PropertyRuleProperty<'a>;
}

impl<'a> ToCursors<'a> for PropertyRuleBlock<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.open.into());
		for property in &self.properties {
			ToCursors::to_cursors(property, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PropertyRuleProperty<'a> {
	pub name: T![Ident],
	pub colon: Option<T![:]>,
	pub value: PropertyRuleStyleValue<'a>,
	pub semicolon: Option<T![;]>,
}

impl<'a> Parse<'a> for PropertyRuleProperty<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important, semicolon) = Self::parse_declaration(p)?;
		if let Some(important) = important {
			let c: Cursor = important.bang.into();
			Err(diagnostics::DisallowedImportant(c.into()))?
		}
		Ok(Self { name, colon, value, semicolon })
	}
}

impl<'a> Declaration<'a> for PropertyRuleProperty<'a> {
	type DeclarationValue = PropertyRuleStyleValue<'a>;
}

impl<'a> ToCursors<'a> for PropertyRuleProperty<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.name.into());
		if let Some(colon) = self.colon {
			s.append(colon.into());
		}
		ToCursors::to_cursors(&self.value, s);
		if let Some(semicolon) = self.semicolon {
			s.append(semicolon.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PropertyRuleStyleValue<'a> {
	InitialValue(ComponentValues<'a>),
	Syntax(T![String]),
	Inherits(T![Ident]),
	Unknown(ComponentValues<'a>),
}

impl<'a> DeclarationValue<'a> for PropertyRuleStyleValue<'a> {
	fn parse_declaration_value(name: Cursor, p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match p.parse_atom_lower(name) {
			atom!("initial-value") => Self::InitialValue(p.parse::<ComponentValues<'a>>()?),
			atom!("inherits") => {
				let value = p.parse::<T![Ident]>()?;
				let c: Cursor = value.into();
				match p.parse_atom_lower(c) {
					atom!("false") => Self::Inherits(value),
					atom!("true") => Self::Inherits(value),
					atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
				}
			}
			atom!("syntax") => Self::Syntax(p.parse::<T![String]>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

impl<'a> ToCursors<'a> for PropertyRuleStyleValue<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::InitialValue(value) => ToCursors::to_cursors(value, s),
			Self::Syntax(string) => {
				s.append(string.into());
			}
			Self::Inherits(ident) => {
				s.append(ident.into());
			}
			Self::Unknown(value) => ToCursors::to_cursors(value, s),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PropertyRule, 80);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PropertyRule, r#"@property --foo{initial-value:0;inherits:false;syntax:"<length>"}"#);
	}
}
