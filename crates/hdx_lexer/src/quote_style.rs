use core::fmt;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "kind", content = "value"))]
pub enum QuoteStyle {
	// Some tokens/ast nodesthat would otherwise be strings (e.g. url(), named fonts) can have no quotes.
	None,
	Single,
	#[default]
	Double,
}

impl QuoteStyle {
	const fn as_char(&self) -> Option<char> {
		match self {
			Self::Single => Some('\''),
			Self::Double => Some('"'),
			Self::None => None,
		}
	}
}

impl fmt::Display for QuoteStyle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(ch) = self.as_char() {
			fmt::Display::fmt(&ch, f)?;
		}
		Ok(())
	}
}

impl PartialEq<char> for QuoteStyle {
	fn eq(&self, other: &char) -> bool {
		self.as_char().map(|char| char == *other).unwrap_or(false)
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<QuoteStyle>(), 1);
}

#[test]
fn test_partial_eq() {
	assert!(QuoteStyle::Single == '\'');
	assert!(QuoteStyle::Double == '"');
	assert!(QuoteStyle::Single != '"');
	assert!(QuoteStyle::Double != '\'');
}
