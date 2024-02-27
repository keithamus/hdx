macro_rules! parse_rect {
	($name: ident, $prop: ident, $top: ident, $bottom: ident, $left: ident, $right: ident) => {
		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<hdx_parser::Spanned<Self>> {
				let span = parser.span();
				if let Some(first) = $prop::from_token(parser.cur()) {
					parser.advance();
					if let Some(second) = $prop::from_token(parser.cur()) {
						parser.advance();
						if let Some(third) = $prop::from_token(parser.cur()) {
							parser.advance();
							if let Some(fourth) = $prop::from_token(parser.cur()) {
								parser.advance();
								Ok($name($top(first), $bottom(third), $left(fourth), $right(second)).spanned(span.end(parser.pos())))
							} else {
								Ok($name($top(first.clone()), $bottom(third), $left(second.clone()), $right(second)).spanned(span.end(parser.pos())))
							}
						} else {
							Ok($name($top(first.clone()), $bottom(first), $left(second.clone()), $right(second)).spanned(span.end(parser.pos())))
						}
					} else {
						Ok($name($top(first.clone()), $bottom(first.clone()), $left(first.clone()), $right(first)).spanned(span.end(parser.pos())))
					}
				} else {
					hdx_parser::unexpected!(parser)
				}
			}
		}
	}
}

pub(crate) use parse_rect;

macro_rules! write_rect {
	($name: ident) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let (top, bottom, left, right) = (&self.0, &self.1, &self.2, &self.3);
				if right.0 == left.0 {
					if top.0 == bottom.0 {
						if top.0 == right.0 {
							top.write_css(sink)?;
						} else {
							top.write_css(sink)?;
							sink.write_char(' ')?;
							right.write_css(sink)?;
						}
					} else {
						top.write_css(sink)?;
						sink.write_char(' ')?;
						right.write_css(sink)?;
						sink.write_char(' ')?;
						bottom.write_css(sink)?;
					}
				} else {
					top.write_css(sink)?;
					sink.write_char(' ')?;
					right.write_css(sink)?;
					sink.write_char(' ')?;
					bottom.write_css(sink)?;
					sink.write_char(' ')?;
					left.write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

pub(crate) use write_rect;

macro_rules! parse_logical_sides {
	($name: ident, $prop: ident, $block: ident, $inline: ident) => {
		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<hdx_parser::Spanned<Self>> {
				let span = parser.span();
				if let Some(first) = $prop::from_token(parser.cur()) {
					parser.advance();
					if let Some(second) = $prop::from_token(parser.cur()) {
						parser.advance();
						Ok($name($block(first), $inline(second)).spanned(span.end(parser.pos())))
					} else {
						Ok($name($block(first.clone()), $inline(first)).spanned(span.end(parser.pos())))
					}
				} else {
					hdx_parser::unexpected!(parser)
				}
			}
		}
	}
}

pub(crate) use parse_logical_sides;

macro_rules! write_logical_sides {
	($name: ident) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let (block, inline) = (&self.0, &self.1);
				if block.0 == inline.0 {
					block.write_css(sink)
				} else {
					block.write_css(sink)?;
					sink.write_char(' ')?;
					inline.write_css(sink)
				}
			}
		}
	}
}

pub(crate) use write_logical_sides;
