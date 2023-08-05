use hdx_ast::css::values::color::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for ColorValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Hex(_) => sink.write_str(&self.to_hex(ToHexStyle::Compact).unwrap()),
			Self::Named(named) => sink.write_str(named.to_atom().as_ref()),
			_ => todo!(),
		}
	}
}
