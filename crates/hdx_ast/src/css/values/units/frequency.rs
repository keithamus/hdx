use hdx_atom::atom;
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::{AbsoluteUnit, CSSFloat};
use crate::{Parsable};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Parsable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Frequency {
	#[parsable(Dimension)]
	Hz(CSSFloat),
	#[parsable(Dimension)]
	Khz(CSSFloat),
}

impl Into<CSSFloat> for Frequency {
	fn into(self) -> CSSFloat {
		match self {
			Self::Hz(f) | Self::Khz(f) => f,
		}
	}
}

impl AbsoluteUnit for Frequency {
	fn to_base(&self) -> Self {
		Self::Hz(match self {
			Self::Khz(f) => *f * 1000.0,
			Self::Hz(f) => *f,
		})
	}
}

impl<'a> WriteCss<'a> for Frequency {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let (f, unit) = match self {
			Self::Khz(f) => (f, atom!("khz")),
			Self::Hz(f) => (f, atom!("hz")),
		};
		f.write_css(sink)?;
		unit.write_css(sink)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Frequency>(), 8);
	}

	#[test]
	fn test_variants() {
		let allocator = Allocator::default();
		test_write::<Frequency>(&allocator, "40hz", "40hz");
		// Truncates to 7dp
		test_write::<Frequency>(&allocator, "1.2345678901234hz", "1.2345679hz");
		// Removes redundant dp
		test_write::<Frequency>(&allocator, "-1.0hz", "-1hz");
	}
}
