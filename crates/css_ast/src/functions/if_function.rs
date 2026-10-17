use super::prelude::*;
use crate::{CssMetadata, MediaCondition, StyleQuery, StyleValue, SupportsCondition};
use css_parse::{
	ComponentValues, Declaration, FeatureConditionList,
	token_macros::{Colon, Semicolon},
};

/// A single `<if-test>` inside an [`IfCondition`] boolean expression.
///
/// <https://drafts.csswg.org/css-values-5/#typedef-if-test>
///
/// ```text,ignore
/// <if-test> =
///   supports( [ <ident> : <declaration-value> ] | <supports-condition> ) |
///   media( <media-feature> | <media-condition> ) |
///   style( <style-query> )
/// ```
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum IfTest<'a> {
	/// `supports( <ident> : <declaration-value> )`
	SupportsDeclaration(
		#[cfg_attr(feature = "visitable", visit(skip))] Function,
		Box<'a, Declaration<'a, StyleValue<'a>, CssMetadata>>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		RightParen,
	),
	/// `supports( <supports-condition> )`
	Supports(
		#[cfg_attr(feature = "visitable", visit(skip))] Function,
		SupportsCondition<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		RightParen,
	),
	/// `media( <media-feature> | <media-condition> )`
	Media(
		#[cfg_attr(feature = "visitable", visit(skip))] Function,
		Box<'a, MediaCondition<'a>>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		RightParen,
	),
	/// `style( <style-query> )`
	Style(
		#[cfg_attr(feature = "visitable", visit(skip))] Function,
		StyleQuery<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		RightParen,
	),
	/// `( <boolean-expr[ <if-test> ]> )`
	Group(
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		LeftParen,
		Box<'a, IfConditionExpr<'a>>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		RightParen,
	),
	/// `<general-enclosed>`: forward-compatible unknown, preserved verbatim.
	#[cfg_attr(feature = "visitable", visit(skip))]
	GeneralEnclosed(
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Function,
		ComponentValues<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		RightParen,
	),
}

impl<'a> Peek<'a> for IfTest<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftParen, Kind::Function]);
}

impl<'a> Parse<'a> for IfTest<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// `( <boolean-expr[ <if-test> ]> )` grouping branch.
		if p.peek::<LeftParen>() {
			let open = p.parse::<LeftParen>()?;
			let expr = p.parse::<IfConditionExpr>()?;
			let close = p.parse::<RightParen>()?;
			return Ok(Self::Group(open, Box::new_in(p.alloc(), expr), close));
		}
		let function = p.parse::<Function>()?;
		match p.to_atom::<CssAtomSet>(function.into()) {
			CssAtomSet::Supports => {
				// `supports( <ident> : <declaration-value> )` vs `supports( <supports-condition> )`.
				// A bare declaration is an ident (or dashed-ident) directly followed by a colon; a
				// `<supports-condition>` always begins with `(`, `not`, or a function.
				let c = p.peek_n(1);
				if c == Kind::Ident && p.peek_n(2) == Kind::Colon {
					let decl = p.parse::<Declaration<'a, StyleValue<'a>, CssMetadata>>()?;
					let close = p.parse::<RightParen>()?;
					Ok(Self::SupportsDeclaration(function, Box::new_in(p.alloc(), decl), close))
				} else {
					let condition = p.parse::<SupportsCondition>()?;
					let close = p.parse::<RightParen>()?;
					Ok(Self::Supports(function, condition, close))
				}
			}
			CssAtomSet::Media => {
				let condition = p.parse::<MediaCondition>()?;
				let close = p.parse::<RightParen>()?;
				Ok(Self::Media(function, Box::new_in(p.alloc(), condition), close))
			}
			CssAtomSet::Style => {
				let query = p.parse::<StyleQuery>()?;
				let close = p.parse::<RightParen>()?;
				Ok(Self::Style(function, query, close))
			}
			// `<general-enclosed>`: any other function is preserved verbatim for forward-compat.
			_ => {
				let values = p.parse::<ComponentValues>()?;
				let close = p.parse::<RightParen>()?;
				Ok(Self::GeneralEnclosed(function, values, close))
			}
		}
	}
}

/// A `<boolean-expr[ <if-test> ]>`: one or more [`IfTest`]s combined with `not`/`and`/`or`.
///
/// <https://drafts.csswg.org/css-values-5/#typedef-boolean-expr>
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum IfConditionExpr<'a> {
	Is(IfTest<'a>),
	Not(Ident, IfTest<'a>),
	#[cfg_attr(feature = "visitable", visit(skip))]
	And(Vec<'a, (IfTest<'a>, Option<Ident>)>),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Or(Vec<'a, (IfTest<'a>, Option<Ident>)>),
}

impl<'a> FeatureConditionList<'a> for IfConditionExpr<'a> {
	type FeatureCondition = IfTest<'a>;
	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: IfTest<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: Ident, feature: IfTest<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(features: Vec<'a, (IfTest<'a>, Option<Ident>)>) -> Self {
		Self::And(features)
	}
	fn build_or(features: Vec<'a, (IfTest<'a>, Option<Ident>)>) -> Self {
		Self::Or(features)
	}
}

impl<'a> Peek<'a> for IfConditionExpr<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftParen, Kind::Function, Kind::Ident]);
}

impl<'a> Parse<'a> for IfConditionExpr<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

/// An `<if-condition>`: either a `<boolean-expr[ <if-test> ]>` or the `else` keyword (always true).
///
/// <https://drafts.csswg.org/css-values-5/#typedef-if-condition>
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum IfCondition<'a> {
	Else(#[atom(CssAtomSet::Else)] Ident),
	Expr(IfConditionExpr<'a>),
}

/// A single `<if-branch>`: `<if-condition> : <declaration-value>?`. The branch value `V` is the
/// enclosing value slot (e.g. `Value<'a, T>`), so substitution functions inside it are preserved.
///
/// <https://drafts.csswg.org/css-values-5/#typedef-if-branch>
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct IfBranch<'a, V> {
	pub condition: IfCondition<'a>,
	#[semantic_eq(skip)]
	pub colon: Colon,
	pub value: Option<V>,
}

/// if() function: conditional CSS value selection.
///
/// <https://drafts.csswg.org/css-values-5/#if-notation>
///
/// ```text,ignore
/// <if()> = if( [ <if-branch> ; ]* <if-branch> ;? )
/// ```
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct IfFunction<'a, V> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub name: Function,
	pub branches: Vec<'a, (IfBranch<'a, V>, Option<Semicolon>)>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

impl<'a, V: Peek<'a>> Peek<'a> for IfFunction<'a, V> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Function]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<Function>::peek(p, c) && p.equals_atom(c, &CssAtomSet::If)
	}
}

impl<'a, V: Parse<'a> + Peek<'a>> Parse<'a> for IfFunction<'a, V> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let name = p.parse::<Function>()?;
		let mut branches = Vec::new_in(p.alloc());
		loop {
			let branch = p.parse::<IfBranch<'a, V>>()?;
			let semicolon = p.parse_if_peek::<Semicolon>()?;
			let had_semicolon = semicolon.is_some();
			branches.push((branch, semicolon));
			if !had_semicolon || p.peek::<RightParen>() {
				break;
			}
		}
		let close = p.parse::<RightParen>()?;
		Ok(Self { name, branches, close })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, Length, Value};
	use css_parse::assert_parse;

	type IfLength<'a> = IfFunction<'a, Value<'a, Length>>;

	#[test]
	fn test_if_function() {
		assert_parse!(CssAtomSet::ATOMS, IfLength, "if(style(--x: 1px): 10px; else: 20px)");
		assert_parse!(CssAtomSet::ATOMS, IfLength, "if(else: 1px)");
		assert_parse!(CssAtomSet::ATOMS, IfLength, "if(supports(color: red): 1px)");
		assert_parse!(CssAtomSet::ATOMS, IfLength, "if(media((width: 100px)): 1px)");
		assert_parse!(CssAtomSet::ATOMS, IfLength, "if(supports(color: red) and style(--x: 1px): 1px)");
		assert_parse!(CssAtomSet::ATOMS, IfLength, "if(not style(--x: 1px): 1px)");
		assert_parse!(CssAtomSet::ATOMS, IfLength, "if(else: var(--fallback))");
	}
}
