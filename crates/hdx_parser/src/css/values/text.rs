use hdx_ast::css::values::{
	Expr, Shorthand, TextWrapValue, WhiteSpaceCollapseValue, WhiteSpaceShorthand,
	WhiteSpaceTrimValue,
};

use crate::{atom, diagnostics, Atomizable, Parse, Parser, Result, Spanned, Token};

impl<'a> Parse<'a> for WhiteSpaceTrimValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut inner = false;
		let mut after = false;
		let mut before = false;
		loop {
			match parser.cur() {
				Token::Ident(ident) => match ident.to_ascii_lowercase() {
					atom!("none") => {
						return Ok(Self::None.spanned(parser.advance()));
					}
					atom!("discard-inner") => {
						parser.advance();
						inner = true;
					}
					atom!("discard-after") => {
						parser.advance();
						after = true;
					}
					atom!("discard-before") => {
						parser.advance();
						before = true;
					}
					_ => break,
				},
				_ => break,
			}
			if inner && after && before {
				break;
			}
		}
		Ok(Self::Discard { inner, after, before }.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for WhiteSpaceShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				//normal | pre | nowrap | pre-wrap | pre-line
				atom!("normal") => {
					return Ok(Self::Normal.spanned(parser.advance()));
				}
				atom!("pre") => {
					return Ok(Self::Pre.spanned(parser.advance()));
				}
				atom!("nowrap") => {
					return Ok(Self::Nowrap.spanned(parser.advance()));
				}
				atom!("pre-wrap") => {
					return Ok(Self::PreWrap.spanned(parser.advance()));
				}
				atom!("pre-line") => {
					return Ok(Self::PreLine.spanned(parser.advance()));
				}
				_ => {}
			},
			_ => {}
		}
		let mut collapse = Shorthand::Implicit;
		let mut wrap = Shorthand::Implicit;
		let mut trim = Shorthand::Implicit;
		loop {
			match parser.cur() {
				Token::Semicolon | Token::Comma | Token::Eof => {
					break;
				}
				Token::Ident(ident) => {
					if collapse.is_implicit()
						&& WhiteSpaceCollapseValue::from_atom(ident.to_ascii_lowercase()).is_some()
					{
						let node = Expr::<WhiteSpaceCollapseValue>::parse(parser)?;
						collapse = Shorthand::Explicit(parser.boxup(node));
					} else if wrap.is_implicit()
						&& TextWrapValue::from_atom(ident.clone()).is_some()
					{
						let node = Expr::<TextWrapValue>::parse(parser)?;
						wrap = Shorthand::Explicit(parser.boxup(node));
					} else if trim.is_implicit()
						&& matches!(
							ident.to_ascii_lowercase(),
							atom!("none")
								| atom!("discard-inner") | atom!("discard-after")
								| atom!("discard-before")
						) {
						let node = Expr::<WhiteSpaceTrimValue>::parse(parser)?;
						trim = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::UnexpectedIdent(ident.clone(), parser.span()))?
					}
				}
				k => {
					let checkpoint = parser.checkpoint();
					if collapse.is_implicit() {
						let node = Expr::<WhiteSpaceCollapseValue>::parse(parser);
						match node {
							Ok(node) => {
								collapse = Shorthand::Explicit(parser.boxup(node));
								continue;
							}
							Err(_) => parser.rewind(checkpoint),
						}
					}
					let checkpoint = parser.checkpoint();
					if wrap.is_implicit() {
						let node = Expr::<TextWrapValue>::parse(parser);
						match node {
							Ok(node) => {
								wrap = Shorthand::Explicit(parser.boxup(node));
								continue;
							}
							Err(_) => parser.rewind(checkpoint),
						}
					}
					let checkpoint = parser.checkpoint();
					if trim.is_implicit() {
						let node = Expr::<WhiteSpaceTrimValue>::parse(parser);
						match node {
							Ok(node) => {
								trim = Shorthand::Explicit(parser.boxup(node));
								continue;
							}
							Err(_) => parser.rewind(checkpoint),
						}
					}
					Err(diagnostics::Unexpected(*k, parser.span()))?
				}
			}
			if collapse.is_explicit() && wrap.is_explicit() && trim.is_explicit() {
				break;
			}
		}
		Ok(Self::Expanded { collapse, wrap, trim }.spanned(span.end(parser.pos())))
	}
}
