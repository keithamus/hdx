use hdx_ast::css::values::units::lengths::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for Length {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		sink.write_str(n.to_string().as_str())?;
		sink.write_str(atom.as_ref())
	}
}

impl<'a> WriteCss<'a> for PositiveLength {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		sink.write_str(n.to_string().as_str())?;
		sink.write_str(atom.as_ref())
	}
}

impl<'a> WriteCss<'a> for LengthOrAuto {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		sink.write_str(n.to_string().as_str())?;
		sink.write_str(atom.as_ref())
	}
}

impl<'a> WriteCss<'a> for LengthPercentageOrNormal {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		if !matches!(self, Self::Normal) {
			sink.write_str(n.to_string().as_str())?;
		}
		sink.write_str(atom.as_ref())
	}
}

impl<'a> WriteCss<'a> for PositiveLengthPercentageOrNormal {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		if !matches!(self, Self::Normal) {
			sink.write_str(n.to_string().as_str())?;
		}
		sink.write_str(atom.as_ref())
	}
}

impl<'a> WriteCss<'a> for LengthPercentage {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		sink.write_str(n.to_string().as_str())?;
		sink.write_str(atom.as_ref())
	}
}

impl<'a> WriteCss<'a> for PositiveLengthPercentage {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		sink.write_str(n.to_string().as_str())?;
		sink.write_str(atom.as_ref())
	}
}

impl<'a> WriteCss<'a> for LengthPercentageOrAuto {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let (n, atom) = self.to_f32_and_atom();
		if self != &LengthPercentageOrAuto::Auto {
			sink.write_str(n.to_string().as_str())?;
		}
		sink.write_str(atom.as_ref())
	}
}
