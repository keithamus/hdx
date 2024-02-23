use hdx_lexer::Token;

use crate::{
	expect,
	parser::Parser,
	span::{Span, Spanned},
	unexpected, Result, State, Vec,
};

pub trait Parse<'a>: Sized {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>>;

	fn spanned(self, span: Span) -> Spanned<Self> {
		Spanned { node: self, span }
	}
}

pub trait Block<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type Rule: Parse<'a>;

	// https://drafts.csswg.org/css-syntax-3/#consume-block-contents
	fn parse_block(parser: &mut Parser<'a>) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::Rule>>)> {
		let span = parser.span();
		expect!(parser, Token::LeftCurly);
		parser.advance();
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			match parser.cur() {
				Token::Semicolon => {
					parser.advance();
				},
				Token::Eof | Token::RightCurly => {
					parser.advance();
					break;
				}
				Token::AtKeyword(_) => {
					parser.set(State::Nested);
					rules.push(Self::Rule::parse(parser)?);
					parser.unset(State::Nested);
				}
				_ => {
					let checkpoint = parser.checkpoint();
					parser.set(State::Nested);
					if let Ok(decl) = Self::Declaration::parse(parser) {
						declarations.push(decl);
						parser.unset(State::Nested);
					} else {
						parser.rewind(checkpoint);
						dbg!("StyleRule::parse(parser)", parser.cur());
						rules.push(Self::Rule::parse(parser)?);
						parser.unset(State::Nested);
					}
				}
			}
		}
		Ok((declarations, rules))
	}
}

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
		// Prelude lands just after the at-keyword token.
		match parser.cur() {
			Token::LeftCurly | Token::Semicolon | Token::Eof => Ok(None),
			_ => Ok(Some(Self::Prelude::parse(parser)?)),
		}
	}

	// AtRules can have an optional block (e.g. @charset, @import must not have
	// one). The default parse_prelude returns an Option, and rules that either
	// require can check in parse() or override parse_prelude() to err.
	fn parse_block(parser: &mut Parser<'a>) -> Result<Option<Spanned<Self::Block>>> {
		match parser.cur() {
			Token::Semicolon | Token::Eof => Ok(None),
			Token::LeftCurly => Ok(Some(Self::Block::parse(parser)?)),
			token => unexpected!(parser, token),
		}
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
	fn parse_at_rule(
		parser: &mut Parser<'a>,
	) -> Result<(Option<Spanned<Self::Prelude>>, Option<Spanned<Self::Block>>)> {
		match parser.cur() {
			Token::AtKeyword(_) => {
				parser.advance();
				let prelude = Self::parse_prelude(parser)?;
				let block = Self::parse_block(parser)?;
				Ok((prelude, block))
			}
			token => unexpected!(parser, token),
		}
	}
}

pub trait QualifiedRule<'a>: Sized + Parse<'a> {
	type Prelude: Parse<'a>;
	type Block: Parse<'a>;

	// QualifiedRules must have a prelude, consequently parse_prelude must be
	// implemented.
	// parse_prelude is called right away, so could start with any token.
	fn parse_prelude(parser: &mut Parser<'a>) -> Result<Spanned<Self::Prelude>> {
		Self::Prelude::parse(parser)
	}

	// QualifiedRules must have a block, consequently parse_prelude must be
	// implemented.
	// parse_block will always start with a {-token.
	fn parse_block(parser: &mut Parser<'a>) -> Result<Spanned<Self::Block>> {
		Self::Block::parse(parser)
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
	fn parse_qualified_rule(parser: &mut Parser<'a>) -> Result<(Spanned<Self::Prelude>, Spanned<Self::Block>)> {
		dbg!("parse_qualified_rule", parser.cur());
		match parser.cur() {
			token @ Token::Eof => unexpected!(parser, token),
			token @ Token::RightCurly if !parser.is(State::Nested) => unexpected!(parser, token),
			Token::Ident(atom) if matches!(parser.peek(), Token::RightCurly) && atom.starts_with("--")	 => {
				unexpected!(parser);
			}
			_ => {}
		}
		let prelude = Self::parse_prelude(parser)?;
		match parser.cur() {
			token @ Token::Eof => unexpected!(parser, token),
			token @ Token::RightCurly if !parser.is(State::Nested) => unexpected!(parser, token),
			Token::Ident(atom) if matches!(parser.peek(), Token::RightCurly) && atom.starts_with("--") => {
				unexpected!(parser);
			}
			_ => {}
		}
		Ok((prelude, Self::parse_block(parser)?))
	}
}

pub trait StyleSheet<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_stylesheet(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		let mut rules: Vec<'a, Spanned<Self::Rule>> = parser.new_vec();
		loop {
			match parser.cur() {
				Token::Eof => {
					return Ok(rules);
				}
				Token::Cdc | Token::Cdo => {
					parser.advance();
				}
				_ => {
					rules.push(Self::Rule::parse(parser)?);
				}
			}
		}
	}
}

pub trait DeclarationRuleList<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type AtRule: AtRule<'a> + Parse<'a>;

	fn parse_declaration_rule_list(
		parser: &mut Parser<'a>,
	) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::AtRule>>)> {
		expect!(parser, Token::LeftCurly);
		parser.advance();
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			match parser.cur() {
				Token::AtKeyword(_) => {
					rules.push(Self::AtRule::parse(parser)?);
				}
				Token::Ident(_) => {
					declarations.push(Self::Declaration::parse(parser)?);
				}
				Token::RightCurly => {
					parser.advance();
					return Ok((declarations, rules));
				}
				token => unexpected!(parser, token),
			}
		}
	}
}
