use bitmask_enum::bitmask;

/// A set of runtime feature flags which can be enabled individually or in combination, which will change the way
/// [Parser][crate::Parser] works.
///
/// To build multiple features, use the bitwise OR operator.
///
/// # Example
///
/// ```
/// use css_lexer::*;
/// let features = Feature::SingleLineComments | Feature::SeparateWhitespace;
/// let mut lexer = Lexer::new_with_features("// foo", features);
/// ```
#[bitmask(u8)]
pub enum Feature {
	/// This flag is forwarded to the [Lexer][crate::Lexer] which, when enabled, will treat single line comments as valid
	/// Comment tokens. If it encounters two consecutative SOLIDUS characters (`//`), it will return a
	/// [Token][css_lexer::Token] with [Kind::Comment][css_lexer::Kind::Comment]. For more information about exactly what
	/// happens here at the lexer level, consult the [css_lexer::Feature::SingleLineComments] feature.
	///
	/// This flag doesn't cause any changes in logic on the [Parser][crate::Parser]; comments will be collected in the
	/// trivia tokens Vec as normal.
	SingleLineComments,

	/// This flag is forwarded to the [Lexer][crate::Lexer] which, when enabled, will treat diffetent whitespace kinds as
	/// descrete. For more information about exactly what happens here at the lexer level, consult the
	/// [css_lexer::Feature::SeparateWhitespace] feature.
	///
	/// This flag doesn't cause any changes in logic on the [Parser][crate::Parser]; whitespace is typically collected in
	/// the trivia Vec. AST nodes which call [Parser::set_skip()][Parser::set_skip] to parse whitespace sensitive nodes
	/// should be cognizant that this feature could be enabled, meaning that adjacent whitespace tokens are possible. To
	/// counter adjacent tokens, simply parse any whitespace in a loop.
	SeparateWhitespace,
}

impl From<Feature> for css_lexer::Feature {
	fn from(value: Feature) -> Self {
		let mut f = Self::none();
		if value.contains(Feature::SingleLineComments) {
			f |= Self::SingleLineComments
		}
		if value.contains(Feature::SeparateWhitespace) {
			f |= Self::SeparateWhitespace
		}
		f
	}
}

impl Default for Feature {
	fn default() -> Self {
		Self::none()
	}
}
