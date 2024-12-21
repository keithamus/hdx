use bitmask_enum::bitmask;

/// A set of runtime feature flags which can be enabled individually or in combination, which will change the way 
/// individual [Lexer][crate::Lexer] produces [Tokens][crate::Token].
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
#[bitmask_config(vec_debug)]
#[derive(Default)]
pub enum Feature {
	/// With this flag enabled the [Lexer][crate::Lexer] will produce [Tokens][crate::Token] with
	/// [Kind::Comment][crate::Kind::Comment] when it encounters two consecutative SOLIDUS characters (`//`), the
	/// [Token][crate::Token] will have a length up to the next newline (`\n`) character. The contents between the two
	/// SOLIDUS (`//`) characters and the `\n` will be consumed by this token, so no tokens will be produced for the
	/// contents of the comment.
	///
	/// If this flag is not enabled, encountering something that looks like a single line commet will produce two
	/// [Kind::Delim][crate::Kind::Delim] tokens for the two SOLIDUS (`//`) characters, and any number of other tokens
	/// depending on the contents of the comment, per the CSS specification.
	///
	/// A comment with two leading SOLIDUS characters is not valid CSS syntax, but might be considered valid syntax in
	/// other CSS-alike languages [for example SASS][1].
	///
	/// [1]: https://sass-lang.com/documentation/syntax/comments/
	///
	/// With this feature turned off comments are tokenized per the CSS specification:
	///
	/// ```md
	/// <comment>
	///            ╭──────────────────────────────────────────╮
	///  │├─ "/*" ─╯-╭─ (anything but "*" followed by "/") ─╮─╰─ "*/" ─┤│
	///              ╰──────────────────────────────────────╯
	/// ```
	///
	/// With this feature turned on comments are tokenized with the additional grammar:
	///
	/// ```md
	/// <comment>
	///               ╭──────────────────────────────────────────╮
	///  │├──╮─ "/*" ─╯-╭─ (anything but "*" followed by "/") ─╮─╰─ "*/" ─╭─┤│
	///      │          ╰──────────────────────────────────────╯          │
	///      │              ╭───────────────────────────╮                 │
	///      ├─ "//" ───────╯-╭─ (anything but "\n") ─╮─╰─ "\n" ──────────╯
	///      ╰─               ╰───────────────────────╯
	/// ```
	///
	/// # Example
	///
	/// ```
	/// use css_lexer::*;
	/// let mut lexer = Lexer::new("// foo");
	/// assert_eq!(lexer.advance(), Kind::Delim); // The first `/`
	/// assert_eq!(lexer.advance(), Kind::Delim); // The second `/`
	/// assert_eq!(lexer.advance(), Kind::Whitespace);
	/// assert_eq!(lexer.advance(), Kind::Ident); // The "foo" in the comment
	///
	/// lexer = Lexer::new_with_features("// foo", Feature::SingleLineComments);
	/// let token = lexer.advance();
	/// assert_eq!(token, Kind::Comment); // The whole comment "// foo"
	/// assert_eq!(token, CommentStyle::Single);
	/// ```
	SingleLineComments = 0b0001,

	/// The CSS Spec mentions that whitespace tokens should be [combined into a single Whitespace token][1]. This means
	/// a single whitespace token can contain a cominbation of newlines, tabs, and space characters. This is often fine
	/// as whitespace is generally ignored during parsing, however for certain IDE features it might be important to
	/// tokenize discrete whitespace [Tokens][crate::Token], each with their own discrete whitespace. Enabling this flag
	/// will enforce that the [Lexer][crate::Lexer] outputs these discrete tokens. In other words with this feature
	/// enabled, multiple contiguous whitespace tokens may be returned from subsequent calls to
	/// [Lexer::advance()][crate::Lexer::advance()], but with this feature off this will never be the case (as whitespace
	/// is collapsed into a single [Token][crate::Token]).
	///
	/// With this feature turned off whitespace-tokens are tokenized per the CSS specification:
	///
	/// ```md
	/// <newline>
	///  │├──╮─ "\n" ───╭──┤│
	///      ├─ "\r\n" ─┤
	///      ├─ "\r" ───┤
	///      ╰─ "\f" ───╯
	///
	/// <whitespace>
	///  │├──╮─ " " ───────╭──┤│
	///      ├─ "\t" ──────┤
	///      ╰─ <newline> ─╯
	///
	/// <whitespace-token>
	///  │├─╭─ <whitespace> ─╮─┤│
	///     ╰────────────────╯
	/// ```
	///
	/// With this feature turned on whitespace-tokens are tokenized with the additional grammar:
	///
	/// ```md
	/// <whitespace-token>
	///  │├──╮─╭─ " " ───────╮─╭──┤│
	///      │ ╰─────────────╯ │
	///      ├─╭─ "\t" ──────╮─┤
	///      │ ╰─────────────╯ │
	///      ╰─╭─ <newline> ─╮─╯
	///        ╰─────────────╯
	/// ```
	///
	/// [1]: https://drafts.csswg.org/css-syntax-3/#whitespace-diagram
	///
	/// # Example
	///
	/// ```
	/// use css_lexer::*;
	/// let mut lexer = Lexer::new("\n\thello world");
	/// {
	///		// This token will be collapsed Whitespace.
	///		let token = lexer.advance();
	///		assert_eq!(token, Kind::Whitespace);
	///		// The Whitespace is comprised of many bits:
	///		assert_eq!(token, Whitespace::Newline | Whitespace::Tab);
	/// }
	///	assert_eq!(lexer.advance(), Kind::Ident);
	/// {
	///		let token = lexer.advance();
	///		assert_eq!(token, Kind::Whitespace);
	///		assert_eq!(token, Whitespace::Space);
	/// }
	///	assert_eq!(lexer.advance(), Kind::Ident);
	///
	/// lexer = Lexer::new_with_features("\n\thello world", Feature::SeparateWhitespace);
	/// {
	///		// This token will be discrete Whitespace, just the `\n`.
	///		let token = lexer.advance();
	///		assert_eq!(token, Kind::Whitespace);
	///		// The Whitespace is comprised of a single bit:
	///		assert_eq!(token, Whitespace::Newline);
	/// }
	/// {
	///		// This token will be discrete Whitespace, just the `\t`.
	///		let token = lexer.advance();
	///		assert_eq!(token, Kind::Whitespace);
	///		// The Whitespace is comprised of a single bit:
	///		assert_eq!(token, Whitespace::Tab);
	/// }
	///	assert_eq!(lexer.advance(), Kind::Ident);
	/// {
	///		let token = lexer.advance();
	///		assert_eq!(token, Kind::Whitespace);
	///		assert_eq!(token, Whitespace::Space);
	/// }
	///	assert_eq!(lexer.advance(), Kind::Ident);
	/// ```
	SeparateWhitespace = 0b0010,
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Feature>(), 1);
}
