mod properties;
mod selector;
mod values;

use std::{fmt::Result, ops::Deref};

use hdx_ast::{
	css::{
		component_values::{ComponentValue, Function, SimpleBlock},
		rules::{CSSCharsetRule, CSSMarginRule, CSSPageRule, PageSelector, PageSelectorList},
		stylesheet::{CSSRule, CSSStyleRule, CSSStyleSheet},
		unknown::{UnknownAtRule, UnknownDeclaration, UnknownPrelude, UnknownRule},
		values::ValueLike,
	},
	Spanned,
};
use hdx_atom::Atomizable;
use hdx_lexer::{Kind, PairWise, Token};
use oxc_allocator::Box;

use crate::{CssWriter, WriteCss};

impl<'a> WriteCss<'a> for CSSCharsetRule {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str("@charset \"")?;
		sink.write_str(self.encoding.as_ref())?;
		sink.write_str("\";")?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for CSSStyleSheet<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		for rule in &self.rules {
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for CSSRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Style(rule) => rule.write_css(sink),
			Self::Charset(rule) => rule.write_css(sink),
			Self::Page(rule) => rule.write_css(sink),
			Self::UnknownAt(rule) => rule.write_css(sink),
			Self::Unknown(rule) => rule.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for CSSStyleRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		self.selectors.write_css(sink)?;
		sink.write_trivia_char(' ')?;
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.declarations.deref().iter().peekable();
		while let Some(decl) = iter.next() {
			sink.write_indent()?;
			decl.write_css(sink)?;
			if iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for CSSPageRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str("@page")?;
		if self.selectors.node.children.len() > 0 {
			sink.write_char(' ')?;
		}
		self.selectors.write_css(sink)?;
		if self.selectors.node.children.len() > 0 {
			sink.write_trivia_char(' ')?;
		}
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.declarations.iter().peekable();
		let mut rule_iter = self.rules.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() && rule_iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in rule_iter {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for PageSelectorList<'a> {
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

impl<'a> WriteCss<'a> for PageSelector<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Some(page_type) = &self.page_type {
			sink.write_str(page_type.as_ref())?;
		}
		for pseudo in self.pseudos.iter() {
			sink.write_char(':')?;
			sink.write_str(pseudo.to_atom().as_ref())?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for CSSMarginRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_char('@')?;
		sink.write_str(self.name.to_atom().as_ref())?;
		sink.write_trivia_char(' ')?;
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.declarations.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for UnknownAtRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str("@")?;
		sink.write_str(self.name.as_ref())?;
		if let Some(prelude) = &*self.prelude {
			prelude.write_css(sink)?;
			sink.write_trivia_char(' ')?;
		}
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.properties.iter().peekable();
		let mut rule_iter = self.rules.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() && rule_iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in rule_iter {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for UnknownRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Some(prelude) = &*self.prelude {
			prelude.write_css(sink)?;
			sink.write_trivia_char(' ')?;
		}
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.properties.iter().peekable();
		let mut rule_iter = self.rules.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() && rule_iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in rule_iter {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for UnknownPrelude<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		let mut iter = self.value.iter().peekable();
		while let Some(value) = iter.next() {
			value.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(' ')?;
			}
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for UnknownDeclaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str(self.name.as_ref())?;
		sink.write_char(':')?;
		sink.write_trivia_char(' ')?;
		match &self.value_like {
			Spanned { span: _, node: ValueLike::Color(color) } => color.write_css(sink)?,
			Spanned { span: _, node: ValueLike::Length(length) } => length.write_css(sink)?,
			Spanned { span: _, node: ValueLike::LengthPercentage(length) } => {
				length.write_css(sink)?
			}
			Spanned { span: _, node: ValueLike::FontFamily(font) } => font.write_css(sink)?,
			_ => {
				let mut values = self.value.iter().peekable();
				while let Some(value) = values.next() {
					value.write_css(sink)?;
					if values.peek().is_some() {
						sink.write_char(' ')?;
					}
				}
			}
		}
		if self.important {
			sink.write_trivia_char(' ')?;
			sink.write_str("!important")?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for ComponentValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::SimpleBlock(b) => b.write_css(sink),
			Self::Function(f) => f.write_css(sink),
			Self::Token(t) => t.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for SimpleBlock<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self.pairwise {
			PairWise::Square => sink.write_char('[')?,
			PairWise::Curly => sink.write_char('{')?,
			PairWise::Paren => sink.write_char('(')?,
		}
		for value in &*self.value {
			value.write_css(sink)?;
		}
		match self.pairwise {
			PairWise::Square => sink.write_char(']')?,
			PairWise::Curly => sink.write_char('}')?,
			PairWise::Paren => sink.write_char(')')?,
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Function<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		sink.write_str(self.name.as_ref())?;
		sink.write_char('(')?;
		for value in &*self.value {
			value.write_css(sink)?;
		}
		sink.write_char(')')?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Token {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self.kind {
			Kind::Ident => sink.write_str(self.value.as_atom().unwrap().as_ref())?,
			Kind::AtKeyword => {
				sink.write_char('@')?;
				sink.write_str(self.value.as_atom().unwrap().as_ref())?;
			}
			Kind::Hash => {
				sink.write_char('#')?;
				if self.escaped {
					sink.write_char('\\')?;
				}
				sink.write_str(self.value.as_atom().unwrap().as_ref())?;
			}
			Kind::BadString | Kind::String => {
				if self.escaped {
					sink.write_char('\\')?;
				}
				sink.write_char('"')?;
				sink.write_str(self.value.as_atom().unwrap().as_ref())?;
				sink.write_char('"')?;
			}
			Kind::BadUrl | Kind::Url => {
				sink.write_str("url(")?;
				if self.escaped {
					sink.write_char('\\')?;
				}
				sink.write_str(self.value.as_atom().unwrap().as_ref())?;
				sink.write_char(')')?;
			}
			Kind::Delim => {
				sink.write_char(self.value.as_char().unwrap())?;
			}
			Kind::Number => sink.write_str(&format!("{}", self.value.as_f32().unwrap()))?,
			Kind::Percentage => {
				sink.write_str(&format!("{}", self.value.as_f32().unwrap()))?;
				sink.write_char('%')?;
			}
			Kind::Dimension => {
				sink.write_str(&format!("{}", self.value.as_f32().unwrap()))?;
				sink.write_str(self.value.as_atom().unwrap().as_ref())?;
			}
			Kind::Whitespace => sink.write_char(' ')?,
			Kind::Cdo => sink.write_str("<!--")?,
			Kind::Cdc => sink.write_str("-->")?,
			Kind::Colon => sink.write_char(':')?,
			Kind::Semicolon => sink.write_char(';')?,
			Kind::Comma => sink.write_char(',')?,
			Kind::LeftSquare => sink.write_char('[')?,
			Kind::RightSquare => sink.write_char(']')?,
			Kind::LeftParen => sink.write_char('(')?,
			Kind::RightParen => sink.write_char(')')?,
			Kind::LeftCurly => sink.write_char('{')?,
			Kind::RightCurly => sink.write_char('}')?,
			Kind::Undetermined => {}
			Kind::Comment => sink.write_trivia_str(self.value.as_atom().unwrap().as_ref())?,
			Kind::Function => {
				sink.write_str(self.value.as_atom().unwrap().as_ref())?;
				sink.write_char('(')?;
			}
			Kind::Eof => {}
		}
		Ok(())
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Box<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		self.deref().write_css(sink)
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for Spanned<T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		self.node.write_css(sink)
	}
}
