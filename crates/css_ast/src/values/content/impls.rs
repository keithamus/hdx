#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BookmarkLevelStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, BookmarkStateStyleValue, "open");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"hello\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"\\f105\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "url(dot.gif)");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "open-quote");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "counter(section,decimal)");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"Chapter\" counter(chapter)");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "\"hello\" / \"alt text\"");
		assert_parse!(CssAtomSet::ATOMS, ContentStyleValue, "url(img.png) / \"alt\"");
		assert_parse!(CssAtomSet::ATOMS, QuotesStyleValue, "match-parent");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ContentStyleValue, "invalid-keyword");
		assert_peek_false!(CssAtomSet::ATOMS, QuotesStyleValue, "bikeshed-this");
	}
}
