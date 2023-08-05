use hdx_ast::css::values::{fonts::*, Angle, MathExpr};
use hdx_syntax::identifier::is_ident_str;

use crate::{CssWriter, Result, Spanned, WriteCss};

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
