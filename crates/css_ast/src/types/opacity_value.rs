use css_lexer::Cursor;
use css_parse::{Build, Parser, Peek, T};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum OpacityValue {
	Number(T![Number]),
	Percent(T![Dimension::%]),
}

impl OpacityValue {
	#[allow(non_upper_case_globals)]
	pub const Zero: OpacityValue = OpacityValue::Number(<T![Number]>::NUMBER_ZERO);
}

impl From<OpacityValue> for i32 {
	fn from(value: OpacityValue) -> Self {
		match value {
			OpacityValue::Number(t) => t.into(),
			OpacityValue::Percent(t) => t.into(),
		}
	}
}

impl From<OpacityValue> for f32 {
	fn from(value: OpacityValue) -> Self {
		match value {
			OpacityValue::Number(t) => t.into(),
			OpacityValue::Percent(t) => t.into(),
		}
	}
}

impl<'a> Peek<'a> for OpacityValue {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		(<T![Number]>::peek(p, c) && (0.0..=1.0).contains(&c.token().value()))
			|| (<T![Dimension::%]>::peek(p, c) && (0.0..=100.0).contains(&c.token().value()))
	}
}

impl<'a> Build<'a> for OpacityValue {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else {
			Self::Percent(<T![Dimension::%]>::build(p, c))
		}
	}
}

impl From<OpacityValue> for Cursor {
	fn from(value: OpacityValue) -> Self {
		match value {
			OpacityValue::Number(t) => t.into(),
			OpacityValue::Percent(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OpacityValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OpacityValue, "0.1");
		assert_parse!(OpacityValue, "1");
		assert_parse!(OpacityValue, "50%");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OpacityValue, "20");
		assert_parse_error!(OpacityValue, "1000%");
	}

	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(OpacityValue, "0.1", {
	// 		"node": [0, 3],
	// 		"start": 0,
	// 		"end": 5
	// 	});
	// }
}
