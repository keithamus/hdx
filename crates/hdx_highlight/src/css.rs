use hdx_ast::css::{Property, StyleDeclaration, StyleValue, Tag, Visit};
use hdx_lexer::Span;

use crate::{SemanticKind, SemanticModifier, TokenHighlighter};

impl<'a> Visit<'a> for TokenHighlighter {
	fn visit_tag(&mut self, tag: &Tag) {
		let span: Span = (*tag).into();
		let mut modifier = SemanticModifier::none();
		match tag {
			Tag::HtmlNonConforming(_) => {
				modifier |= SemanticModifier::Deprecated;
			}
			Tag::Html(_) => {}
			Tag::HtmlNonStandard(_) => {
				modifier |= SemanticModifier::Experimental;
			}
			Tag::Svg(_) => {}
			Tag::Mathml(_) => {}
			Tag::CustomElement(_) => {
				modifier |= SemanticModifier::Custom;
			}
			Tag::Unknown(_) => {
				modifier |= SemanticModifier::Unknown;
			}
		}
		self.insert(span, SemanticKind::Tag, modifier);
	}

	fn visit_style_declaration(&mut self, rule: &StyleDeclaration<'a>) {
		self.insert(rule.open.into(), SemanticKind::Punctuation, SemanticModifier::none());
		if let Some(close) = rule.close {
			self.insert(close.into(), SemanticKind::Punctuation, SemanticModifier::none());
		}
	}

	fn visit_property(&mut self, property: &Property<'a>) {
		let span: Span = property.name.into();
		let mut modifier = SemanticModifier::none();
		if matches!(&property.value, StyleValue::Unknown(_)) {
			modifier |= SemanticModifier::Unknown;
		}
		self.insert(span, SemanticKind::Property, modifier);
		self.insert(property.colon.into(), SemanticKind::Punctuation, SemanticModifier::none());
		if let Some(semicolon) = property.semicolon {
			self.insert(semicolon.into(), SemanticKind::Punctuation, SemanticModifier::none());
		}
	}
}
