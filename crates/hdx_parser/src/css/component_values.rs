use hdx_ast::css::component_values::{ComponentValue, Function, SimpleBlock};

use crate::{diagnostics, Kind, Parse, Parser, Result, Spanned, Vec};

impl<'a> Parser<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	pub(crate) fn parse_component_values(
		&mut self,
		stop_token: Kind,
		nested: bool,
	) -> Result<Vec<'a, Spanned<ComponentValue<'a>>>> {
		let mut values = self.new_vec();
		loop {
			match self.cur().kind {
				Kind::Eof => {
					return Ok(values);
				}
				Kind::RightCurly => {
					if nested {
						return Ok(values);
					}
					self.advance();
				}
				c => {
					if c == stop_token {
						return Ok(values);
					}
					values.push(ComponentValue::parse(self)?)
				}
			}
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		match parser.cur().kind {
			Kind::LeftCurly | Kind::LeftSquare | Kind::LeftParen => {
				Ok(Self::SimpleBlock(SimpleBlock::parse(parser)?)
					.spanned(span.up_to(&parser.cur().span)))
			}
			Kind::Function => {
				Ok(Self::Function(Function::parse(parser)?).spanned(span.up_to(&parser.cur().span)))
			}
			_ => {
				let token = parser.cur().clone();
				parser.advance();
				Ok(Self::Token(token).spanned(span))
			}
		}
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let pairwise = parser
			.cur()
			.to_pairwise()
			.ok_or_else(|| diagnostics::Unexpected(parser.cur().kind, span))?;
		parser.advance();
		let value = parser.parse_component_values(pairwise.end(), true)?;
		Ok(Self { value: parser.boxup(value), pairwise }.spanned(span.up_to(&parser.cur().span)))
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for Function<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		let name = parser.expect_function()?;
		let value = parser.parse_component_values(Kind::RightParen, false)?;
		Ok(Self { name, value: parser.boxup(value) }.spanned(span.up_to(&parser.cur().span)))
	}
}
