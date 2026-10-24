use super::prelude::*;
use crate::Syntax;
use css_parse::ComponentValues;

/// <https://drafts.csswg.org/css-values-5/#attr-notation>
///
/// ```text,ignore
/// attr() = attr( <attr-name> <attr-type>? , <declaration-value>?)
/// <attr-type> = type( <syntax> ) | raw-string | <attr-unit>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AttrFunction<'a> {
	#[atom(CssAtomSet::Attr)]
	pub name: T![Function],
	pub params: AttrFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AttrFunctionParams<'a>(
	pub AttrName,
	pub Option<AttrType<'a>>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<ComponentValues<'a>>,
);

/// ```text,ignore
/// <attr-name> = [ <ident-token>? '|' ]? <ident-token>
/// ```
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AttrName(pub Option<T![Ident]>, #[semantic_eq(skip)] pub Option<T![|]>, pub Option<T![Ident]>);

impl<'a> Peek<'a> for AttrName {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);
}

impl<'a> Parse<'a> for AttrName {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let a = p.parse_if_peek::<T![Ident]>()?;
		let b = p.parse_if_peek::<T![|]>()?;

		if a.is_some() && b.is_none() {
			return Ok(Self(None, None, a));
		}

		if a.is_none() && b.is_some() {
			return Ok(Self(None, b, Some(p.parse::<T![Ident]>()?)));
		}

		if a.is_none() && b.is_none() {
			Err(Diagnostic::new(p.next(), Diagnostic::expected_ident))?
		}

		debug_assert!(a.is_some() && b.is_some());

		Ok(Self(a, b, Some(p.parse::<T![Ident]>()?)))
	}
}

/// ```text,ignore
/// <attr-type> = type( <syntax> ) | raw-string | <attr-unit>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum AttrType<'a> {
	#[atom(CssAtomSet::Type)]
	Type(T![Function], Syntax<'a>, #[semantic_eq(skip)] T![')']),
	#[atom(CssAtomSet::RawString)]
	RawString(T![Ident]),
	Unit(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(bar px)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo|bar px)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo|bar)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(|bar)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(|bar px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AttrName, "a|b|c");
		assert_peek_false!(CssAtomSet::ATOMS, AttrFunction, "attrr(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, AttrFunction, "attr()");
		assert_parse_error!(CssAtomSet::ATOMS, AttrFunction, "attr(|)");
	}
}
