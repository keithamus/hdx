use hdx_atom::Atom;
use hdx_lexer::{Kind, Span, Spanned};
use smallvec::{smallvec, SmallVec};

use crate::{diagnostics, parser::Parser, Result, State, Vec, T};

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
	fn parse_prelude(p: &mut Parser<'a>) -> Result<Option<Spanned<Self::Prelude>>> {
		let next_token_kind = p.peek::<T![Any]>().map(|t| t.kind()).unwrap_or(Kind::Eof);
		if p.at_end() || matches!(next_token_kind, Kind::LeftCurly | Kind::Semicolon) {
			return Ok(None);
		}
		Ok(Some(Self::Prelude::parse_spanned(p)?))
	}

	// AtRules can have an optional block (e.g. @charset, @import must not have
	// one). The default parse_prelude returns an Option, and rules that either
	// require can check in parse() or override parse_prelude() to err.
	fn parse_block(p: &mut Parser<'a>) -> Result<Option<Spanned<Self::Block>>> {
		let token = p.peek_next();
		match token.kind() {
			Kind::Semicolon | Kind::Eof => {
				p.next();
				Ok(None)
			}
			Kind::LeftCurly => Ok(Some(Self::Block::parse_spanned(p)?)),
			_ => Err(diagnostics::Unexpected(token, token.span()))?,
		}
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
	fn parse_at_rule(
		p: &mut Parser<'a>,
		name: Option<Atom>,
	) -> Result<(Option<Spanned<Self::Prelude>>, Option<Spanned<Self::Block>>)> {
		let token = *p.parse::<T![AtKeyword]>()?;
		let span = token.span();
		if let Some(name) = name {
			let atom = p.parse_atom_lower(token);
			if atom != name {
				Err(diagnostics::UnexpectedAtRule(atom, span))?;
			}
		}
		let prelude = Self::parse_prelude(p)?;
		let block = Self::parse_block(p)?;
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
	fn parse_prelude(p: &mut Parser<'a>) -> Result<Spanned<Self::Prelude>> {
		p.parse_spanned::<Self::Prelude>()
	}

	// QualifiedRules must have a block, consequently parse_prelude must be
	// implemented.
	// parse_block will always start with a {-token.
	fn parse_block(p: &mut Parser<'a>) -> Result<Spanned<Self::Block>> {
		p.parse_spanned::<Self::Block>()
	}

	// QualifiedRules must be able to consume a block from their input when encountering
	// a custom property like declaration that doesn't end but opens a `{` block. This
	// is implemented as parsing the existing block as that' simplifies downstream logic
	// but consumers of this trait can instead opt to implement an optimised version of
	// this which doesn't build up an AST and just throws away tokens.
	fn consume_block(p: &mut Parser<'a>) {
		p.parse::<Self::Block>().ok();
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
	fn parse_qualified_rule(p: &mut Parser<'a>) -> Result<(Spanned<Self::Prelude>, Spanned<Self::Block>)> {
		// Let rule be a new qualified rule with its prelude, declarations, and child rules all initially set to empty lists.

		// Process input:

		// <EOF-token>
		// stop token (if passed)
		//   This is a parse error. Return nothing.
		if p.at_end() {
			Err(diagnostics::UnexpectedEnd())?
		}
		// <}-token>
		//   This is a parse error. If nested is true, return nothing. Otherwise, consume a token and append the result to rule’s prelude.
		if p.is(State::Nested) {
			if let Some(token) = p.peek::<T![RightCurly]>() {
				Err(diagnostics::UnexpectedCloseCurly(token.span()))?;
			}
		}

		// <{-token>
		//	If the first two non-<whitespace-token> values of rule’s prelude are an <ident-token> whose value starts with "--" followed by a <colon-token>, then:
		let checkpoint = p.checkpoint();
		if let Some(token) = p.peek::<T![Ident]>() {
			if token.is_dashed_ident() {
				p.hop(token);
				if p.peek::<T![:]>().is_some() {
					// If nested is true, consume the remnants of a bad declaration from input, with nested set to true, and return nothing.
					if p.is(State::Nested) {
						p.rewind(checkpoint);
						p.parse::<Self::BadDeclaration>()?;
						let token = p.peek::<T![Any]>().unwrap();
						Err(diagnostics::BadDeclaration(Span {
							start: checkpoint.span().start,
							end: token.span().end,
						}))?;
					// If nested is false, consume a block from input, and return nothing.
					} else {
						Self::consume_block(p);
						let token = p.peek::<T![Any]>().unwrap();
						Err(diagnostics::BadDeclaration(Span {
							start: checkpoint.span().start,
							end: token.span().end,
						}))?;
					}
				}
				p.rewind(checkpoint);
			}
		}

		let mut prelude = Self::parse_prelude(p);

		// Otherwise, consume a block from input, and let child rules be the result.
		// If the first item of child rules is a list of declarations,
		// remove it from child rules and assign it to rule’s declarations.
		// If any remaining items of child rules are lists of declarations,
		// replace them with nested declarations rules containing the list as its sole child.
		// Assign child rules to rule’s child rules.
		if let Ok(prelude) = prelude {
			Ok((prelude, Self::parse_block(p)?))
		} else {
			let token = p.peek::<T![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-rule-list
pub trait RuleList<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_rule_list(p: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		p.parse::<T![LeftCurly]>()?;
		let mut rules = p.new_vec();
		loop {
			p.parse::<T![;]>().ok();
			if p.parse::<T![RightCurly]>().is_ok() {
				return Ok(rules);
			}
			rules.push(Self::Rule::parse_spanned(p)?);
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-declaration-rule-list
pub trait DeclarationRuleList<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type AtRule: AtRule<'a> + Parse<'a>;

	fn parse_declaration_rule_list(
		p: &mut Parser<'a>,
	) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::AtRule>>)> {
		p.parse::<T![LeftCurly]>()?;
		let mut declarations = p.new_vec();
		let mut rules = p.new_vec();
		loop {
			if p.at_end() || p.parse::<T![RightCurly]>().is_ok() {
				return Ok((declarations, rules));
			}
			if p.peek::<T![AtKeyword]>().is_some() {
				rules.push(p.parse_spanned::<Self::AtRule>()?);
			} else if p.peek::<T![Ident]>().is_some() {
				declarations.push(p.parse_spanned::<Self::Declaration>()?);
			} else {
				let token = p.peek::<T![Any]>().unwrap();
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

	fn parse_condition(p: &mut Parser<'a>) -> Result<Self> {
		// handle double parens
		let mut wrapped = true;
		let feature = if let Some(token) = p.peek::<T![LeftParen]>() {
			let checkpoint = p.checkpoint();
			p.hop(token);
			if p.peek::<T![LeftParen]>().is_none() {
				wrapped = false;
				p.rewind(checkpoint);
			}
			Some(p.parse::<Self::Feature>()?)
		} else {
			None
		};
		let mut features = smallvec![];
		if let Some(feature) = feature {
			if p.peek::<T![Ident]>().is_none() {
				return Ok(Self::create_is(feature));
			}
			features.push(feature);
		}
		if p.peek::<kw::And>().is_some() {
			loop {
				p.parse::<kw::And>()?;
				features.push(p.parse::<Self::Feature>()?);
				if p.peek::<kw::And>().is_none() {
					if wrapped {
						p.parse::<T![RightParen]>()?;
					}
					return Ok(Self::create_and(features));
				}
			}
		} else if p.peek::<kw::Or>().is_some() {
			loop {
				p.parse::<kw::Or>()?;
				features.push(p.parse::<Self::Feature>()?);
				if p.peek::<kw::Or>().is_none() {
					if wrapped {
						p.parse::<T![RightParen]>()?;
					}
					return Ok(Self::create_or(features));
				}
			}
		} else {
			p.parse::<kw::Not>()?;
			Ok(Self::create_not(p.parse::<Self>()?))
		}
	}
}
