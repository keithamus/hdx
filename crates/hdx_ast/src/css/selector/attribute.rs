use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{Build, CursorSink, Is, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

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
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<T![=]>()
			|| p.peek::<T![~=]>()
			|| p.peek::<T![|=]>()
			|| p.peek::<T![^=]>()
			|| p.peek::<T!["$="]>()
			|| p.peek::<T![*=]>()
	}
}

impl<'a> Parse<'a> for AttributeOperator {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![=]>() {
			p.parse::<T![=]>().map(AttributeOperator::Exact)
		} else if p.peek::<T![~=]>() {
			p.parse::<T![~=]>().map(AttributeOperator::SpaceList)
		} else if p.peek::<T![|=]>() {
			p.parse::<T![|=]>().map(AttributeOperator::LangPrefix)
		} else if p.peek::<T![^=]>() {
			p.parse::<T![^=]>().map(AttributeOperator::Prefix)
		} else if p.peek::<T!["$="]>() {
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

impl<'a> Is<'a> for AttributeValue {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::is(p, c) || <T![String]>::is(p, c)
	}
}

impl<'a> Build<'a> for AttributeValue {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Ident]>::is(p, c) {
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

impl<'a> Is<'a> for AttributeModifier {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::is(p, c) && matches!(p.parse_atom_lower(c), atom!("i") | atom!("s"))
	}
}

impl<'a> Build<'a> for AttributeModifier {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if p.parse_atom_lower(c) == atom!("s") {
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
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Attribute, 128);
		assert_size!(AttributeOperator, 28);
		assert_size!(AttributeModifier, 16);
		assert_size!(AttributeValue, 16);
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
