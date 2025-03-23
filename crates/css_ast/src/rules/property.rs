use crate::{Visit, Visitable};
use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	diagnostics, keyword_set, syntax::ComponentValues, AtRule, Build, CursorSink, Declaration, DeclarationList,
	DeclarationValue, Parse, Parser, Peek, Result as ParserResult, ToCursors, T,
};
use csskit_proc_macro::visit;

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PropertyRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub name: T![DashedIdent],
	pub block: PropertyRuleBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for PropertyRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, name, block) = Self::parse_at_rule(p)?;
		if let Some(name) = name {
			Ok(Self { at_keyword, name, block })
		} else {
			let c: Cursor = at_keyword.into();
			Err(diagnostics::MissingAtRulePrelude(c.into()))?
		}
	}
}

impl<'a> AtRule<'a> for PropertyRule<'a> {
	const NAME: Option<&'static str> = Some("property");
	type Prelude = T![DashedIdent];
	type Block = PropertyRuleBlock<'a>;
}

impl<'a> ToCursors for PropertyRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		s.append(self.name.into());
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for PropertyRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_property_rule(self);
		for (property, _) in &self.block.properties {
			Visitable::accept(property, v);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PropertyRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, (PropertyRuleProperty<'a>, Option<T![;]>)>,
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

impl<'a> ToCursors for PropertyRuleBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for (property, semicolon) in &self.properties {
			ToCursors::to_cursors(property, s);
			if let Some(semicolon) = semicolon {
				s.append(semicolon.into());
			}
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PropertyRuleProperty<'a> {
	pub name: T![Ident],
	pub colon: T![:],
	pub value: PropertyRuleStyleValue<'a>,
}

impl<'a> Parse<'a> for PropertyRuleProperty<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important) = Self::parse_declaration(p)?;
		if let Some(important) = important {
			let c: Cursor = important.bang.into();
			Err(diagnostics::DisallowedImportant(c.into()))?
		}
		Ok(Self { name, colon, value })
	}
}

impl<'a> Declaration<'a> for PropertyRuleProperty<'a> {
	type DeclarationValue = PropertyRuleStyleValue<'a>;
}

impl<'a> ToCursors for PropertyRuleProperty<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		s.append(self.colon.into());
		ToCursors::to_cursors(&self.value, s);
	}
}

impl<'a> Visitable<'a> for PropertyRuleProperty<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_property_rule_property(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PropertyRuleStyleValue<'a> {
	InitialValue(ComponentValues<'a>),
	Syntax(T![String]),
	Inherits(InheritsStyleValue),
	Unknown(ComponentValues<'a>),
}

keyword_set!(PropertyRulePropertyId { InitialValue: "initial-value", Inherits: "inherits", Syntax: "syntax" });

keyword_set!(InheritsStyleValue { True: "true", False: "false" });

impl<'a> DeclarationValue<'a> for PropertyRuleStyleValue<'a> {
	fn parse_declaration_value(p: &mut Parser<'a>, c: Cursor) -> ParserResult<Self> {
		if !PropertyRulePropertyId::peek(p, c) {
			Ok(Self::Unknown(p.parse::<ComponentValues<'a>>()?))
		} else {
			Ok(match PropertyRulePropertyId::build(p, c) {
				PropertyRulePropertyId::InitialValue(_) => Self::InitialValue(p.parse::<ComponentValues<'a>>()?),
				PropertyRulePropertyId::Inherits(_) => Self::Inherits(p.parse::<InheritsStyleValue>()?),
				PropertyRulePropertyId::Syntax(_) => Self::Syntax(p.parse::<T![String]>()?),
			})
		}
	}
}

impl ToCursors for PropertyRuleStyleValue<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PropertyRule>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PropertyRule, r#"@property --foo{initial-value:0;inherits:false;syntax:"<length>"}"#);
	}
}
