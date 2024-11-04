use bitmask_enum::bitmask;
#[cfg(feature = "serde")]
use serde::ser::SerializeStruct;

use crate::{DimensionUnit, Include, Span};

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

	// Variable length Atom containing Tokens (mask: 0b0_1XXX)
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
	pub fn from_bits(bits: u8) -> Self {
		match bits {
			0b0001 => Self::Whitespace,
			0b0010 => Self::Comment,
			0b0011 => Self::CdcOrCdo,
			0b0100 => Self::Number,
			0b0101 => Self::Dimension,
			0b0110 => Self::BadString,
			0b0111 => Self::BadUrl,
			0b1000 => Self::Ident,
			0b1001 => Self::Function,
			0b1010 => Self::AtKeyword,
			0b1011 => Self::Hash,
			0b1100 => Self::String,
			0b1101 => Self::Url,
			0b1_0000 => Self::Delim,
			0b1_0001 => Self::Colon,
			0b1_0010 => Self::Semicolon,
			0b1_0011 => Self::Comma,
			0b1_0100 => Self::LeftSquare,
			0b1_0101 => Self::RightSquare,
			0b1_0110 => Self::LeftParen,
			0b1_0111 => Self::RightParen,
			0b1_1000 => Self::LeftCurly,
			0b1_1001 => Self::RightCurly,
			_ => Self::Eof,
		}
	}

	pub fn as_str(&self) -> &str {
		match *self {
			Kind::Eof => "Eof",
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
		}
	}
}

impl core::fmt::Debug for Kind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Kind::{}", self.as_str())
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
	//                          010 = Has Sign
	//                        If K is Number:
	//                          100 = Number was small enough to fit into NL region
	//                        If K is Dimension:
	//                          100 = Number and Unit small enough to fit into NL region
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
	//                        If K is Delim this maps to the length of the raw string, which might
	//                        be different from the encoded delim given \0 and surrogates are replaced
	//                        with \u{FFFD}
	//   K   = Kind Flags.    Maps to `Kind` enum
	//   L   = Lengthdata.    If K is Delim then this is `char` (see C below)
	//                        If K is Number/Dimension then this is split further (see NL below)
	//                        If K is a non-delim single char, i.e. Colon->RightCurly then this is
	//                        `char` (see C below).
	//                        If K is Whitespace, Comment, String, Url, Ident, Function, AtKeyword,
	//                        then this is the length of the token's character count in source, as
	//                        24-bits. This means the token length is 16777216 aka ~16MB. This sounds
	//                        very long but also CSS can host very large image data URLs. 16MB seems
	//                        like a good limitation for this parser though; while browsers need to
	//                        accomodate much larger data URLs (https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs#common_problems
	//                        claims Firefox supports 32mb, Chrome over 512mb, and Safari over 2gb)
	//                        the reality is that if someone has such a large data URL in their CSS
	//                        they probably should split it out.
	//
	//   C   = Char.          This is `char` type. All available `chars` in unicode can be
	//                        represented in 24bits as the upper bits are for surrogates.
	//   NL  = Number Length. If the Kind is a Number then this may be broken down into further
	//                        segments. In the basic case, where TF&100 == 000, the 24 bits
	//                        represents the length of the number in the source. If, however,
	//                        TF&100 == 100 then the number was considered "small" enough to
	//                        fit into an i16, and so the 24-bits is broken into an 8 and a 16 bit
	//                        segment:
	//
	//     |----------|------------------|
	//     | NL       | PACK             |
	//     | 00000000 | 0000000000000100 |
	//     |----------|------------------|
	//     | 8------- | 16-------------- |
	//
	//     NL = Number Length The first 8 bits are to store the character length of the number.
	//                        In practice means these numeric values can only be 255 codepoints long
	//                        (it's possible to make a number representing an f16 that is longer than
	//                        255 codepoints (for example more than 255 zero characters), but in
	//                        practice this makes for nonsensical representations of numbers, and in
	//                        those cases the tokenizer will not set the TF&100 flag, so they'll go
	//                        down the "slow" path when parsed.
	//
	//     PACK = f16 value   If TF&100 == 100, then during tokenization it was possible to parse the
	//                        number as f16 and stuff it into the remaining 16 bit space, meaning when
	//                        re-parsing the number, it doesn't need to be read from the source string.
	//                        If TF&100 == 000 then this wasn't the case, and instead this space is
	//                        just 0s. The _majority_ of numbers used in CSS are indeed less than f16
	//                        (±65,504) and so it maes sense to store them here if possible.
	//
	//                        If thie Kind is a Dimension then this is broken into further segments.
	//                        segments. In the basic case, where TF&100 == 000, the 24 bits
	//                        represents is broken into 2 12-bit lengths representing the Number
	//                        Length and the Dimension Unit length. This means dimensions can only
	//                        have a maximum numerical length of 4095 characters, and the dimension
	//                        unit also has a maximum numerical length of 4095 characters.
	//
	//     |--------------|--------------|
	//     | NL           | DUL          |
	//     | 000000000000 | 000000000000 |
	//     |--------------|--------------|
	//     | 12---------- | 12---------- |
	//
	//                        In reality 99.99% of CSS in the wild is using one of the built-in
	//                        dimension units, of which there exists a few dozen. Also the vast
	//                        majority of dimensions are small values, (e.g 4rem, 1024px). If the flag
	//                        TF&100 == 100 is set, this means that during the tokenization the unit
	//                        was found to be a recognised unit (e.g. px, rem, dvw etc), _and_ that the
	//                        number was small enough to stuff to represent in the remaining memory.
	//                        And so the packing for this is as follows:
	//
	//     |------|--------|---|---------------|
	//     | NL   | D      | + |               |
	//     | 0000 | 000000 | 0 | 0000000000000 |
	//     |------|--------|---|---------------|
	//     | 4--- | 6----- | 1 | 13----------- |
	//
	//		 NL = Number Length The first 4 bits represent the number length. This is dramatically shorter
	//		                    than number's 8 bit character width, which can store 255 characters; this
	//		                    only stores 15-character numbers. However the _also_ shorter 13 bits for
	//		                    the value storage means that this can store numbers up to ±8191. The sign
	//		                    bit makes up for an equivalent "int14". The last 6 bits here are to store
	//		                    the known dimension unit, provided the source represents this in the ascii
	//		                    format (e.g. `px` and not doing weird escaping like `p\u0078`). This is
	//		                    enough to represent the majority of CSS dimensions seen in the wild and
	//		                    still be able to point to the correct offset/length within the source.
}

impl Default for TokenFlags {
	fn default() -> Self {
		Self { bits: (Kind::Whitespace as u32) << 24 }
	}
}

#[derive(Copy, Clone, PartialEq, Default, Hash)]
pub struct Token {
	flags: TokenFlags,
	offset: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "kind", content = "value"))]
pub enum QuoteStyle {
	// Some tokens/ast nodesthat would otherwise be strings (e.g. url(), named fonts) can have no quotes.
	None,
	Single,
	#[default]
	Double,
}

static TYPEFLAG_MASK: u32 = !((1 << 29) - 1);
static KIND_MASK: u32 = !((1 << 24) - 1);
static LENGTH_MASK: u32 = (1 << 24) - 1;
static KNOWN_DIMENSION_NUMBER_LENGTH_MASK: u32 = !((1 << 20) - 1);
static NUMBER_PACK_MASK: u32 = !((1 << 15) - 1);
static KNOWN_DIMENSION_NUMBER_PACK_MASK: u32 = !((1 << 13) - 1);
static DIMENSION_UNIT_LENGTH_MASK: u32 = !((1 << 12) - 1);

impl Token {
	pub fn new(kind: Kind, type_flags: u8, offset: u32, len: u32) -> Self {
		let flags = TokenFlags {
			bits: (((type_flags as u32) << 29) & TYPEFLAG_MASK)
				| (((kind as u32) << 24) & KIND_MASK)
				| (len & LENGTH_MASK),
		};
		Self { flags, offset }
	}

	pub fn new_number(type_flags: u8, offset: u32, num_len: u32, value: i32) -> Self {
		let mut type_flags = type_flags;
		let pack_data = if type_flags & 0b100 == 0b100 && num_len <= 255 && (-32767..=32767).contains(&value) {
			((num_len << 16) & NUMBER_PACK_MASK)
				| ((value.is_positive() as u32) << 15 & NUMBER_PACK_MASK)
				| value.unsigned_abs()
		} else {
			type_flags &= 0b011;
			num_len & LENGTH_MASK
		};
		let flags = TokenFlags {
			bits: (((type_flags as u32) << 29) & TYPEFLAG_MASK)
				| (((Kind::Number as u8 as u32) << 24) & KIND_MASK)
				| pack_data,
		};
		Self { flags, offset }
	}

	pub fn new_dimension(
		type_flags: u8,
		offset: u32,
		num_len: u32,
		unit_len: u32,
		value: i32,
		known_unit: DimensionUnit,
	) -> Self {
		let mut type_flags = type_flags;
		let pack_data = if type_flags & 0b100 == 0b100
			&& num_len <= 15
			&& (-8191..=8191).contains(&value)
			&& known_unit != DimensionUnit::Unknown
		{
			let num_len = (num_len << 20) & KNOWN_DIMENSION_NUMBER_LENGTH_MASK;
			let known_unit = ((known_unit as u32) << 14) & KNOWN_DIMENSION_NUMBER_PACK_MASK;
			let sign = ((!value.is_negative() as u32) << 13) & KNOWN_DIMENSION_NUMBER_PACK_MASK;
			num_len | known_unit | sign | value.unsigned_abs()
		} else {
			type_flags &= 0b011;
			let num_len = (num_len << 12) & DIMENSION_UNIT_LENGTH_MASK;
			let unit_len = unit_len & !DIMENSION_UNIT_LENGTH_MASK;
			(num_len | unit_len) & LENGTH_MASK
		};
		let flags = TokenFlags {
			bits: (((type_flags as u32) << 29) & TYPEFLAG_MASK)
				| (((Kind::Dimension as u8 as u32) << 24) & KIND_MASK)
				| pack_data,
		};
		Self { flags, offset }
	}

	#[inline(always)]
	pub fn offset(&self) -> u32 {
		self.offset
	}

	#[inline(always)]
	pub fn end_offset(&self) -> u32 {
		self.offset + self.len()
	}

	#[inline(always)]
	pub fn span(&self) -> Span {
		Span::new(self.offset(), self.end_offset())
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
	pub fn is_ident_like(&self) -> bool {
		self.kind_bits() & 0b11000 == 0b01000 && self.kind_bits() != Kind::String as u8
	}

	#[inline]
	pub fn kind(&self) -> Kind {
		Kind::from_bits(self.kind_bits())
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn len(&self) -> u32 {
		if self.kind_bits() == Kind::Eof as u8 {
			debug_assert!(self.kind() == Kind::Eof);
			0
		} else if self.kind_bits() == Kind::Delim as u8 {
			debug_assert!(self.kind() == Kind::Delim);
			self.flags.bits >> 29
		// Delim-like flag is set
		} else if self.kind_bits() & 0b10000 == 0b10000 {
			debug_assert!(matches!(
				self.kind(),
				Kind::Colon
					| Kind::Semicolon
					| Kind::Comma | Kind::LeftSquare
					| Kind::RightSquare
					| Kind::LeftParen
					| Kind::RightParen
					| Kind::LeftCurly
					| Kind::RightCurly
			));
			1
		} else if self.kind_bits() == Kind::CdcOrCdo as u8 {
			debug_assert!(self.kind() == Kind::CdcOrCdo);
			4 - (self.third_bit_is_set() as u32)
		} else if self.kind_bits() == Kind::Number as u8 {
			debug_assert!(self.kind() == Kind::Number);
			self.numeric_len()
		} else if self.kind_bits() == Kind::Dimension as u8 {
			debug_assert!(self.kind() == Kind::Dimension);
			if self.first_bit_is_set() {
				self.numeric_len() + self.dimension_unit().len()
			} else {
				((self.flags.bits & LENGTH_MASK) >> 12) + (self.flags.bits & !DIMENSION_UNIT_LENGTH_MASK)
			}
		} else {
			self.flags.bits & LENGTH_MASK
		}
	}

	pub fn char(&self) -> Option<char> {
		// Delim flag is set
		if self.kind_bits() & 0b10000 == 0b10000 {
			return char::from_u32(self.flags.bits & LENGTH_MASK);
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
		debug_assert!(matches!(self.kind(), Kind::Number | Kind::Dimension));
		if self.kind_bits() == Kind::Dimension as u8 {
			if self.first_bit_is_set() {
				(self.flags.bits & LENGTH_MASK) >> 20
			} else {
				(self.flags.bits & LENGTH_MASK) >> 12
			}
		} else if self.first_bit_is_set() {
			(self.flags.bits & LENGTH_MASK) >> 16
		} else {
			self.flags.bits & LENGTH_MASK
		}
	}

	#[inline]
	pub fn stored_small_number(&self) -> Option<f32> {
		if !self.first_bit_is_set() {
			None
		} else if self.kind_bits() == Kind::Number as u8 {
			Some(if ((self.flags.bits >> 15) & 0b1) == 1 {
				(self.flags.bits & !NUMBER_PACK_MASK) as f32
			} else {
				-((self.flags.bits & !NUMBER_PACK_MASK) as f32)
			})
		} else if self.kind_bits() == Kind::Dimension as u8 {
			Some(if (self.flags.bits >> 13 & 0b1) == 1 {
				let bits = self.flags.bits & !KNOWN_DIMENSION_NUMBER_PACK_MASK;
				bits as f32
			} else {
				-((self.flags.bits & !KNOWN_DIMENSION_NUMBER_PACK_MASK) as f32)
			})
		} else {
			None
		}
	}

	// Dimension style checks
	#[inline]
	pub fn dimension_unit(&self) -> DimensionUnit {
		if !self.first_bit_is_set() || self.kind_bits() != Kind::Dimension as u8 {
			DimensionUnit::Unknown
		} else {
			let unit_bits = (self.flags.bits >> 14 & 0b111111) as u8;
			unit_bits.into()
		}
	}

	// String style checks
	#[inline]
	pub fn quote_style(&self) -> QuoteStyle {
		if self.kind_bits() == Kind::String as u8 {
			if self.third_bit_is_set() {
				return QuoteStyle::Double;
			} else if self.second_bit_is_set() {
				return QuoteStyle::Single;
			}
		}
		QuoteStyle::None
	}

	#[inline]
	pub fn string_has_closing_quote(&self) -> bool {
		self.kind_bits() == Kind::String as u8 && self.second_bit_is_set()
	}

	// Escape style checks
	#[inline]
	pub fn can_escape(&self) -> bool {
		self.kind_bits() == Kind::String as u8 || self.kind_bits() == Kind::Dimension as u8 || self.is_ident_like()
	}

	#[inline]
	pub fn contains_escape_chars(&self) -> bool {
		if self.kind_bits() == Kind::Dimension as u8 {
			return !self.first_bit_is_set();
		}
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
		self.kind_bits() == Kind::Url as u8 && self.second_bit_is_set()
	}

	#[inline]
	pub fn url_has_closing_paren(&self) -> bool {
		self.kind_bits() == Kind::Url as u8 && self.third_bit_is_set()
	}

	// Whitespace/Comment Style checks
	#[inline]
	pub fn contains_newline(&self) -> bool {
		(self.kind_bits() == Kind::Whitespace as u8 || self.kind_bits() == Kind::Comment as u8)
			&& self.second_bit_is_set()
	}

	#[inline]
	pub fn contains_tab(&self) -> bool {
		(self.kind_bits() == Kind::Whitespace as u8 || self.kind_bits() == Kind::Comment as u8)
			&& self.third_bit_is_set()
	}

	#[inline]
	pub fn hash_is_id_like(&self) -> bool {
		(self.kind_bits() == Kind::Hash as u8) && self.second_bit_is_set()
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
				(self.flags.bits & LENGTH_MASK) >> 16,
				self.flags.bits & DIMENSION_UNIT_LENGTH_MASK,
			),
			_ => format!(
				"{:03b}_{:05b}_{:024b}",
				&self.flags.bits >> 29,
				self.kind_bits(),
				self.flags.bits & LENGTH_MASK
			),
		};
		match self.kind() {
			Kind::Eof => write!(f, "Token::Eof {{ bits: {}, offset: {} }}", bits, &self.offset),
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
					&self.offset,
					&self.len()
				)
			}
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
		S: serde::ser::Serializer,
	{
		let mut state = serializer.serialize_struct("Token", 3)?;
		state.serialize_field("kind", self.kind().as_str())?;
		state.serialize_field("offset", &self.offset)?;
		state.serialize_field("len", &self.len())?;
		if self.kind_bits() == Kind::Dimension as u8 {
			state.serialize_field("unit", &self.dimension_unit())?;
		}
		state.end()
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
