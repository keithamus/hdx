use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{diagnostics, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use crate::{types::Color, units::LengthPercentageOrFlex};

// https://drafts.csswg.org/css-images-4/#typedef-image-1d
// <image-1D> = <stripes()>
// <stripes()> = stripes( <color-stripe># )
// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Image1D<'a> {
	pub function: T![Function],
	pub stripes: Vec<'a, ColorStripe>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for Image1D<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "stripes")
	}
}

impl<'a> Parse<'a> for Image1D<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if !p.eq_ignore_ascii_case(c, "stripes") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
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

impl<'a> ToCursors for Image1D<'a> {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
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

impl<'a> ToCursors for ColorStripe {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Image1D>(), 56);
		assert_eq!(std::mem::size_of::<ColorStripe>(), 192);
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
