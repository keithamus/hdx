use std::ops::Deref;

use hdx_ast::css::values::expr::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Expr<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::GlobalValue(g) => sink.write_str(g.to_atom().as_ref()),
			Self::Literal(val) => val.write_css(sink),
			Self::Reference(f) => f.write_css(sink),
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for MathExpr<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::GlobalValue(g) => sink.write_str(g.to_atom().as_ref()),
			Self::Literal(val) => val.write_css(sink),
			Self::Reference(f) => f.write_css(sink),
			Self::Math(f) => f.write_css(sink),
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for ExprList<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::GlobalValue(g) => sink.write_str(g.to_atom().as_ref()),
			Self::Values(v) => {
				let mut values = v.iter().peekable();
				while let Some(value) = values.next() {
					value.write_css(sink)?;
					if values.peek().is_some() {
						sink.write_char(',')?;
						sink.write_trivia_char(' ')?;
					}
				}
				Ok(())
			}
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for MathExprList<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::GlobalValue(g) => sink.write_str(g.to_atom().as_ref()),
			Self::Values(v) => {
				let mut values = v.iter().peekable();
				while let Some(value) = values.next() {
					value.write_css(sink)?;
					if values.peek().is_some() {
						sink.write_char(',')?;
						sink.write_trivia_char(' ')?;
					}
				}
				Ok(())
			}
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for ExprListItem<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Literal(val) => val.write_css(sink),
			Self::Reference(f) => f.write_css(sink),
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for MathExprListItem<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Literal(val) => val.write_css(sink),
			Self::Reference(f) => f.write_css(sink),
			Self::Math(f) => f.write_css(sink),
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Reference<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Var(atom, opt) => {
				sink.write_str("var(")?;
				sink.write_str(atom.as_ref())?;
				if let Some(val) = opt.deref() {
					sink.write_str(",")?;
					sink.write_trivia_char(' ')?;
					val.write_css(sink)?;
				}
				sink.write_str(")")
			}
			Self::Env(atom, opt) => {
				sink.write_str("var(")?;
				sink.write_str(atom.as_ref())?;
				if let Some(val) = opt.deref() {
					sink.write_str(",")?;
					sink.write_trivia_char(' ')?;
					val.write_css(sink)?;
				}
				sink.write_str(")")
			}
		}
	}
}

impl<'a> WriteCss<'a> for MathFunc<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		todo!()
	}
}
