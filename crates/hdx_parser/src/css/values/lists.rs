use hdx_ast::css::values::{
	CounterStyle, Expr, Image, ListStyleImageValue, ListStylePositionValue, ListStyleShorthand,
	ListStyleTypeValue, Shorthand,
};

use crate::{atom, diagnostics, Parse, Parser, Result, Spanned, Token};

impl<'a> Parse<'a> for ListStyleShorthand<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut position = Shorthand::Implicit;
		let mut image = Shorthand::Implicit;
		let mut marker = Shorthand::Implicit;
		loop {
			match parser.cur() {
				Token::Semicolon | Token::Comma | Token::Eof => {
					break;
				}
				Token::Ident(ident) => {
					if position.is_implicit()
						&& matches!(ident.to_ascii_lowercase(), atom!("inside") | atom!("outside"))
					{
						let node = Expr::<ListStylePositionValue>::parse(parser)?;
						position = Shorthand::Explicit(parser.boxup(node));
					} else if image.is_implicit()
						&& matches!(ident.to_ascii_lowercase(), atom!("none"))
					{
						let node = Expr::<ListStyleImageValue>::parse(parser)?;
						image = Shorthand::Explicit(parser.boxup(node));
					} else if marker.is_implicit() {
						let node = Expr::<ListStyleTypeValue>::parse(parser)?;
						marker = Shorthand::Explicit(parser.boxup(node));
					} else {
						Err(diagnostics::UnexpectedIdent(*ident, parser.span()))?
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
					Err(diagnostics::Unexpected(*k, parser.span()))?
				}
			}
			if position.is_explicit() && image.is_explicit() && marker.is_explicit() {
				break;
			}
		}
		Ok(Self { position, image, marker }.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for ListStyleTypeValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		match parser.cur() {
			Token::Ident(ident) => {
				if ident.to_ascii_lowercase() == atom!("none") {
					Ok(Self::None.spanned(parser.advance()))
				} else {
					let span = parser.span();
					let node = CounterStyle::parse(parser)?;
					Ok(Self::CounterStyle(node).spanned(span.end(parser.pos())))
				}
			}
			Token::String(value) => Ok(Self::String(*value).spanned(parser.advance())),
			_ => {
				let span = parser.span();
				let node = CounterStyle::parse(parser)?;
				Ok(Self::CounterStyle(node).spanned(span.end(parser.pos())))
			}
		}
	}
}

impl<'a> Parse<'a> for ListStyleImageValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		match parser.cur() {
			Token::Ident(ident) => {
				if ident.to_ascii_lowercase() == atom!("none") {
					Ok(Self::None.spanned(parser.advance()))
				} else {
					let span = parser.span();
					let node = Image::parse(parser)?;
					Ok(Self::Image(node).spanned(span.end(parser.pos())))
				}
			}
			_ => {
				let span = parser.span();
				let node = Image::parse(parser)?;
				Ok(Self::Image(node).spanned(span.end(parser.pos())))
			}
		}
	}
}
