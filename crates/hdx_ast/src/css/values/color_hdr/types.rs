use bumpalo::collections::Vec;
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, CursorStream, Is, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

mod func {
	use hdx_parser::custom_function;
	custom_function!(DynamicRangeLimitMix, atom!("dynamic-range-limit-mix"));
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DynamicRangeLimitMix<'a> {
	function: T![Function],
	values: Vec<'a, (T![Ident], T![Dimension::%], Option<T![,]>)>,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for DynamicRangeLimitMix<'a> {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<func::DynamicRangeLimitMix>()
	}
}

impl<'a> Parse<'a> for DynamicRangeLimitMix<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<T![Function]>()?;
		let c = function.into();
		if !func::DynamicRangeLimitMix::is(p, c) {
			Err(diagnostics::Unexpected(c.into(), c.into()))?;
		}
		let mut values = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				return Ok(Self { function, values, close: None });
			}
			if p.peek::<T![')']>() {
				return Ok(Self { function, values, close: Some(p.parse::<T![')']>()?) });
			}
			let ident = p.parse::<T![Ident]>()?;
			let length = p.parse::<T![Dimension::%]>()?;
			let c: Cursor = length.into();
			if !(0.0..=100.0).contains(&c.token().value()) {
				Err(diagnostics::NumberOutOfBounds(c.token().value(), format!("{:?}", 0.0..=100.0), c.into()))?
			}
			let comma = p.parse_if_peek::<T![,]>()?;
			values.push((ident, length, comma));
		}
	}
}

impl<'a> ToCursors<'a> for DynamicRangeLimitMix<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.function.into());
		for (ident, length, comma) in &self.values {
			s.append(ident.into());
			s.append(length.into());
			if let Some(comma) = comma {
				s.append(comma.into());
			}
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DynamicRangeLimitMix, 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeLimitMix, "dynamic-range-limit-mix(high 80%,standard 20%)");
		assert_parse!(DynamicRangeLimitMix, "dynamic-range-limit-mix(high 8%,standard 2%)");
	}
}
