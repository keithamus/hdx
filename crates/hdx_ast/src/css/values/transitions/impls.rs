pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		// assert_size!(TransitionPropertyStyleValue, 1);
		assert_size!(TransitionDurationStyleValue, 32);
		// assert_size!(TransitionTimingFunctionStyleValue, 1);
		assert_size!(TransitionDelayStyleValue, 32);
		// assert_size!(TransitionStyleValue, 1);
		assert_size!(TransitionBehaviorStyleValue, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransitionBehaviorStyleValue, "allow-discrete");
	}
}
