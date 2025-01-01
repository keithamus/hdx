/// A macro for defining pseudo elements.
///
/// This makes it much easier to define a pseudo element, which would otherwise need to define a
/// [keyword_set][crate::keyword_set] or similar, in order to build up the two [Cursors][css_lexer::Cursor] required to
/// parse. Parsing is also a little bit delicate, as the two [Cursors][css_lexer::Cursor] must appear next to each
/// other - no whitespace nor comments can be present betwixt the colon and ident.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// pseudo_element!(
///   /// Some docs on this type...
///   MyPseudoElement {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(MyPseudoElement, "::FoO");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(MyPseudoElement, "::bar");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(MyPseudoElement, "::bing");
/// ```
#[macro_export]
macro_rules! pseudo_element {
	($(#[$meta:meta])*$name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $name {
			$($variant($crate::T![::], $crate::T![Ident]),)+
		}

		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> bool {
				let c2 = p.peek_n(2);
				let c3 = p.peek_n(3);
				c == ::css_lexer::Kind::Colon
				&& c2 == ::css_lexer::Kind::Colon
				&& c3 == ::css_lexer::Kind::Ident
				&& Self::MAP.get(&p.parse_str_lower(c3)).is_some()
			}
		}

		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let colons = p.parse::<$crate::T![::]>()?;
				let skip = p.set_skip(::css_lexer::KindSet::NONE);
				let ident = p.parse::<$crate::T![Ident]>();
				p.set_skip(skip);
				let ident = ident?;
				if let Some(val) = Self::MAP.get(&p.parse_str_lower(ident.into())) {
					match val {
						$(Self::$variant(_, _) => Ok(Self::$variant(colons, ident)),)+
					}
				} else {
					Err($crate::diagnostics::UnexpectedIdent(p.parse_str(ident.into()).into(), ident.into()))?
				}
			}
		}

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				match self {
					$(Self::$variant(colons, ident) => {
						$crate::ToCursors::to_cursors(colons, s);
						s.append(ident.into());
					})+
				}
			}
		}

		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
					$($variant_str => $name::$variant(<$crate::T![::]>::dummy(), <$crate::T![Ident]>::dummy()),)+
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
