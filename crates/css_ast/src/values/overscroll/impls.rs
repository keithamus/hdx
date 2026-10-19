#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OverscrollBehaviorStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, OverscrollBehaviorStyleValue, "contain none");
		assert_parse!(CssAtomSet::ATOMS, OverscrollBehaviorInlineStyleValue, "contain");
		assert_parse!(CssAtomSet::ATOMS, OverscrollContainerTypeStyleValue, "push");
		assert_parse!(CssAtomSet::ATOMS, OverscrollContainerTypeStyleValue, "overlay");
	}
}
