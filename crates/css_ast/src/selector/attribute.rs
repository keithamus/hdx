use css_lexer::{Cursor, KindSet};
use css_parse::{Build, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::{Visit, Visitable};

use super::NamespacePrefix;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct Attribute {
	pub open: T!['['],
	pub namespace_prefix: Option<NamespacePrefix>,
	pub attribute: T![Ident],
	pub operator: Option<AttributeOperator>,
	pub value: Option<AttributeValue>,
	pub modifier: Option<AttributeModifier>,
	pub close: Option<T![']']>,
}

impl<'a> Parse<'a> for Attribute {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse::<T!['[']>()?;
		let mut namespace_prefix = if p.peek::<T![*|]>() { Some(p.parse::<NamespacePrefix>()?) } else { None };
		let mut attribute = p.parse::<T![Ident]>()?;
		let skip = p.set_skip(KindSet::NONE);
		// namespace_prefix might be `<Ident> '|' <Ident>`
		if namespace_prefix.is_none() && p.peek::<T![|]>() && !p.peek::<T![|=]>() {
			let pipe = p.parse::<T![|]>();
			let ident = p.parse::<T![Ident]>();
			p.set_skip(skip);
			namespace_prefix = Some(NamespacePrefix::Name(attribute, pipe?));
			attribute = ident?;
		}
		p.set_skip(skip);
		let operator = p.parse_if_peek::<AttributeOperator>()?;
		let value = if operator.is_some() { Some(p.parse::<AttributeValue>()?) } else { None };
		let modifier =
			if value.is_some() && p.peek::<AttributeModifier>() { Some(p.parse::<AttributeModifier>()?) } else { None };
		let close = p.parse_if_peek::<T![']']>()?;
		Ok(Self { open, namespace_prefix, attribute, operator, value, modifier, close })
	}
}

impl<'a> ToCursors for Attribute {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		if let Some(namespace_prefix) = &self.namespace_prefix {
			ToCursors::to_cursors(namespace_prefix, s);
		}
		s.append(self.attribute.into());
		if let Some(operator) = &self.operator {
			ToCursors::to_cursors(operator, s);
		}
		if let Some(value) = self.value {
			s.append(value.into());
		}
		if let Some(modifier) = self.modifier {
			s.append(modifier.into());
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

impl<'a> Visitable<'a> for Attribute {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_attribute(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeOperator {
	Exact(T![=]),
	SpaceList(T![~=]),
	LangPrefix(T![|=]),
	Prefix(T![^=]),
	Suffix(T!["$="]),
	Contains(T![*=]),
}

impl<'a> Peek<'a> for AttributeOperator {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![=]>::peek(p, c)
			|| <T![~=]>::peek(p, c)
			|| <T![|=]>::peek(p, c)
			|| <T![^=]>::peek(p, c)
			|| <T!["$="]>::peek(p, c)
			|| <T![*=]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for AttributeOperator {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let c = p.peek_n(1);
		if <T![=]>::peek(p, c) {
			p.parse::<T![=]>().map(AttributeOperator::Exact)
		} else if <T![~=]>::peek(p, c) {
			p.parse::<T![~=]>().map(AttributeOperator::SpaceList)
		} else if <T![|=]>::peek(p, c) {
			p.parse::<T![|=]>().map(AttributeOperator::LangPrefix)
		} else if <T![^=]>::peek(p, c) {
			p.parse::<T![^=]>().map(AttributeOperator::Prefix)
		} else if <T!["$="]>::peek(p, c) {
			p.parse::<T!["$="]>().map(AttributeOperator::Suffix)
		} else {
			p.parse::<T![*=]>().map(AttributeOperator::Contains)
		}
	}
}

impl<'a> ToCursors for AttributeOperator {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Exact(c) => s.append(c.into()),
			Self::SpaceList(c) => ToCursors::to_cursors(c, s),
			Self::LangPrefix(c) => ToCursors::to_cursors(c, s),
			Self::Prefix(c) => ToCursors::to_cursors(c, s),
			Self::Suffix(c) => ToCursors::to_cursors(c, s),
			Self::Contains(c) => ToCursors::to_cursors(c, s),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeValue {
	String(T![String]),
	Ident(T![Ident]),
}

impl<'a> Peek<'a> for AttributeValue {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) || <T![String]>::peek(p, c)
	}
}

impl<'a> Build<'a> for AttributeValue {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Ident]>::peek(p, c) {
			Self::Ident(<T![Ident]>::build(p, c))
		} else {
			Self::String(<T![String]>::build(p, c))
		}
	}
}

impl From<AttributeValue> for Cursor {
	fn from(value: AttributeValue) -> Cursor {
		match value {
			AttributeValue::Ident(c) => c.into(),
			AttributeValue::String(c) => c.into(),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AttributeModifier {
	Sensitive(T![Ident]),
	Insensitive(T![Ident]),
}

impl<'a> Peek<'a> for AttributeModifier {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) && matches!(p.parse_str(c), "i" | "s" | "I" | "S")
	}
}

impl<'a> Build<'a> for AttributeModifier {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if matches!(p.parse_str(c), "s" | "S") {
			Self::Sensitive(<T![Ident]>::build(p, c))
		} else {
			Self::Insensitive(<T![Ident]>::build(p, c))
		}
	}
}

impl From<AttributeModifier> for Cursor {
	fn from(value: AttributeModifier) -> Self {
		match value {
			AttributeModifier::Sensitive(c) => c.into(),
			AttributeModifier::Insensitive(c) => c.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Attribute>(), 128);
		assert_eq!(std::mem::size_of::<AttributeOperator>(), 28);
		assert_eq!(std::mem::size_of::<AttributeModifier>(), 16);
		assert_eq!(std::mem::size_of::<AttributeValue>(), 16);
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
		assert_parse!(Attribute, "[attr|=foo i]");
		assert_parse!(Attribute, "[attr|=foo s]");
		assert_parse!(Attribute, "[attr|='foo'i]");
		assert_parse!(Attribute, "[attr|='foo's]");
	}
}
