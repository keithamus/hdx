use hdx_derive::Writable;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};
use std::{
	fmt::{Display, Result as DisplayResult},
	ops::{Add, Div, Mul, Sub},
};

// CSS floats are different to i32s in that they do not represent NaN
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CSSInt(i32);

impl CSSInt {
	#[allow(non_upper_case_globals)]
	pub const Zero: CSSInt = CSSInt(0);

	pub fn to_i32(&self) -> i32 {
		self.0
	}
}

impl From<CSSInt> for i32 {
	fn from(value: CSSInt) -> Self {
		value.to_i32()
	}
}

impl From<CSSInt> for f32 {
	fn from(value: CSSInt) -> Self {
		value.to_i32() as f32
	}
}

impl From<f32> for CSSInt {
	fn from(f: f32) -> Self {
		Self(f as i32)
	}
}

impl Display for CSSInt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> DisplayResult {
		self.0.fmt(f)
	}
}

impl From<&f32> for CSSInt {
	fn from(f: &f32) -> Self {
		Self(*f as i32)
	}
}

impl From<i32> for CSSInt {
	fn from(f: i32) -> Self {
		Self(f)
	}
}

impl From<&i32> for CSSInt {
	fn from(f: &i32) -> Self {
		Self(*f)
	}
}

impl Mul<i32> for CSSInt {
	type Output = Self;

	fn mul(self, rhs: i32) -> Self::Output {
		Self(self.0 * rhs)
	}
}

impl Div<i32> for CSSInt {
	type Output = Self;

	fn div(self, rhs: i32) -> Self::Output {
		Self(self.0 / rhs)
	}
}

impl Add<i32> for CSSInt {
	type Output = Self;

	fn add(self, rhs: i32) -> Self::Output {
		Self(self.0 + rhs)
	}
}

impl Sub<i32> for CSSInt {
	type Output = Self;

	fn sub(self, rhs: i32) -> Self::Output {
		Self(self.0 - rhs)
	}
}

impl PartialEq<i32> for CSSInt {
	fn eq(&self, rhs: &i32) -> bool {
		self.0.eq(rhs)
	}
}

impl PartialOrd<i32> for CSSInt {
	fn lt(&self, rhs: &i32) -> bool {
		self.0.lt(rhs)
	}

	fn le(&self, rhs: &i32) -> bool {
		self.0.le(rhs)
	}

	fn gt(&self, rhs: &i32) -> bool {
		self.0.gt(rhs)
	}

	fn ge(&self, rhs: &i32) -> bool {
		self.0.ge(rhs)
	}

	fn partial_cmp(&self, rhs: &i32) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(rhs)
	}
}

impl<'a> Peek<'a> for CSSInt {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Number]>().filter(|t| !t.is_float())
	}
}

impl<'a> Parse<'a> for CSSInt {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Number]>()?;
		let number = p.parse_number(token);
		if token.is_float() {
			Err(diagnostics::ExpectedInt(number, token.span()))?;
		}
		Ok(number.into())
	}
}
