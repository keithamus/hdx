use std::{
	f32::consts::TAU,
	hash::{Hash, Hasher},
};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom};

// https://drafts.csswg.org/css-values/#angles
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum Angle {
	Zero,
	Deg(f32),
	Grad(f32),
	Rad(f32),
	Turn(f32),
}

impl Default for Angle {
	fn default() -> Self {
		Self::Deg(0.0)
	}
}

impl Angle {
	fn to_f32_and_atom(self) -> (f32, Atom) {
		match self {
			Self::Zero => (0.0, atom!("")),
			Self::Deg(n) => (n, atom!("deg")),
			Self::Grad(n) => (n, atom!("grad")),
			Self::Rad(n) => (n, atom!("rad")),
			Self::Turn(n) => (n, atom!("turn")),
		}
	}

	fn from_f32_and_atom(n: f32, unit: Atom) -> Option<Self> {
		match unit {
			atom!("deg") => Some(Self::Deg(n)),
			atom!("grad") => Some(Self::Grad(n)),
			atom!("rad") => Some(Self::Rad(n)),
			atom!("turn") => Some(Self::Turn(n)),
			_ => None,
		}
	}

	fn to_deg(self) -> Self {
		match self {
			Self::Zero => Self::Deg(0.0),
			Self::Deg(_) => self,
			Self::Grad(n) => Self::Deg(n * 0.9),
			Self::Rad(n) => Self::Deg(n.to_degrees()),
			Self::Turn(n) => Self::Deg(n / 360.0),
		}
	}

	fn to_grad(&self) -> Self {
		match self {
			Self::Zero => Self::Grad(0.0),
			Self::Deg(n) => Self::Grad(n * 1.111_111_2),
			Self::Grad(_) => *self,
			Self::Rad(n) => Self::Grad(n * 63.661_976),
			Self::Turn(n) => Self::Grad(n * 400.0),
		}
	}

	fn to_rad(&self) -> Self {
		match self {
			Self::Zero => Self::Rad(0.0),
			Self::Deg(n) => Self::Rad(n.to_radians()),
			Self::Grad(n) => Self::Rad(n * 0.015_707_963),
			Self::Rad(_) => *self,
			Self::Turn(n) => Self::Rad(n * TAU),
		}
	}

	fn to_turn(&self) -> Self {
		match self {
			Self::Zero => Self::Turn(0.0),
			Self::Deg(n) => Self::Turn(n * 0.002_777_777_8),
			Self::Grad(n) => Self::Turn(n * 0.0025),
			Self::Rad(n) => Self::Turn(n * 0.159_154_94),
			Self::Turn(_) => *self,
		}
	}
}

impl Hash for Angle {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::Zero => {
				state.write_u8(0);
			}
			Self::Deg(n) => {
				state.write_u8(1);
				state.write_u32(n.to_bits());
			}
			Self::Grad(n) => {
				state.write_u8(2);
				state.write_u32(n.to_bits());
			}
			Self::Rad(n) => {
				state.write_u8(3);
				state.write_u32(n.to_bits());
			}
			Self::Turn(n) => {
				state.write_u8(4);
				state.write_u32(n.to_bits());
			}
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Angle>(), 8);
	}

	#[test]
	fn test_degrees_to_radians() {
		assert_eq!(Angle::Deg(14.0).to_rad(), Angle::Rad(0.244346095279));
		assert_eq!(Angle::Deg(0.0).to_rad(), Angle::Rad(0.0));
		assert_eq!(Angle::Deg(360.0).to_rad(), Angle::Rad(6.28318530718));
	}

	#[test]
	fn test_radians_to_degrees() {
		assert_eq!(Angle::Rad(0.244346095279).to_deg(), Angle::Deg(14.0));
		assert_eq!(Angle::Rad(0.0).to_deg(), Angle::Deg(0.0));
		assert_eq!(Angle::Rad(6.28318530718).to_deg(), Angle::Deg(360.0));
	}
}
