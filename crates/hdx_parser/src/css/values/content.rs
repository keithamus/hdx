use hdx_ast::css::values::content::{ContentElement, ContentList, ContentsValue, QuotesValue};

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for ContentsValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		if parser.at(Kind::Ident) {
			match parser.cur_atom_lower().unwrap() {
				atom!("normal") => {
					parser.advance();
					return Ok(Self::Normal.spanned(span.end(parser.pos())));
				}
				atom!("none") => {
					parser.advance();
					return Ok(Self::None.spanned(span.end(parser.pos())));
				}
				_ => {}
			}
		}
		let list = ContentList::parse(parser)?;
		// TODO: Replacement??
		// if list.values.len() == 1 {
		//     let element = list.values[0];
		//     if let ContentElement::Image(image) = element {
		//         return Ok(Self::Replacement(ContentReplacement { image, alt: list.alt }));
		//     }
		// }
		Ok(Self::List(list).spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for ContentList<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut values = parser.new_vec();
		let alt = parser.new_vec();
		loop {
			match parser.cur() {
				Kind::String => {
					values.push(ContentElement::String(parser.cur_atom().unwrap()));
					parser.advance();
				}
				Kind::Semicolon | Kind::Eof | Kind::RightCurly => {
					break;
				}
				_ => Err(diagnostics::Unimplemented(parser.span()))?,
			}
		}
		Ok(Self { values, alt }.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for QuotesValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				match ident {
					atom!("none") => {
						parser.advance();
						return Ok(Self::None.spanned(span.end(parser.pos())));
					}
					atom!("auto") => {
						parser.advance();
						return Ok(Self::Auto.spanned(span.end(parser.pos())));
					}
					_ => Err(diagnostics::UnexpectedIdent(ident, parser.span()))?,
				}
			}
			Kind::String => {
				let mut custom = parser.new_vec();
				loop {
					let open = parser.expect_string()?;
					let close = parser.expect_string()?;
					custom.push((open, close));
					if !parser.at(Kind::String) {
						break;
					}
				}
				Ok(Self::Custom(custom).spanned(span.end(parser.pos())))
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
	}
}
