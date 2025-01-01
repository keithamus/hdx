use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{diagnostics, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DynamicRangeLimitMix<'a> {
	function: T![Function],
	values: Vec<'a, (T![Ident], T![Dimension::%], Option<T![,]>)>,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for DynamicRangeLimitMix<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "dynamic-range-limit-mix")
	}
}

impl<'a> Parse<'a> for DynamicRangeLimitMix<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<T![Function]>()?;
		let c = function.into();
		if !p.eq_ignore_ascii_case(c, "dynamic-range-limit-mix") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?;
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

impl<'a> ToCursors for DynamicRangeLimitMix<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DynamicRangeLimitMix>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeLimitMix, "dynamic-range-limit-mix(high 80%,standard 20%)");
		assert_parse!(DynamicRangeLimitMix, "dynamic-range-limit-mix(high 8%,standard 2%)");
	}
}
