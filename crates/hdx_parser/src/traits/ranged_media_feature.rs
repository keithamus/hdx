use hdx_atom::Atom;
use hdx_lexer::Cursor;

use crate::{diagnostics, Comparison, Parse, Parser, Result, T};

// https://drafts.csswg.org/mediaqueries/#mq-range-context
pub trait RangedMediaFeature<'a>: Sized {
	type Type: Parse<'a>;

	fn new_legacy(atom: Atom, ident: T![Ident], colon: T![:], value: Self::Type) -> Self;
	fn new_left(ident: T![Ident], comparison: Comparison, value: Self::Type) -> Self;
	fn new_right(value: Self::Type, comparison: Comparison, ident: T![Ident]) -> Self;
	fn new_ranged(
		left: Self::Type,
		left_comparison: Comparison,
		ident: T![Ident],
		right_comparison: Comparison,
		value: Self::Type,
	) -> Self;

	fn parse_ranged_media_feature(p: &mut Parser<'a>, name: Atom) -> Result<Self> {
		if p.peek::<T![Ident]>() {
			let ident = p.parse::<T![Ident]>()?;
			let c: Cursor = ident.into();
			match p.parse_atom_lower(c) {
				atom if atom == name => {
					if p.peek::<T![:]>() {
						let colon = p.parse::<T![:]>()?;
						let value = p.parse::<Self::Type>()?;
						return Ok(Self::new_legacy(atom, ident, colon, value));
					}
				}
				atom if atom.strip_prefix("max-").unwrap_or("") == name.as_ref() => {
					let colon = p.parse::<T![:]>()?;
					let value = p.parse::<Self::Type>()?;
					return Ok(Self::new_legacy(atom, ident, colon, value));
				}
				atom if atom.strip_prefix("min-").unwrap_or("") == name.as_ref() => {
					let colon = p.parse::<T![:]>()?;
					let value = p.parse::<Self::Type>()?;
					return Ok(Self::new_legacy(atom, ident, colon, value));
				}
				atom => Err(diagnostics::ExpectedIdentOf(name, atom, c.into()))?,
			};
			let comparison = p.parse::<Comparison>()?;
			let value = p.parse::<Self::Type>()?;
			return Ok(Self::new_left(ident, comparison, value));
		}

		let left = p.parse::<Self::Type>()?;
		let left_comparison = p.parse::<Comparison>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let atom = p.parse_atom_lower(c);
		if atom != name {
			Err(diagnostics::ExpectedIdentOf(name, atom, c.into()))?
		}
		if !p.peek::<T![Delim]>() {
			return Ok(Self::new_right(left, left_comparison, ident));
		}
		let right_comparison = p.parse::<Comparison>()?;
		let right = p.parse::<Self::Type>()?;
		Ok(Self::new_ranged(left, left_comparison, ident, right_comparison, right))
	}
}

#[macro_export]
macro_rules! ranged_media_feature {
	($feat: tt[atom!($atom: tt)], $ty: ty) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $feat {
			Left($crate::T![Ident], $crate::Comparison, $ty),
			Right($ty, $crate::Comparison, $crate::T![Ident]),
			Range($ty, $crate::Comparison, $crate::T![Ident], $crate::Comparison, $ty),
	LegacyMax($crate::T![Ident], $crate::T![:], $ty),
	LegacyMin($crate::T![Ident], $crate::T![:], $ty),
	Legacy($crate::T![Ident], $crate::T![:], $ty),
		}

		impl<'a> $crate::Parse<'a> for $feat {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				use $crate::RangedMediaFeature;
				Self::parse_ranged_media_feature(p, hdx_atom::atom!($atom))
			}
		}

		impl<'a> $crate::RangedMediaFeature<'a> for $feat {
			type Type = $ty;

			fn new_legacy(
				atom: hdx_atom::Atom,
				ident: $crate::T![Ident],
				colon: $crate::T![:],
				value: Self::Type,
			) -> Self {
				if atom.starts_with("max-") {
					Self::LegacyMax(ident, colon, value)
				} else if atom.starts_with("min-") {
					Self::LegacyMin(ident, colon, value)
				} else {
					Self::Legacy(ident, colon, value)
				}
			}

			fn new_left(ident: $crate::T![Ident], comparison: $crate::Comparison, value: Self::Type) -> Self {
				Self::Left(ident, comparison, value)
			}

			fn new_right(value: Self::Type, comparison: $crate::Comparison, ident: $crate::T![Ident]) -> Self {
				Self::Right(value, comparison, ident)
			}

			fn new_ranged(
				left: Self::Type,
				left_comparison: $crate::Comparison,
				ident: $crate::T![Ident],
				right_comparison: $crate::Comparison,
				value: Self::Type,
			) -> Self {
				Self::Range(left, left_comparison, ident, right_comparison, value)
			}
		}

		impl<'a> $crate::ToCursors<'a> for $feat {
			fn to_cursors(&self, s: &mut $crate::CursorStream<'a>) {
				match self {
					Self::Left(ident, comparison, value) => {
						s.append(ident.into());
						$crate::ToCursors::to_cursors(comparison, s);
						$crate::ToCursors::to_cursors(value, s);
					}
					Self::Right(value, comparison, ident) => {
						$crate::ToCursors::to_cursors(value, s);
						$crate::ToCursors::to_cursors(comparison, s);
						s.append(ident.into());
					}
					Self::Range(left, left_comparison, ident, right_comparison, right) => {
						$crate::ToCursors::to_cursors(left, s);
						$crate::ToCursors::to_cursors(left_comparison, s);
						s.append(ident.into());
						$crate::ToCursors::to_cursors(right_comparison, s);
						$crate::ToCursors::to_cursors(right, s);
					}
					Self::LegacyMax(ident, colon, value) => {
						s.append(ident.into());
						s.append(colon.into());
						$crate::ToCursors::to_cursors(value, s);
					}
					Self::LegacyMin(ident, colon, value) => {
						s.append(ident.into());
						s.append(colon.into());
						$crate::ToCursors::to_cursors(value, s);
					}
					Self::Legacy(ident, colon, value) => {
						s.append(ident.into());
						s.append(colon.into());
						$crate::ToCursors::to_cursors(value, s);
					}
				}
			}
		}
	};
}
