/// An enum representing the "Style" the [Kind::Comment][crate::Kind::Comment] token represents.
///
/// A [Token][crate::Token] with [Kind::Comment][crate::Kind::Comment] will store this data internal to the token.
/// Using [Token::comment_style()][crate::Token::comment_style()] will return this enum, depending on what characters
/// make up the beginning of the comment token. By default the [Lexer][crate::Lexer] will only produce multi-line - aka
/// "Block" - comments, but adding [Feature::SeparateWhitespace][crate::Feature::SingleLineComments] will allow the
/// [Lexer][crate::Lexer] to produce single line comments too.
///
/// A basic [Block][CommentStyle::Block] comment style uses the `/*` leading characters, but sub-styles of the block
/// style are also computed, for example [BlockStar][CommentStyle::BlockStar] represents a comment using the "double
/// star" syntax to open the comment, i.e. `/**`. Determing if these comments are using these alternate style can help a
/// parser (or writer) determine if it should retain these comments or otherwise treat them differently to regular block
/// comments.
///
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
pub enum CommentStyle {
	#[default]
	/// A basic block comment which uses `/*` as the leading style. The third character may be a whitespace, or may
	/// include a character that _isn't_ `!`, `#`, `=`, `-`.
	Block = 0b000,
	/// A block comment which uses `/**` as the leading style. The two `*`s must be adjacent, so this does not count
	/// `/* *`.
	BlockStar = 0b001,
	/// A block comment which uses `/*!` as the leading style. The `*` and `!` must be adjacent, so this does not count
	/// `/* !`.
	BlockBang = 0b010,
	/// A block comment which uses `/*#` as the leading style. The `*` and `#` must be adjacent, so this does not count
	/// `/* #`.
	BlockPound = 0b011,
	/// A block comment which uses `/*=` or `/*-` as the leading style. The `*` and `-` or `=` must be adjacent, so this
	/// does not count `/* #`.
	BlockHeading = 0b100,
	/// A basic single line  comment which uses `//` as the leading style. The third character may be a whitespace, or
	/// may include a character that _isn't_ `*`, `!`. The [Lexer][crate::Lexer] can only produce a [Token][crate::Token]
	/// with this style if [Feature::SingleLineComments][crate::Feature::SingleLineComments] is enabled.
	Single = 0b101,
	/// A single line comment which uses `//*` as the leading style. The `*` be adjacent to the `//`, so this does not
	/// count `// *`. The [Lexer][crate::Lexer] can only produce a [Token][crate::Token] with this style if
	/// [Feature::SingleLineComments][crate::Feature::SingleLineComments] is enabled.
	SingleStar = 0b110,
	/// A single line comment which uses `//!` as the leading style. The `!` be adjacent to the `//`, so this does not
	/// count `// !`. The [Lexer][crate::Lexer] can only produce a [Token][crate::Token] with this style if
	/// [Feature::SingleLineComments][crate::Feature::SingleLineComments] is enabled.
	SingleBang = 0b111,
}

impl CommentStyle {
	#[inline]
	pub fn is_block(&self) -> bool {
		matches!(self, Self::Block | Self::BlockStar | Self::BlockBang | Self::BlockPound | Self::BlockHeading)
	}

	#[inline]
	pub fn is_non_standard(&self) -> bool {
		matches!(self, Self::Single | Self::SingleStar | Self::SingleBang)
	}

	#[inline]
	pub fn retain(&self) -> bool {
		matches!(self, Self::Single | Self::SingleStar | Self::SingleBang)
	}

	pub(crate) fn from_bits(bits: u8) -> Option<Self> {
		match bits {
			0b000 => Some(Self::Block),
			0b001 => Some(Self::BlockStar),
			0b010 => Some(Self::BlockBang),
			0b011 => Some(Self::BlockPound),
			0b100 => Some(Self::BlockHeading),
			0b101 => Some(Self::Single),
			0b110 => Some(Self::SingleStar),
			0b111 => Some(Self::SingleBang),
			_ => None,
		}
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<CommentStyle>(), 1);
}

#[test]
fn test_from_bits() {
	assert_eq!(CommentStyle::from_bits(CommentStyle::Block as u8), Some(CommentStyle::Block));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockStar as u8), Some(CommentStyle::BlockStar));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockBang as u8), Some(CommentStyle::BlockBang));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockPound as u8), Some(CommentStyle::BlockPound));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockHeading as u8), Some(CommentStyle::BlockHeading));
	assert_eq!(CommentStyle::from_bits(CommentStyle::Single as u8), Some(CommentStyle::Single));
	assert_eq!(CommentStyle::from_bits(CommentStyle::SingleStar as u8), Some(CommentStyle::SingleStar));
	assert_eq!(CommentStyle::from_bits(CommentStyle::SingleBang as u8), Some(CommentStyle::SingleBang));
}
