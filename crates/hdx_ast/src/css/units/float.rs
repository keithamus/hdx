use hdx_derive::Writable;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};
use std::{
	fmt::{Display, Result as DisplayResult},
	hash::{Hash, Hasher},
	ops::{Add, Div, Mul, Sub},
};

// CSS floats are different to f32s in that they do not represent NaN
#[derive(Writable, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CSSFloat(f32);

impl CSSFloat {
	pub fn normalize(&self) -> Self {
		if self.0.is_nan() {
			Self(0.0)
		} else {
			Self(self.0)
		}
	}
}

impl Display for CSSFloat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> DisplayResult {
		self.normalize().0.fmt(f)
	}
}

impl Hash for CSSFloat {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let n = if self.0.is_nan() { 0.0 } else { self.0 };
		let sign = n.signum() as i8;
		sign.hash(state);
		(n as u32).hash(state);
	}
}

impl From<f32> for CSSFloat {
	fn from(f: f32) -> Self {
		Self(f).normalize()
	}
}

impl From<&f32> for CSSFloat {
	fn from(f: &f32) -> Self {
		Self(*f).normalize()
	}
}

impl Mul<f32> for CSSFloat {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self(self.0 * rhs).normalize()
	}
}

impl Div<f32> for CSSFloat {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		Self(self.0 / rhs).normalize()
	}
}

impl Add<f32> for CSSFloat {
	type Output = Self;

	fn add(self, rhs: f32) -> Self::Output {
		Self(self.0 + rhs).normalize()
	}
}

impl Sub<f32> for CSSFloat {
	type Output = Self;

	fn sub(self, rhs: f32) -> Self::Output {
		Self(self.0 - rhs).normalize()
	}
}

impl PartialEq<f32> for CSSFloat {
	fn eq(&self, rhs: &f32) -> bool {
		self.0.eq(rhs)
	}
}

impl PartialOrd<f32> for CSSFloat {
	fn lt(&self, rhs: &f32) -> bool {
		self.0.lt(rhs)
	}

	fn le(&self, rhs: &f32) -> bool {
		self.0.le(rhs)
	}

	fn gt(&self, rhs: &f32) -> bool {
		self.0.gt(rhs)
	}

	fn ge(&self, rhs: &f32) -> bool {
		self.0.ge(rhs)
	}

	fn partial_cmp(&self, rhs: &f32) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(rhs)
	}
}

impl<'a> Parse<'a> for CSSFloat {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.next() {
			Token::Number(f, _) => Ok(f.into()),
			token => unexpected!(parser, token),
		}
	}
}
