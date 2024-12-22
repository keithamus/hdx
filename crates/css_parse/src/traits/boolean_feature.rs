use css_lexer::Cursor;

use crate::{diagnostics, Parser, Result, T};

/// This trait provides an implementation for parsing a ["Media Feature" in the "Boolean" context][1]. This is
/// complementary to the other media features: [RangedFeature][crate::RangedFeature] and
/// [DiscreteFeature][crate::DiscreteFeature].
///
///	[1]: https://drafts.csswg.org/mediaqueries/#boolean-context
///
///	Rather than implementing this trait on an enum, use the [boolean_feature!][crate::boolean_feature] macro which
///	expands to define the enum and necessary traits ([Parse][crate::Parse], this trait, and
///	[ToCursors][crate::ToCursors]) in a single macro call.
///
/// It does not implement [Parse][crate::Parse], but provides
/// `parse_boolean_feature(&mut Parser<'a>, name: &str) -> Result<Self>`, which can make for a trivial
/// [Parse][crate::Parse] implementation. The `name: &str` parameter refers to the `<feature-name>` token, which will
/// be parsed as an Ident.
///
/// CSS defines the Media Feature generally as:
///
/// ```md
///  │├─ "(" ─╮─ <feature-name> ─ ":" ─ <value> ─╭─ ")" ─┤│
///           ├─ <feature-name> ─────────────────┤
///           ╰─ <ranged-feature> ───────────────╯
///
/// ```
///
/// The [RangedFeature][crate::RangedFeature] trait provides algorithms for parsing `<ranged-feature>` productions, but
/// boolean features use the other two productions, with some rules around the `<value>`.
///
/// A boolean media query:
///
/// - Can omit the the `:` and `<value>`.
/// - Must allow any token as the `<value>`, but the `<dimension>` of `0`, `<number>` of `0` and `<ident>` of `none`
/// will mean the query evaluates to false.
///
/// Given these, this trait parses as:
///
/// ```md
/// <boolean-feature>
///  │├─ "(" ─╮─ <feature-name> ─ ":" ─ <any> ─╭─ ")" ─┤│
///           ╰─ <feature-name> ───────────────╯
///
/// ```
///
pub trait BooleanFeature<'a>: Sized {
	fn parse_boolean_feature(
		p: &mut Parser<'a>,
		name: &'static str,
	) -> Result<(T!['('], T![Ident], Option<(T![:], T![Any])>, T![')'])> {
		let open = p.parse::<T!['(']>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		if !p.eq_ignore_ascii_case(c, name) {
			Err(diagnostics::ExpectedIdentOf(name, p.parse_str(c).into(), c.into()))?
		}
		if p.peek::<T![:]>() {
			let colon = p.parse::<T![:]>()?;
			let value = p.parse::<T![Any]>()?;
			let close = p.parse::<T![')']>()?;
			return Ok((open, ident, Some((colon, value)), close));
		} else {
			let close = p.parse::<T![')']>()?;
			Ok((open, ident, None, close))
		}
	}
}

/// This macro expands to define an enum which already implements [Parse][crate::Parse] and [BooleanFeature], for a
/// one-liner definition of a [BooleanFeature].
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use css_lexer::{Token, Kind};
/// use bumpalo::Bump;
///
/// // Define the Boolean Feature.
/// boolean_feature! {
///		/// A boolean media feature: `(test-feature)`
///		TestFeature, "test-feature"
///	}
///
///	// Test!
/// let allocator = Bump::new();
/// let mut p = Parser::new(&allocator, "(test-feature)");
/// let result = p.parse_entirely::<TestFeature>();
/// assert!(matches!(result.output, Some(TestFeature::Bare(open, ident, close))));
///
/// let mut p = Parser::new(&allocator, "(test-feature: none)");
/// let result = p.parse_entirely::<TestFeature>();
/// assert!(matches!(result.output, Some(TestFeature::WithValue(open, ident, colon, any, close))));
/// ```
///
#[macro_export]
macro_rules! boolean_feature {
	($(#[doc = $usage:literal])* $feature: ident, $feature_name: tt $(,)*) => {
		$(#[doc = $usage])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $feature {
			WithValue($crate::T!['('], $crate::T![Ident], $crate::T![:], $crate::T![Any], $crate::T![')']),
			Bare($crate::T!['('], $crate::T![Ident], $crate::T![')']),
		}

		impl<'a> $crate::Parse<'a> for $feature {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				use $crate::BooleanFeature;
				let (open, ident, opt, close) = Self::parse_boolean_feature(p, $feature_name)?;
				if let Some((colon, number)) = opt {
					Ok(Self::WithValue(open, ident, colon, number, close))
				} else {
					Ok(Self::Bare(open, ident, close))
				}
			}
		}

		impl<'a> $crate::BooleanFeature<'a> for $feature {}

		impl<'a> $crate::ToCursors for $feature {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				match self {
					$feature::WithValue(open, ident, colon, any, close) => {
						s.append(open.into());
						s.append(ident.into());
						s.append(colon.into());
						s.append(any.into());
						s.append(close.into());
					}
					$feature::Bare(open, ident, close) => {
						s.append(open.into());
						s.append(ident.into());
						s.append(close.into());
					}
				}
			}
		}
	};
}
