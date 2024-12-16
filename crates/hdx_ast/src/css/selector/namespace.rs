use hdx_lexer::{Cursor, KindSet, Span};
use hdx_parser::{Build, CursorSink, Is, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

use super::Tag;

// https://drafts.csswg.org/selectors/#combinators
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub struct Namespace {
	pub prefix: Option<NamespacePrefix>,
	pub tag: NamespaceTag,
}

impl<'a> Parse<'a> for Namespace {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![*|]>() {
			let prefix = p.parse::<NamespacePrefix>()?;
			let tag = p.parse::<NamespaceTag>()?;
			return Ok(Self { prefix: Some(prefix), tag });
		}
		if p.peek::<T![|]>() {
			let prefix = p.parse::<NamespacePrefix>()?;
			let tag = p.parse::<NamespaceTag>()?;
			return Ok(Self { prefix: Some(prefix), tag });
		}

		let ident = p.parse::<T![Ident]>()?;
		let skip = p.set_skip(KindSet::NONE);
		if p.peek::<T![|]>() && !p.peek::<T![|=]>() {
			let pipe = p.parse::<T![|]>();
			let tag = p.parse::<NamespaceTag>();
			p.set_skip(skip);
			let prefix = NamespacePrefix::Name(ident, pipe?);
			return Ok(Self { prefix: Some(prefix), tag: tag? });
		}
		let tag = p.parse::<NamespaceTag>()?;
		Ok(Self { prefix: None, tag })
	}
}

impl<'a> ToCursors for Namespace {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(prefix) = &self.prefix {
			ToCursors::to_cursors(prefix, s);
		}
		s.append(self.tag.into());
	}
}

impl From<&Namespace> for Span {
	fn from(value: &Namespace) -> Self {
		if let Some(prefix) = value.prefix {
			Into::<Span>::into(&prefix) + (&value.tag).into()
		} else {
			(&value.tag).into()
		}
	}
}

impl<'a> Visitable<'a> for Namespace {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_namespace(self);
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NamespacePrefix {
	None(T![|]),
	Name(T![Ident], T![|]),
	Wildcard(T![*], T![|]),
}

impl<'a> Parse<'a> for NamespacePrefix {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![|]>() {
			let pipe = p.parse::<T![|]>()?;
			Ok(Self::None(pipe))
		} else if p.peek::<T![*]>() {
			let star = p.parse::<T![*]>()?;
			let skip = p.set_skip(KindSet::NONE);
			let pipe = p.parse::<T![|]>();
			p.set_skip(skip);
			let pipe = pipe?;
			Ok(Self::Wildcard(star, pipe))
		} else {
			let star = p.parse::<T![Ident]>()?;
			let skip = p.set_skip(KindSet::NONE);
			let pipe = p.parse::<T![|]>();
			p.set_skip(skip);
			let pipe = pipe?;
			Ok(Self::Name(star, pipe))
		}
	}
}

impl<'a> ToCursors for NamespacePrefix {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::None(pipe) => {
				s.append(pipe.into());
			}
			Self::Name(ident, pipe) => {
				s.append(ident.into());
				s.append(pipe.into());
			}
			Self::Wildcard(star, pipe) => {
				s.append(star.into());
				s.append(pipe.into());
			}
		}
	}
}

impl From<&NamespacePrefix> for Span {
	fn from(value: &NamespacePrefix) -> Self {
		match value {
			NamespacePrefix::None(pipe) => pipe.into(),
			NamespacePrefix::Name(ident, pipe) => Into::<Span>::into(ident) + pipe.into(),
			NamespacePrefix::Wildcard(star, pipe) => Into::<Span>::into(star) + pipe.into(),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NamespaceTag {
	Tag(Tag),
	Wildcard(T![*]),
}

impl<'a> Is<'a> for NamespaceTag {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![*]>::is(p, c) || Tag::is(p, c)
	}
}

impl<'a> Build<'a> for NamespaceTag {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![*]>::is(p, c) {
			Self::Wildcard(<T![*]>::build(p, c))
		} else {
			Self::Tag(Tag::build(p, c))
		}
	}
}

impl From<NamespaceTag> for Cursor {
	fn from(value: NamespaceTag) -> Self {
		match value {
			NamespaceTag::Tag(c) => c.into(),
			NamespaceTag::Wildcard(c) => c.into(),
		}
	}
}

impl From<&NamespaceTag> for Span {
	fn from(value: &NamespaceTag) -> Self {
		Into::<Cursor>::into(*value).into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Namespace, 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Namespace, "*|a");
		assert_parse!(Namespace, "html|div");
		assert_parse!(Namespace, "|span");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Namespace, "* | a");
	}
}
