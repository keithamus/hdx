use bitmask_enum::bitmask;

/// A [bitmask][bitmask_enum] representing the characters that make up a [Kind::Whitespace][crate::Kind::Whitespace]
/// token.
///
/// A [Token][crate::Token] with [Kind::Whitespace][crate::Kind::Whitespace] will store this data internal to the
/// token. Using [Token::whitespace_style()][crate::Token::whitespace_style()] will return this bitmask, depending on
/// what characters make up the whitespace token. By default the [Lexer][crate::Lexer] will produce combine multiple
/// whitespaces into a single [Token][crate::Token], so it is possible that
/// [Token::whitespace_style()][crate::Token::whitespace_style()] could contain all of the available bits here. With
/// [Feature::SeparateWhitespace][crate::Feature::SeparateWhitespace] the [Lexer][crate::Lexer] will produce discrete
/// tokens each which can only have one of the available bits in this bitmask.
///
/// ```
/// use css_lexer::*;
/// let mut lexer = Lexer::new("\n\t");
/// {
///		// This token will be collapsed Whitespace.
///		let token = lexer.advance();
///		assert_eq!(token, Kind::Whitespace);
///		// The Whitespace is comprised of many bits:
///		assert_eq!(token, Whitespace::Newline | Whitespace::Tab);
/// }
/// ```
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum Whitespace {
	/// The whitespace token contains at least 1 Space (` `) character.
	Space = 0b001,
	/// The whitespace token contains at least 1 Tab (`\t`) character.
	Tab = 0b010,
	/// The whitespace token contains at least 1 Newline (`\n`) or newline-adjacent (`\r`, `\r\n`, `\u{c}`) character.
	Newline = 0b100,
}

impl Whitespace {
	pub(crate) const fn from_bits(bits: u8) -> Self {
		Self { bits: bits & 0b111 }
	}

	pub(crate) const fn to_bits(&self) -> u8 {
		self.bits
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Whitespace>(), 1);
}

#[test]
fn test_from_bits() {
	assert!(Whitespace::from_bits(Whitespace::Space.bits).contains(Whitespace::Space));
	assert!(Whitespace::from_bits(Whitespace::Tab.bits).contains(Whitespace::Tab));
	assert!(Whitespace::from_bits(Whitespace::Newline.bits).contains(Whitespace::Newline));
}
