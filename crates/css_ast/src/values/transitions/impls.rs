pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<TransitionPropertyStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<TransitionDurationStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<TransitionTimingFunctionStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<TransitionDelayStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<TransitionStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<TransitionBehaviorStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransitionBehaviorStyleValue, "allow-discrete");
	}
}
