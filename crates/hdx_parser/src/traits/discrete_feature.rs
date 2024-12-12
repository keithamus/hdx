use hdx_atom::Atom;
use hdx_lexer::Cursor;

use crate::{diagnostics, Parser, Result, T};

pub trait DiscreteFeature<'a>: Sized {
	fn parse_descrete_feature(name: Atom, p: &mut Parser<'a>) -> Result<(T![Ident], Option<(T![:], T![Ident])>)> {
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let atom = p.parse_atom_lower(c);
		if atom != name {
			Err(diagnostics::ExpectedIdentOf(name, atom, c.into()))?
		}
		if p.peek::<T![:]>() {
			let colon = p.parse::<T![:]>()?;
			let value = p.parse::<T![Ident]>()?;
			Ok((ident, Some((colon, value))))
		} else {
			Ok((ident, None))
		}
	}
}

#[macro_export]
macro_rules! discrete_feature {
	($feat: tt[atom!($atom: tt)] { $( $name: ident: atom!($name_atom: tt),)+ }) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $feat {
			Bare($crate::T![Ident]),
			$( $name($crate::T![Ident], $crate::T![:], $crate::T![Ident]), )+
		}

		impl<'a> $crate::Parse<'a> for $feat {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				use $crate::DiscreteFeature;
				let (ident, opt) = Self::parse_descrete_feature(hdx_atom::atom!($atom), p)?;
				if let Some((colon, value)) = opt {
					let c: ::hdx_lexer::Cursor = value.into();
					match p.parse_atom_lower(c) {
						$(
							::hdx_atom::atom!($name_atom) => Ok(Self::$name(ident, colon, value)),
						)+
						atom => Err($crate::diagnostics::UnexpectedIdent(atom, c.into()))?
					}
				} else {
					Ok(Self::Bare(ident))
				}
			}
		}

		impl<'a> $crate::DiscreteFeature<'a> for $feat {}

		impl<'a> $crate::ToCursors<'a> for $feat {
			fn to_cursors(&self, s: &mut $crate::CursorStream<'a>) {
				match self {
					$(
						Self::$name(ident, colon, number) => {
							s.append(ident.into());
							s.append(colon.into());
							s.append(number.into());
						},
					)+
					$feat::Bare(c) => s.append(c.into()),
				}
			}
		}
	};
}
