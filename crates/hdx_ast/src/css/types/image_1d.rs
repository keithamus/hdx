use bumpalo::collections::Vec;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use crate::css::{types::Color, units::LengthPercentageOrFlex};

mod func {
	use hdx_parser::custom_function;
	custom_function!(Stripes, atom!("stripes"));
}

// https://drafts.csswg.org/css-images-4/#typedef-image-1d
// <image-1D> = <stripes()>
// <stripes()> = stripes( <color-stripe># )
// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Image1D<'a> {
	pub function: func::Stripes,
	pub stripes: Vec<'a, ColorStripe>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for Image1D<'a> {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<func::Stripes>()
	}
}

impl<'a> Parse<'a> for Image1D<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<func::Stripes>()?;
		let mut stripes = Vec::new_in(p.bump());
		loop {
			if p.peek::<T![')']>() {
				let close = p.parse::<T![')']>()?;
				return Ok(Self { function, stripes, close });
			}
			stripes.push(p.parse::<ColorStripe>()?);
		}
	}
}

impl<'a> ToCursors<'a> for Image1D<'a> {
	fn to_cursors(&self, s: &mut hdx_parser::CursorStream<'a>) {
		s.append(self.function.into());
		for stripe in &self.stripes {
			ToCursors::to_cursors(stripe, s);
		}
		s.append(self.close.into());
	}
}

// https://drafts.csswg.org/css-images-4/#typedef-color-stripe
// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ColorStripe {
	pub color: Color,
	pub thickness: Option<LengthPercentageOrFlex>,
	pub comma: Option<T![,]>,
}

impl<'a> Parse<'a> for ColorStripe {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut color = p.parse_if_peek::<Color>()?;
		let thickness = p.parse_if_peek::<LengthPercentageOrFlex>()?;
		if color.is_none() {
			color = Some(p.parse::<Color>()?);
		}
		let comma = p.parse_if_peek::<T![,]>()?;
		Ok(Self { color: color.unwrap(), thickness, comma })
	}
}

impl<'a> ToCursors<'a> for ColorStripe {
	fn to_cursors(&self, s: &mut hdx_parser::CursorStream<'a>) {
		ToCursors::to_cursors(&self.color, s);
		if let Some(thickness) = self.thickness {
			s.append(thickness.into());
		}
		if let Some(comma) = self.comma {
			s.append(comma.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Image1D, 56);
		assert_size!(ColorStripe, 164);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Image1D, "stripes(red 1fr,green 2fr,blue 100px)");
		assert_parse!(
			Image1D,
			"stripes(0.1fr red,0.2fr green,100px blue)",
			"stripes(red 0.1fr,green 0.2fr,blue 100px)"
		);
		assert_parse!(Image1D, "stripes(red 1fr,2fr green,blue 100px)", "stripes(red 1fr,green 2fr,blue 100px)");
	}
}
