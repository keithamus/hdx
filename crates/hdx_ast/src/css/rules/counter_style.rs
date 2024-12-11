use crate::Todo;

// https://drafts.csswg.org/css-counter-styles-3/#the-counter-style-rule
pub type CounterStyleRule = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(CounterStyleRule, 0);
	}

	#[test]
	fn test_writes() {
		//assert_parse!(CounterStyleRule, "@counter-style thumbs {}");
	}
}
