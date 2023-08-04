use std::{fmt::Result, ops::Deref};

use hdx_ast::{css::values::*, Spanned};
use hdx_atom::Atomizable;
use hdx_syntax::identifier::is_ident_str;

use crate::{CssWriter, WriteCss};

macro_rules! write_atomizable_values {
    {$( $prop: ident, )+} => {
        $(
            impl<'a> WriteCss<'a> for $prop {
                fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
                    sink.write_str(self.to_atom().as_ref())
                }
            }
        )+
    }
}

write_atomizable_values! {
	AbsoluteSize,
	AlignmentBaselineValue,
	AutoOrNone,
	BaselineSourceValue,
	BorderCollapseValue,
	BoxSizingValue,
	CaptionSideValue,
	ClearValue,
	DominantBaselineValue,
	EmptyCellsValue,
	FloatReferenceValue,
	InlineSizingValue,
	LineStyle,
	ListStylePositionValue,
	MinIntrinsicSizingValue,
	OverflowKeyword,
	PositionValue,
	RelativeSize,
	TableLayoutValue,
	TextAlignAllValue,
	TextAlignLastValue,
	TextAlignValue,
	TextDecorationSkipInkValue,
	TextDecorationStyleValue,
	VisibilityValue,
}

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

impl<'a> WriteCss<'a> for LineHeightValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Normal => sink.write_str("normal"),
			Self::Number(n) => sink.write_str(n.to_string().as_str()),
			Self::LengthPercentage(n) => n.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for TimeOrAuto {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for MarginTrimValue {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for ColorValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Hex(_) => sink.write_str(&self.to_hex(ToHexStyle::Compact).unwrap()),
			Self::Named(named) => sink.write_str(named.to_atom().as_ref()),
			_ => todo!(),
		}
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for BoxShorthand<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match &self.top {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => val.write_css(sink)?,
		}
		match &self.right {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		match &self.bottom {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		match &self.left {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		Ok(())
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for XYShorthand<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match &self.x {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => val.write_css(sink)?,
		}
		match &self.y {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		Ok(())
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for DoubleShorthand<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match &self.0 {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => val.write_css(sink)?,
		}
		match &self.1 {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for NoNonGlobalValuesAllowed {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		Ok(())
	}
}

impl<'a> WriteCss<'a> for ContentsValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			ContentsValue::Normal => sink.write_str("normal"),
			ContentsValue::None => sink.write_str("none"),
			ContentsValue::Replacement(replacement) => replacement.write_css(sink),
			ContentsValue::List(list) => list.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for ContentReplacement<'a> {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for ContentList<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let mut iter = self.values.iter().peekable();
		while let Some(value) = iter.next() {
			match value {
				ContentElement::String(atom) => {
					sink.write_char('"')?;
					sink.write_str(atom.as_ref())?;
					sink.write_char('"')?;
				}
				ContentElement::Contents => sink.write_str("contents")?,
				ContentElement::Image(_) => todo!(),
				ContentElement::Counter(_) => todo!(),
				ContentElement::Quote(_) => todo!(),
				ContentElement::Leader(_) => todo!(),
			}
			if iter.peek().is_some() {
				sink.write_char(' ')?;
			}
		}
		for alt in &self.alt {
			todo!()
		}
		Ok(())
	}
}

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

impl<'a> WriteCss<'a> for FontWeightValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Normal => sink.write_str("normal"),
			Self::Bold => sink.write_str("bold"),
			Self::Bolder => sink.write_str("bolder"),
			Self::Lighter => sink.write_str("lighter"),
			Self::Number(num) => sink.write_str(num.to_string().as_str()),
		}
	}
}

impl<'a> WriteCss<'a> for FontSizeValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Absolute(size) => size.write_css(sink),
			Self::Relative(size) => size.write_css(sink),
			Self::LengthPercentage(size) => size.write_css(sink),
			Self::Math => sink.write_str("math"),
		}
	}
}

impl<'a> WriteCss<'a> for FontFamilyValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Named(atom) => {
				let bare = atom.as_ref().split_ascii_whitespace().all(is_ident_str);
				if !bare {
					sink.write_char('"')?;
				}
				sink.write_str(atom.as_ref())?;
				if !bare {
					sink.write_char('"')?;
				}
				Ok(())
			}
			Self::Serif => sink.write_str("serif"),
			Self::SansSerif => sink.write_str("sans-serif"),
			Self::Cursive => sink.write_str("cursive"),
			Self::Fantasy => sink.write_str("fantasy"),
			Self::Monospace => sink.write_str("monospace"),
			Self::SystemUi => sink.write_str("system-ui"),
			Self::Emoji => sink.write_str("emoji"),
			Self::Math => sink.write_str("math"),
			Self::Fangsong => sink.write_str("fangsong"),
			Self::UiSerif => sink.write_str("ui-serif"),
			Self::UiSansSerif => sink.write_str("ui-sans-serif"),
			Self::UiMonospace => sink.write_str("ui-monospace"),
			Self::UiRounded => sink.write_str("ui-rounded"),
		}
	}
}

impl<'a> WriteCss<'a> for LineWidth {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for FloatValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Some(atom) = self.to_atom() {
			sink.write_str(atom.as_ref())?;
		} else if let FloatValue::SnapBlockFunction(first, second) = self {
			first.write_css(sink)?;
			sink.write_char(' ')?;
			sink.write_str(second.to_atom().as_ref())?;
		} else if let FloatValue::SnapInlineFunction(first, second) = self {
			first.write_css(sink)?;
			sink.write_char(' ')?;
			sink.write_str(second.to_atom().as_ref())?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for FloatDeferValue {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for InsetShorthand<'a> {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for Sizing {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Auto => sink.write_str("auto"),
			Self::LengthPercentage(val) => val.write_css(sink),
			Self::MinContent => sink.write_str("min-content"),
			Self::MaxContent => sink.write_str("max-content"),
			Self::FitContentFunction(val) => {
				sink.write_str("fit-content(")?;
				val.write_css(sink)?;
				sink.write_char(')')
			}
			Self::Stretch => sink.write_str("stretch"),
			Self::FitContent => sink.write_str("fit-content"),
			Self::Contain => sink.write_str("contain"),
		}
	}
}

impl<'a> WriteCss<'a> for MaxSizing {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::LengthPercentage(val) => val.write_css(sink),
			Self::MinContent => sink.write_str("min-content"),
			Self::MaxContent => sink.write_str("max-content"),
			Self::FitContentFunction(val) => {
				sink.write_str("fit-content(")?;
				val.write_css(sink)?;
				sink.write_char(')')
			}
			Self::Stretch => sink.write_str("stretch"),
			Self::FitContent => sink.write_str("fit-content"),
			Self::Contain => sink.write_str("contain"),
		}
	}
}

impl<'a> WriteCss<'a> for RatioOrAuto {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for ZoomValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Normal => sink.write_str("normal"),
			Self::Reset => sink.write_str("reset"),
			Self::Number(n) => sink.write_str(&n.to_string()),
			Self::Percentage(n) => sink.write_str(&format!("{}%", n)),
		}
	}
}

impl<'a> WriteCss<'a> for TextDecorationLineValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Style { underline, overline, line_through, blink } => {
				if *underline {
					sink.write_str("underline")?;
				}
				if *overline {
					if *underline {
						sink.write_char(' ')?;
					}
					sink.write_str("overline")?;
				}
				if *line_through {
					if *underline || *overline {
						sink.write_char(' ')?;
					}
					sink.write_str("line-through")?;
				}
				if *blink {
					if *underline || *overline || *line_through {
						sink.write_char(' ')?;
					}
					sink.write_str("blink")?;
				}
				Ok(())
			}
		}
	}
}

impl<'a> WriteCss<'a> for TextDecorationShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(color) = &self.color {
			color.write_css(sink)?;
			if self.style.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(line) = &self.line {
			line.write_css(sink)?;
			if self.style.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(style) = &self.style {
			style.write_css(sink)?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for FontStyleValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Normal => sink.write_str("normal"),
			Self::Italic => sink.write_str("italic"),
			Self::Oblique(Spanned { node: angle, .. }) => {
				sink.write_str("oblique")?;
				match angle {
					MathExpr::Literal(Spanned { node: Angle::Deg(deg), .. }) => {
						if *deg != 14.0 {
							sink.write_char(' ')?;
							angle.write_css(sink)?;
						}
					}
					_ => {
						sink.write_char(' ')?;
						angle.write_css(sink)?;
					}
				}
				Ok(())
			}
		}
	}
}

impl<'a> WriteCss<'a> for Angle {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Zero => sink.write_char('0'),
			Self::Deg(val) => sink.write_str(&format!("{}deg", val)),
			Self::Grad(val) => sink.write_str(&format!("{}grad", val)),
			Self::Rad(val) => sink.write_str(&format!("{}rad", val)),
			Self::Turn(val) => sink.write_str(&format!("{}turn", val)),
		}
	}
}

impl<'a> WriteCss<'a> for TextSizeAdjustValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Auto => sink.write_str("auto"),
			Self::Percentage(val) => sink.write_str(&format!("{}%", val)),
		}
	}
}

impl<'a> WriteCss<'a> for CursorValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Custom(g) => todo!(),
			_ => sink.write_str(self.to_atom().as_ref()),
		}
	}
}

impl<'a> WriteCss<'a> for BaselineShiftValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Sub => sink.write_str("sub"),
			Self::Super => sink.write_str("super"),
			Self::Top => sink.write_str("top"),
			Self::Center => sink.write_str("center"),
			Self::Bottom => sink.write_str("bottom"),
			Self::LengthPercentage(val) => val.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for VerticalAlignShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(baseline_source) = &self.baseline_source {
			baseline_source.write_css(sink)?;
			if self.alignment_baseline.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(alignment_baseline) = &self.alignment_baseline {
			alignment_baseline.write_css(sink)?;
			if self.baseline_shift.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(baseline_shift) = &self.baseline_shift {
			baseline_shift.write_css(sink)?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for BorderShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(line_width) = &self.line_width {
			line_width.write_css(sink)?;
			if self.line_style.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(line_style) = &self.line_style {
			line_style.write_css(sink)?;
			if self.color.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(color) = &self.color {
			color.write_css(sink)?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for ListStyleImageValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Image(image) => image.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for ListStyleTypeValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::CounterStyle(c) => c.write_css(sink),
			Self::String(s) => {
				sink.write_char('"')?;
				sink.write_str(s.as_ref())?;
				sink.write_char('"')
			}
		}
	}
}

impl<'a> WriteCss<'a> for ListStyleShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(position) = &self.position {
			position.write_css(sink)?;
			if self.image.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(image) = &self.image {
			image.write_css(sink)?;
			if self.marker.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(marker) = &self.marker {
			marker.write_css(sink)?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for CounterStyle<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for Image<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		todo!()
	}
}
