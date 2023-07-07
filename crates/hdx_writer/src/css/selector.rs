use std::fmt::Result;

use hdx_ast::css::{
	selector::{Attribute, AttributeMatch, Combinator, Component, NSPrefix, Selector},
	stylesheet::SelectorSet,
};
use hdx_atom::Atomizable;

use crate::{CssWriter, WriteCss};

impl<'a> WriteCss<'a> for SelectorSet<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let mut iter = self.children.iter().peekable();
		while let Some(selector) = iter.next() {
			selector.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_trivia_char(' ')?;
			}
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Selector<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		for component in &(*self.components) {
			component.write_css(sink)?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Component<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Type(ty) => {
				sink.write_str(ty)?;
			}
			Self::Id(id) => {
				sink.write_char('#')?;
				sink.write_str(id)?;
			}
			Self::Class(class) => {
				sink.write_char('.')?;
				sink.write_str(class)?;
			}
			Self::PseudoClass(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())?;
			}
			Self::LegacyPseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())?;
			}
			Self::PseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())?;
			}
			Self::Attribute(attr) => {
				attr.write_css(sink)?;
			}
			Self::Combinator(combinator) => {
				sink.write_trivia_char(' ')?;
				match combinator {
					Combinator::Descendant => {
						sink.write_char(' ')?;
					}
					Combinator::Child => {
						sink.write_char('>')?;
					}
					Combinator::NextSibling => {
						sink.write_char('+')?;
					}
					Combinator::SubsequentSibling => {
						sink.write_char('~')?;
					}
					Combinator::ColumnCombintor => {
						sink.write_char('|')?;
						sink.write_char('|')?;
					}
				}
				sink.write_trivia_char(' ')?;
			}
			Self::Wildcard => {
				sink.write_char('*')?;
			}
			_ => todo!(),
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Attribute {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_char('[')?;
		match &self.ns_prefix {
			NSPrefix::None => {}
			NSPrefix::Named(ns) => {
				sink.write_str(ns.as_ref())?;
				sink.write_char('|')?;
			}
			NSPrefix::Wildcard => {
				sink.write_char('*')?;
				sink.write_char('|')?;
			}
		}
		sink.write_str(self.name.as_ref())?;
		match &self.matcher {
			AttributeMatch::Any => {}
			AttributeMatch::Exact => {
				sink.write_char('=')?;
			}
			AttributeMatch::SpaceList => {
				sink.write_char('~')?;
				sink.write_char('=')?;
			}
			AttributeMatch::LangPrefix => {
				sink.write_char('|')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Prefix => {
				sink.write_char('^')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Suffix => {
				sink.write_char('$')?;
				sink.write_char('=')?;
			}
			AttributeMatch::Contains => {
				sink.write_char('*')?;
				sink.write_char('=')?;
			}
		}
		if &self.matcher != &AttributeMatch::Any {
			sink.write_char('"')?;
			sink.write_str(self.value.as_ref())?;
			sink.write_char('"')?;
		}

		sink.write_char(']')?;
		Ok(())
	}
}
