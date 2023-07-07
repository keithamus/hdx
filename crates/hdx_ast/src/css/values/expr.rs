use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable, Box, Spanned, Vec};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Expr<'a, T> {
	GlobalValue(GlobalValue),
	Literal(Spanned<T>),
	Reference(Spanned<Reference<'a, Expr<'a, T>>>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MathExpr<'a, T> {
	GlobalValue(GlobalValue),
	Literal(Spanned<T>),
	Reference(Spanned<Reference<'a, MathExpr<'a, T>>>),
	Math(Spanned<MathFunc<'a>>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ExprList<'a, T> {
	GlobalValue(GlobalValue),
	Values(Vec<'a, Spanned<ExprListItem<'a, T>>>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MathExprList<'a, T> {
	GlobalValue(GlobalValue),
	Values(Vec<'a, Spanned<MathExprListItem<'a, T>>>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum ExprListItem<'a, T> {
	Literal(Spanned<T>),
	Reference(Spanned<Reference<'a, Expr<'a, T>>>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MathExprListItem<'a, T> {
	Literal(Spanned<T>),
	Reference(Spanned<Reference<'a, Expr<'a, T>>>),
	Math(Spanned<MathFunc<'a>>),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Reference<'a, T> {
	Var(Atom, Box<'a, Option<Spanned<T>>>),
	Env(Atom, Box<'a, Option<Spanned<T>>>),
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum GlobalValue {
	Inherit,     // atom!("inherit")
	Initial,     // atom!("initial")
	Unset,       // atom!("unset")
	Revert,      // atom!("revert")
	RevertLayer, // atom!("revert-layer")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MathFunc<'a> {
	Calc(Box<'a, CalcSum<'a>>),
	Min(Box<'a, Vec<'a, CalcSum<'a>>>),
	Max(Box<'a, Vec<'a, CalcSum<'a>>>),
	Clamp(Box<'a, (CalcSum<'a>, CalcSum<'a>, CalcSum<'a>)>),
	Round(Box<'a, (RoundStrategy, CalcSum<'a>, CalcSum<'a>)>),
	Mod(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Rem(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Sin(Box<'a, CalcSum<'a>>),
	Cos(Box<'a, CalcSum<'a>>),
	Tan(Box<'a, CalcSum<'a>>),
	ASin(Box<'a, CalcSum<'a>>),
	ACos(Box<'a, CalcSum<'a>>),
	ATan(Box<'a, CalcSum<'a>>),
	ATan2(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Pow(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Sqrt(Box<'a, CalcSum<'a>>),
	Hypot(Box<'a, Vec<'a, CalcSum<'a>>>),
	Log(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Exp(Box<'a, CalcSum<'a>>),
	Abs(Box<'a, CalcSum<'a>>),
	Sign(Box<'a, CalcSum<'a>>),
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum CalcSum<'a> {
	Number(f32),
	Dimension(f32, Atom),
	Percentage(f32),
	E,
	Pi,
	Infinity,
	NegInfinity,
	NaN,
	Div(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Mul(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Add(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
	Sub(Box<'a, (CalcSum<'a>, CalcSum<'a>)>),
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum RoundStrategy {
	Nearest, // atom!("nearest")
	Up,      // atom!("up")
	Down,    // atom!("down")
	ToZero,  // atom!("to-zero")
}

impl<'a, T> Default for Expr<'a, T>
where
	T: Default,
{
	fn default() -> Self {
		Self::Literal(Spanned::dummy(T::default()))
	}
}

impl<'a, T> Default for MathExpr<'a, T>
where
	T: Default,
{
	fn default() -> Self {
		Self::Literal(Spanned::dummy(T::default()))
	}
}

impl<'a, T> Default for ExprList<'a, T>
where
	T: Default,
{
	fn default() -> Self {
		Self::GlobalValue(GlobalValue::Initial)
	}
}

impl<'a, T> Default for MathExprList<'a, T>
where
	T: Default,
{
	fn default() -> Self {
		Self::GlobalValue(GlobalValue::Initial)
	}
}
impl Hash for CalcSum<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			CalcSum::Number(n) => {
				state.write_u8(0);
				n.to_bits().hash(state);
			}
			CalcSum::Dimension(n, u) => {
				state.write_u8(1);
				n.to_bits().hash(state);
				u.hash(state);
			}
			CalcSum::Percentage(n) => {
				state.write_u8(2);
				n.to_bits().hash(state);
			}
			CalcSum::E => state.write_u8(3),
			CalcSum::Pi => state.write_u8(4),
			CalcSum::Infinity => state.write_u8(5),
			CalcSum::NegInfinity => state.write_u8(6),
			CalcSum::NaN => state.write_u8(7),
			CalcSum::Div(a) => {
				state.write_u8(8);
				a.hash(state);
			}
			CalcSum::Mul(a) => {
				state.write_u8(9);
				a.hash(state);
			}
			CalcSum::Add(a) => {
				state.write_u8(10);
				a.hash(state);
			}
			CalcSum::Sub(a) => {
				state.write_u8(11);
				a.hash(state);
			}
		}
	}
}

#[cfg(test)]
mod tests {

	use super::super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Expr<Length>>(), 32);
		assert_eq!(size_of::<MathExpr<Length>>(), 32);
		assert_eq!(size_of::<MathExpr<ColorValue>>(), 32);
		assert_eq!(size_of::<ExprList<Length>>(), 32);
		assert_eq!(size_of::<MathExprList<Length>>(), 32);
		assert_eq!(size_of::<ExprListItem<Length>>(), 32);
		assert_eq!(size_of::<MathExprListItem<Length>>(), 32);
		assert_eq!(size_of::<Reference<Length>>(), 24);
		assert_eq!(size_of::<GlobalValue>(), 1);
		assert_eq!(size_of::<MathFunc>(), 16);
		assert_eq!(size_of::<CalcSum>(), 16);
		assert_eq!(size_of::<RoundStrategy>(), 1);
	}
}
