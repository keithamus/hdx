/// A macro for defining pseudo classes.
///
/// This makes it much easier to define a pseudo class, which would otherwise need to define a
/// [keyword_set][crate::keyword_set] or similar, in order to build up the two [Cursors][css_lexer::Cursor] required to
/// parse. Parsing is also a little bit delicate, as the two [Cursors][css_lexer::Cursor] must appear next to each
/// other - no whitespace nor comments can be present betwixt the colon and ident.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// pseudo_class!(
///   /// Some docs on this type...
///   MyPseudoClass {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(MyPseudoClass, ":foo");
///
/// // Matches are case insensitive
/// assert_parse!(MyPseudoClass, ":BaR");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(MyPseudoClass, ":bing");
///
/// // The `:` is also required
/// assert_parse_error!(MyPseudoClass, "baz");
///
/// // Any tokens between the `:` and ident result in a parse error:
/// assert_parse_error!(MyPseudoClass, ": foo");
/// ```
#[macro_export]
macro_rules! pseudo_class {
	($(#[doc = $usage:literal])*$name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[doc = $usage])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $name {
			$($variant($crate::T![:], $crate::T![Ident]),)+
		}

		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> bool {
				let c2 = p.peek_n(2);
				c == ::css_lexer::Kind::Colon && (c2 == ::css_lexer::Kind::Ident && Self::MAP.get(&p.parse_str_lower(c2)).is_some())
			}
		}

		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let colon = p.parse::<$crate::T![:]>()?;
				let skip = p.set_skip(::css_lexer::KindSet::NONE);
				let ident = p.parse::<$crate::T![Ident]>();
				p.set_skip(skip);
				let ident = ident?;
				if let Some(val) = Self::MAP.get(&p.parse_str_lower(ident.into())) {
					match val {
						$(Self::$variant(_, _) => Ok(Self::$variant(colon, ident)),)+
					}
				} else {
					Err($crate::diagnostics::UnexpectedIdent(p.parse_str(ident.into()).into(), ident.into()))?
				}
			}
		}

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				match self {
					$(Self::$variant(colon, ident) => {
						s.append(colon.into());
						s.append(ident.into());
					})+
				}
			}
		}

		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
					$($variant_str => $name::$variant(<$crate::T![:]>::dummy(), <$crate::T![Ident]>::dummy()),)+
			};
		}

		impl From<$name> for css_lexer::Span {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(a, b) => Into::<::css_lexer::Span>::into(a) + b.into(),)+
				}
			}
		}

		impl From<&$name> for css_lexer::Span {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(a, b) => Into::<::css_lexer::Span>::into(a) + b.into(),)+
				}
			}
		}
	}
}
