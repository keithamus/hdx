use hdx_ast::css::values::display::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for DisplayValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Some(atom) = self.to_atom() {
			sink.write_str(atom.as_ref())?;
		} else if let DisplayValue::Pair(outside, inside) = self {
			if outside != &DisplayOutside::Implicit {
				sink.write_str(outside.to_atom().unwrap().as_ref())?;
				sink.write_char(' ')?;
			}
			sink.write_str(inside.to_atom().unwrap().as_ref())?;
		} else if let DisplayValue::PairAndMarker(outside, inside, marker) = self {
			if outside != &DisplayOutside::Implicit {
				sink.write_str(outside.to_atom().unwrap().as_ref())?;
				sink.write_char(' ')?;
			}
			if inside != &DisplayInside::Implicit {
				sink.write_str(inside.to_atom().unwrap().as_ref())?;
				sink.write_char(' ')?;
			}
			sink.write_str(marker.to_atom().unwrap().as_ref())?;
		}
		Ok(())
	}
}
