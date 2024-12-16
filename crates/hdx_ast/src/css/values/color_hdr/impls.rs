pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DynamicRangeLimitStyleValue, 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeLimitStyleValue, "high");
		assert_parse!(DynamicRangeLimitStyleValue, "dynamic-range-limit-mix(high 80%,standard 20%)");
		assert_parse!(DynamicRangeLimitStyleValue, "dynamic-range-limit-mix(high 8%,standard 2%)");
	}
}
