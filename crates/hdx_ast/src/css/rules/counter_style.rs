use crate::css::values::Todo;

// https://drafts.csswg.org/css-counter-styles-3/#the-counter-style-rule
pub type CounterStyle = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CounterStyle, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(CounterStyle, "@counter-style thumbs {}");
	}
}
