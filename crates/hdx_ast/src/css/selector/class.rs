use hdx_parser::{Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct Class {
	pub dot: T![.],
	pub name: T![Ident],
}

impl<'a> Parse<'a> for Class {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let dot = p.parse::<T![.]>()?;
		let name = p.parse::<T![Ident]>()?;
		Ok(Self { dot, name })
	}
}

impl<'a> ToCursors for Class {
	fn to_cursors(&self, s: &mut impl hdx_parser::CursorSink) {
		s.append(self.dot.into());
		s.append(self.name.into());
	}
}

impl<'a> Visitable<'a> for Class {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_class(self);
	}
}
