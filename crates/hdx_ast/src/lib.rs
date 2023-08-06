#![feature(slice_concat_trait)]

extern crate hdx_atomizable_derive;

pub use hdx_atomizable_derive::Atomizable;
#[cfg(feature = "serde")]
use serde::Serialize;

pub mod css;
pub mod traits;

pub(crate) use hdx_atom::{atom, Atom, Atomizable};
pub(crate) use hdx_lexer::{PairWise, Span, Token};
pub(crate) use oxc_allocator::{Allocator, Box, Vec};
pub use traits::Unit;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Spanned<T> {
	pub node: T,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub span: Span,
}

impl<T> Spanned<T> {
	pub fn dummy(node: T) -> Self {
		Self { node, span: Span::dummy() }
	}
}

impl<T: Atomizable> Atomizable for Spanned<T> {
	fn from_atom(atom: Atom) -> Option<Self> {
		T::from_atom(atom).map(|node| Self { node, span: Span::dummy() })
	}

	fn to_atom(&self) -> Atom {
		self.node.to_atom()
	}
}

impl<T: ToSpecificity> ToSpecificity for Spanned<T> {
	fn specificity(&self) -> Specificity {
		self.node.specificity()
	}
}

impl<T: Default> Default for Spanned<T> {
	fn default() -> Self {
		Self::dummy(T::default())
	}
}

pub trait ToSpecificity: Sized {
	fn specificity(&self) -> Specificity;
}

#[derive(Debug, PartialEq, Hash)]
pub struct Specificity(u8, u8, u8);

impl std::ops::AddAssign for Specificity {
	fn add_assign(&mut self, other: Self) {
		self.0 |= other.0;
		self.1 |= other.1;
		self.2 |= other.2;
	}
}
