use crate::Kind;

/// Match a token against one or more [Kinds][Kind].
///
/// Each [Kind] represents the token "type". [KindSet] is a bitmask of all possible [Kinds][Kind]. This is useful for
/// efficiently comparing a token to see if it matches N token [Kinds][Kind].
///
/// # Example
///
/// ```
/// use css_lexer::*;
/// let mut lexer = Lexer::new("width: 1px");
/// // The first token is either an AtKeyword, Ident or Function:
/// assert_eq!(lexer.advance(), KindSet::new(&[Kind::AtKeyword, Kind::Ident, Kind::Function]));
/// ```
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindSet(u32);

impl KindSet {
	/// A [KindSet] that matches no [Kinds][Kind].
	pub const NONE: KindSet = KindSet::new(&[]);

	/// A [KindSet] that matches all trivia; [Kind::Whitespace] and [Kind::Comment].
	pub const TRIVIA: KindSet = KindSet::new(&[Kind::Whitespace, Kind::Comment]);

	/// A [KindSet] that matches just Whitespace. This is the same as [Kind::Whitespace] but can be useful to apply to
	/// functions that expect a [KindSet] rather than [Kind].
	pub const WHITESPACE: KindSet = KindSet::new(&[Kind::Whitespace]);

	/// A [KindSet] that matches just Whitespace. This is the same as [Kind::Comment] but can be useful to apply to
	/// functions that expect a [KindSet] rather than [Kind].
	pub const COMMENTS: KindSet = KindSet::new(&[Kind::Comment]);

	/// A [KindSet] that matches either [Kind::RightCurly] or [Kind::Semicolon]. This is useful for matching
	/// stop-tokens, for example checking the end of a declaration.
	pub const RIGHT_CURLY_OR_SEMICOLON: KindSet = KindSet::new(&[Kind::RightCurly, Kind::Semicolon]);

	/// A [KindSet] that matches either [Kind::LeftCurly] or [Kind::Semicolon]. This is useful for matching
	/// stop-tokens, for example checking the end of an at-rule prelude.
	pub const LEFT_CURLY_OR_SEMICOLON: KindSet = KindSet::new(&[Kind::LeftCurly, Kind::Semicolon]);

	/// A [KindSet] that matches either [Kind::LeftCurly] or [Kind::RightParen] or [Kind::Semicolon]. This is useful for
	/// matching stop-tokens, for example checking the end of a function.
	pub const LEFT_CURLY_RIGHT_PAREN_OR_SEMICOLON: KindSet =
		KindSet::new(&[Kind::LeftCurly, Kind::RightParen, Kind::Semicolon]);

	/// A [KindSet] that matches either [Kind::LeftCurly] or [Kind::RightParen] or [Kind::Comma] or [Kind::Semicolon].
	/// This is useful for matching stop-tokens, for example checking the end of a function or Selector.
	pub const LEFT_CURLY_RIGHT_PAREN_COMMA_OR_SEMICOLON: KindSet =
		KindSet::new(&[Kind::LeftCurly, Kind::RightParen, Kind::Comma, Kind::Semicolon]);

	/// Creates a new [KindSet] with the combination of all given [Kinds][Kind].
	///
	/// This function is marked `const` to allow creation of const [KindSets][KindSet].
	pub const fn new(kinds: &[Kind]) -> Self {
		let mut u = 0;
		let mut i = 0;
		let len = kinds.len();
		while i < len {
			u |= 1 << (kinds[i] as u8 % 32);
			i += 1;
		}
		Self(u)
	}

	/// Returns a new [KindSet] with the addition of the supplied [Kind].
	///
	/// This function is marked `const` to allow creation of const [KindSets][KindSet].
	pub const fn add(&self, kind: Kind) -> Self {
		Self(self.0 | (1 << (kind as u8 % 32)))
	}

	/// Check if a [KindSet] contains the subpplied [Kind].
	pub fn contains(&self, kind: Kind) -> bool {
		self.0 & (1 << (kind as u8 % 32)) != 0
	}

	pub(crate) const fn contains_bits(&self, kind_bits: u8) -> bool {
		self.0 & (1 << (kind_bits % 32)) != 0
	}
}

#[test]
fn test_kindset_contains() {
	let set = KindSet::new(&[Kind::Eof, Kind::Whitespace, Kind::Comment]);
	assert!(set.contains(Kind::Eof));
	assert!(set.contains(Kind::Whitespace));
	assert!(set.contains(Kind::Comment));
	assert!(!set.contains(Kind::String));
	assert!(!set.contains(Kind::Url));
}
