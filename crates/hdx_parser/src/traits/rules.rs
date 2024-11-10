use hdx_atom::Atom;
use hdx_lexer::{Kind, Span, Spanned};
use smallvec::{smallvec, SmallVec};

use crate::{diagnostics, discard, parser::Parser, Delim, Result, State, Token, Vec};

use super::Parse;

// An AtRule represents a block or statement with an @keyword in the leading
// position, such as @media, @charset, @import and so-on.
pub trait AtRule<'a>: Sized + Parse<'a> {
	type Prelude: Parse<'a>;
	type Block: Parse<'a>;

	// AtRules can have an optional prelude (e.g. @supoports requires one,
	// @starting-style must not have one, and in @page it is optional). Consequently
	// parse_prelude returns an Option, and rules that either require can check
	// in parse() or override parse_prelude() to err.
	fn parse_prelude(parser: &mut Parser<'a>) -> Result<Option<Spanned<Self::Prelude>>> {
		let next_token_kind = parser.peek::<Token![Any]>().map(|t| t.kind()).unwrap_or(Kind::Eof);
		if parser.at_end() || matches!(next_token_kind, Kind::LeftCurly | Kind::Semicolon) {
			return Ok(None);
		}
		Ok(Some(Self::Prelude::parse_spanned(parser)?))
	}

	// AtRules can have an optional block (e.g. @charset, @import must not have
	// one). The default parse_prelude returns an Option, and rules that either
	// require can check in parse() or override parse_prelude() to err.
	fn parse_block(parser: &mut Parser<'a>) -> Result<Option<Spanned<Self::Block>>> {
		let token = parser.peek_next();
		match token.kind() {
			Kind::Semicolon | Kind::Eof => {
				parser.next();
				Ok(None)
			}
			Kind::LeftCurly => Ok(Some(Self::Block::parse_spanned(parser)?)),
			_ => Err(diagnostics::Unexpected(token, token.span()))?,
		}
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
	fn parse_at_rule(
		parser: &mut Parser<'a>,
		name: Option<Atom>,
	) -> Result<(Option<Spanned<Self::Prelude>>, Option<Spanned<Self::Block>>)> {
		let token = *parser.parse::<Token![AtKeyword]>()?;
		let span = token.span();
		if let Some(name) = name {
			let atom = parser.parse_atom_lower(token);
			if atom != name {
				Err(diagnostics::UnexpectedAtRule(atom, span))?;
			}
		}
		let prelude = Self::parse_prelude(parser)?;
		let block = Self::parse_block(parser)?;
		Ok((prelude, block))
	}
}

pub trait QualifiedRule<'a>: Sized + Parse<'a> {
	type Prelude: Parse<'a>;
	type Block: Parse<'a>;

	// QualifiedRules must be able to consume a bad declaration, for when
	// a custom property like declaration is encountered.
	type BadDeclaration: Parse<'a>;

	// QualifiedRules must have a prelude, consequently parse_prelude must be
	// implemented.
	// parse_prelude is called right away, so could start with any token.
	fn parse_prelude(parser: &mut Parser<'a>) -> Result<Spanned<Self::Prelude>> {
		parser.parse_spanned::<Self::Prelude>()
	}

	// QualifiedRules must have a block, consequently parse_prelude must be
	// implemented.
	// parse_block will always start with a {-token.
	fn parse_block(parser: &mut Parser<'a>) -> Result<Spanned<Self::Block>> {
		parser.parse_spanned::<Self::Block>()
	}

	// QualifiedRules must be able to consume a block from their input when encountering
	// a custom property like declaration that doesn't end but opens a `{` block. This
	// is implemented as parsing the existing block as that' simplifies downstream logic
	// but consumers of this trait can instead opt to implement an optimised version of
	// this which doesn't build up an AST and just throws away tokens.
	fn consume_block(parser: &mut Parser<'a>) {
		parser.parse::<Self::Block>().ok();
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
	fn parse_qualified_rule(parser: &mut Parser<'a>) -> Result<(Spanned<Self::Prelude>, Spanned<Self::Block>)> {
		// Let rule be a new qualified rule with its prelude, declarations, and child rules all initially set to empty lists.

		// Process input:

		// <EOF-token>
		// stop token (if passed)
		//   This is a parse error. Return nothing.
		if parser.at_end() {
			Err(diagnostics::UnexpectedEnd())?
		}
		// <}-token>
		//   This is a parse error. If nested is true, return nothing. Otherwise, consume a token and append the result to rule’s prelude.
		if parser.is(State::Nested) {
			if let Some(token) = parser.peek::<Token![RightCurly]>() {
				Err(diagnostics::UnexpectedCloseCurly(token.span()))?;
			}
		}

		// <{-token>
		//	If the first two non-<whitespace-token> values of rule’s prelude are an <ident-token> whose value starts with "--" followed by a <colon-token>, then:
		let checkpoint = parser.checkpoint();
		if let Some(token) = parser.peek::<Token![Ident]>() {
			if token.is_dashed_ident() {
				parser.hop(token);
				if parser.peek::<Delim![:]>().is_some() {
					// If nested is true, consume the remnants of a bad declaration from input, with nested set to true, and return nothing.
					if parser.is(State::Nested) {
						parser.rewind(checkpoint);
						parser.parse::<Self::BadDeclaration>()?;
						let token = parser.peek::<Token![Any]>().unwrap();
						Err(diagnostics::BadDeclaration(Span {
							start: checkpoint.span().start,
							end: token.span().end,
						}))?;
					// If nested is false, consume a block from input, and return nothing.
					} else {
						Self::consume_block(parser);
						let token = parser.peek::<Token![Any]>().unwrap();
						Err(diagnostics::BadDeclaration(Span {
							start: checkpoint.span().start,
							end: token.span().end,
						}))?;
					}
				}
				parser.rewind(checkpoint);
			}
		}

		let mut prelude = Self::parse_prelude(parser);

		// Otherwise, consume a block from input, and let child rules be the result.
		// If the first item of child rules is a list of declarations,
		// remove it from child rules and assign it to rule’s declarations.
		// If any remaining items of child rules are lists of declarations,
		// replace them with nested declarations rules containing the list as its sole child.
		// Assign child rules to rule’s child rules.
		if let Ok(prelude) = prelude {
			Ok((prelude, Self::parse_block(parser)?))
		} else {
			let token = parser.peek::<Token![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-rule-list
pub trait RuleList<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_rule_list(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		parser.parse::<Token![LeftCurly]>()?;
		let mut rules = parser.new_vec();
		loop {
			discard!(parser, Semicolon);
			if discard!(parser, RightCurly) {
				return Ok(rules);
			}
			rules.push(Self::Rule::parse_spanned(parser)?);
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-declaration-rule-list
pub trait DeclarationRuleList<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type AtRule: AtRule<'a> + Parse<'a>;

	fn parse_declaration_rule_list(
		parser: &mut Parser<'a>,
	) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::AtRule>>)> {
		parser.parse::<Token![LeftCurly]>()?;
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			if parser.at_end() || discard!(parser, RightCurly) {
				return Ok((declarations, rules));
			}
			if parser.peek::<Token![AtKeyword]>().is_some() {
				rules.push(parser.parse_spanned::<Self::AtRule>()?);
			} else if parser.peek::<Token![Ident]>().is_some() {
				declarations.push(parser.parse_spanned::<Self::Declaration>()?);
			} else {
				let token = parser.peek::<Token![Any]>().unwrap();
				Err(diagnostics::Unexpected(token, token.span()))?;
			}
		}
	}
}

mod kw {
	use crate::custom_keyword;
	custom_keyword!(And, atom!("and"));
	custom_keyword!(Or, atom!("or"));
	custom_keyword!(Not, atom!("not"));
}

pub trait ConditionalAtRule<'a>: Sized + Parse<'a> {
	type Feature: Sized + Parse<'a>;

	fn create_is(feature: Self::Feature) -> Self;
	fn create_not(features: Self) -> Self;
	fn create_and(features: SmallVec<[Self::Feature; 2]>) -> Self;
	fn create_or(features: SmallVec<[Self::Feature; 2]>) -> Self;

	fn parse_condition(parser: &mut Parser<'a>) -> Result<Self> {
		// handle double parens
		let mut wrapped = true;
		let feature = if let Some(token) = parser.peek::<Token![LeftParen]>() {
			let checkpoint = parser.checkpoint();
			parser.hop(token);
			if parser.peek::<Token![LeftParen]>().is_none() {
				wrapped = false;
				parser.rewind(checkpoint);
			}
			Some(parser.parse::<Self::Feature>()?)
		} else {
			None
		};
		let mut features = smallvec![];
		if let Some(feature) = feature {
			if parser.peek::<Token![Ident]>().is_none() {
				return Ok(Self::create_is(feature));
			}
			features.push(feature);
		}
		if parser.peek::<kw::And>().is_some() {
			loop {
				parser.parse::<kw::And>()?;
				features.push(parser.parse::<Self::Feature>()?);
				if parser.peek::<kw::And>().is_none() {
					if wrapped {
						parser.parse::<Token![RightParen]>()?;
					}
					return Ok(Self::create_and(features));
				}
			}
		} else if parser.peek::<kw::Or>().is_some() {
			loop {
				parser.parse::<kw::Or>()?;
				features.push(parser.parse::<Self::Feature>()?);
				if parser.peek::<kw::Or>().is_none() {
					if wrapped {
						parser.parse::<Token![RightParen]>()?;
					}
					return Ok(Self::create_or(features));
				}
			}
		} else {
			parser.parse::<kw::Not>()?;
			Ok(Self::create_not(parser.parse::<Self>()?))
		}
	}
}
