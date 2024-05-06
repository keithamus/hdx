use hdx_derive::Writable;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};
use std::{
	fmt::{Display, Result as DisplayResult},
	ops::{Add, Div, Mul, Sub},
};

// CSS floats are different to i32s in that they do not represent NaN
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CSSInt(i32);

impl Display for CSSInt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> DisplayResult {
		self.0.fmt(f)
	}
}

impl From<f32> for CSSInt {
	fn from(f: f32) -> Self {
		Self(f as i32)
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

impl<'a> Parse<'a> for CSSInt {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.next() {
			Token::Number(f, ty) if !ty.is_float() => Ok(f.into()),
			token => unexpected!(parser, token),
		}
	}
}
