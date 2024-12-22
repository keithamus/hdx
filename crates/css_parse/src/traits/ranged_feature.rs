use crate::{diagnostics, Comparison, Parse, Parser, Peek, Result, T};

pub trait RangedFeatureKeyword {
	fn is_legacy(&self) -> bool {
		false
	}
}

/// This trait provides an implementation for parsing a ["Media Feature" in the "Range" context][1].
///
/// [1]: https://drafts.csswg.org/mediaqueries/#range-context
///
/// Rather than implementing this trait on an enum, use the [ranged_feature!][crate::ranged_feature] macro which
/// expands to define the enum and necessary traits ([Parse], this trait, and [ToCursors][crate::ToCursors]) in a
/// single macro call.
///
/// It does not implement [Parse], but provides `parse_ranged_feature(&mut Parser<'a>) -> Result<Self>`, which can make
/// for a trivial [Parse] implementation. The type [Self::FeatureName] must be defined, and represents the
/// `<feature-name>` token(s), while [Self::Value] represents the `<value>` token(s). The grammar of both `<value>` and
/// `<feature-name>` aren't mandated by this spec but are very likely be an `Ident` for the `<feature-name>` and either
/// a `Dimension` or `Number` for the `<value>` portion. [Self::FeatureName] must also implement the
/// [crate::RangedFeatureKeyword] trait which provides [RangedFeatureKeyword::is_legacy] to determine if the
/// `<feature-name>` is unambiguously a legacy "min-" or "max-" prefixed name, for ["legacy" ranged media conditions][2].
///
/// [2]: https://drafts.csswg.org/mediaqueries/#mq-min-max
///
/// CSS defines the Media Feature in Ranged context as:
///
/// ```md
///                                           ╭─ "="  ─╮
///                                           ├─ "<"  ─┤
///                                           ├─ "<=" ─┤
///                                           ├─ ">"  ─┤
///  │├─ "(" ─╮─ [<feature-name> or <value>] ─╯─ ">=" ─╰─ [<feature-name> or <value>] ─╭─ ")" ─┤│
///           ├────── <value> ─╮─ "<"  ─╭── <feature-name> ─╮─ "<"  ─╭── <value> ──────┤
///           │                ╰─ "<=" ─╯                   ╰─ "<=" ─╯                 │
///           ╰────── <value> ─╮─ ">"  ─╭── <feature-name> ─╮─ ">"  ─╭── <value> ──────╯
///                            ╰─ ">=" ─╯                   ╰─ ">=" ─╯
///
/// ```
///
/// This trait deviates slightly from the CSS spec ever so slightly for a few reasons:
///
/// - It uses a `<comparison>` token to represent each of the comparison operators, implemented as [Comparison]. This
///   makes for much more convenient parsing and subsequent analyses.
/// - The CSS defined railroad diagram doesn't quite fully convey that `<value> <comparison> <value>` and
///   `<feature-name> <comparison> <feature-name>` are not valid productions. This trait will fail to parse such
///   productions, as do all existing implementations of CSS (i.e browsers).
/// - It does not do the extra validation to ensure a left/right comparison are "directionally equivalent" - in other
///   words `<value> "<=" <feature-name> "=>" <value>` is a valid production in this trait - this allows for ASTs to
///   factor in error tolerance. If an AST node wishes to be strict, it can check the comparators inside of
/// [RangedFeature::new_ranged] and return an [Err] there.
/// - It supports the "Legacy" modes which are defined for certain ranged media features. These legacy productions use
///   a colon token and typically have `min` and `max` variants of the [RangedFeature::FeatureName]. For example
///   `width: 1024px` is equivalent to `width >= 1024px`, while `max-width: 1024px` is equivalent to
///   `max-width <= 1024px`. If an AST node wishes to _not_ support legacy feature-names, it can return an [Err] in
///   [RangedFeature::new_legacy].
///
/// Given the above differences, the trait `RangedFeature` parses a grammar defined as:
///
/// ```md
/// <comparison>
///  │├──╮─ "="  ─╭──┤│
///      ├─ "<"  ─┤
///      ├─ "<=" ─┤
///      ├─ ">"  ─┤
///      ╰─ ">=" ─╯
///
/// <ranged-feature-trait>
///  │├─ "(" ─╮─ <feature-name> ─ <comparison> ─ <value> ─────────────────────────────────╭─ ")" ─┤│
///           ├─ <value> ─ <comparison> ─ <ranged-feautre-name> ──────────────────────────┤
///           ├─ <value> ─ <comparison> ─ <ranged-feature-name> ─ <comparison> ─ <value> ─┤
///           ╰─ <feature-name> ─ ":" ─ <value> ──────────────────────────────────────────╯
///
/// ```
///
pub trait RangedFeature<'a>: Sized {
	type Value: Parse<'a>;
	type FeatureName: Peek<'a> + Parse<'a> + RangedFeatureKeyword;

	/// Method for constructing a "legacy" media feature. Legacy features always include a colon token.
	fn new_legacy(
		open: T!['('],
		name: Self::FeatureName,
		colon: T![:],
		value: Self::Value,
		close: T![')'],
	) -> Result<Self>;

	/// Method for constructing a "left" media feature. This method is called when the parsed tokens encountered
	/// the `<value>` token before the `<feature-name>`.
	fn new_left(
		open: T!['('],
		name: Self::FeatureName,
		comparison: Comparison,
		value: Self::Value,
		close: T![')'],
	) -> Result<Self>;

	/// Method for constructing a "right" media feature. This method is called when the parsed tokens
	/// encountered the `<feature-name>` token before the `<value>`.
	fn new_right(
		open: T!['('],
		value: Self::Value,
		comparison: Comparison,
		name: Self::FeatureName,
		close: T![')'],
	) -> Result<Self>;

	/// Method for constructing a "ranged" media feature. This method is called when the parsed tokens
	/// encountered the `<value>` token, followed by a `<comparison>`, followed by a `<feature-name>`, followed by a
	/// `<comparison>` followed lastly by a `<value>`.
	fn new_ranged(
		open: T!['('],
		left: Self::Value,
		left_comparison: Comparison,
		name: Self::FeatureName,
		right_comparison: Comparison,
		value: Self::Value,
		close: T![')'],
	) -> Result<Self>;

	fn parse_ranged_feature(p: &mut Parser<'a>) -> Result<Self> {
		let open = p.parse::<T!['(']>()?;
		let c = p.peek_next();
		if let Some(name) = p.parse_if_peek::<Self::FeatureName>()? {
			if p.peek::<T![:]>() {
				let colon = p.parse::<T![:]>()?;
				let value = p.parse::<Self::Value>()?;
				let close = p.parse::<T![')']>()?;
				return Self::new_legacy(open, name, colon, value, close);
			} else if name.is_legacy() {
				Err(diagnostics::UnexpectedIdent(p.parse_str(c.into()).into(), c.into()))?
			}
			let comparison = p.parse::<Comparison>()?;
			let value = p.parse::<Self::Value>()?;
			let close = p.parse::<T![')']>()?;
			return Self::new_left(open, name, comparison, value, close);
		}

		let left = p.parse::<Self::Value>()?;
		let left_comparison = p.parse::<Comparison>()?;
		let c = p.peek_next();
		let name = p.parse::<Self::FeatureName>()?;
		if name.is_legacy() {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		if !p.peek::<T![Delim]>() {
			let close = p.parse::<T![')']>()?;
			return Self::new_right(open, left, left_comparison, name, close);
		}
		let right_comparison = p.parse::<Comparison>()?;
		let right = p.parse::<Self::Value>()?;
		let close = p.parse::<T![')']>()?;
		Self::new_ranged(open, left, left_comparison, name, right_comparison, right, close)
	}
}

/// This macro expands to define an enum which already implements [Parse] and [RangedFeature], for a one-liner
/// definition of a [RangedFeature].
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use css_lexer::{Token, Kind};
/// use bumpalo::Bump;
///
/// // Defined the "FeatureName"
/// keyword_set!(TestKeyword { Thing: "thing", MaxThing: "max-thing", MinThing: "min-thing" });
/// impl RangedFeatureKeyword for TestKeyword {
///   fn is_legacy(&self) -> bool {
///     matches!(self, Self::MaxThing(_) | Self::MinThing(_))
///   }
/// }
///
/// // Define the Ranged Feature.
/// ranged_feature! {
///   /// A ranged media feature: (thing: 1), or (1 <= thing < 10)
///   TestFeature, TestKeyword, T![Number],
/// }
///
/// // Test!
/// assert_parse!(TestFeature, "(thing:2)");
/// assert_parse!(TestFeature, "(max-thing:2)");
/// assert_parse!(TestFeature, "(min-thing:2)");
/// assert_parse!(TestFeature, "(4<=thing>8)");
/// assert_parse!(TestFeature, "(thing>=2)");
///
/// assert_parse_error!(TestFeature, "(max-thing>2)");
/// assert_parse_error!(TestFeature, "(4<=max-thing<=8)");
/// ```
///
#[macro_export]
macro_rules! ranged_feature {
	($(#[doc = $usage:literal])* $feature: ident, $feature_name: ty, $value: ty $(,)*) => {
		#[rustfmt::skip] // removing this seems to cause the `Legacy` variants to oddly dedent
		$(#[doc = $usage])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $feature {
			Left($crate::T!['('], $feature_name, $crate::Comparison, $value, $crate::T![')']),
			Right($crate::T!['('], $value, $crate::Comparison, $feature_name, $crate::T![')']),
			Range($crate::T!['('], $value, $crate::Comparison, $feature_name, $crate::Comparison, $value, $crate::T![')']),
			Legacy($crate::T!['('], $feature_name, $crate::T![:], $value, $crate::T![')']),
		}

		impl<'a> $crate::Parse<'a> for $feature {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				use $crate::RangedFeature;
				Self::parse_ranged_feature(p)
			}
		}

		impl<'a> $crate::RangedFeature<'a> for $feature {
			type Value = $value;
			type FeatureName = $feature_name;

			fn new_legacy(
				open: $crate::T!['('],
				ident: Self::FeatureName,
				colon: $crate::T![:],
				value: Self::Value,
				close: $crate::T![')'],
			) -> $crate::Result<Self> {
				Ok(Self::Legacy(open, ident, colon, value, close))
			}

			fn new_left(
				open: $crate::T!['('],
				ident: Self::FeatureName,
				comparison: $crate::Comparison,
				value: Self::Value,
				close: $crate::T![')'],
			) -> $crate::Result<Self> {
				Ok(Self::Left(open, ident, comparison, value, close))
			}

			fn new_right(
				open: $crate::T!['('],
				value: Self::Value,
				comparison: $crate::Comparison,
				ident: Self::FeatureName,
				close: $crate::T![')'],
			) -> $crate::Result<Self> {
				Ok(Self::Right(open, value, comparison, ident, close))
			}

			fn new_ranged(
				open: $crate::T!['('],
				left: Self::Value,
				left_comparison: $crate::Comparison,
				ident: Self::FeatureName,
				right_comparison: $crate::Comparison,
				value: Self::Value,
				close: $crate::T![')'],
			) -> $crate::Result<Self> {
				Ok(Self::Range(open, left, left_comparison, ident, right_comparison, value, close))
			}
		}

		impl<'a> $crate::ToCursors for $feature {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				match self {
					Self::Left(open, ident, comparison, value, close) => {
						s.append(open.into());
						s.append(ident.into());
						$crate::ToCursors::to_cursors(comparison, s);
						$crate::ToCursors::to_cursors(value, s);
						s.append(close.into());
					}
					Self::Right(open, value, comparison, ident, close) => {
						s.append(open.into());
						$crate::ToCursors::to_cursors(value, s);
						$crate::ToCursors::to_cursors(comparison, s);
						s.append(ident.into());
						s.append(close.into());
					}
					Self::Range(open, left, left_comparison, ident, right_comparison, right, close) => {
						s.append(open.into());
						$crate::ToCursors::to_cursors(left, s);
						$crate::ToCursors::to_cursors(left_comparison, s);
						s.append(ident.into());
						$crate::ToCursors::to_cursors(right_comparison, s);
						$crate::ToCursors::to_cursors(right, s);
						s.append(close.into());
					}
					Self::Legacy(open, ident, colon, value, close) => {
						s.append(open.into());
						s.append(ident.into());
						s.append(colon.into());
						$crate::ToCursors::to_cursors(value, s);
						s.append(close.into());
					}
				}
			}
		}
	};
}
