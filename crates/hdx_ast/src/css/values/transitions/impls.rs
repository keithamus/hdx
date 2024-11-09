pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		// assert_size!(TransitionProperty, 1);
		assert_size!(TransitionDuration, 24);
		// assert_size!(TransitionTimingFunction, 1);
		assert_size!(TransitionDelay, 24);
		// assert_size!(Transition, 1);
		assert_size!(TransitionBehavior, 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransitionBehavior, "allow-discrete");
	}
}
