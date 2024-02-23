extern crate hdx_derive;

pub mod css;
pub mod traits;
pub mod macros;
pub mod syntax;

#[cfg(test)]
pub mod test_helpers;

pub(crate) use bitmask_enum::bitmask;
pub(crate) use hdx_atom::{atom, Atom, Atomizable};
pub(crate) use hdx_derive::{Atomizable, Parsable, Writable};
pub(crate) use hdx_parser::{Box, Spanned, Vec};
pub(crate) use macros::*;
pub use traits::Unit;

pub trait ToSpecificity: Sized {
	fn specificity(&self) -> Specificity;
}

impl<T: ToSpecificity> ToSpecificity for Spanned<T> {
	fn specificity(&self) -> Specificity {
		self.node.specificity()
	}
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
