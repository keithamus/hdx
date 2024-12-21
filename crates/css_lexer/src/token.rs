use crate::{
	whitespace_style::Whitespace, CommentStyle, Cursor, DimensionUnit, Kind, KindSet, PairWise, QuoteStyle,
	SourceOffset,
};

/// An abstract representation of the chunk of the source text, retaining certain "facts" about the source.
///
/// # Design
///
/// The [Token] type is an immutable packing of two [u32s][u32] that represents a unit in the source text, but without
/// the associated offset data that points to its position in the source text. This is important because it means that
/// equivalent [Tokens][Token] are equal even in different parts of the document. For the most part a [Token] doesn't
/// represent data that can be put into a text file because it lacks the underlying character data. It is lossy. For
/// example a [Token] with [Kind::Ident] just represents _an_ ident, but it doesn't retain what the keyword is).
/// Storing raw-character data would require either storing tokens on the heap (and therefore they couldn't be [Sized])
/// or by keeping a reference to `&'a str` which means larger token sizes and lifetime tracking. By _not_ storing
/// character data we can keep [Token] [Sized] and keep it to `size_of` `8`, avoiding the heap, avoiding
/// references/lifetimes, and keeping [Token] entirely in the stack. For a lot of tokens this is _fine_ because the
/// underlying character data isn't that useful past a certain point.
///
/// A [Token] retains certain "facts" about the underlying unit of text, though. For example it retains the [Kind], how
/// many characters the token consumed, and various other pieces of information, depending on the [Kind]. In some
/// cases, it's entirely possible to represent the full token, including character data, into the available bits (for
/// example [Kind::Delim] stores its [char], [Kind::Number] stores its [f32]). Taking the time in the tokenizer to
/// gather these facts and values can keep cache-lines hot, which speeds up subsequent checks in the parser.
///
/// If you're familiar with "red green" syntax trees such as [Swiftlang's libsyntax][1], or [Rust-Analyzer's Rowan][2]
/// or [Roslyn][3] this might be a little familiar in some concepts. However [Token] does not represent a tree, and
/// relies on resorting back to the string data to find out keyword values.
///
/// [1]: https://gh.io/AAtdqpg
/// [2]: https://gh.io/AAtf8pt
/// [3]: https://gh.io/AAtab90
///
/// This representation of facts, kind, length, or other metadata can be quite complex - so here's a
/// full breakdown:
///
/// # Anatomy of Token
///
/// A [Token] is a struct of `(u32, u32)`. The second u32 is _usually_ the token length (hence keeping them separate).
/// The first [u32], however, is split into 3 (sometimes 5) parts. The two u32s can be thought of like so:
///
/// ```md
///   |-----|-------|--------------------------|---------------------------------|
///   | TF  | K     | VD                       | Value                           |
/// 0b| 000 | 00000 | 000000000000000000000000 | 0000000000000000000000000000000 |
///   |-----|-------|--------------------------|---------------------------------|
///   | 3-- | 5---- | 24---------------------- | 32----------------------------- |
/// ```
///
/// ## TF = Type Flags (or "Token Facts")
///
/// This represents a bit-mask in the upper-most 3 bits. The flags are general purpose and change meaning depending on
/// the Token's [Kind]. Each flag generally maps to a method so it's not necessary to remenber the contents of this
/// table, but it can serve as a useful reference. Note that not all methods return a [bool], so footnotes have been
/// added to explain these further.
///
/// | Kind::             | Flag  | Description                 | Method                                   |
/// |--------------------|-------|-----------------------------|------------------------------------------|
/// | [Kind::Number]     | `001` | Floating Point              | [Token::is_float()]                      |
/// |                    | `010` | Has a "Sign" (-/+)          | [Token::has_sign()]                      |
/// |                    | `100` | (Reserved)                  | --                                       |
/// | [Kind::Dimension]  | `001` | Floating Point              | [Token::is_float()]                      |
/// |                    | `010` | Has a "Sign" (-/+)          | [Token::has_sign()]                      |
/// |                    | `100` | Unit is a known dimension   | [Token::dimension_unit()][^dimension]    |
/// | [Kind::String]     | `001` | Uses Double Quotes          | [Token::quote_style()][^quotes]          |
/// |                    | `010` | Has a closing quote         | [Token::has_close_quote()]               |
/// |                    | `100` | Contains escape characters  | [Token::contains_escape_chars()]         |
/// | [Kind::Ident]      | `001` | Contains non-lower-ASCII    | [Token::is_lower_case()]                 |
/// |                    | `010` | Is a "Dashed Ident"         | [Token::is_dashed_ident()]               |
/// |                    | `100` | Contains escape characters  | [Token::contains_escape_chars()]         |
/// | [Kind::Function]   | `001` | Contains non-lower-ASCII    | [Token::is_lower_case()]                 |
/// |                    | `010` | Is a "Dashed Ident"         | [Token::is_dashed_ident()]               |
/// |                    | `100` | Contains escape characters  | [Token::contains_escape_chars()]         |
/// | [Kind::AtKeyword]  | `001` | Contains non-lower-ASCII    | [Token::is_lower_case()]                 |
/// |                    | `010` | Is a "Dashed Ident"         | [Token::is_dashed_ident()]               |
/// |                    | `100` | Contains escape characters  | [Token::contains_escape_chars()]         |
/// | [Kind::Hash]       | `001` | Contains non-lower-ASCII    | [Token::is_lower_case()]                 |
/// |                    | `010` | First character is ASCII    | [Token::hash_is_id_like()]               |
/// |                    | `100` | Contains escape characters  | [Token::contains_escape_chars()]         |
/// | [Kind::Url]        | `001` | Has a closing paren )       | [Token::url_has_closing_paren()]         |
/// |                    | `010` | Contains whitespace after ( | [Token::url_has_leading_space()]         |
/// |                    | `100` | Contains escape characters  | [Token::contains_escape_chars()]         |
/// | [Kind::CdcOrCdo]   | `001` | Is CDO (`000` would be CDC) | [Token::is_cdc()]                        |
/// |                    | `010` | (Reserved)                  | --                                       |
/// |                    | `100` | (Reserved)                  | --                                       |
/// | [Kind::Whitespace] | `001` | Contains at least 1 space   | [Token::whitespace_style()][^whitespace] |
/// |                    | `010` | Contains at least 1 tab     | [Token::whitespace_style()][^whitespace] |
/// |                    | `100` | Contains at least 1 newline | [Token::whitespace_style()][^whitespace] |
/// | [Kind::Comment]    | `---` | (Special)                   | [Token::comment_style()][^comments]      |
/// | [Kind::Delim]      | `---` | (Special)                   | Stores the char length[^delim]           |
///
///	[^dimension]: Dimensions do not have a [bool] returning method for whether or not the dimension is known, instead
///	[Token::dimension_unit()] `==` [DimensionUnit::Unknown] can be consulted.
///	[^quotes]: Strings do not have a [bool] returning method for whether or not the quote is using double or single
///	quotes, instead the [Token::quote_style()] method will returning the [QuoteStyle] enum for better readability.
///	[^whitespace]: Whitespace tokens to not have a [bool] returning method, instead [Token::whitespace_style()] will return
///	the [Whitespace] enum for improved readability.
///	[^comments]: Rather than using the 3 bits as a bit-mask, Comment tokens use the data to store the [CommentStyle]
///	enum, which is capable of representing 8 discrete comment styles.
///	[^delim]: Delims do not store additional "facts" about the character (as the character is stored in the token
///	itself and so can be fully reasoned about). Instead the `TF` space is used to store the length of the character in
///	source. This is due to a featute of the CSS syntax which dictates that the rendered character may differ from the
///	encoded delim; as `\0` and surrogates are replaced with `\u{FFFD}`.
///
/// ## K = Kind Bits
///
/// The `K` value - upper-most bits 4-9 stores the 5-bit [Kind].
///
/// ## VD = Value Data
///
/// The `VD` value - the lower-most 24-bits - stores data depending on the [Token] [Kind]. For most kinds this data is
/// reserved (just 0s). The value data cannot be interrogated manually, but it packs in additional data about the
/// underlying string to make the string easier to parse without doing the same lookups that the tokenizer already had
/// to - such as determining lengths of the various parts of the token, or packing values so that consulting the string
/// can be avoided (which keeps cache-lines hot).
///
/// Below describes the special kinds which use the Value Data to store yet more information about the token...
///
/// ### Value Data for [Kind::Number]
///
/// If the [Kind] is [Kind::Number], Value Data represents the length of that number (this means the parser is
/// restricted from representing numbers longer than 16,777,216 characters which is probably an acceptable limit). Note
/// that this does not affect the _value_ of a number, just the characters in a string. Numbers in CSS are [f32]. The
/// vast majority of [f32s][f32] can be represented in 16MM characters, but it's possible to author a document that
/// contains a set of numeric characters longer than 16MM code points. These scenarios are considered [undefined
/// behaviour][1].
///
/// [4]: https://en.wikipedia.org/wiki/Undefined_behavior
///
/// ### Value Data for [Kind::Url]
///
/// If the [Kind] is [Kind::Url], Value Data represents the "leading length" and "trailing length" of the URL. This
/// means the value data is split into two 12 bit numbers:
///
/// ```md
/// |--------------|--------------|
/// | LL           | TL           |
/// | 000000000000 | 000000000000 |
/// |--------------|--------------|
/// | 12---------- | 12---------- |
/// ```
///
/// The "leading" length represents the `url(` part of the token. Typically this will be `4`, however it's possible
/// (for legacy compatibility reasons within CSS) to add whitespace between the opening parenthesis and the URL value.
/// It's also possible to escape the `url` ident portion. This means `\75\52\6c(   ` is also a valid leading section of
/// a URL ident (which has a character length of 13), as is `\000075 \000052 \00006c (   ` (28 characters). 12 bits
/// allows for a maximum character length of 4,096. It is not possible to represent a URL token's leading section using
/// 4,096 characters so there is some headroom (wasted bytes) here.
///
/// The "trailing" length represents the `)` part of the token. Typically this will be `1`, however it's possible to
/// add any number of whitespace characters between the end of the URL and the closing parenthesis. If a CSS document
/// contains more than 4095 whitespace characters then this is considered [undefined behaviour][4].
///
/// ### Value Data for [Kind::Dimension]
///
/// If K is a Dimension, then this represents both the number of characters in the numeric portion of the dimension
/// and the length of the ident portion of the dimension... or the dimension unit itself (more on that below). This
/// means the value data is split into two 12 bit numbers:
///
/// ```md
/// |--------------|--------------|
/// | NL           | DUL          |
/// | 000000000000 | 000000000000 |
/// |--------------|--------------|
/// | 12---------- | 12---------- |
/// ```
///
/// The NL portion - the numeric length - represents the length of characters the number contains. This means the
/// numeric portion of a dimension can only be 4,096 characters long. This is dramatically shorter than the 16MM
/// allowed for numbers but it's still also incredibly generous such that it's highly unlikely to ever be hit unless
/// someone is intentionally trying to break the parser. The [Lexer][super::Lexer] encountering a dimension with a
/// numeric portion longer than 4,096 characters is considered [undefined behaviour][4].
///
/// The DUL portion (if `TF & 100 == 0`) will represent the length of characters the ident portion of the dimension
/// (aka the dimension unit) contains. This means the ident portion of a dimension can only be 4,096 characters long.
/// For practical purposes CSS has a fixed set of dimensions - the longest of which (at the time of writing) are 5
/// characters long (e.g. `svmax`). Through the use of escaping shenanigans it's possible to create a valid CSS
/// dimension longer than 5 characters though (every ident can be made 8 times longer by using escape characters, e.g.
/// `1svmax` at 6 characters can be instead written as `1\000073 \000076 \00006d \000061 \000078` at 40 characters). In
/// addition to these factors, it's worth pointing out that there is scope for further dimensions and some [proposals
/// for "custom" dimensions][5], and lastly this library is designed for CSS _and CSS-alike_ languages, which may
/// invent their own dimension units. In other words being too restrictive on dimension ident length could be costly
/// in the future, therefore 4,096 characters seems like a reasonable, if generous, trade-off.
///
/// There's a giant caveat here though, and a carve out for parsing CSS as it exists today. If `TF & 100 != 0`, then
/// the dimension is considered "known" and DUL will be encoded differently. Instead of being the dimension unit
/// length, which requires consulting the underlying `&str` to get the actual dimension, it will be used to store the
/// [DimensionUnit] - an enum of known CSS dimensions. In this mode [Token::dimension_unit()] will return a valid
/// [DimensionUnit] (excluding [DimensionUnit::Unknown]). When it comes to reasoning about dimensions from the
/// outside, this won't make a significant difference but it does provide a nice performance boost in parser
/// implementations without slowing down the [Lexer][super::Lexer] by any significant amount. However, if a dimension
/// unit is escaped in any way it will _not_ be represented as a known [DimensionUnit], due to the variability in the
/// length encoding which would otherwise be lost if using the enum variant.
///
/// [5]: https://github.com/w3c/csswg-drafts/issues/7379
///
/// ## Value
///
/// The `Value` portion of [Token] represents the length of the token for most token kinds. However, for some tokens
/// their length is already packed into the first u32. So it would make more sense to use this u32 to store more
/// interesting data.
///
/// ## Value for [Kind::Delim] and single character tokens
///
/// [Kind::Delim] and single-character tokens (i.e. [Kind::Colon]->[Kind::RightCurly]) typically have a length of `1`
/// ([Kind::Delim] can have a varied length for surrogates[^delim]). Instead of storing the length and wasting a whole
/// [u32], this region stores the [char]. Calling [Token::char()] will return an [Option] which will always be [Some]
/// for [Kind::Delim] and single-character tokens.
///
/// ## Value for [Kind::Number] and [Kind::Dimension]
///
/// As these tokens store their length data in the `VD` portion, this [u32] instead stores the _value_ of the number,
/// stored as [f32::to_bits()].
///
/// ## Value data for other tokens.
///
/// In all other cases, this represents the length of the token as utf-8 bytes. This means the token length is
/// 4,294,967,296 aka ~4GB. This sounds very long but also CSS can host very large image data and browsers will
/// accomodate very large URLs. [An mdn article on Data URLs][6] claims that Firefox supports 32mb Data URLs, Chrome
/// supports over 512mb, and Safari over 2gb. The reality is that if someone has such a large data URL in their CSS
/// they probably should split it out, but we have a whole 32 bits to store the length so we may as well use it...
///
/// [6]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs#common_problems
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
	/// Represents an EOF token.
	pub const EOF: Token = Token(0b0, 0);

	/// Represents a CDO (`<!--`) token.
	pub const CDO: Token = Token(((Kind::CdcOrCdo as u32) << 24) & KIND_MASK, 4);

	/// Represents a CDC (`-->`) token.
	pub const CDC: Token = Token((((Kind::CdcOrCdo as u32) | 0b001_00000) << 24) & KIND_MASK, 3);

	/// Represents a single ' ' space token.
	pub const SPACE: Token = Token::new_whitespace(Whitespace::Space, 1);

	/// Represents a single Tab token.
	pub const TAB: Token = Token::new_whitespace(Whitespace::Tab, 1);

	/// Represents a single `\n` token.
	pub const NEWLINE: Token = Token::new_whitespace(Whitespace::Newline, 1);

	/// Represents the Number `0`. This is not equal to other representations of zero, such as `00`, `0e0`, `0.0` and so
	/// on.
	pub const NUMBER_ZERO: Token = Token((((Kind::Number as u32) | 0b100_00000) << 24) & KIND_MASK, 1);

	/// Represents the `:` token.
	pub const COLON: Token = Token((((Kind::Colon as u32) | 0b001_00000) << 24) & KIND_MASK, ':' as u32);

	/// Represents the `;` token.
	pub const SEMICOLON: Token = Token((((Kind::Semicolon as u32) | 0b001_00000) << 24) & KIND_MASK, ';' as u32);

	/// Represents the `,` token.
	pub const COMMA: Token = Token((((Kind::Comma as u32) | 0b001_00000) << 24) & KIND_MASK, ',' as u32);

	/// Represents the `[` token.
	pub const LEFT_SQUARE: Token = Token((((Kind::LeftSquare as u32) | 0b001_00000) << 24) & KIND_MASK, '[' as u32);

	/// Represents the `]` token.
	pub const RIGHT_SQUARE: Token = Token((((Kind::RightSquare as u32) | 0b001_00000) << 24) & KIND_MASK, ']' as u32);

	/// Represents the `(` token.
	pub const LEFT_PAREN: Token = Token((((Kind::LeftParen as u32) | 0b001_00000) << 24) & KIND_MASK, '(' as u32);

	/// Represents the `)` token.
	pub const RIGHT_PAREN: Token = Token((((Kind::RightParen as u32) | 0b001_00000) << 24) & KIND_MASK, ')' as u32);

	/// Represents the `{` token.
	pub const LEFT_CURLY: Token = Token((((Kind::LeftCurly as u32) | 0b001_00000) << 24) & KIND_MASK, '{' as u32);

	/// Represents the `}` token.
	pub const RIGHT_CURLY: Token = Token((((Kind::RightCurly as u32) | 0b001_00000) << 24) & KIND_MASK, '}' as u32);

	/// Represents a `!` [Kind::Delim] token.
	pub const BANG: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '!' as u32);

	/// Represents a `#` [Kind::Delim] token.
	pub const HASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '#' as u32);

	/// Represents a `$` [Kind::Delim] token.
	pub const DOLLAR: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '$' as u32);

	/// Represents a `%` [Kind::Delim] token - not to be confused with the `%` dimension.
	pub const PERCENT: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '%' as u32);

	/// Represents a `&` [Kind::Delim] token.
	pub const AMPERSAND: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '&' as u32);

	/// Represents a `*` [Kind::Delim] token.
	pub const ASTERISK: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '*' as u32);

	/// Represents a `+` [Kind::Delim] token.
	pub const PLUS: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '+' as u32);

	/// Represents a `-` [Kind::Delim] token.
	pub const DASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '-' as u32);

	/// Represents a `.` [Kind::Delim] token.
	pub const PERIOD: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '.' as u32);

	/// Represents a `/` [Kind::Delim] token.
	pub const SLASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '/' as u32);

	/// Represents a `<` [Kind::Delim] token.
	pub const LESS_THAN: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '<' as u32);

	/// Represents a `=` [Kind::Delim] token.
	pub const EQUALS: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '=' as u32);

	/// Represents a `>` [Kind::Delim] token.
	pub const GREATER_THAN: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '>' as u32);

	/// Represents a `?` [Kind::Delim] token.
	pub const QUESTION: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '?' as u32);

	/// Represents a `@` [Kind::Delim] token. Not to be confused with the @keyword token.
	pub const AT: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '@' as u32);

	/// Represents a `\\` [Kind::Delim] token.
	pub const BACKSLASH: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '\\' as u32);

	/// Represents a `^` [Kind::Delim] token.
	pub const CARET: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '^' as u32);

	/// Represents a `_` [Kind::Delim] token.
	pub const UNDERSCORE: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '_' as u32);

	/// Represents a `\`` [Kind::Delim] token.
	pub const BACKTICK: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '`' as u32);

	/// Represents a `|` [Kind::Delim] token.
	pub const PIPE: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '|' as u32);

	/// Represents a `~` [Kind::Delim] token.
	pub const TILDE: Token = Token((((Kind::Delim as u32) | 0b001_00000) << 24) & KIND_MASK, '~' as u32);

	/// Creates a "Dummy" token with no additional data, just the [Kind].
	#[inline]
	pub const fn dummy(kind: Kind) -> Self {
		Self((kind as u32) << 24, 0)
	}

	/// Creates a "Dummy" token with no additional data, just [Kind::Ident].
	#[inline]
	pub const fn dummy_ident() -> Self {
		Self((Kind::Ident as u32) << 24, 0)
	}

	/// Creates a [Kind::Whitesapce] token.
	#[inline]
	pub(crate) const fn new_whitespace(style: Whitespace, len: u32) -> Self {
		let flags: u32 = Kind::Whitespace as u32 | ((style.to_bits() as u32) << 5);
		Self((flags << 24) & KIND_MASK, len)
	}

	/// Creates a [Kind::Comment] token.
	#[inline]
	pub(crate) fn new_comment(style: CommentStyle, len: u32) -> Self {
		let flags: u32 = Kind::Comment as u32 | ((style as u32) << 5);
		Self((flags << 24) & KIND_MASK, len)
	}

	/// Creates a [Kind::Number] token.
	#[inline]
	pub(crate) fn new_number(is_float: bool, has_sign: bool, len: u32, value: f32) -> Self {
		let flags: u32 = Kind::Number as u32 | ((is_float as u32) << 5) | ((has_sign as u32) << 6);
		Self((flags << 24) & KIND_MASK | (len & LENGTH_MASK), value.to_bits())
	}

	/// Creates a new [Kind::Dimension] token.
	#[inline]
	pub(crate) fn new_dimension(
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

	/// Creates a new [Kind::BadString] token. Bad Strings are like String tokens but during lexing they failed to fully tokenize
	/// into a proper string token, usually due to containing newline characters.
	#[inline]
	pub(crate) fn new_bad_string(len: u32) -> Self {
		Self(((Kind::BadString as u32) << 24) & KIND_MASK, len)
	}

	/// Creates a new [Kind::BadUrl] token. Bad URLs are like URL tokens but during lexing they failed to fully tokenize into a
	/// proper URL token, usually due to containing newline characters.
	#[inline]
	pub(crate) fn new_bad_url(len: u32) -> Self {
		Self(((Kind::BadUrl as u32) << 24) & KIND_MASK, len)
	}

	/// Creates a new [Kind::Ident] token.
	#[inline]
	pub(crate) fn new_ident(contains_non_lower_ascii: bool, dashed: bool, contains_escape: bool, len: u32) -> Self {
		let flags: u32 = Kind::Ident as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((dashed as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	/// Creates a new [Kind::Function] token.
	#[inline]
	pub(crate) fn new_function(contains_non_lower_ascii: bool, dashed: bool, contains_escape: bool, len: u32) -> Self {
		let flags: u32 = Kind::Function as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((dashed as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	/// Creates a new [Kind::AtKeyword] token.
	#[inline]
	pub(crate) fn new_atkeyword(contains_non_lower_ascii: bool, dashed: bool, contains_escape: bool, len: u32) -> Self {
		let flags: u32 = Kind::AtKeyword as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((dashed as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	/// Creates a new [Kind::Hash] token.
	#[inline]
	pub(crate) fn new_hash(
		contains_non_lower_ascii: bool,
		first_is_ascii: bool,
		contains_escape: bool,
		len: u32,
	) -> Self {
		let flags: u32 = Kind::Hash as u32
			| ((contains_non_lower_ascii as u32) << 5)
			| ((first_is_ascii as u32) << 6)
			| ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	/// Creates a new [Kind::String] token.
	#[inline]
	pub(crate) fn new_string(quotes: QuoteStyle, has_close_quote: bool, contains_escape: bool, len: u32) -> Self {
		debug_assert!(quotes != QuoteStyle::None);
		let quotes = if quotes == QuoteStyle::Double { 0b001_00000 } else { 0b0 };
		let flags: u32 =
			Kind::String as u32 | quotes | ((has_close_quote as u32) << 6) | ((contains_escape as u32) << 7);
		Self((flags << 24) & KIND_MASK, len)
	}

	/// Creates a new [Kind::Url] token.
	#[inline]
	pub(crate) fn new_url(
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

	/// Creates a new [Kind::Delim] token.
	#[inline]
	pub(crate) fn new_delim(char: char) -> Self {
		let len = char.len_utf8() as u32;
		debug_assert!(len <= 7);
		let flags: u32 = Kind::Delim as u32 | (len << 5);
		Self((flags << 24) & KIND_MASK | (len & LENGTH_MASK), char as u32)
	}

	/// Returns the raw bits representing the [Kind].
	#[inline(always)]
	pub(crate) const fn kind_bits(&self) -> u8 {
		(self.0 >> 24 & 0b11111) as u8
	}

	/// Returns the [Kind].
	#[inline]
	pub const fn kind(&self) -> Kind {
		Kind::from_bits(self.kind_bits())
	}

	/// Check if the TF upper-most bit is set.
	#[inline(always)]
	const fn first_bit_is_set(&self) -> bool {
		self.0 >> 31 == 1
	}

	/// Check if the TF second-upper-most bit is set.
	#[inline(always)]
	const fn second_bit_is_set(&self) -> bool {
		self.0 >> 30 & 0b1 == 1
	}

	/// Check if the TF third-upper-most bit is set.
	#[inline(always)]
	const fn third_bit_is_set(&self) -> bool {
		self.0 >> 29 & 0b1 == 1
	}

	/// Check if the [Kind] is "Ident Like", i.e. it is [Kind::Ident], [Kind::AtKeyword], [Kind::Function], [Kind::Hash].
	#[inline(always)]
	pub(crate) fn is_ident_like(&self) -> bool {
		self.kind_bits() & 0b11000 == 0b01000 && self.kind_bits() != Kind::String as u8
	}

	/// Check if the [Kind] is "Delim Like", i.e. it is [Kind::Delim], [Kind::Colon], [Kind::Semicolon], [Kind::Comma],
	/// [Kind::LeftSquare], [Kind::RightSquare], [Kind::LeftParen], [Kind::RightParen], [Kind::LeftCurly],
	/// [Kind::RightCurly].
	#[inline(always)]
	pub(crate) fn is_delim_like(&self) -> bool {
		self.kind_bits() & 0b10000 == 0b10000
	}

	/// The only token with an empty length is EOF, but this method is available for symmetry with `len()`.
	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.kind_bits() == Kind::Eof as u8
	}

	/// Returns the amount of characters (utf-8 code points) this Token represents in the underlying source text.
	#[inline]
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

	/// If the [Kind] is "Delim Like" (i.e. it is [Kind::Delim], [Kind::Colon], [Kind::Semicolon], [Kind::Comma],
	/// [Kind::LeftSquare], [Kind::RightSquare], [Kind::LeftParen], [Kind::RightParen], [Kind::LeftCurly],
	/// [Kind::RightCurly]) then this will return a [Some] with a [char] representing the value.
	/// For non-delim-like tokens this will return [None].
	pub fn char(&self) -> Option<char> {
		if self.is_delim_like() {
			return char::from_u32(self.1);
		}
		None
	}

	/// The [Token] is a [Kind::Dimension] or [Kind::Number] and is an integer - i.e. it has no `.`.
	#[inline]
	pub fn is_int(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && !self.third_bit_is_set()
	}

	/// The [Token] is a [Kind::Dimension] or [Kind::Number] and is a float - i.e. it has decimal places. This will be
	/// `true` even if the decimal places are 0. e.g. `0.0`.
	#[inline]
	pub fn is_float(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && self.third_bit_is_set()
	}

	/// The [Token] is a [Kind::Dimension] or [Kind::Number] and the underlying character data included a `-` or `+`
	/// character. Note that a positive number may not necessarily have a sign, e.g. `3` will return false, while `+3`
	/// will return `true`.
	#[inline]
	pub fn has_sign(&self) -> bool {
		self.kind_bits() & 0b11100 == 0b00100 && self.second_bit_is_set()
	}

	/// If the [Token] is a [Kind::Dimension] or [Kind::Number] then this returns the amount of characters used to
	/// represent this number in the underlying source text. Numbers may be inefficiently encoded in the source text,
	/// e.g. `0.0000`.
	///
	/// Asserts: the `kind()` is [Kind::Dimension] or [Kind::Number].
	#[inline]
	pub fn numeric_len(&self) -> u32 {
		debug_assert!(matches!(self.kind(), Kind::Number | Kind::Dimension));
		if self.kind_bits() == Kind::Dimension as u8 {
			(self.0 & LENGTH_MASK) >> 12
		} else if self.first_bit_is_set() {
			(self.0 & LENGTH_MASK) >> 16
		} else {
			self.0 & LENGTH_MASK
		}
	}

	/// If the [Token] is a [Kind::Dimension] or [Kind::Number] then this returns the [f32] representation of the number's
	/// value.
	///
	/// Asserts: the `kind()` is [Kind::Dimension] or [Kind::Number].
	#[inline]
	pub fn value(&self) -> f32 {
		debug_assert!(matches!(self.kind(), Kind::Number | Kind::Dimension));
		f32::from_bits(self.1)
	}

	/// Returns the [Whitespace].
	///
	/// If the [Token] is not a [Kind::Whitespace] this will return [Whitespace::none()].
	#[inline]
	pub fn whitespace_style(&self) -> Whitespace {
		if self.kind_bits() == Kind::Whitespace as u8 {
			Whitespace::from_bits((self.0 >> 29) as u8)
		} else {
			Whitespace::none()
		}
	}

	/// Returns the [CommentStyle].
	///
	/// If the [Token] is not a [Kind::Comment] this will return [None].
	#[inline]
	pub fn comment_style(&self) -> Option<CommentStyle> {
		if self.kind_bits() == Kind::Comment as u8 {
			CommentStyle::from_bits((self.0 >> 29) as u8)
		} else {
			None
		}
	}

	/// Returns the [DimensionUnit].
	///
	/// If the [Token] is not a [Kind::Dimension] this will return [DimensionUnit::Unknown].
	/// If the [Token] _is_ a [Kind::Dimension], but the dimension unit is custom (e.g. dashed), has escape characters,
	/// or is not a recognised CSS Dimension, this will return [DimensionUnit::Unknown].
	#[inline]
	pub fn dimension_unit(&self) -> DimensionUnit {
		if !self.first_bit_is_set() || self.kind_bits() != Kind::Dimension as u8 {
			DimensionUnit::Unknown
		} else {
			let unit_bits = (self.0 & !HALF_LENGTH_MASK) as u8;
			unit_bits.into()
		}
	}

	/// Returns the [QuoteStyle].
	///
	/// If the [Token] is not a [Kind::String] this will return [QuoteStyle::None].
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

	/// If the [Token] is a [Kind::String] this checks if the string ended in a close quote.
	/// It is possible to have a valid String token that does not end in a close quote, by eliding the quote at the end of
	/// a file.
	///
	/// Asserts: The [Kind] is [Kind::String].
	#[inline]
	pub fn has_close_quote(&self) -> bool {
		debug_assert!(self.kind_bits() == Kind::String as u8);
		self.second_bit_is_set()
	}

	/// Checks if it is possible for the [Token] to contain escape characters. Numbers, for example, cannot. Idents can.
	#[inline]
	pub fn can_escape(&self) -> bool {
		self.kind_bits() == Kind::String as u8 || self.kind_bits() == Kind::Dimension as u8 || self.is_ident_like()
	}

	/// If the [Token] can escape, checks if the underlying source text contained escape characters.
	///
	/// Asserts: The token can escape ([Token::can_escape()]).
	#[inline]
	pub fn contains_escape_chars(&self) -> bool {
		if self.kind_bits() == Kind::Dimension as u8 {
			return !self.first_bit_is_set();
		}
		self.can_escape() && self.first_bit_is_set()
	}

	/// If the [Token] is Ident like, checks if the first two code points are HYPHEN-MINUS (`-`).
	///
	/// Asserts: The token is "ident like", i.e. it is [Kind::Ident], [Kind::AtKeyword], [Kind::Function], [Kind::Hash].
	#[inline]
	pub fn is_dashed_ident(&self) -> bool {
		debug_assert!(self.is_ident_like());
		self.second_bit_is_set()
	}

	/// Checks if the [Token] is Ident like and none of the characters are ASCII upper-case.
	#[inline]
	pub fn is_lower_case(&self) -> bool {
		self.is_ident_like() && !self.third_bit_is_set()
	}

	/// Checks if the [Token] is Trivia-like, that is [Kind::Comment], [Kind::Whitespace], [Kind::Eof]
	#[inline]
	pub fn is_trivia(&self) -> bool {
		self.kind_bits() & 0b000011 == self.kind_bits()
	}

	/// If the [Token] is [Kind::Url], checks if there are leading Whitespace characters before the inner value.
	///
	/// Asserts: The token is [Kind::Url].
	#[inline]
	pub fn url_has_leading_space(&self) -> bool {
		debug_assert!(self.kind_bits() == Kind::Url as u8);
		self.second_bit_is_set()
	}

	/// If the [Token] is [Kind::Url], checks if the closing parenthesis is present.
	///
	/// Asserts: The token is [Kind::Url].
	#[inline]
	pub fn url_has_closing_paren(&self) -> bool {
		debug_assert!(self.kind_bits() == Kind::Url as u8);
		self.third_bit_is_set()
	}

	/// If the [Token] is [Kind::Hash], checks if the Hash is "ID-like" (i.e its first character is ASCII).
	///
	/// Asserts: The token is [Kind::Hash].
	#[inline]
	pub fn hash_is_id_like(&self) -> bool {
		debug_assert!(self.kind_bits() == Kind::Hash as u8);
		self.second_bit_is_set()
	}

	/// Checks if the [Token] is [Kind::BadString] or [Kind::BadUrl].
	#[inline]
	pub fn is_bad(&self) -> bool {
		(self.kind_bits() | 0b00001) & 0b11001 == 1
	}

	/// Checks if the [Token] is [Kind::CdcOrCdo] and is the CDC variant of that token.
	#[inline]
	pub fn is_cdc(&self) -> bool {
		self.kind_bits() == (Kind::CdcOrCdo as u8) && self.third_bit_is_set()
	}

	/// Some tokens may have a "leading" part:
	///  - [Kind::AtKeyword] always starts with a `@`,
	///  - [Kind::Hash] with a `#`.
	///  - [Kind::String] with a `"` or `'`.
	///  - [Kind::Comment] with a leading `/*` (or `//`).
	///  - [Kind::Dimension] has a leading numeric portion.
	///  - [Kind::Url] has the leading `url(` ident (which may vary in exact representation).
	///
	/// This function returns the length of that, irrespective of the [Kind]. For other kinds not listed, this will return
	/// `0`, but for the above kinds it will calculate the leading length. This is useful for parsing out the underlying
	/// data which is likely to be of greater use.
	pub fn leading_len(&self) -> u32 {
		match self.kind() {
			Kind::AtKeyword | Kind::Hash | Kind::String => 1,
			Kind::Dimension => self.numeric_len(),
			Kind::Comment => 2,
			Kind::Url => (self.0 & LENGTH_MASK) >> 12,
			_ => 0,
		}
	}

	/// Some tokens may have a "trailing" part:
	///  - [Kind::Function] will always have an opening `(`.
	///  - [Kind::String] may have a closing `"` or `'`.
	///  - [Kind::Comment] may have a closing `*/`
	///  - [Kind::Url] may have a clsoing `)`.
	///
	/// This function returns the length of that, irrespective of the [Kind]. For other kinds not listed, this will return
	/// `0`, but for the above kinds it will calculate the leading length. This is useful for parsing out the underlying
	/// data which is likely to be of greater use.
	pub fn trailing_len(&self) -> u32 {
		match self.kind() {
			Kind::Function => 1,
			Kind::String => self.has_close_quote() as u32,
			Kind::Comment if self.comment_style().unwrap().is_block() => 2,
			Kind::Url => self.0 & !HALF_LENGTH_MASK,
			_ => 0,
		}
	}

	/// Certain kinds have a [PairWise] equivalent:
	///  - [Kind::LeftParen] has [Kind::RightParen]
	///  - [Kind::LeftCurly] has [Kind::RightCurly]
	///  - [Kind::LeftSquare] has [Kind::RightSquare]
	///
	/// This function returns the [PairWise] enum, if the [Token] is one of the above listed [Kinds][Kind]. For any other
	/// [Kind] this returns [None].
	#[inline]
	pub fn to_pairwise(&self) -> Option<PairWise> {
		PairWise::from_token(self)
	}

	/// A convenience function for `Cursor::new(offset, token)`.
	#[inline(always)]
	pub fn with_cursor(self, offset: SourceOffset) -> Cursor {
		Cursor::new(offset, self)
	}

	/// If this [Token] is preceded by the [Token] `other` then a separating token (e.g. a comment) will need to be
	/// inserted between these the two tokens during serialization, in order for them to be able to be re-tokenized as
	/// the same tokens. For example an Ident ("a") adjacent to an Ident ("b"), if serialized without whitespace, would
	/// create a single Ident ("ab"). The rules for estbalishing whether or not these tokens needs whitespace are quite
	/// simple and are effectively [defined in the serialization section of the spec][1]. To reproduce the table:
	///
	/// [1]: https://drafts.csswg.org/css-syntax/#serialization
	///
	/// |            | ident | function | url | bad url | - | number | percentage | dimension | CDC | ( | * | % |
	/// |:-----------|:-----:|:--------:|:---:|:-------:|:-:|:------:|:----------:|:---------:|:---:|:-:|:-:|:-:|
	/// | ident      |   ✗   |    ✗     |  ✗  |    ✗    | ✗ |    ✗   |      ✗     |     ✗     |  ✗  | ✗ |   |   |
	/// | at-keyword |   ✗   |    ✗     |  ✗  |    ✗    | ✗ |    ✗   |      ✗     |     ✗     |  ✗  |   |   |   |
	/// | hash       |   ✗   |    ✗     |  ✗  |    ✗    | ✗ |    ✗   |      ✗     |     ✗     |  ✗  |   |   |   |
	/// | dimension  |   ✗   |    ✗     |  ✗  |    ✗    | ✗ |    ✗   |      ✗     |     ✗     |  ✗  |   |   |   |
	/// | #          |   ✗   |    ✗     |  ✗  |    ✗    | ✗ |    ✗   |      ✗     |     ✗     |  ✗  |   |   |   |
	/// | \-         |   ✗   |    ✗     |  ✗  |    ✗    | ✗ |    ✗   |      ✗     |     ✗     |  ✗  |   |   |   |
	/// | number     |   ✗   |    ✗     |  ✗  |    ✗    |   |    ✗   |      ✗     |     ✗     |  ✗  |   |   | ✗ |
	/// | @          |   ✗   |    ✗     |  ✗  |    ✗    | ✗ |        |            |           |  ✗  |   |   |   |
	/// | .          |       |          |     |         |   |    ✗   |      ✗     |     ✗     |     |   |   |   |
	/// | +          |       |          |     |         |   |    ✗   |      ✗     |     ✗     |     |   |   |   |
	/// | /          |       |          |     |         |   |        |            |           |     |   | ✗ |   |
	///
	/// The one exception not in this table is that two consecutive `/` characters should also be separated by spaces in
	/// order to avoid abmiguities with CSS-alike languages that treat two consecutive `/` characters as a single line
	/// comment.
	///
	/// # Example
	///
	/// ```
	/// use css_lexer::*;
	/// let mut lexer = Lexer::new("10 %");
	/// let first = lexer.advance();
	/// let _ = lexer.advance(); // Whitespace
	/// let second = lexer.advance();
	/// assert!(first.needs_separator_for(second));
	/// ```
	pub fn needs_separator_for(&self, second: Token) -> bool {
		match self.kind() {
			Kind::Ident => {
				(matches!(second.kind(), Kind::Number | Kind::Dimension) &&
					// numbers with a `-` need separating, but with `+` they do not.
					(!second.has_sign() || second.value() < 0.0))
					|| matches!(second.kind(), Kind::Ident | Kind::Function | Kind::Url | Kind::BadUrl)
					|| matches!(second.char(), Some('('))
					|| second.is_cdc()
			}
			Kind::AtKeyword | Kind::Hash | Kind::Dimension => {
				(matches!(second.kind(), Kind::Number | Kind::Dimension) &&
					// numbers with a `-` need separating, but with `+` they do not.
					(!second.has_sign() || second.value() < 0.0))
					|| matches!(second.kind(), Kind::Ident | Kind::Function | Kind::Url | Kind::BadUrl)
					|| second.is_cdc()
			}
			Kind::Number => {
				matches!(
					second.kind(),
					Kind::Ident | Kind::Function | Kind::Url | Kind::BadUrl | Kind::Number | Kind::Dimension
				) || matches!(second.char(), Some('%'))
					|| second.is_cdc()
			}
			_ => match self.char() {
				Some('#') => {
					matches!(
						second.kind(),
						Kind::Ident | Kind::Function | Kind::Url | Kind::BadUrl | Kind::Number | Kind::Dimension
					) || matches!(second.char(), Some('-'))
						|| second.is_cdc()
				}
				Some('-') => {
					matches!(
						second.kind(),
						Kind::Ident | Kind::Function | Kind::Url | Kind::BadUrl | Kind::Number | Kind::Dimension
					) || matches!(second.char(), Some('-'))
						|| second.is_cdc()
				}
				Some('@') => {
					matches!(second.kind(), Kind::Ident | Kind::Function | Kind::Url | Kind::BadUrl)
						|| matches!(second.char(), Some('-'))
						|| second.is_cdc()
				}
				Some('.') => matches!(second.kind(), Kind::Number | Kind::Dimension),
				Some('+') => matches!(second.kind(), Kind::Number | Kind::Dimension),
				Some('/') => matches!(second.char(), Some('*' | '/')),
				_ => false,
			},
		}
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
			Kind::String => d
				.field("quote_stylee", &if self.first_bit_is_set() { "Double" } else { "Single" })
				.field("has_close_quote", &self.second_bit_is_set())
				.field("contains_escape_chars", &self.third_bit_is_set())
				.field("len", &self.len()),
			Kind::Ident | Kind::Function | Kind::AtKeyword => d
				.field("is_lower_case", &self.first_bit_is_set())
				.field("is_dashed_ident", &self.second_bit_is_set())
				.field("contains_escape_chars", &self.third_bit_is_set())
				.field("len", &self.len()),
			Kind::Hash => d
				.field("is_lower_case", &self.first_bit_is_set())
				.field("hash_is_id_like", &self.second_bit_is_set())
				.field("contains_escape_chars", &self.third_bit_is_set())
				.field("len", &self.len()),
			Kind::Url => d
				.field("url_has_closing_paren", &self.first_bit_is_set())
				.field("url_has_leading_space", &self.second_bit_is_set())
				.field("contains_escape_chars", &self.third_bit_is_set())
				.field("len", &self.len()),
			Kind::CdcOrCdo => d.field("is_cdc", &self.first_bit_is_set()).field("len", &self.len()),
			Kind::Whitespace => d.field("contains", &self.whitespace_style()).field("len", &self.len()),
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

impl From<Token> for Whitespace {
	fn from(token: Token) -> Self {
		token.whitespace_style()
	}
}

impl PartialEq<Whitespace> for Token {
	fn eq(&self, other: &Whitespace) -> bool {
		self.whitespace_style().intersects(*other)
	}
}

impl PartialEq<CommentStyle> for Token {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.comment_style().map(|style| &style == other).unwrap_or(false)
	}
}

impl PartialEq<PairWise> for Token {
	fn eq(&self, other: &PairWise) -> bool {
		self.to_pairwise().map(|style| &style == other).unwrap_or(false)
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
	assert_eq!(Token::SPACE, Whitespace::Space);
	assert_eq!(Token::TAB, Kind::Whitespace);
	assert_eq!(Token::TAB, Whitespace::Tab);
	assert_eq!(Token::NEWLINE, Kind::Whitespace);
	assert_eq!(Token::NEWLINE, Whitespace::Newline);
	assert_eq!(Token::new_whitespace(Whitespace::Space, 4), Kind::Whitespace);
	assert_eq!(Token::new_whitespace(Whitespace::Space | Whitespace::Newline, 4), Whitespace::Space);
	assert_eq!(Token::new_whitespace(Whitespace::Space, 4).len(), 4);
	assert_eq!(Token::new_whitespace(Whitespace::Tab | Whitespace::Space, 4), Whitespace::Tab);
	assert_eq!(Token::new_whitespace(Whitespace::Newline, 4), Whitespace::Newline);
	assert_eq!(Token::new_whitespace(Whitespace::Newline, 4).len(), 4);
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
