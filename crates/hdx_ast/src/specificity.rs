use core::iter::Sum;
use core::ops;

pub trait ToSpecificity: Sized {
	fn specificity(&self) -> Specificity;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Specificity(pub u8, pub u8, pub u8);

impl ops::AddAssign for Specificity {
	fn add_assign(&mut self, other: Self) {
		self.0 |= other.0;
		self.1 |= other.1;
		self.2 |= other.2;
	}
}

impl ops::Add for Specificity {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self(self.0 | other.0, self.1 | other.1, self.2 | other.2)
	}
}

impl Sum for Specificity {
	fn sum<I: Iterator<Item = Specificity>>(iter: I) -> Specificity {
		let mut out = Specificity(0, 0, 0);
		for specificity in iter {
			out += specificity
		}
		out
	}
}
