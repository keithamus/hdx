use core::fmt;

/// An enum representing the "Style" the [Kind::String][crate::Kind::String] token represents.
///
/// A [Token][crate::Token] with [Kind::String][crate::Kind::String] will store this data internal to the token. Using
/// [Token::quote_style()][crate::Token::quote_style()] will return this enum, depending on what character is used to
/// represent the [Kind::String][crate::Kind::String] token.
/// ```
/// use css_lexer::*;
/// let mut lexer = Lexer::new("/* Normal Comment */  /** Double Star Comment */");
/// {
///		// This token will be collapsed Whitespace.
///		let token = lexer.advance();
///		assert_eq!(token, Kind::Comment);
///		assert_eq!(token, CommentStyle::Block);
/// }
/// assert_eq!(lexer.advance(), Kind::Whitespace);
/// {
///		// This token will be collapsed Whitespace.
///		let token = lexer.advance();
///		assert_eq!(token, Kind::Comment);
///		assert_eq!(token, CommentStyle::BlockStar);
/// }
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "kind", content = "value"))]
pub enum QuoteStyle {
	/// Tokens such as [Kind::Url][crate::Kind::Url] may also refer to [QuoteStyle], but a URL is not required to contain
	/// quote characters, as the parenthesese are sufficient to disambiguate the token. In these case the
	/// [QuoteStyle::None] variant exists to encode this information. [Kind::String][crate::Kind::String] tokens must
	/// always have a quote style that is not [QuoteStyle::None].
	None,
	/// The single quote character or APOSTROPHE (`'`) was used.
	Single,
	#[default]
	/// The double quote character or QUOTATION MARK (`"`) was used.
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
