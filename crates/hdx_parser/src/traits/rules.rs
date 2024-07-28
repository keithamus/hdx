use hdx_lexer::Kind;

use crate::{discard, expect, parser::Parser, peek, span::Spanned, unexpected, Result, State, Vec};

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
		match parser.peek().kind() {
			Kind::LeftCurly | Kind::Semicolon | Kind::Eof => Ok(None),
			_ => Ok(Some(Self::Prelude::parse_spanned(parser)?)),
		}
	}

	// AtRules can have an optional block (e.g. @charset, @import must not have
	// one). The default parse_prelude returns an Option, and rules that either
	// require can check in parse() or override parse_prelude() to err.
	fn parse_block(parser: &mut Parser<'a>) -> Result<Option<Spanned<Self::Block>>> {
		let token = parser.peek();
		match token.kind() {
			Kind::Semicolon | Kind::Eof => {
				parser.advance();
				Ok(None)
			}
			Kind::LeftCurly => Ok(Some(Self::Block::parse_spanned(parser)?)),
			_ => unexpected!(parser, token),
		}
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
	fn parse_at_rule(
		parser: &mut Parser<'a>,
	) -> Result<(Option<Spanned<Self::Prelude>>, Option<Spanned<Self::Block>>)> {
		expect!(parser.cur(), Kind::AtKeyword);
		let prelude = Self::parse_prelude(parser)?;
		let block = Self::parse_block(parser)?;
		Ok((prelude, block))
	}
}

pub trait QualifiedRule<'a>: Sized + Parse<'a> {
	type Prelude: Parse<'a>;
	type Block: Parse<'a>;

	// QualifiedRules must have a prelude, consequently parse_prelude must be
	// implemented.
	// parse_prelude is called right away, so could start with any token.
	fn parse_prelude(parser: &mut Parser<'a>) -> Result<Spanned<Self::Prelude>> {
		Self::Prelude::parse_spanned(parser)
	}

	// QualifiedRules must have a block, consequently parse_prelude must be
	// implemented.
	// parse_block will always start with a {-token.
	fn parse_block(parser: &mut Parser<'a>) -> Result<Spanned<Self::Block>> {
		Self::Block::parse_spanned(parser)
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
	fn parse_qualified_rule(parser: &mut Parser<'a>) -> Result<(Spanned<Self::Prelude>, Spanned<Self::Block>)> {
		let token = parser.peek().clone();
		match token.kind() {
			Kind::Eof => unexpected!(parser, token),
			Kind::RightCurly if !parser.is(State::Nested) => unexpected!(parser, token),
			Kind::Ident if peek!(parser, 2, Kind::RightCurly) && token.is_dashed_ident() => {
				unexpected!(parser);
			}
			_ => {}
		}
		let prelude = Self::parse_prelude(parser)?;
		let token = parser.peek().clone();
		match token.kind() {
			Kind::Eof => unexpected!(parser, token),
			Kind::RightCurly if !parser.is(State::Nested) => unexpected!(parser, token),
			Kind::Ident if peek!(parser, 2, Kind::RightCurly) && token.is_dashed_ident() => {
				unexpected!(parser);
			}
			_ => {}
		}
		Ok((prelude, Self::parse_block(parser)?))
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-rule-list
pub trait RuleList<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_rule_list(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		expect!(parser.next(), Kind::LeftCurly);
		let mut rules = parser.new_vec();
		loop {
			discard!(parser, Kind::Semicolon);
			if discard!(parser, Kind::RightCurly) {
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
		expect!(parser.next(), Kind::LeftCurly);
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			let token = parser.peek();
			match token.kind() {
				Kind::AtKeyword => {
					rules.push(Self::AtRule::parse_spanned(parser)?);
				}
				Kind::Ident => {
					declarations.push(Self::Declaration::parse_spanned(parser)?);
				}
				Kind::RightCurly => {
					parser.advance();
					return Ok((declarations, rules));
				}
				_ => unexpected!(parser, token),
			}
		}
	}
}
