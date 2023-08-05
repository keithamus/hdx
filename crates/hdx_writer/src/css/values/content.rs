use hdx_ast::css::values::content::*;

use crate::{CssWriter, Result, WriteCss};

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

impl<'a> WriteCss<'a> for QuotesValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none")?,
			Self::Auto => sink.write_str("auto")?,
			Self::Custom(quotes) => {
				let mut iter = quotes.iter().peekable();
				while let Some((open, close)) = iter.next() {
					sink.write_char('"')?;
					sink.write_str(open.as_ref())?;
					sink.write_char('"')?;
					sink.write_char(' ')?;
					sink.write_char('"')?;
					sink.write_str(close.as_ref())?;
					sink.write_char('"')?;
					if iter.peek().is_some() {
						sink.write_char(' ')?;
					}
				}
			}
		}
		Ok(())
	}
}
