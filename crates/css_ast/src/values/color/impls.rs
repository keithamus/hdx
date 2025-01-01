pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<OpacityStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorStyleValue, "red");
		assert_parse!(OpacityStyleValue, "1");
	}
}
