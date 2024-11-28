use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

const DEG_GRAD: f32 = 0.9;
const DEG_RAD: f32 = 57.295_78;
const DEG_TURN: f32 = 360.0;

// https://drafts.csswg.org/css-values/#angles
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Angle {
	Grad(T![Dimension::Grad]),
	Rad(T![Dimension::Rad]),
	Turn(T![Dimension::Turn]),
	Deg(T![Dimension::Deg]),
}

impl Default for Angle {
	fn default() -> Self {
		Self::Deg(Default::default())
	}
}

impl From<Angle> for f32 {
	fn from(val: Angle) -> Self {
		match val {
			Angle::Grad(f) => f.into(),
			Angle::Rad(f) => f.into(),
			Angle::Turn(f) => f.into(),
			Angle::Deg(f) => f.into(),
		}
	}
}

impl<'a> Is<'a> for Angle {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::is(p, c)
			&& matches!(p.parse_atom_lower(c), atom!("grad") | atom!("rad") | atom!("turn") | atom!("deg"))
	}
}

impl<'a> Build<'a> for Angle {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("grad") => Self::Grad(<T![Dimension::Grad]>::build(p, c)),
			atom!("rad") => Self::Rad(<T![Dimension::Rad]>::build(p, c)),
			atom!("turn") => Self::Turn(<T![Dimension::Turn]>::build(p, c)),
			atom!("deg") => Self::Deg(<T![Dimension::Deg]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<Angle> for Cursor {
	fn from(value: Angle) -> Self {
		match value {
			Angle::Grad(t) => t.into(),
			Angle::Rad(t) => t.into(),
			Angle::Turn(t) => t.into(),
			Angle::Deg(t) => t.into(),
		}
	}
}

impl From<&Angle> for Cursor {
	fn from(value: &Angle) -> Self {
		(*value).into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Angle, 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Angle, "0grad");
		assert_parse!(Angle, "0deg");
	}
}
