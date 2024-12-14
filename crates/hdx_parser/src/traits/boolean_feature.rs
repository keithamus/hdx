use hdx_atom::Atom;
use hdx_lexer::Cursor;

use crate::{diagnostics, Parser, Result, T};

pub trait BooleanFeature<'a>: Sized {
	fn parse_boolean_feature(p: &mut Parser<'a>, name: Atom) -> Result<(T![Ident], Option<(T![:], T![Number])>)> {
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let atom = p.parse_atom_lower(c);
		if atom != name {
			Err(diagnostics::ExpectedIdentOf(name, atom, c.into()))?
		}
		if p.peek::<T![:]>() {
			let colon = p.parse::<T![:]>()?;
			let number = p.parse::<T![Number]>()?;
			let c: Cursor = number.into();
			if c.token().is_int() {
				let val = c.token().value();
				if val == 1.0 || val == 0.0 {
					return Ok((ident, Some((colon, number))));
				}
			}
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		} else {
			Ok((ident, None))
		}
	}
}

#[macro_export]
macro_rules! bool_feature {
	($feat: tt[atom!($atom: tt)]) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $feat {
	Zero($crate::T![Ident], $crate::T![:], $crate::T![Number]),
	One($crate::T![Ident], $crate::T![:], $crate::T![Number]),
			Bare($crate::T![Ident]),
		}

		impl<'a> $crate::Parse<'a> for $feat {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				use $crate::BooleanFeature;
				let (ident, opt) = Self::parse_boolean_feature(p, hdx_atom::atom!($atom))?;
				if let Some((colon, number)) = opt {
					let c: hdx_lexer::Cursor = number.into();
					if c.token().value() == 1.0 {
						Ok(Self::Zero(ident, colon, number))
					} else {
						Ok(Self::Zero(ident, colon, number))
					}
				} else {
					Ok(Self::Bare(ident))
				}
			}
		}

		impl<'a> $crate::BooleanFeature<'a> for $feat {}

		impl<'a> $crate::ToCursors for $feat {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				match self {
					$feat::Zero(ident, colon, number) => {
						s.append(ident.into());
						s.append(colon.into());
						s.append(number.into());
					}
					$feat::One(ident, colon, number) => {
						s.append(ident.into());
						s.append(colon.into());
						s.append(number.into());
					}
					$feat::Bare(c) => s.append(c.into()),
				}
			}
		}
	};
}
