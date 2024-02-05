use hdx_ast::css::values::MarginTrimValue;

use crate::{atom, diagnostics, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for MarginTrimValue {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let ident = parser.expect_ident()?;
		if ident == atom!("none") {
			Ok(Self {
				block_start: false,
				block_end: false,
				inline_start: false,
				inline_end: false,
			}
			.spanned(span.end(parser.pos())))
		} else if ident == atom!("block") {
			Ok(Self { block_start: true, block_end: true, inline_start: false, inline_end: false }
				.spanned(span.end(parser.pos())))
		} else if ident == atom!("inline") {
			Ok(Self { block_start: false, block_end: false, inline_start: true, inline_end: true }
				.spanned(span.end(parser.pos())))
		} else {
			let mut value = Self {
				block_start: ident == atom!("block-start"),
				block_end: ident == atom!("block-end"),
				inline_start: ident == atom!("inline-start"),
				inline_end: ident == atom!("inline-end"),
			};
			loop {
				if !parser.at(Kind::Ident) {
					break;
				}
				let span = parser.span();
				let ident = parser.expect_ident()?;
				match ident {
					atom!("block-start") => {
						if value.block_start {
							Err(diagnostics::UnexpectedDuplicateIdent(ident, span))?;
						}
						value.block_start = true
					}
					atom!("block-end") => {
						if value.block_end {
							Err(diagnostics::UnexpectedDuplicateIdent(ident, span))?;
						}
						value.block_end = true
					}
					atom!("inline-start") => {
						if value.inline_start {
							Err(diagnostics::UnexpectedDuplicateIdent(ident, span))?;
						}
						value.inline_start = true
					}
					atom!("inline-end") => {
						if value.inline_end {
							Err(diagnostics::UnexpectedDuplicateIdent(ident, span))?;
						}
						value.inline_end = true
					}
					_ => break,
				}
			}
			Ok(value.spanned(span.end(parser.pos())))
		}
	}
}
