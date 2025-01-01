use css_parse::keyword_set;

use bitmask_enum::bitmask;

keyword_set!(Whitepoint { D50: "d50", D65: "d65" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<ColorFunctionSyntax>(), 1);
		assert_eq!(std::mem::size_of::<ColorMixSyntax>(), 1);
	}
}
