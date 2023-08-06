use hdx_ast::css::values::{
	CounterStyle, Expr, Image, ListStyleImageValue, ListStylePositionValue, ListStyleShorthand,
	ListStyleTypeValue, Shorthand,
};

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for ListStyleShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let mut position = Shorthand::Implicit;
		let mut image = Shorthand::Implicit;
		let mut marker = Shorthand::Implicit;
		loop {
			match parser.cur().kind {
				Kind::Semicolon | Kind::Comma | Kind::Eof => {
					break;
				}
				Kind::Ident => {
					let ident = parser.cur_atom().unwrap();
					if position.is_implicit() && matches!(ident, atom!("inside") | atom!("outside"))
					{
						let node = Expr::<ListStylePositionValue>::parse(parser)?;
						position = Shorthand::Explicit(parser.boxup(node));
					} else if image.is_implicit() && matches!(ident, atom!("none")) {
						let node = Expr::<ListStyleImageValue>::parse(parser)?;
						image = Shorthand::Explicit(parser.boxup(node));
					} else if marker.is_implicit() {
						let node = Expr::<ListStyleTypeValue>::parse(parser)?;
						marker = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::UnexpectedIdent(ident.clone(), parser.cur().span))?
					}
				}
				k => {
					let checkpoint = parser.checkpoint();
					if image.is_implicit() {
						let node = Expr::<ListStyleImageValue>::parse(parser);
						match node {
							Ok(node) => {
								image = Shorthand::Explicit(parser.boxup(node));
								continue;
							}
							Err(_) => parser.rewind(checkpoint),
						}
					}
					let checkpoint = parser.checkpoint();
					if marker.is_implicit() {
						let node = Expr::<ListStyleTypeValue>::parse(parser);
						match node {
							Ok(node) => {
								marker = Shorthand::Explicit(parser.boxup(node));
								continue;
							}
							Err(_) => parser.rewind(checkpoint),
						}
					}
					Err(diagnostics::Unexpected(k, parser.cur().span))?
				}
			}
			if position.is_explicit() && image.is_explicit() && marker.is_explicit() {
				break;
			}
		}
		Ok(Self { position, image, marker }.spanned(span.until(parser.cur().span)))
	}
}

impl<'a> Parse<'a> for ListStyleTypeValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.cur_atom().unwrap();
				if ident == atom!("none") {
					parser.advance();
					Ok(Self::None.spanned(span))
				} else {
					let node = CounterStyle::parse(parser)?;
					Ok(Self::CounterStyle(node).spanned(span.until(parser.cur().span)))
				}
			}
			Kind::String => Ok(Self::String(parser.expect_string()?).spanned(span)),
			_ => {
				let node = CounterStyle::parse(parser)?;
				Ok(Self::CounterStyle(node).spanned(span.until(parser.cur().span)))
			}
		}
	}
}

impl<'a> Parse<'a> for ListStyleImageValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::Ident => {
				let ident = parser.cur_atom().unwrap();
				if ident == atom!("none") {
					parser.advance();
					Ok(Self::None.spanned(span))
				} else {
					let node = Image::parse(parser)?;
					Ok(Self::Image(node).spanned(span.until(parser.cur().span)))
				}
			}
			_ => {
				let node = Image::parse(parser)?;
				Ok(Self::Image(node).spanned(span.until(parser.cur().span)))
			}
		}
	}
}
