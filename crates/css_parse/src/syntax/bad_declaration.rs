use crate::{syntax::ComponentValue, Parse, Parser, Result as ParserResult, State, T};

pub struct BadDeclaration;

// https://drafts.csswg.org/css-syntax-3/#consume-the-remnants-of-a-bad-declaration
impl<'a> Parse<'a> for BadDeclaration {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		// To consume the remnants of a bad declaration from a token stream input, given a bool nested:
		//
		// Process input:
		loop {
			// <eof-token>
			// <semicolon-token>
			//
			//     Discard a token from input, and return nothing.
			if p.at_end() || p.peek::<T![;]>() {
				p.parse::<T![;]>().ok();
				return Ok(Self);
			}
			// <}-token>
			//
			//     If nested is true, return nothing. Otherwise, discard a token.
			if p.peek::<T!['}']>() {
				if p.is(State::Nested) {
					return Ok(Self);
				} else {
					p.parse::<T!['}']>()?;
				}
			}
			// anything else
			//
			//     Consume a component value from input, and do nothing.
			//
			p.parse::<ComponentValue>()?;
		}
	}
}
