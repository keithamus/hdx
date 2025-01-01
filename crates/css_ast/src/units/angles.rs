use css_lexer::Cursor;
use css_parse::{Build, Parser, Peek, T};

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

impl<'a> Peek<'a> for Angle {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && matches!(p.parse_str_lower(c), "grad" | "rad" | "turn" | "deg")
	}
}

impl<'a> Build<'a> for Angle {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match p.parse_str_lower(c) {
			"grad" => Self::Grad(<T![Dimension::Grad]>::build(p, c)),
			"rad" => Self::Rad(<T![Dimension::Rad]>::build(p, c)),
			"turn" => Self::Turn(<T![Dimension::Turn]>::build(p, c)),
			"deg" => Self::Deg(<T![Dimension::Deg]>::build(p, c)),
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Angle>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Angle, "0grad");
		assert_parse!(Angle, "0deg");
	}
}
