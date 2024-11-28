use hdx_atom::Atom;
use hdx_lexer::{Cursor, Kind, KindSet};

use crate::{diagnostics, parser::Parser, Result, State, Vec, T};

use super::{Declaration, Parse};

pub struct NoPreludeAllowed;
impl<'a> Parse<'a> for NoPreludeAllowed {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<T![LeftCurly]>() || p.peek::<T![;]>() {
			Ok(Self {})
		} else {
			let c = p.peek_next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

pub struct NoBlockAllowed;
impl<'a> Parse<'a> for NoBlockAllowed {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.at_end() || p.peek::<T![;]>() {
			Ok(Self {})
		} else {
			let c = p.peek_next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

// An AtRule represents a block or statement with an @keyword in the leading
// position, such as @media, @charset, @import and so-on.
pub trait AtRule<'a>: Sized + Parse<'a> {
	type Prelude: Parse<'a>;
	// Ideally Block would either implement QualifiedRuleList/DeclarationList/RuleList/DeclarationRuleList; but there is
	// no way to enforce that with Rust so it just has to implement Parse.
	type Block: Parse<'a>;

	// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
	fn parse_at_rule(
		p: &mut Parser<'a>,
		name: Option<Atom>,
	) -> Result<(T![AtKeyword], Option<Self::Prelude>, Self::Block)> {
		let at = p.parse::<T![AtKeyword]>()?;
		if let Some(name) = name {
			let atom = p.parse_atom_lower(at.into());
			let cursor: Cursor = at.into();
			if atom != name {
				Err(diagnostics::UnexpectedAtRule(atom, cursor.into()))?;
			}
		}
		// AtRules can have an optional prelude (e.g. @supoports requires one,
		// @starting-style must not have one, and in @page it is optional). Consequently
		// parse_prelude returns an Option, and rules that either require can check
		// in parse() or override parse_prelude() to err.
		let c = p.peek_next();
		let prelude =
			if p.at_end() || c == KindSet::LEFT_CURLY_OR_SEMICOLON { None } else { Some(p.parse::<Self::Prelude>()?) };

		Ok((at, prelude, p.parse::<Self::Block>()?))
	}
}

// A QualifiedRule represents a block with a prelude which may contain other rules.
// Examples of QualifiedRules are StyleRule, KeyframeRule (no s!).
pub trait QualifiedRule<'a>: Sized + Parse<'a> {
	// Prelude MAY implement PreludeList if it accepts multiple values.
	type Prelude: Parse<'a>;
	// Ideally Block would either implement QualifiedRuleList/DeclarationList/RuleList/DeclarationRuleList; but there is
	// no way to enforce that with Rust so it just has to implement Parse.
	type Block: Parse<'a>;

	// QualifiedRules must be able to consume a bad declaration, for when
	// a custom property like declaration is encountered.
	type BadDeclaration: Parse<'a>;

	// QualifiedRules must be able to consume a block from their input when encountering
	// a custom property like declaration that doesn't end but opens a `{` block. This
	// is implemented as parsing the existing block as that' simplifies downstream logic
	// but consumers of this trait can instead opt to implement an optimised version of
	// this which doesn't build up an AST and just throws away tokens.
	fn consume_block(p: &mut Parser<'a>) {
		p.parse::<Self::Block>().ok();
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
	fn parse_qualified_rule(p: &mut Parser<'a>) -> Result<(Self::Prelude, Self::Block)> {
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
		if p.is(State::Nested) && p.peek::<T!['}']>() {
			Err(diagnostics::UnexpectedCloseCurly(p.peek_n(1).into()))?;
		}

		// <{-token>
		//	If the first two non-<whitespace-token> values of rule’s prelude are an <ident-token> whose value starts with "--" followed by a <colon-token>, then:
		let checkpoint = p.checkpoint();
		if p.peek::<T![DashedIdent]>() {
			p.parse::<T![DashedIdent]>().ok();
			if p.peek::<T![:]>() {
				// If nested is true, consume the remnants of a bad declaration from input, with nested set to true, and return nothing.
				if p.is(State::Nested) {
					p.rewind(checkpoint);
					p.parse::<Self::BadDeclaration>()?;
					Err(diagnostics::BadDeclaration(checkpoint.span()))?
				// If nested is false, consume a block from input, and return nothing.
				} else {
					Self::consume_block(p);
					Err(diagnostics::BadDeclaration(checkpoint.span()))?
				}
			}
			p.rewind(checkpoint);
		}

		// Set the StopOn Curly to signify to prelude parsers that they shouldn't consume beyond the curly
		let old_stop = p.stop;
		p.set_stop(old_stop.add(Kind::LeftCurly).add(Kind::RightCurly));
		let prelude = p.parse::<Self::Prelude>();
		p.set_stop(old_stop);

		// Otherwise, consume a block from input, and let child rules be the result.
		// If the first item of child rules is a list of declarations,
		// remove it from child rules and assign it to rule’s declarations.
		// If any remaining items of child rules are lists of declarations,
		// replace them with nested declarations rules containing the list as its sole child.
		// Assign child rules to rule’s child rules.
		if let Ok(prelude) = prelude {
			Ok((prelude, p.parse::<Self::Block>()?))
		} else {
			Err(diagnostics::Unexpected(checkpoint.into(), checkpoint.span()))?
		}
	}
}

pub trait PreludeList<'a>: Sized + Parse<'a> {
	type PreludeItem: Parse<'a>;

	fn parse_prelude_list(p: &mut Parser<'a>) -> Result<Vec<'a, Self::PreludeItem>> {
		let mut items = Vec::new_in(p.bump());
		loop {
			items.push(p.parse::<Self::PreludeItem>()?);
			if p.peek_next() == KindSet::LEFT_CURLY_OR_SEMICOLON {
				return Ok(items);
			}
		}
	}
}

pub trait PreludeCommaList<'a>: Sized + Parse<'a> {
	type PreludeItem: Parse<'a>;

	fn parse_prelude_list(p: &mut Parser<'a>) -> Result<Vec<'a, (Self::PreludeItem, Option<T![,]>)>> {
		let mut items = Vec::new_in(p.bump());
		loop {
			let item = p.parse::<Self::PreludeItem>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			items.push((item, comma));
			if p.peek_next() == KindSet::LEFT_CURLY_OR_SEMICOLON {
				return Ok(items);
			}
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-declaration-list
// <declaration-list>: only declarations are allowed; at-rules and qualified rules are automatically invalid.
pub trait DeclarationList<'a>: Sized + Parse<'a> {
	type Declaration: Declaration<'a>;

	fn parse_declaration_list(p: &mut Parser<'a>) -> Result<(T!['{'], Vec<'a, Self::Declaration>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.bump());
		loop {
			p.parse_if_peek::<T![;]>().ok();
			if p.at_end() {
				return Ok((left, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, rules, Some(p.parse::<T!['}']>()?)));
			}
			rules.push(p.parse::<Self::Declaration>()?);
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-qualified-rule-list
// <qualified-rule-list>: only qualified rules are allowed; declarations and at-rules are automatically invalid.
pub trait QualifiedRuleList<'a>: Sized + Parse<'a> {
	type QualifiedRule: QualifiedRule<'a>;

	fn parse_qualified_rule_list(
		p: &mut Parser<'a>,
	) -> Result<(T!['{'], Vec<'a, Self::QualifiedRule>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.bump());
		loop {
			p.parse_if_peek::<T![;]>().ok();
			if p.at_end() {
				return Ok((left, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, rules, Some(p.parse::<T!['}']>()?)));
			}
			rules.push(p.parse::<Self::QualifiedRule>()?);
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-at-rule-list
// <at-rule-list>: only at-rules are allowed; declarations and qualified rules are automatically invalid.
pub trait AtRuleList<'a>: Sized + Parse<'a> {
	type AtRule: AtRule<'a>;

	fn parse_at_rule_list(p: &mut Parser<'a>) -> Result<(T!['{'], Vec<'a, Self::AtRule>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.bump());
		loop {
			p.parse_if_peek::<T![;]>().ok();
			if p.at_end() {
				return Ok((left, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, rules, Some(p.parse::<T!['}']>()?)));
			}
			rules.push(p.parse::<Self::AtRule>()?);
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-declaration-rule-list
// <declaration-rule-list>: declarations and at-rules are allowed; qualified rules are automatically invalid.
pub trait DeclarationRuleList<'a>: Sized + Parse<'a> {
	type Declaration: Declaration<'a>;
	type AtRule: AtRule<'a>;

	fn parse_declaration_rule_list(
		p: &mut Parser<'a>,
	) -> Result<(T!['{'], Vec<'a, Self::Declaration>, Vec<'a, Self::AtRule>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut rules = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				return Ok((left, declarations, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, declarations, rules, Some(p.parse::<T!['}']>()?)));
			}
			if p.peek::<T![AtKeyword]>() {
				rules.push(p.parse::<Self::AtRule>()?);
			} else if p.peek::<T![Ident]>() {
				declarations.push(p.parse::<Self::Declaration>()?);
			} else {
				let c = p.peek_n(1);
				Err(diagnostics::Unexpected(c.into(), c.into()))?;
			}
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-rule-list
// <rule-list>: qualified rules and at-rules are allowed; declarations are automatically invalid.
pub trait RuleList<'a>: Sized + Parse<'a> {
	// To simplify typings and orderings, a generic "Rule" type houses both At/Qualified rules
	type Rule: Parse<'a>;

	fn parse_rule_list(p: &mut Parser<'a>) -> Result<(T!['{'], Vec<'a, Self::Rule>, Option<T!['}']>)> {
		let left = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.bump());
		loop {
			p.parse_if_peek::<T![;]>().ok();
			if p.at_end() {
				return Ok((left, rules, None));
			}
			if p.peek::<T!['}']>() {
				return Ok((left, rules, Some(p.parse::<T!['}']>()?)));
			}
			rules.push(p.parse::<Self::Rule>()?);
		}
	}
}

mod kw {
	use crate::custom_keyword;
	custom_keyword!(And, atom!("and"));
	custom_keyword!(Or, atom!("or"));
	custom_keyword!(Not, atom!("not"));
}

pub trait ConditionalAtRule<'a>: Sized + Parse<'a>
where
	Self: 'a,
{
	type Feature: Sized + Parse<'a>;

	fn new_is(feature: Self::Feature) -> Self;
	fn new_not(features: Self) -> Self;
	fn new_and(features: Vec<'a, Self::Feature>) -> Self;
	fn new_or(features: Vec<'a, Self::Feature>) -> Self;

	fn parse_condition(p: &mut Parser<'a>) -> Result<Self> {
		// handle double parens
		let mut wrapped = true;
		let feature = if p.peek::<T!['(']>() {
			let checkpoint = p.checkpoint();
			p.parse::<T!['(']>()?;
			if !p.peek::<T!['(']>() {
				wrapped = false;
				p.rewind(checkpoint);
			}
			Some(p.parse::<Self::Feature>()?)
		} else {
			None
		};
		let mut features = Vec::new_in(p.bump());
		if let Some(feature) = feature {
			if !p.peek::<T![Ident]>() {
				return Ok(Self::new_is(feature));
			}
			features.push(feature);
		}
		if p.peek::<kw::And>() {
			loop {
				p.parse::<kw::And>()?;
				features.push(p.parse::<Self::Feature>()?);
				if !p.peek::<kw::And>() {
					if wrapped {
						p.parse::<T![')']>()?;
					}
					return Ok(Self::new_and(features));
				}
			}
		} else if p.peek::<kw::Or>() {
			loop {
				p.parse::<kw::Or>()?;
				features.push(p.parse::<Self::Feature>()?);
				if !p.peek::<kw::Or>() {
					if wrapped {
						p.parse::<T![')']>()?;
					}
					return Ok(Self::new_or(features));
				}
			}
		} else {
			p.parse::<kw::Not>()?;
			Ok(Self::new_not(p.parse::<Self>()?))
		}
	}
}
