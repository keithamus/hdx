use hdx_parser::{Parse, Parser, Result as ParserResult, ToCursors, T};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
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

impl<'a> ToCursors<'a> for Class {
	fn to_cursors(&self, s: &mut hdx_parser::CursorStream<'a>) {
		s.append(self.dot.into());
		s.append(self.name.into());
	}
}
