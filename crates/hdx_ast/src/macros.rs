macro_rules! parse_rect {
	($name: ident, $prop: ident, $top: ident, $bottom: ident, $left: ident, $right: ident) => {
		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				if let Ok(first) = $prop::parse(parser) {
					if let Ok(second) = $prop::parse(parser) {
						if let Ok(third) = $prop::parse(parser) {
							if let Ok(fourth) = $prop::parse(parser) {
								Ok($name($top(first), $bottom(third), $left(fourth), $right(second)))
							} else {
								Ok($name($top(first.clone()), $bottom(third), $left(second.clone()), $right(second)))
							}
						} else {
							Ok($name($top(first.clone()), $bottom(first), $left(second.clone()), $right(second)))
						}
					} else {
						Ok($name($top(first.clone()), $bottom(first.clone()), $left(first.clone()), $right(first)))
					}
				} else {
					hdx_parser::unexpected!(parser)
				}
			}
		}
	};
}

pub(crate) use parse_rect;

macro_rules! write_rect {
	($name: ident) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let (top, bottom, left, right) = (&self.0, &self.1, &self.2, &self.3);
				if right.0 == left.0 && !sink.can_output(hdx_writer::OutputOption::RedundantShorthandValues) {
					if top.0 == bottom.0 && !sink.can_output(hdx_writer::OutputOption::RedundantShorthandValues) {
						if top.0 == right.0 && !sink.can_output(hdx_writer::OutputOption::RedundantShorthandValues) {
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
	};
}

pub(crate) use write_rect;

macro_rules! parse_logical_sides {
	($name: ident, $prop: ident, $block: ident, $inline: ident) => {
		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				if let Ok(first) = $prop::parse(parser) {
					if let Ok(second) = $prop::parse(parser) {
						Ok($name($block(first), $inline(second)))
					} else {
						Ok($name($block(first.clone()), $inline(first)))
					}
				} else {
					hdx_parser::unexpected!(parser)
				}
			}
		}
	};
}

pub(crate) use parse_logical_sides;

macro_rules! write_logical_sides {
	($name: ident) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let (block, inline) = (&self.0, &self.1);
				if block.0 == inline.0 && !sink.can_output(hdx_writer::OutputOption::RedundantShorthandValues) {
					block.write_css(sink)
				} else {
					block.write_css(sink)?;
					sink.write_char(' ')?;
					inline.write_css(sink)
				}
			}
		}
	};
}

pub(crate) use write_logical_sides;

macro_rules! parse_option_shorthand {
	($name: ident, $first: ty, $second: ty, $third: ty) => {
		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				let mut first = None;
				let mut second = None;
				let mut third = None;
				loop {
					if first.is_none() {
						if let Ok(val) = <$first>::try_parse(parser) {
							first = Some(val);
							continue;
						}
					}
					if second.is_none() {
						if let Ok(val) = <$second>::try_parse(parser) {
							second = Some(val);
							continue;
						}
					}
					if third.is_none() {
						if let Ok(val) = <$third>::try_parse(parser) {
							third = Some(val);
							continue;
						}
					}
					break;
				}
				if first.is_none() && second.is_none() && third.is_none() {
					hdx_parser::unexpected!(parser);
				}
				Ok(Self(first, second, third))
			}
		}
	};
}
pub(crate) use parse_option_shorthand;

macro_rules! write_option_shorthand {
	($name: ident, 3) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				// Option<T> has a write_css impl
				self.0.write_css(sink)?;
				if self.0.is_some() && self.1.is_some() {
					sink.write_char(' ')?;
				}
				self.1.write_css(sink)?;
				if (self.0.is_some() || self.1.is_some()) && self.2.is_some() {
					sink.write_char(' ')?;
				}
				self.2.write_css(sink)?;
				Ok(())
			}
		}
	};
	($name: ident, 2) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				// Option<T> has a write_css impl
				self.0.write_css(sink)?;
				if self.0.is_some() && self.1.is_some() {
					sink.write_char(' ')?;
				}
				self.1.write_css(sink)?;
				Ok(())
			}
		}
	};
}
pub(crate) use write_option_shorthand;

macro_rules! write_simple_shorthand {
	($name: ident, $first: ty, $second: ty, $third: ty, $fourth: ty, $fifth: ty, $sixth: ty, $seventh: ty, $eighth: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let mut wrote = false;
				if self.0 != <$first>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					self.0.write_css(sink)?;
					wrote = true
				}
				if self.1 != <$second>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.1.write_css(sink)?;
					wrote = true
				}
				if self.2 != <$third>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.2.write_css(sink)?;
					wrote = true
				}
				if self.3 != <$fourth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.3.write_css(sink)?;
					wrote = true
				}
				if self.4 != <$fifth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.4.write_css(sink)?;
					wrote = true
				}
				if self.5 != <$sixth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.5.write_css(sink)?;
					wrote = true
				}
				if self.6 != <$seventh>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues)
				{
					if wrote {
						sink.write_char(' ')?;
					}
					self.6.write_css(sink)?;
					wrote = true
				}
				if self.7 != <$eigth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.7.write_css(sink)?;
					wrote = true
				}
				if !wrote {
					self.0.write_css(sink)?;
				}
				Ok(())
			}
		}
	};
	($name: ident, $first: ty, $second: ty, $third: ty, $fourth: ty, $fifth: ty, $sixth: ty, $seventh: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let mut wrote = false;
				if self.0 != <$first>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					self.0.write_css(sink)?;
					wrote = true
				}
				if self.1 != <$second>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.1.write_css(sink)?;
					wrote = true
				}
				if self.2 != <$third>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.2.write_css(sink)?;
					wrote = true
				}
				if self.3 != <$fourth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.3.write_css(sink)?;
					wrote = true
				}
				if self.4 != <$fifth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.4.write_css(sink)?;
					wrote = true
				}
				if self.5 != <$sixth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.5.write_css(sink)?;
					wrote = true
				}
				if self.6 != <$seventh>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues)
				{
					if wrote {
						sink.write_char(' ')?;
					}
					self.6.write_css(sink)?;
					wrote = true
				}
				if !wrote {
					self.0.write_css(sink)?;
				}
				Ok(())
			}
		}
	};
	($name: ident, $first: ty, $second: ty, $third: ty, $fourth: ty, $fifth: ty, $sixth: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let mut wrote = false;
				if self.0 != <$first>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					self.0.write_css(sink)?;
					wrote = true
				}
				if self.1 != <$second>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.1.write_css(sink)?;
					wrote = true
				}
				if self.2 != <$third>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.2.write_css(sink)?;
					wrote = true
				}
				if self.3 != <$fourth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.3.write_css(sink)?;
					wrote = true
				}
				if self.4 != <$fifth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.4.write_css(sink)?;
					wrote = true
				}
				if self.5 != <$sixth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.5.write_css(sink)?;
					wrote = true
				}
				if !wrote {
					self.0.write_css(sink)?;
				}
				Ok(())
			}
		}
	};
	($name: ident, $first: ty, $second: ty, $third: ty, $fourth: ty, $fifth: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let mut wrote = false;
				if self.0 != <$first>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					self.0.write_css(sink)?;
					wrote = true
				}
				if self.1 != <$second>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.1.write_css(sink)?;
					wrote = true
				}
				if self.2 != <$third>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.2.write_css(sink)?;
					wrote = true
				}
				if self.3 != <$fourth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.3.write_css(sink)?;
					wrote = true
				}
				if self.4 != <$fifth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.4.write_css(sink)?;
					wrote = true
				}
				if !wrote {
					self.0.write_css(sink)?;
				}
				Ok(())
			}
		}
	};
	($name: ident, $first: ty, $second: ty, $third: ty, $fourth: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let mut wrote = false;
				if self.0 != <$first>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					self.0.write_css(sink)?;
					wrote = true
				}
				if self.1 != <$second>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.1.write_css(sink)?;
					wrote = true
				}
				if self.2 != <$third>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.2.write_css(sink)?;
					wrote = true
				}
				if self.3 != <$fourth>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.3.write_css(sink)?;
					wrote = true
				}
				if !wrote {
					self.0.write_css(sink)?;
				}
				Ok(())
			}
		}
	};
	($name: ident, $first: ty, $second: ty, $third: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let mut wrote = false;
				if self.0 != <$first>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					self.0.write_css(sink)?;
					wrote = true
				}
				if self.1 != <$second>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.1.write_css(sink)?;
					wrote = true
				}
				if self.2 != <$third>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					if wrote {
						sink.write_char(' ')?;
					}
					self.2.write_css(sink)?;
					wrote = true
				}
				if !wrote {
					self.0.write_css(sink)?;
				}
				Ok(())
			}
		}
	};
	($name: ident, $first: ty, $second: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				let mut wrote = false;
				if self.0 != <$first>::default() || sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues) {
					self.0.write_css(sink)?;
					wrote = true
				}
				if !wrote
					|| self.1 != <$second>::default()
					|| sink.can_output(hdx_writer::OutputOption::RedundantDefaultValues)
				{
					if wrote {
						sink.write_char(' ')?;
					}
					self.1.write_css(sink)?;
					wrote = true
				}
				if !wrote {
					self.0.write_css(sink)?;
				}
				Ok(())
			}
		}
	};
	($name: ident, $first: ty) => {
		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				self.0.write_css(sink)
			}
		}
	};
}

pub(crate) use write_simple_shorthand;

macro_rules! discrete_media_feature {
	($feat: tt[atom!($atom: tt)] { $( $name: ident: atom!($name_atom: tt),)+ }) => {
		#[derive(PartialEq, Default, Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
		pub enum $feat {
			#[default]
			Any,
			$( $name, )+
		}

		impl<'a> hdx_parser::Parse<'a> for $feat {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				use hdx_parser::MediaFeature;
				Self::parse_media_feature(hdx_atom::atom!($atom), parser)
			}
		}

		impl<'a> hdx_parser::MediaFeature<'a> for $feat {
			fn parse_media_feature_value(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				hdx_parser::expect_ignore_case!{ parser.next(), Token::Ident(_):
					$(
						hdx_atom::atom!($name_atom) => Ok(Self::$name),
					)+
				}
			}
		}

		impl<'a> hdx_writer::WriteCss<'a> for $feat {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				hdx_atom::atom!($atom).write_css(sink)?;
				match self {
				$(
					Self::$name => {
						sink.write_char(':')?;
						sink.write_whitespace()?;
						hdx_atom::atom!($name_atom).write_css(sink)
					}
				)+
					Self::Any => Ok(())
				}
			}
		}

	};
}

pub(crate) use discrete_media_feature;
