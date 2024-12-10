pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DynamicRangeLimit, 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeLimit, "high");
		assert_parse!(DynamicRangeLimit, "dynamic-range-limit-mix(high 80%,standard 20%)");
		assert_parse!(DynamicRangeLimit, "dynamic-range-limit-mix(high 8%,standard 2%)");
	}
}
