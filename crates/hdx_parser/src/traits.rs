use hdx_atom::{atom, Atom};
use hdx_lexer::Token;

use crate::{expect, parser::Parser, span::Spanned, unexpected, unexpected_ident, Result, State, Vec, discard, peek, expect_ignore_case};

// The FromToken trait produces a result of Self from an individual parser Token, guaranteeing that the parser will not
// roll forward. Instead, the caller should advance the parser.
pub trait FromToken: Sized {
	fn from_token(token: Token) -> Option<Self>;
}

impl<'a, T: FromToken> Parse<'a> for T {
	fn parse(parser: &mut Parser<'a>) -> Result<Self> {
		if let Some(result) = Self::from_token(parser.cur()) {
			parser.advance();
			Ok(result)
		} else {
			unexpected!(parser)
		}
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for Vec<'a, T> {
	fn parse(parser: &mut Parser<'a>) -> Result<Vec<'a, T>> {
		loop {
			let mut vec = parser.new_vec();
			if let Ok(t) = T::parse(parser) {
				vec.push(t);
			} else {
				return Ok(vec);
			}
		}
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for Spanned<T> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<T>> {
		T::parse_spanned(parser)
	}
}

pub trait Parse<'a>: Sized {
	fn parse(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_spanned(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let node = Self::parse(parser)?;
		Ok(Spanned { node, span: span.end(parser.prev_pos) })
	}
}

pub trait Block<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type Rule: Parse<'a>;

	// https://drafts.csswg.org/css-syntax-3/#consume-block-contents
	fn parse_block(
		parser: &mut Parser<'a>,
	) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::Rule>>)> {
		expect!(parser, Token::LeftCurly);
		parser.advance();
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			match parser.cur() {
				Token::Semicolon => {
					parser.advance();
				}
				Token::Eof | Token::RightCurly => {
					parser.advance();
					break;
				}
				Token::AtKeyword(_) => {
					parser.set(State::Nested);
					rules.push(Self::Rule::parse_spanned(parser)?);
					parser.unset(State::Nested);
				}
				_ => {
					let checkpoint = parser.checkpoint();
					parser.set(State::Nested);
					if let Ok(decl) = Self::Declaration::parse_spanned(parser) {
						declarations.push(decl);
						parser.unset(State::Nested);
					} else {
						parser.rewind(checkpoint);
						rules.push(Self::Rule::parse_spanned(parser)?);
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
			_ => Ok(Some(Self::Prelude::parse_spanned(parser)?)),
		}
	}

	// AtRules can have an optional block (e.g. @charset, @import must not have
	// one). The default parse_prelude returns an Option, and rules that either
	// require can check in parse() or override parse_prelude() to err.
	fn parse_block(parser: &mut Parser<'a>) -> Result<Option<Spanned<Self::Block>>> {
		match parser.cur() {
			Token::Semicolon | Token::Eof => Ok(None),
			Token::LeftCurly => Ok(Some(Self::Block::parse_spanned(parser)?)),
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

// An AtRule represents a block or statement with an @keyword in the leading
// position, such as @media, @supports
pub trait RuleGroup<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_rules(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		match parser.cur() {
			Token::LeftCurly => {
				parser.advance();
				let mut rules = parser.new_vec();
				loop {
					discard!(parser, Token::Semicolon);
					if matches!(parser.cur(), Token::RightCurly) {
						parser.advance();
						return Ok(rules);
					}
					rules.push(Self::Rule::parse_spanned(parser)?);
				}
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
		match parser.cur() {
			token @ Token::Eof => unexpected!(parser, token),
			token @ Token::RightCurly if !parser.is(State::Nested) => unexpected!(parser, token),
			Token::Ident(atom) if peek!(parser, Token::RightCurly) && atom.starts_with("--") => {
				unexpected!(parser);
			}
			_ => {}
		}
		let prelude = Self::parse_prelude(parser)?;
		match parser.cur() {
			token @ Token::Eof => unexpected!(parser, token),
			token @ Token::RightCurly if !parser.is(State::Nested) => unexpected!(parser, token),
			Token::Ident(atom) if peek!(parser, Token::RightCurly) && atom.starts_with("--") => {
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
					rules.push(Self::Rule::parse_spanned(parser)?);
				}
			}
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#typedef-rule-list
pub trait RuleList<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_rule_list(
		parser: &mut Parser<'a>,
	) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		expect!(parser, Token::LeftCurly);
		parser.advance();
		let mut rules = parser.new_vec();
		loop {
			match parser.cur() {
				Token::RightCurly => {
					parser.advance();
					return Ok(rules);
				}
				_ => {
					rules.push(Self::Rule::parse_spanned(parser)?);
				}
			}
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
		expect!(parser, Token::LeftCurly);
		parser.advance();
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			match parser.cur() {
				Token::AtKeyword(_) => {
					rules.push(Self::AtRule::parse_spanned(parser)?);
				}
				Token::Ident(_) => {
					declarations.push(Self::Declaration::parse_spanned(parser)?);
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

pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn parse_name(parser: &mut Parser<'a>) -> Result<Atom> {
		match parser.cur() {
			Token::Ident(atom) => {
				parser.advance();
				expect!(parser, Token::Colon);
				parser.advance();
				Ok(atom.to_ascii_lowercase())
			}
			token => unexpected!(parser, token),
		}
	}

	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> Result<Self::DeclarationValue>;

	fn parse_important(parser: &mut Parser<'a>) -> Result<bool> {
		if matches!(parser.cur(), Token::Delim('!')) && peek!(parser, Token::Ident(_)) {
			parser.advance_including_whitespace_and_comments();
			match parser.cur() {
				Token::Ident(ident) => match ident.to_ascii_lowercase() {
					atom!("important") => {}
					_ => unexpected_ident!(parser, ident),
				},
				token => unexpected!(parser, token),
			}
			parser.advance();
			Ok(true)
		} else {
			Ok(false)
		}
	}

	fn parse_declaration(parser: &mut Parser<'a>) -> Result<(Atom, Self::DeclarationValue, bool)> {
		let name = Self::parse_name(parser)?;
		let value = Self::parse_declaration_value(&name, parser)?;
		let important = Self::parse_important(parser)?;
		discard!(parser, Token::Semicolon);
		Ok((name, value, important))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(name: &Atom, parser: &mut Parser<'a>) -> Result<Self>;
}

pub trait MediaFeature<'a>: Sized + Default {
	fn parse_media_feature_value(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_media_feature(name: Atom, parser: &mut Parser<'a>) -> Result<Self> {
		expect!(parser, Token::LeftParen);
		parser.advance();
		expect_ignore_case!(parser, name);
		parser.advance();
		let value = match parser.cur() {
			Token::RightParen => Self::default(),
			Token::Colon => {
				parser.advance();
				Self::parse_media_feature_value(parser)?
			}
			token => unexpected!(parser, token),
		};
		expect!(parser, Token::RightParen);
		parser.advance();
		Ok(value)
	}
}
