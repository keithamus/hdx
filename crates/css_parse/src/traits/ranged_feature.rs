use super::prelude::*;
use crate::{AtomSet, Comparison, Result};

/// This trait provides an implementation for parsing a ["Media Feature" in the "Range" context][1].
///
/// [1]: https://drafts.csswg.org/mediaqueries/#range-context
///
/// Rather than implementing this trait on an enum, use the [ranged_feature!][crate::ranged_feature] macro which
/// expands to define the enum and necessary traits ([Parse], this trait, and [ToCursors][crate::ToCursors]) in a
/// single macro call.
///
/// It does not implement [Parse], but provides `parse_ranged_feature(&mut Parser<'a>) -> Result<Self>`, which can make
/// for a trivial [Parse] implementation. The type  [Self::Value] represents the `<value>` token(s). The grammar of both
/// `<value>` isn't mandated by this spec but is very likely a `Dimension` or `Number`. The `<feature-name>` is
/// determined by the three given arguments to `parse_ranged_feature` - each must implement AtomSet, so they can be
/// compared to the given ident atom in that position. Passing the third and fourth arguments for min & max atoms allows
/// the "legacy" min/max variants to be parsed also.
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
///   [RangedFeature::new_ranged] and return an [Err] there.
/// - It supports the "Legacy" modes which are defined for certain ranged media features. These legacy productions use
///   a colon token and typically have `min` and `max` variants. For example `width: 1024px` is equivalent to
///   `width >= 1024px`, while `max-width: 1024px` is equivalent to `max-width <= 1024px`. If an AST node wishes to
///   _not_ support legacy feature-names, it can supply `None`s to [RangedFeature::parse_ranged_feature].
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

	/// Method for constructing a "legacy max" media feature. Legacy features always include a colon token.
	fn new_max(_open: T!['('], name: T![Ident], _colon: T![:], _value: Self::Value, _close: T![')']) -> Result<Self> {
		Err(Diagnostic::new(name.into(), Diagnostic::unexpected_ident))?
	}

	/// Method for constructing a "legacy min" media feature. Legacy features always include a colon token.
	fn new_min(_open: T!['('], name: T![Ident], _colon: T![:], _value: Self::Value, _close: T![')']) -> Result<Self> {
		Err(Diagnostic::new(name.into(), Diagnostic::unexpected_ident))?
	}

	/// Method for constructing a "exact" media feature. Exact features always include a colon token.
	fn new_exact(open: T!['('], name: T![Ident], colon: T![:], value: Self::Value, close: T![')']) -> Result<Self>;

	/// Method for constructing a "left" media feature. This method is called when the parsed tokens encountered
	/// the `<value>` token before the `<feature-name>`.
	fn new_left(
		open: T!['('],
		name: T![Ident],
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
		name: T![Ident],
		close: T![')'],
	) -> Result<Self>;

	/// Method for constructing a "ranged" media feature. This method is called when the parsed tokens
	/// encountered the `<value>` token, followed by a `<comparison>`, followed by a `<feature-name>`, followed by a
	/// `<comparison>` followed lastly by a `<value>`.
	fn new_ranged(
		open: T!['('],
		left: Self::Value,
		left_comparison: Comparison,
		name: T![Ident],
		right_comparison: Comparison,
		value: Self::Value,
		close: T![')'],
	) -> Result<Self>;

	fn parse_ranged_feature<I, A: AtomSet + PartialEq>(
		p: &mut Parser<'a, I>,
		name: &A,
		min: Option<&A>,
		max: Option<&A>,
	) -> Result<Self>
	where
		I: Iterator<Item = crate::Cursor> + Clone,
	{
		let open = p.parse::<T!['(']>()?;
		let c = p.peek_n(1);
		if <T![Ident]>::peek(p, c) {
			let atom = p.to_atom::<A>(c);
			let ident = p.parse::<T![Ident]>()?;
			if <T![:]>::peek(p, p.peek_n(1)) {
				let colon = p.parse::<T![:]>()?;
				let value = p.parse::<Self::Value>()?;
				let close = p.parse::<T![')']>()?;
				if &atom == name {
					return Self::new_exact(open, ident, colon, value, close);
				} else if min.is_some_and(|min| &atom == min) {
					return Self::new_min(open, ident, colon, value, close);
				} else if max.is_some_and(|max| &atom == max) {
					return Self::new_max(open, ident, colon, value, close);
				} else {
					Err(Diagnostic::new(c, Diagnostic::unexpected_ident))?
				}
			}
			if &atom != name {
				Err(Diagnostic::new(c, Diagnostic::unexpected_ident))?
			}
			let comparison = p.parse::<Comparison>()?;
			let value = p.parse::<Self::Value>()?;
			let close = p.parse::<T![')']>()?;
			return Self::new_left(open, ident, comparison, value, close);
		}

		let left = p.parse::<Self::Value>()?;
		let left_comparison = p.parse::<Comparison>()?;
		let c = p.peek_n(1);
		let ident = p.parse::<T![Ident]>()?;
		if &p.to_atom::<A>(ident.into()) != name {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?
		}
		if !<T![Delim]>::peek(p, p.peek_n(1)) {
			let close = p.parse::<T![')']>()?;
			return Self::new_right(open, left, left_comparison, ident, close);
		}
		let right_comparison = p.parse::<Comparison>()?;
		let right = p.parse::<Self::Value>()?;
		let close = p.parse::<T![')']>()?;
		Self::new_ranged(open, left, left_comparison, ident, right_comparison, right, close)
	}
}

/// Looks ahead from a ranged feature's opening `(` to its `<feature-name>`, skipping any leading `<value>`, including
/// nested blocks such as the `calc()` in `(calc(1px + 2px) < width)`. Returns the cursor of the first top-level ident,
/// or of the closing `)` or EOF when there is none. Lookahead stops at the `u8::MAX`th token, returning that cursor.
pub fn peek_ranged_feature_name<'a, I>(p: &Parser<'a, I>) -> Cursor
where
	I: Iterator<Item = Cursor> + Clone,
{
	let mut n = 2;
	let mut depth = 0u32;
	loop {
		let c = p.peek_n(n);
		if c == crate::Kind::Eof || (depth == 0 && (c == crate::Kind::Ident || c == crate::Kind::RightParen)) {
			return c;
		}
		if c == crate::Kind::Function || c == crate::Kind::LeftParen {
			depth += 1;
		} else if c == crate::Kind::RightParen {
			depth -= 1;
		}
		let Some(next) = n.checked_add(1) else {
			return c;
		};
		n = next;
	}
}

/// This macro expands to define an enum which already implements [Parse] and [RangedFeature], for a one-liner
/// definition of a [RangedFeature].
///
/// # Examples
///
/// ## No Legacy syntax
///
/// ```
/// use css_parse::*;
/// use csskit_derives::{ToCursors, ToSpan};
/// use derive_atom_set::AtomSet;
///
/// #[derive(Debug, Default, AtomSet, Copy, Clone, PartialEq)]
/// pub enum MyAtomSet {
///   #[default]
///   _None,
///   Thing,
///   MaxThing,
///   MinThing,
/// }
/// impl MyAtomSet {
///   const ATOMS: MyAtomSet = MyAtomSet::_None;
/// }
///
/// // Define the Ranged Feature.
/// ranged_feature! {
///   /// A ranged media feature: (thing: 1), or (1 <= thing < 10)
///   #[derive(ToCursors, ToSpan, Debug)]
///   pub enum TestFeature{MyAtomSet::Thing, T![Number]}
/// }
///
/// // Test!
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(thing:2)");
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(4<=thing>8)");
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(thing>=2)");
///
/// assert_parse_error!(MyAtomSet::ATOMS, TestFeature, "(max-thing>2)");
/// assert_parse_error!(MyAtomSet::ATOMS, TestFeature, "(4<=max-thing<=8)");
/// assert_parse_error!(MyAtomSet::ATOMS, TestFeature, "(max-thing:2)");
/// assert_parse_error!(MyAtomSet::ATOMS, TestFeature, "(min-thing:2)");
/// ```
///
/// ## With legacy syntax
///
/// ```
/// use css_parse::*;
/// use csskit_derives::*;
/// use derive_atom_set::*;
///
/// #[derive(Debug, Default, AtomSet, Copy, Clone, PartialEq)]
/// pub enum MyAtomSet {
///   #[default]
///   _None,
///   Thing,
///   MaxThing,
///   MinThing,
/// }
/// impl MyAtomSet {
///   const ATOMS: MyAtomSet = MyAtomSet::_None;
/// }
///
/// // Define the Ranged Feature.
/// ranged_feature! {
///   /// A ranged media feature: (thing: 1), or (1 <= thing < 10)
///   #[derive(Debug, ToCursors, ToSpan)]
///   pub enum TestFeature{MyAtomSet::Thing | MyAtomSet::MinThing | MyAtomSet::MaxThing, T![Number]}
/// }
///
/// // Test!
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(thing:2)");
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(4<=thing>8)");
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(thing>=2)");
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(max-thing:2)");
/// assert_parse!(MyAtomSet::ATOMS, TestFeature, "(min-thing:2)");
///
/// assert_parse_error!(MyAtomSet::ATOMS, TestFeature, "(max-thing>2)");
/// assert_parse_error!(MyAtomSet::ATOMS, TestFeature, "(4<=max-thing<=8)");
/// ```
#[macro_export]
macro_rules! ranged_feature {
	(@parse_call $p:ident, $feature_name:path) => {
		Self::parse_ranged_feature($p, &$feature_name, None, None)
	};
	(@parse_call $p:ident, $feature_name:path, $min_name:path, $max_name:path) => {
		Self::parse_ranged_feature($p, &$feature_name, Some(&$min_name), Some(&$max_name))
	};
	($(#[$meta:meta])* $vis:vis enum $feature: ident $(<$lt:lifetime>)? {$feature_name: path $(| $min_name: path | $max_name: path)?, $value: ty}) => {
		$(#[$meta])*
		$vis enum $feature $(<$lt>)? {
			Left($crate::T!['('], T![Ident], $crate::Comparison, $value, $crate::T![')']),
			Right($crate::T!['('], $value, $crate::Comparison, T![Ident], $crate::T![')']),
			Range($crate::T!['('], $value, $crate::Comparison, T![Ident], $crate::Comparison, $value, $crate::T![')']),
			$(
				#[doc = stringify!($min_name)]
				Min($crate::T!['('], T![Ident], $crate::T![:], $value, $crate::T![')']),
				#[doc = stringify!($max_name)]
				Max($crate::T!['('], T![Ident], $crate::T![:], $value, $crate::T![')']),
			)?
			Exact($crate::T!['('], T![Ident], $crate::T![:], $value, $crate::T![')']),
		}

		impl<'a> $crate::Peek<'a> for $feature $(<$lt>)? {
			fn peek<Iter>(p: &$crate::Parser<'a, Iter>, c: $crate::Cursor) -> bool
			where
				Iter: Iterator<Item = $crate::Cursor> + Clone,
			{
				c == $crate::Kind::LeftParen && p.peek_n(2) == $crate::KindSet::new(&[$crate::Kind::Ident, $crate::Kind::Number, $crate::Kind::Dimension, $crate::Kind::Function])
			}
		}

		impl<'a> $crate::Parse<'a> for $feature $(<$lt>)? {
			fn parse<I>(p: &mut $crate::Parser<'a, I>) -> $crate::Result<Self>
			where
				I: Iterator<Item = $crate::Cursor> + Clone,
			{
				use $crate::RangedFeature;
				$crate::ranged_feature! {@parse_call p, $feature_name $(, $min_name, $max_name)?}
			}
		}

		impl<'a> $crate::RangedFeature<'a> for $feature $(<$lt>)? {
			type Value = $value;

			$(
				#[doc = stringify!($max_name)]
				fn new_max(
					open: $crate::T!['('],
					ident: T![Ident],
					colon: $crate::T![:],
					value: Self::Value,
					close: $crate::T![')'],
				) -> $crate::Result<Self> {
					Ok(Self::Max(open, ident, colon, value, close))
				}

				#[doc = stringify!($min_name)]
				fn new_min(
					open: $crate::T!['('],
					ident: T![Ident],
					colon: $crate::T![:],
					value: Self::Value,
					close: $crate::T![')'],
				) -> $crate::Result<Self> {
					Ok(Self::Min(open, ident, colon, value, close))
				}
			)?

			fn new_exact(
				open: $crate::T!['('],
				ident: T![Ident],
				colon: $crate::T![:],
				value: Self::Value,
				close: $crate::T![')'],
			) -> $crate::Result<Self> {
				Ok(Self::Exact(open, ident, colon, value, close))
			}

			fn new_left(
				open: $crate::T!['('],
				ident: T![Ident],
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
				ident: T![Ident],
				close: $crate::T![')'],
			) -> $crate::Result<Self> {
				Ok(Self::Right(open, value, comparison, ident, close))
			}

			fn new_ranged(
				open: $crate::T!['('],
				left: Self::Value,
				left_comparison: $crate::Comparison,
				ident: T![Ident],
				right_comparison: $crate::Comparison,
				value: Self::Value,
				close: $crate::T![')'],
			) -> $crate::Result<Self> {
				Ok(Self::Range(open, left, left_comparison, ident, right_comparison, value, close))
			}
		}
	};
}
