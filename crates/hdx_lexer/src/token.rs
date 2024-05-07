use bitmask_enum::bitmask;
#[cfg(feature = "serde")]
use serde::{
	ser::{SerializeStruct, Serializer},
	Serialize,
};

use crate::Include;

#[derive(Default)]
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum NumType {
	#[default]
	Float = 0x01,
	Signed = 0x10,
}

impl NumType {
	#[inline]
	pub fn is_int(&self) -> bool {
		self.and(NumType::Float) != NumType::Float
	}

	#[inline]
	pub fn is_float(&self) -> bool {
		self.contains(NumType::Float)
	}

	#[inline]
	pub fn is_signed(&self) -> bool {
		self.contains(NumType::Signed)
	}

	#[inline]
	pub fn is_signed_int(&self) -> bool {
		self.contains(NumType::Signed) && !self.contains(NumType::Float)
	}

	#[inline]
	pub fn signed(&self) -> NumType {
		self.or(NumType::Signed)
	}

	#[inline]
	pub fn float(&self) -> NumType {
		self.or(NumType::Float)
	}
}

#[derive(Default)]
#[bitmask(u8)] // Actually more like a "u5" as the 3 LMB are unused
pub enum Kind {
	// Trivias (mask as 0b0_00XX)
	Eof = 0b0000, // https://drafts.csswg.org/css-syntax/#typedef-eof-token
	#[default]
	Whitespace = 0b0001, // https://drafts.csswg.org/css-syntax/#whitespace-token-diagram
	Comment = 0b0010, // https://drafts.csswg.org/css-syntax/#comment-diagram
	// Stand in for both the CDC and CDO tokens
	CdcOrCdo = 0b0011, // https://drafts.csswg.org/css-syntax/#CDO-token-diagram, https://drafts.csswg.org/css-syntax/#CDC-token-diagram

	// Numerics (mask as 0b0_010X)
	Number = 0b0100,    // https://drafts.csswg.org/css-syntax/#number-token-diagram
	Dimension = 0b0101, // https://drafts.csswg.org/css-syntax/#dimension-token-diagram

	// Errors (mask as 0b0_011X)
	BadString = 0b0110, // https://drafts.csswg.org/css-syntax/#typedef-bad-string-token
	BadUrl = 0b0111,    // https://drafts.csswg.org/css-syntax/#typedef-bad-url-token

	// Variable length Atom containing Tokens (mask: 0b0_1XXXX)
	Ident = 0b1000,     // https://drafts.csswg.org/css-syntax/#ident-token-diagram
	Function = 0b1001,  // https://drafts.csswg.org/css-syntax/#function-token-diagram
	AtKeyword = 0b1010, // https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram
	Hash = 0b1011,      // https://drafts.csswg.org/css-syntax/#hash-token-diagram
	String = 0b1100,    // https://drafts.csswg.org/css-syntax/#string-token-diagram
	Url = 0b1101,       // https://drafts.csswg.org/css-syntax/#url-token-diagram

	// Single character Tokens (mask 0b1_XXXX)
	Delim = 0b1_0000,       // https://drafts.csswg.org/css-syntax/#typedef-delim-token
	Colon = 0b1_0001,       // https://drafts.csswg.org/css-syntax/#typedef-colon-token
	Semicolon = 0b1_0010,   // https://drafts.csswg.org/css-syntax/#typedef-semicolon-token
	Comma = 0b1_0011,       // https://drafts.csswg.org/css-syntax/#typedef-comma-token
	LeftSquare = 0b1_0100,  // https://drafts.csswg.org/css-syntax/#tokendef-open-square
	RightSquare = 0b1_0101, // https://drafts.csswg.org/css-syntax/#tokendef-close-square
	LeftParen = 0b1_0110,   // https://drafts.csswg.org/css-syntax/#tokendef-open-paren
	RightParen = 0b1_0111,  // https://drafts.csswg.org/css-syntax/#tokendef-close-paren
	LeftCurly = 0b1_1000,   // https://drafts.csswg.org/css-syntax/#tokendef-open-curly
	RightCurly = 0b1_1001,  // https://drafts.csswg.org/css-syntax/#tokendef-close-curly
}

impl Kind {
	pub fn as_str(&self) -> &str {
		match *self {
			Kind::Eof => "EOF",
			Kind::Whitespace => "Whitespace",
			Kind::Comment => "Comment",
			Kind::CdcOrCdo => "CdcOrCdo",
			Kind::Number => "Number",
			Kind::Dimension => "Dimension",
			Kind::BadString => "BadString",
			Kind::BadUrl => "BadUrl",
			Kind::Ident => "Ident",
			Kind::Function => "Function",
			Kind::AtKeyword => "AtKeyword",
			Kind::Hash => "Hash",
			Kind::String => "String",
			Kind::Url => "Url",
			Kind::Delim => "Delim",
			Kind::Colon => "Colon",
			Kind::Semicolon => "Semicolon",
			Kind::Comma => "Comma",
			Kind::LeftSquare => "LeftSquare",
			Kind::RightSquare => "RightSquare",
			Kind::LeftParen => "LeftParen",
			Kind::RightParen => "RightParen",
			Kind::LeftCurly => "LeftCurly",
			Kind::RightCurly => "RightCurly",
			_ => unreachable!(),
		}
	}
}

#[bitmask(u32)]
pub enum TokenFlags {
	// Anatomy of TokenFlags
	//
	//   |-----|-------|--------------------------|
	//   | TF  | K     | L                        |
	// 0b| 000 | 00000 | 000000000000000000000000 |
	//   |-----|-------|--------------------------|
	//   | 3-- | 5---- | 24---------------------- |
	//
	//   TF  = Type Flags.    If K is Number/Dimension:
	//                          001 = Floating Point
	//                          010 = Has Sign Digit
	//                          100 = (Reserved)
	//                        If K is String:
	//                          001 = Uses Double Quotes (0 would be Single)
	//                          010 = Has a closing quote
	//                          100 = Includes escape characters
	//                        If K is Ident, Function, AtKeyword:
	//                          001 = Contains non-lower-ASCII (e.g. uppercase or unicode)
	//                          010 = Is a "Custom Ident" - starts with two dashes
	//                          100 = Includes escape characters
	//                        If K is Hash:
	//                          001 = Contains non-lower-ASCII (e.g. uppercase or unicode)
	//                          010 = First character is ascii, aka it's a "HashId"
	//                          100 = Includes escape characters
	//                        If K is Url:
	//                          001 = Ended with a `)`
	//                          010 = Contains whitespace after the `(`
	//                          100 = Includes escape characters
	//                        If K is Whitespace/Comment:
	//                          001 = Contains at least 1 tab character.
	//                          010 = Contains at least 1 newline character.
	//                          100 = (Reserved)
	//                        If K is CdcOrCdo
	//                          000 = Is Cdo
	//                          001 = Is Cdc
	//   K   = Kind Flags.    Maps to `Kind` enum
	//   L   = Lengthdata.    If K is Delim then this is `char` (see C below)
	//                        If K is Number/Dimension then this is split further (see NL below)
	//                        If K is Whitespace, Comment, String, Url, Ident, Function, AtKeyword,
	//                        If K is a non-delim single char, i.e. Colon->RightCurly then this is
	//                        `char` (see C below).
	//                        Max token length is 16777216 aka 16MB. This sounds very long
	//                        but also CSS can host very large image data URLs. 16MB seems like a
	//                        good limitation for this parser though; while browsers need
	//                        to accomodate much larger data URLs
	//                        (https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs#common_problems
	//                        claims Firefox supports 32mb, Chrome over 512mb, and Safari over
	//                        2gb) the reality is that if someone has such a large data URL in
	//                        their CSS they probably should split it out.
	//
	//   C   = Char.          This is `char` type. All available `chars` in unicode can be
	//                        represented in 24bits as the upper bits are for surrogates.
	//   NL  = Number Length. If thie Kind is a Numbers or Dimensions then the 8LMB become the
	//                        number of code points required to produce the numeric length, while
	//                        the remaining 17bits are used to determine the Dimensions identifier
	//                        length. This in practice means that numeric values can only be 255
	//                        codepoints long (it's possible to make a f32 that is longer than 255
	//                        codepoints as the exponent can be a whole bunch of zeroes, but in
	//                        practice this makes for nonsensical numbers that people would only
	//                        use to break the parser). The 17 bits for the dimension identifier
	//                        means the dimension identifier can only be 131,071 codepoints long.
	//                        In reality 130k characters is 100 pages of printed text, so its
	//                        very unlikely someone will be using a custom dimension this long.
}

impl Default for TokenFlags {
	fn default() -> Self {
		Self { bits: (Kind::Whitespace.bits as u32) << 24 }
	}
}

#[derive(Copy, Clone, PartialEq, Default, Hash)]
pub struct Token {
	flags: TokenFlags,
	pub offset: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "kind", content = "value"))]
pub enum QuoteStyle {
	// Some tokens/ast nodesthat would otherwise be strings (e.g. url(), named fonts) can have no quotes.
	None,
	Single,
	#[default]
	Double,
}

static TF_MASK: u32 = !((1 << 29) - 1);
static K_MASK: u32 = !((1 << 24) - 1);
static L_MASK: u32 = (1 << 24) - 1;
static NL_R_MASK: u32 = (1 << 16) - 1;

impl Token {
	pub fn new(kind: Kind, type_flags: u8, offset: u32, len: u32) -> Self {
		let len_masked = if kind == Kind::Number { (len << 16) & L_MASK } else { len & L_MASK };
		let flags = TokenFlags {
			bits: (((type_flags as u32) << 29) & TF_MASK) | (((kind.bits as u32) << 24) & K_MASK) | len_masked,
		};
		Self { flags, offset }
	}

	pub fn new_dimension(type_flags: u8, offset: u32, num_len: u32, unit_len: u32) -> Self {
		let flags = TokenFlags {
			bits: (((type_flags as u32) << 29) & TF_MASK)
				| (((Kind::Dimension.bits as u32) << 24) & K_MASK)
				| (((num_len << 16) | (unit_len & NL_R_MASK)) & L_MASK),
		};
		Self { flags, offset }
	}

	#[inline(always)]
	pub(crate) fn kind_bits(&self) -> u8 {
		(self.flags.bits >> 24 & 0b11111) as u8
	}

	#[inline(always)]
	pub(crate) fn should_skip(&self, inc: Include) -> bool {
		let b = self.kind_bits();
		b != 0 && b & 0b00011 == b && inc.bits & b != b
	}

	#[inline(always)]
	fn first_bit_is_set(&self) -> bool {
		self.flags.bits >> 31 == 1
	}

	#[inline(always)]
	fn second_bit_is_set(&self) -> bool {
		self.flags.bits >> 30 & 0b1 == 1
	}

	#[inline(always)]
	fn third_bit_is_set(&self) -> bool {
		self.flags.bits >> 29 & 0b1 == 1
	}

	#[inline(always)]
	pub(crate) fn is_ident_like(&self) -> bool {
		self.kind_bits() & 0b11000 == 0b01000 && self.kind_bits() != Kind::String.bits
	}

	#[inline]
	pub fn kind(&self) -> Kind {
		Kind { bits: self.kind_bits() }
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn len(&self) -> u32 {
		// EOF
		if self.kind_bits() & 0b11111 == 0 {
			debug_assert!(self.kind() == Kind::Eof);
			0
		} else if self.kind_bits() == Kind::Delim.bits {
			self.char().unwrap().len_utf8() as u32
		// Delim-like flag is set
		} else if self.kind_bits() & 0b10000 == 0b10000 {
			debug_assert!(matches!(self.kind(), Kind::Colon | Kind::Semicolon | Kind::Comma | Kind::LeftSquare | Kind::RightSquare | Kind::LeftParen | Kind::RightParen | Kind::LeftCurly | Kind::RightCurly));
			1
		// CdcOrCdo
		} else if self.kind_bits() == Kind::CdcOrCdo.bits {
			debug_assert!(self.kind() == Kind::CdcOrCdo);
			4 - (self.third_bit_is_set() as u32)
		// Number
		} else if self.kind_bits() == Kind::Number.bits {
			(self.flags.bits & L_MASK) >> 16
		// Delim
		} else if self.kind_bits() == Kind::Dimension.bits {
			((self.flags.bits & L_MASK) >> 16) + (self.flags.bits & NL_R_MASK)
		} else {
			self.flags.bits & L_MASK
		}
	}

	pub fn char(&self) -> Option<char> {
		// Delim flag is set
		if self.kind_bits() & 0b10000 == 0b10000 {
			return char::from_u32(self.flags.bits & L_MASK);
		}
		None
	}

	// Number Style Checks
	#[inline]
	pub fn is_int(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && !self.third_bit_is_set()
	}

	#[inline]
	pub fn is_float(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && self.third_bit_is_set()
	}

	#[inline]
	pub fn has_sign(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && self.second_bit_is_set()
	}

	#[inline]
	pub fn numeric_len(&self) -> u32 {
		(self.flags.bits & L_MASK) >> 16
	}

	// String style checks
	#[inline]
	pub fn quote_style(&self) -> QuoteStyle {
		if self.kind_bits() == Kind::String.bits {
			if self.third_bit_is_set() {
				return QuoteStyle::Single;
			} else if self.second_bit_is_set() {
				return QuoteStyle::Double;
			}
		}
		QuoteStyle::None
	}

	#[inline]
	pub fn string_has_closing_quote(&self) -> bool {
		self.kind_bits() == Kind::String.bits && self.second_bit_is_set()
	}

	// Escape style checks
	#[inline]
	pub fn can_escape(&self) -> bool {
		self.kind_bits() == Kind::String.bits || self.is_ident_like()
	}

	#[inline]
	pub fn contains_escape_chars(&self) -> bool {
		self.can_escape() && self.first_bit_is_set()
	}

	#[inline]
	pub fn is_dashed_ident(&self) -> bool {
		self.is_ident_like() && self.second_bit_is_set()
	}

	#[inline]
	pub fn is_lower_case(&self) -> bool {
		self.is_ident_like() && self.third_bit_is_set()
	}

	#[inline]
	pub fn is_trivia(&self) -> bool {
		self.kind_bits() & 0b000011 == self.kind_bits()
	}

	// Url style checks
	#[inline]
	pub fn url_has_leading_space(&self) -> bool {
		self.kind_bits() == Kind::Url.bits && self.second_bit_is_set()
	}

	#[inline]
	pub fn url_has_closing_paren(&self) -> bool {
		self.kind_bits() == Kind::Url.bits && self.third_bit_is_set()
	}

	// Whitespace/Comment Style checks
	#[inline]
	pub fn contains_newline(&self) -> bool {
		(self.kind_bits() == Kind::Whitespace.bits || self.kind_bits() == Kind::Comment.bits)
			&& self.second_bit_is_set()
	}

	#[inline]
	pub fn contains_tab(&self) -> bool {
		(self.kind_bits() == Kind::Whitespace.bits || self.kind_bits() == Kind::Comment.bits) && self.third_bit_is_set()
	}


	#[inline]
	pub fn is_bad(&self) -> bool {
		(self.kind_bits() | 0b00001) & 0b11001 == 1
	}

	#[inline]
	pub fn is_cdc(&self) -> bool {
		self.kind() == Kind::CdcOrCdo && self.third_bit_is_set()
	}

	#[inline]
	pub fn to_pairwise(&self) -> Option<PairWise> {
		PairWise::from_token(self)
	}
}

impl core::fmt::Debug for Token {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let bits = match self.kind() {
			Kind::Number | Kind::Delim => format!(
				"{:03b}_{:05b}_{:08b}_{:016b}",
				&self.flags.bits >> 29,
				self.kind_bits(),
				(self.flags.bits & L_MASK) >> 16,
				self.flags.bits & NL_R_MASK,
			),
			_ => format!("{:03b}_{:05b}_{:024b}", &self.flags.bits >> 29, self.kind_bits(), self.flags.bits & L_MASK),
		};
		match self.kind() {
			Kind::Eof => write!(f, "Token::Eof {{ bits: {} }}", bits),
			Kind::Delim => {
				write!(
					f,
					"Token::Delim {{ bits: {}, char: {:?}, offset: {}, len: 1\n}}",
					bits,
					&self.char().unwrap(),
					&self.offset
				)
			}
			_ => {
				write!(
					f,
					"Token::{} {{ bits: {}, offset: {}, len: {} }}",
					&self.kind().as_str(),
					bits,
					&self.len(),
					&self.offset
				)
			}
		}
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum PairWise {
	Paren,
	Curly,
	Square,
}

impl PairWise {
	pub fn from_token(token: &Token) -> Option<Self> {
		match token.kind() {
			Kind::LeftParen | Kind::Function => Some(Self::Paren),
			Kind::LeftCurly => Some(Self::Curly),
			Kind::LeftSquare => Some(Self::Square),
			Kind::RightParen => Some(Self::Paren),
			Kind::RightCurly => Some(Self::Curly),
			Kind::RightSquare => Some(Self::Square),
			_ => None,
		}
	}

	pub fn start(&self) -> Kind {
		match self {
			Self::Paren => Kind::LeftParen,
			Self::Curly => Kind::LeftCurly,
			Self::Square => Kind::LeftSquare,
		}
	}

	pub fn end(&self) -> Kind {
		match self {
			Self::Paren => Kind::RightParen,
			Self::Curly => Kind::RightCurly,
			Self::Square => Kind::RightSquare,
		}
	}
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.kind() {
			Kind::Delim => write!(f, "Delim({})", self.char().unwrap()),
			k => write!(f, "{}", k.as_str()),
		}
	}
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for Token {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("Token", 3)?;
		state.serialize_field("kind", self.kind().as_str())?;
		state.serialize_field("offset", &self.offset)?;
		state.serialize_field("len", &self.len())?;
		state.end()
	}
}
