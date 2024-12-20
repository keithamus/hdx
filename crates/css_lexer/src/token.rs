use crate::{
	whitespace_style::WhitespaceStyle, CommentStyle, Cursor, DimensionUnit, Kind, KindSet, PairWise, QuoteStyle,
	SourceOffset,
};

// The `Token` type is an immutable packing of two u32s that represents a unit in the source text,
// but without the associated span (offset) data that points to its position in a text document.
// For the most part a Token doesn't represent data that can be put into a text editor because it
// lacks concepts such as a "value" (for example the `Ident` token just represents _an_ ident, but
// it doesn't retain what the keyword is).
//
// If you're familiar with "red green" syntax trees such as Swiftlang's libsyntax
// (https://gh.io/AAtdqpg), or Rust-Analyzer's Rowan (https://gh.io/AAtf8pt) or
// Roslyn (https://gh.io/AAtab90) this might be familiar.
//
// A Token retains certain "facts" about the underlying unit of text, though. For example it retains
// the `Kind` of a token, how many characters the token consumed, and various other facts depending
// on the token.
//
// In some cases, it's entirely possible to squeeze the "value" of the token into the this u32 (for
// example chars, or numbers) and this can speed up subsequent parsing and so it is worth taking the
// time in the tokenizer to gather facts and values to store them in the Tokens bits.
//
// This representation of facts, kind, length, or other metadata can be quite complex - so here's a
// full breakdown:
//
// Anatomy of Token
//
//   |-----|-------|--------------------------|---------------------------------|
//   | TF  | K     | VD                       | "Value"                         |
// 0b| 000 | 00000 | 000000000000000000000000 | 0000000000000000000000000000000 |
//   |-----|-------|--------------------------|---------------------------------|
//   | 3-- | 5---- | 24---------------------- | 32----------------------------- |
//
//   TF  = Type Flags.    If K is Number/Dimension:
//                          001 = Floating Point
//                          010 = Has Sign
//                        If K is Dimension:
//                          100 = Unit is a known dimension
//                        If K is String:
//                          001 = Uses Double Quotes (0 would be Single)
//                          010 = Has a closing quote
//                          100 = Contains escape characters
//                        If K is Ident, Function, AtKeyword:
//                          001 = Contains non-lower-ASCII (e.g. uppercase or unicode)
//                          010 = Is a "Dashed Ident" - starts with two dashes
//                          100 = Contains escape characters
//                        If K is Hash:
//                          001 = Contains non-lower-ASCII (e.g. uppercase or unicode)
//                          010 = First character is ascii, aka it's a "HashId"
//                          100 = Contains escape characters
//                        If K is Url:
//                          001 = Ended with a `)`
//                          010 = Contains whitespace after the `(`
//                          100 = Contains escape characters
//                        If K is CdcOrCdo
//                          000 = Is Cdo
//                          001 = Is Cdc
//                        If K is Whitespace this maps to the `WhitespaceStyle` enum which
//                        determines what kind of whitespace this is.
//                        If K is Comment this maps to the `CommentStyle` enum which determines
//                        the kind of comment this is.
//                        If K is Delim this maps to the length of the raw string, which might
//                        be different from the encoded delim given \0 and surrogates are replaced
//                        with \u{FFFD}
//   K   = Kind Flags.    Maps to `Kind` enum
//
//   VD  = Value Data.    Depending on the type of the token, this can represent different data.
//                        If K is Number, this represents the length of that number (this means
//                        number tokens cannot be longer than 16,777,216 characters which is
//                        probably an acceptable limit).
//                        if K is URL, this represents the "leading length" and "trailing length".
//                        This value is split into two 12 bit numbers.
//
//                        |--------------|--------------|
//                        | LL           | TL           |
//                        | 000000000000 | 000000000000 |
//                        |--------------|--------------|
//                        | 12---------- | 12---------- |
//
//                        This is because while most of the time this will be the literal `url(`,
//                        it can also be escaped, so `\u\r\l(` is also valid, as is `\\75\\52\\6c(`
//                        Additionally while the trailing end is usually `)`, it can also have a
//                        number of trailing whitespace, e.g. ` )` or `  )`.
//
//                        If K is a Dimension, then this is split further (see table just below).
//                        For all other kinds this is left reserved (zeroed out).
//
//                        |--------------|--------------|
//                        | NL           | DUL          |
//                        | 000000000000 | 000000000000 |
//                        |--------------|--------------|
//                        | 12---------- | 12---------- |
//
//                        When a dimension is parsed, the length data is split into 2 regions,
//                        one storing 12 bits representing the length of the number, and 1 storing
//                        the length of the dimension unit string. This means in this parser that
//                        Dimensions are limited; their numerical component can only be 4096
//                        characters long, same for their unit. However, in reality 99.99% of CSS
//                        in the wild is using one of the built-in dimension units, of which there
//                        exists a few dozen, the longest being 5 characters. If the flag
//                        TF&100 == 100 is set, this means that during the tokenization the unit
//                        was found to be a recognised unit (e.g. px, rem, dvw etc). In this case
//                        the DUL region instead stores a `DimensionUnit` enum token. This only
//                        accounts for unescaped tokens (e.g. `px` and not doing weird escaping
//                        like `p\u0078`).
//
//  Value                 If K is Delim, or one of the non-delim single single char, i.e.
//                        Colon->RightCurly then this is the packed `char`, as u32.
//                        If K is Number or Dimension this will be the f32.to_bits().
//                        In all other cases, this represents the length of the token as utf-8
//                        bytes. This means the token length is 4294967296 aka ~4GB. This sounds
//                        very long but also CSS can host very large image data and browsers will
//                        accomodate very large URLs.
//                        (https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs#common_problems)
//                        claims Firefox supports 32mb, Chrome over 512mb, and Safari over 2gb)
//                        the reality is that if someone has such a large data URL in their CSS
//                        they probably should split it out, but we have 32 bits to store the
//                        length so we may as well use it..
//
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token(u32, u32);

impl Default for Token {
	fn default() -> Self {
		Self((Kind::Whitespace as u32) << 24, 0)
	}
}

const KIND_MASK: u32 = !((1 << 24) - 1);
const LENGTH_MASK: u32 = (1 << 24) - 1;
const HALF_LENGTH_MASK: u32 = !((1 << 12) - 1);

impl Token {
	pub const EOF: Token = Token(0b0, 0);
	pub const CDO: Token = Token(((Kind::CdcOrCdo as u32) << 24) & KIND_MASK, 4);
	pub const CDC: Token = Token((((Kind::CdcOrCdo as u32) | 0b001_00000) << 24) & KIND_MASK, 3);
	pub const SPACE: Token =
		Token((((Kind::Whitespace as u32) | (WhitespaceStyle::Space as u32) << 5) << 24) & KIND_MASK, 1);
	pub const TAB: Token =
		Token((((Kind::Whitespace as u32) | (WhitespaceStyle::Tab as u32) << 5) << 24) & KIND_MASK, 1);
	pub const NEWLINE: Token =
		Token((((Kind::Whitespace as u32) | (WhitespaceStyle::Newline as u32) << 5) << 24) & KIND_MASK | (1 << 12), 1);
	pub const NUMBER_ZERO: Token = Token((((Kind::Number as u32) | 0b100_00000) << 24) & KIND_MASK, 1);
	pub const COLON: Token = Token((((Kind::Colon as u32) | 0b001_00000) << 24) & KIND_MASK, ':' as u32);
	pub const SEMICOLON: Token = Token((((Kind::Semicolon as u32) | 0b001_00000) << 24) & KIND_MASK, ';' as u32);
	pub const COMMA: Token = Token((((Kind::Comma as u32) | 0b001_00000) << 24) & KIND_MASK, ',' as u32);
	pub const LEFT_SQUARE: Token = Token((((Kind::LeftSquare as u32) | 0b001_00000) << 24) & KIND_MASK, '[' as u32);
	pub const RIGHT_SQUARE: Token = Token((((Kind::RightSquare as u32) | 0b001_00000) << 24) & KIND_MASK, ']' as u32);
	pub const LEFT_PAREN: Token = Token((((Kind::LeftParen as u32) | 0b001_00000) << 24) & KIND_MASK, '(' as u32);
	pub const RIGHT_PAREN: Token = Token((((Kind::RightParen as u32) | 0b001_00000) << 24) & KIND_MASK, ')' as u32);
	pub const LEFT_CURLY: Token = Token((((Kind::LeftCurly as u32) | 0b001_00000) << 24) & KIND_MASK, '{' as u32);
	pub const RIGHT_CURLY: Token = Token((((Kind::RightCurly as u32) | 0b001_00000) << 24) & KIND_MASK, '}' as u32);
	// Comon delims
	pub const BANG: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '!' as u32);
	pub const HASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '#' as u32);
	pub const DOLLAR: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '$' as u32);
	pub const PERCENT: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '%' as u32);
	pub const AMPERSAND: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '&' as u32);
	pub const ASTERISK: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '*' as u32);
	pub const PLUS: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '+' as u32);
	pub const DASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '-' as u32);
	pub const PERIOD: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '.' as u32);
	pub const SLASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '/' as u32);
	pub const LESS_THAN: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '<' as u32);
	pub const EQUALS: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '=' as u32);
	pub const GREATER_THAN: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '>' as u32);
	pub const QUESTION: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '?' as u32);
	pub const AT: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '@' as u32);
	pub const BACKSLASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '\\' as u32);
	pub const CARET: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '^' as u32);
	pub const UNDERSCORE: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '_' as u32);
	pub const BACKTICK: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '`' as u32);
	pub const PIPE: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '|' as u32);
	pub const TILDE: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '~' as u32);

	#[inline]
	pub fn call_site(kind: Kind) -> Self {
		Self((kind as u32) << 24, 0)
	}

	#[inline]
	pub fn new_whitespace(style: WhitespaceStyle, len: u32) -> Self {
		let flags: u32 = Kind::Whitespace as u32 | ((style as u32) << 5);
		Self((flags << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_comment(style: CommentStyle, len: u32) -> Self {
		let flags: u32 = Kind::Comment as u32 | ((style as u32) << 5);
		Self((flags << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_number(is_float: bool, has_sign: bool, len: u32, value: f32) -> Self {
		let flags: u32 = Kind::Number as u32 | ((is_float as u32) << 5) | ((has_sign as u32) << 6);
		Self((flags << 24) & KIND_MASK | (len & LENGTH_MASK), value.to_bits())
	}

	#[inline]
	pub fn new_dimension(
		is_float: bool,
		has_sign: bool,
		num_len: u32,
		unit_len: u32,
		value: f32,
		unit: DimensionUnit,
	) -> Self {
		debug_assert!(num_len <= 4097);
		let num_len = (num_len << 12) & HALF_LENGTH_MASK;
		let (is_known_unit, known_or_len) =
			if unit == DimensionUnit::Unknown { (0, unit_len) } else { (0b100_00000, unit as u32) };
		let flags: u32 = Kind::Dimension as u32 | is_known_unit | ((is_float as u32) << 5) | ((has_sign as u32) << 6);
		Self(((flags << 24) & KIND_MASK) | ((num_len | known_or_len) & LENGTH_MASK), value.to_bits())
	}

	#[inline]
	pub fn new_bad_string(len: u32) -> Self {
		Self(((Kind::BadString as u32) << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_bad_url(len: u32) -> Self {
		Self(((Kind::BadUrl as u32) << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_ident(contains_non_lower_ascii: bool, dashed: bool, contains_escape: bool, len: u32) -> Self {
		let flags: u32 = Kind::Ident as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((dashed as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_function(contains_non_lower_ascii: bool, dashed: bool, contains_escape: bool, len: u32) -> Self {
		let flags: u32 = Kind::Function as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((dashed as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_atkeyword(contains_non_lower_ascii: bool, dashed: bool, contains_escape: bool, len: u32) -> Self {
		let flags: u32 = Kind::AtKeyword as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((dashed as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_hash(contains_non_lower_ascii: bool, first_is_ascii: bool, contains_escape: bool, len: u32) -> Self {
		let flags: u32 = Kind::Hash as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((first_is_ascii as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_string(quotes: QuoteStyle, has_close_quote: bool, contains_escape: bool, len: u32) -> Self {
		debug_assert!(quotes != QuoteStyle::None);
		let quotes = if quotes == QuoteStyle::Double { 0b001_00000 } else { 0b0 };
		let flags: u32 =
			Kind::String as u32 | quotes | ((has_close_quote as u32) << 6) | ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	#[inline]
	pub fn new_url(
		ends_with_paren: bool,
		contains_whitespace_after_open_paren: bool,
		contains_escape: bool,
		leading_length: u32,
		trailing_length: u32,
		len: u32,
	) -> Self {
		let leading_length = (leading_length << 12) & HALF_LENGTH_MASK;
		let flags: u32 = Kind::Url as u32
			| ((ends_with_paren as u32) << 5)
			| ((contains_whitespace_after_open_paren as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK | ((leading_length | trailing_length) & LENGTH_MASK), len)
	}

	#[inline]
	pub fn new_delim(char: char) -> Self {
		let len = char.len_utf8() as u32;
		debug_assert!(len <= 7);
		let flags: u32 = Kind::Delim as u32 | (len << 5);
		Self((flags << 24) & KIND_MASK | (len & LENGTH_MASK), char as u32)
	}

	#[inline(always)]
	pub(crate) const fn kind_bits(&self) -> u8 {
		(self.0 >> 24 & 0b11111) as u8
	}

	#[inline]
	pub const fn kind(&self) -> Kind {
		Kind::from_bits(self.kind_bits())
	}

	#[inline(always)]
	const fn first_bit_is_set(&self) -> bool {
		self.0 >> 31 == 1
	}

	#[inline(always)]
	const fn second_bit_is_set(&self) -> bool {
		self.0 >> 30 & 0b1 == 1
	}

	#[inline(always)]
	const fn third_bit_is_set(&self) -> bool {
		self.0 >> 29 & 0b1 == 1
	}

	#[inline(always)]
	pub const fn is_ident_like(&self) -> bool {
		self.kind_bits() & 0b11000 == 0b01000 && self.kind_bits() != Kind::String as u8
	}

	#[inline(always)]
	pub const fn is_delim_like(&self) -> bool {
		self.kind_bits() & 0b10000 == 0b10000
	}

	// The only token with an empty length is EOF
	pub const fn is_empty(&self) -> bool {
		self.kind_bits() == Kind::Eof as u8
	}

	pub fn len(&self) -> u32 {
		if self.kind_bits() == Kind::Eof as u8 {
			debug_assert!(self.kind() == Kind::Eof);
			0
		} else if self.is_delim_like() {
			debug_assert!(matches!(
				self.kind(),
				Kind::Delim
					| Kind::Colon | Kind::Semicolon
					| Kind::Comma | Kind::LeftSquare
					| Kind::RightSquare
					| Kind::LeftParen
					| Kind::RightParen
					| Kind::LeftCurly
					| Kind::RightCurly
			));
			self.0 >> 29
		} else if self.kind_bits() == Kind::Number as u8 {
			debug_assert!(self.kind() == Kind::Number);
			self.numeric_len()
		} else if self.kind_bits() == Kind::Dimension as u8 {
			debug_assert!(self.kind() == Kind::Dimension);
			if self.first_bit_is_set() {
				self.numeric_len() + self.dimension_unit().len()
			} else {
				((self.0 & LENGTH_MASK) >> 12) + (self.0 & !HALF_LENGTH_MASK)
			}
		} else {
			self.1
		}
	}

	pub fn char(&self) -> Option<char> {
		if self.is_delim_like() {
			return char::from_u32(self.1);
		}
		None
	}

	// Number Style Checks
	#[inline]
	pub const fn is_int(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && !self.third_bit_is_set()
	}

	#[inline]
	pub const fn is_float(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && self.third_bit_is_set()
	}

	#[inline]
	pub const fn has_sign(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && self.second_bit_is_set()
	}

	#[inline]
	pub const fn numeric_len(&self) -> u32 {
		debug_assert!(matches!(self.kind(), Kind::Number | Kind::Dimension));
		if self.kind_bits() == Kind::Dimension as u8 {
			(self.0 & LENGTH_MASK) >> 12
		} else if self.first_bit_is_set() {
			(self.0 & LENGTH_MASK) >> 16
		} else {
			self.0 & LENGTH_MASK
		}
	}

	#[inline]
	pub const fn value(&self) -> f32 {
		f32::from_bits(self.1)
	}

	// Whitespace style checks
	#[inline]
	pub const fn whitespace_style(&self) -> WhitespaceStyle {
		if self.kind_bits() == Kind::Whitespace as u8 {
			WhitespaceStyle::from_bits((self.0 >> 29) as u8)
		} else {
			WhitespaceStyle::None
		}
	}

	// Comment style checks
	#[inline]
	pub fn comment_style(&self) -> Option<CommentStyle> {
		if self.kind_bits() == Kind::Comment as u8 {
			CommentStyle::from_bits((self.0 >> 29) as u8)
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
			let unit_bits = (self.0 & 0b111111) as u8;
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
	pub const fn string_has_closing_quote(&self) -> bool {
		self.kind_bits() == Kind::String as u8 && self.second_bit_is_set()
	}

	// Escape style checks
	#[inline]
	pub const fn can_escape(&self) -> bool {
		self.kind_bits() == Kind::String as u8 || self.kind_bits() == Kind::Dimension as u8 || self.is_ident_like()
	}

	#[inline]
	pub const fn contains_escape_chars(&self) -> bool {
		if self.kind_bits() == Kind::Dimension as u8 {
			return !self.first_bit_is_set();
		}
		self.can_escape() && self.first_bit_is_set()
	}

	// Ident style checks
	#[inline]
	pub const fn is_dashed_ident(&self) -> bool {
		self.is_ident_like() && self.second_bit_is_set()
	}

	#[inline]
	pub const fn is_lower_case(&self) -> bool {
		self.is_ident_like() && !self.third_bit_is_set()
	}

	#[inline]
	pub const fn is_trivia(&self) -> bool {
		self.kind_bits() & 0b000011 == self.kind_bits()
	}

	// Url style checks
	#[inline]
	pub const fn url_has_leading_space(&self) -> bool {
		self.kind_bits() == Kind::Url as u8 && self.second_bit_is_set()
	}

	#[inline]
	pub const fn url_has_closing_paren(&self) -> bool {
		self.kind_bits() == Kind::Url as u8 && self.third_bit_is_set()
	}

	// Whitespace/Comment Style checks
	#[inline]
	pub const fn contains_newline(&self) -> bool {
		(self.kind_bits() == Kind::Whitespace as u8 || self.kind_bits() == Kind::Comment as u8)
			&& self.second_bit_is_set()
	}

	#[inline]
	pub const fn contains_tab(&self) -> bool {
		(self.kind_bits() == Kind::Whitespace as u8 || self.kind_bits() == Kind::Comment as u8)
			&& self.third_bit_is_set()
	}

	#[inline]
	pub const fn hash_is_id_like(&self) -> bool {
		(self.kind_bits() == Kind::Hash as u8) && self.second_bit_is_set()
	}

	#[inline]
	pub const fn is_bad(&self) -> bool {
		(self.kind_bits() | 0b00001) & 0b11001 == 1
	}

	#[inline]
	pub const fn is_cdc(&self) -> bool {
		self.kind_bits() == (Kind::CdcOrCdo as u8) && self.third_bit_is_set()
	}

	pub fn get_leading_len(&self) -> u32 {
		match self.kind() {
			Kind::AtKeyword | Kind::Hash | Kind::String => 1,
			Kind::Dimension => self.numeric_len(),
			Kind::Comment => 2,
			Kind::Url => (self.0 & LENGTH_MASK) >> 12,
			_ => 0,
		}
	}

	pub fn get_trailing_len(&self) -> u32 {
		match self.kind() {
			Kind::Function => 1,
			Kind::String => self.string_has_closing_quote() as u32,
			Kind::Comment if self.comment_style().unwrap().is_block() => 2,
			Kind::Url => self.0 & !HALF_LENGTH_MASK,
			_ => 0,
		}
	}

	#[inline]
	pub fn to_pairwise(&self) -> Option<PairWise> {
		PairWise::from_token(self)
	}

	#[inline]
	pub fn with_cursor(self, offset: SourceOffset) -> Cursor {
		Cursor::new(offset, self)
	}
}

impl core::fmt::Debug for Token {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut d = f.debug_struct(format!("Token::{}", self.kind().as_str()).as_str());
		match self.kind() {
			Kind::Eof => &mut d,
			Kind::Number => d.field("value", &self.value()).field("len", &self.numeric_len()),
			Kind::Dimension => d
				.field("value", &self.value())
				.field("len", &self.numeric_len())
				.field("dimension", &self.dimension_unit())
				.field("dimension_len", &self.len()),
			_ if self.is_delim_like() => d.field("char", &self.char().unwrap()).field("len", &(self.0 >> 29)),
			_ => d
				.field("flag_0", &self.first_bit_is_set())
				.field("flag_1", &self.second_bit_is_set())
				.field("flag_2", &self.third_bit_is_set())
				.field("len", &self.len()),
		}
		.finish()
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
		use serde::ser::SerializeStruct;
		let mut state = serializer.serialize_struct("Token", 3)?;
		state.serialize_field("kind", self.kind().as_str())?;
		state.serialize_field("len", &self.len())?;
		if self.kind_bits() == Kind::Dimension as u8 {
			state.serialize_field("unit", &self.dimension_unit())?;
		}
		state.end()
	}
}

impl From<Token> for Kind {
	fn from(token: Token) -> Self {
		token.kind()
	}
}

impl PartialEq<Kind> for Token {
	fn eq(&self, other: &Kind) -> bool {
		self.kind_bits() == *other as u8
	}
}

impl From<Token> for KindSet {
	fn from(token: Token) -> Self {
		KindSet::new(&[token.kind()])
	}
}

impl PartialEq<KindSet> for Token {
	fn eq(&self, other: &KindSet) -> bool {
		other.contains_bits(self.kind_bits())
	}
}

impl From<Token> for QuoteStyle {
	fn from(token: Token) -> Self {
		token.quote_style()
	}
}

impl PartialEq<QuoteStyle> for Token {
	fn eq(&self, other: &QuoteStyle) -> bool {
		&self.quote_style() == other
	}
}

impl From<Token> for WhitespaceStyle {
	fn from(token: Token) -> Self {
		token.whitespace_style()
	}
}

impl PartialEq<WhitespaceStyle> for Token {
	fn eq(&self, other: &WhitespaceStyle) -> bool {
		self.whitespace_style() == *other
	}
}

impl PartialEq<CommentStyle> for Token {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.comment_style().map(|style| &style == other).unwrap_or(false)
	}
}

impl PartialEq<char> for Token {
	fn eq(&self, other: &char) -> bool {
		self.char().map(|char| char == *other).unwrap_or(false)
	}
}

impl From<Token> for DimensionUnit {
	fn from(token: Token) -> Self {
		token.dimension_unit()
	}
}

impl PartialEq<DimensionUnit> for Token {
	fn eq(&self, other: &DimensionUnit) -> bool {
		self.dimension_unit() == *other
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Token>(), 8);
}

#[test]
fn test_new_whitespace() {
	assert_eq!(Token::SPACE, Kind::Whitespace);
	assert_eq!(Token::SPACE, WhitespaceStyle::Space);
	assert_eq!(Token::TAB, Kind::Whitespace);
	assert_eq!(Token::TAB, WhitespaceStyle::Tab);
	assert_eq!(Token::NEWLINE, Kind::Whitespace);
	assert_eq!(Token::NEWLINE, WhitespaceStyle::Newline);
	assert_eq!(Token::new_whitespace(WhitespaceStyle::Space, 4), Kind::Whitespace);
	assert_eq!(Token::new_whitespace(WhitespaceStyle::Space, 4), WhitespaceStyle::Space);
	assert_eq!(Token::new_whitespace(WhitespaceStyle::Space, 4).len(), 4);
	assert_eq!(Token::new_whitespace(WhitespaceStyle::Tab, 4), WhitespaceStyle::Tab);
	assert_eq!(Token::new_whitespace(WhitespaceStyle::Newline, 4), WhitespaceStyle::Newline);
	assert_eq!(Token::new_whitespace(WhitespaceStyle::Newline, 4).len(), 4);
	assert_eq!(Token::new_whitespace(WhitespaceStyle::NewlineUsingCarriageReturn, 4).len(), 4);
}

#[test]
fn test_new_comment() {
	assert_eq!(Token::new_comment(CommentStyle::Block, 4), Kind::Comment);
	assert_eq!(Token::new_comment(CommentStyle::Block, 4), CommentStyle::Block);
	assert_eq!(Token::new_comment(CommentStyle::Single, 4), CommentStyle::Single);
}

#[test]
fn test_new_number() {
	assert_eq!(Token::new_number(false, false, 3, 4.2), Kind::Number);
	assert_eq!(Token::new_number(false, false, 3, 4.2).value(), 4.2);
	assert_eq!(Token::new_number(false, false, 3, 4.2).len(), 3);
	assert_eq!(Token::new_number(false, true, 9, 4.2), Kind::Number);
	assert_eq!(Token::new_number(false, true, 9, 4.2).value(), 4.2);
	assert_eq!(Token::new_number(false, true, 9, 4.2).len(), 9);
	assert!(!Token::new_number(false, false, 3, 4.2).has_sign());
	assert!(Token::new_number(false, true, 3, 4.2).has_sign());
	assert!(!Token::new_number(false, true, 3, 4.0).is_float());
	assert!(Token::new_number(true, false, 3, 4.2).is_float());
}

#[test]
fn test_new_dimension() {
	{
		let token = Token::new_dimension(false, false, 3, 3, 999.0, DimensionUnit::Rad);
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.value(), 999.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Rad);
		assert_eq!(token.numeric_len(), 3);
		assert_eq!(token.len(), 6);
		assert!(!token.is_float());
		assert!(!token.has_sign());
	}
	{
		let token = Token::new_dimension(false, false, 5, 2, 8191.0, DimensionUnit::Px);
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.value(), 8191.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Px);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 7);
		assert!(!token.is_float());
		assert!(!token.has_sign());
	}
	for i in -8191..8191 {
		let token = Token::new_dimension(false, false, 9, 3, i as f32, DimensionUnit::Rem);
		assert_eq!(token.value(), i as f32);
	}
}
