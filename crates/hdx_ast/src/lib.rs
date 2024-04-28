extern crate hdx_derive;

pub mod css;
pub mod macros;
pub mod syntax;
pub mod traits;

#[cfg(test)]
pub mod test_helpers;

extern crate self as hdx_ast;
use hdx_parser::Spanned;
pub use traits::Value;

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
