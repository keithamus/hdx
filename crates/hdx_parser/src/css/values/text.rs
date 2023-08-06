use hdx_ast::css::values::{
	Expr, Shorthand, TextWrapValue, WhiteSpaceCollapseValue, WhiteSpaceShorthand,
	WhiteSpaceTrimValue,
};
use hdx_lexer::Kind;

use crate::{atom, diagnostics, Atomizable, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for WhiteSpaceTrimValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut inner = false;
		let mut after = false;
		let mut before = false;
		loop {
			match parser.cur().kind {
				Kind::Ident => match parser.cur_atom_lower().unwrap() {
					atom!("none") => {
						parser.advance();
						return Ok(Self::None.spanned(span));
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
		Ok(Self::Discard { inner, after, before }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> Parse<'a> for WhiteSpaceShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		if parser.at(Kind::Ident) {
			match parser.cur_atom_lower().unwrap() {
				//normal | pre | nowrap | pre-wrap | pre-line
				atom!("normal") => {
					parser.advance();
					return Ok(Self::Normal.spanned(span));
				}
				atom!("pre") => {
					parser.advance();
					return Ok(Self::Pre.spanned(span));
				}
				atom!("nowrap") => {
					parser.advance();
					return Ok(Self::Nowrap.spanned(span));
				}
				atom!("pre-wrap") => {
					parser.advance();
					return Ok(Self::PreWrap.spanned(span));
				}
				atom!("pre-line") => {
					parser.advance();
					return Ok(Self::PreLine.spanned(span));
				}
				_ => {}
			}
		}
		let mut collapse = Shorthand::Implicit;
		let mut wrap = Shorthand::Implicit;
		let mut trim = Shorthand::Implicit;
		loop {
			match parser.cur().kind {
				Kind::Semicolon | Kind::Comma | Kind::Eof => {
					break;
				}
				Kind::Ident => {
					let ident = parser.cur_atom_lower().unwrap();
					if collapse.is_implicit()
						&& WhiteSpaceCollapseValue::from_atom(ident.clone()).is_some()
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
							ident,
							atom!("none")
								| atom!("discard-inner") | atom!("discard-after")
								| atom!("discard-before")
						) {
						let node = Expr::<WhiteSpaceTrimValue>::parse(parser)?;
						trim = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::UnexpectedIdent(ident.clone(), parser.cur().span))?
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
					Err(diagnostics::Unexpected(k, parser.cur().span))?
				}
			}
			if collapse.is_explicit() && wrap.is_explicit() && trim.is_explicit() {
				break;
			}
		}
		Ok(Self::Expanded { collapse, wrap, trim }.spanned(span.until(parser.cur().span)))
	}
}
