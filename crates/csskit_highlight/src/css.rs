use css_ast::{
	CSSInt, Color, CssMetadata, Declaration, DeclarationValue, NodeId, NodeVisitor, PropertyRule, StyleRule,
	ToChromashift, Visit, VisitNode, visitor,
};
use css_lexer::ToSpan;
use css_parse::NodeWithMetadata;

use crate::{SemanticDecoration, SemanticKind, SemanticModifier, TokenHighlighter};

impl SemanticKind {
	/// Maps a CSS AST NodeId to a SemanticKind, if one exists.
	/// Returns None for nodes that don't have a direct semantic kind mapping.
	pub fn from_node_id(node_id: NodeId) -> Option<Self> {
		use css_ast::NodeId;
		match node_id {
			// Selector components - Tag and all its variants
			NodeId::Tag
			| NodeId::HtmlTag
			| NodeId::HtmlNonConformingTag
			| NodeId::HtmlNonStandardTag
			| NodeId::SvgTag
			| NodeId::MathmlTag
			| NodeId::CustomElementTag
			| NodeId::UnknownTag => Some(SemanticKind::Tag),
			NodeId::Class => Some(SemanticKind::Class),
			NodeId::Id => Some(SemanticKind::Id),
			NodeId::PseudoClass
			| NodeId::MozPseudoClass
			| NodeId::MsPseudoClass
			| NodeId::WebkitPseudoClass
			| NodeId::OPseudoClass => Some(SemanticKind::PseudoClass),
			NodeId::PseudoElement
			| NodeId::MozPseudoElement
			| NodeId::MsPseudoElement
			| NodeId::WebkitPseudoElement
			| NodeId::OPseudoElement => Some(SemanticKind::PseudoElement),
			NodeId::Wildcard => Some(SemanticKind::Wildcard),

			// At-rules
			NodeId::MediaRule
			| NodeId::KeyframesRule
			| NodeId::SupportsRule
			| NodeId::FontFaceRule
			| NodeId::FontFeatureValuesRule
			| NodeId::FontFeatureValueTypeRule
			| NodeId::FontPaletteValuesRule
			| NodeId::ContainerRule
			| NodeId::PageRule
			| NodeId::LayerRule
			| NodeId::MarginRule
			| NodeId::WebkitKeyframesRule
			| NodeId::DocumentRule
			| NodeId::MozDocumentRule
			| NodeId::PropertyRule
			| NodeId::CounterStyleRule
			| NodeId::NamespaceRule
			| NodeId::ColorProfileRule
			| NodeId::ScopeRule
			| NodeId::CustomMediaRule
			| NodeId::StartingStyleRule => Some(SemanticKind::AtKeyword),

			// Value types
			NodeId::Color => Some(SemanticKind::StyleValueColor),
			NodeId::Url => Some(SemanticKind::StyleValueUrl),
			NodeId::Length
			| NodeId::LengthPercentage
			| NodeId::Angle
			| NodeId::Time
			| NodeId::Flex
			| NodeId::Percentage
			| NodeId::Decibel => Some(SemanticKind::StyleValueDimension),

			// Most nodes don't have a direct semantic kind mapping
			_ => None,
		}
	}
}

impl From<&CssMetadata> for SemanticModifier {
	fn from(metadata: &CssMetadata) -> Self {
		use css_ast::NodeKinds;
		let mut modifier = SemanticModifier::none();

		if metadata.node_kinds.contains(NodeKinds::Unknown) {
			modifier |= SemanticModifier::Unknown;
		}
		if metadata.node_kinds.contains(NodeKinds::Deprecated) {
			modifier |= SemanticModifier::Deprecated;
		}
		if metadata.node_kinds.contains(NodeKinds::Experimental) {
			modifier |= SemanticModifier::Experimental;
		}
		if metadata.node_kinds.contains(NodeKinds::NonStandard) {
			modifier |= SemanticModifier::Vendor;
		}
		if metadata.node_kinds.contains(NodeKinds::Custom) {
			modifier |= SemanticModifier::Custom;
		}
		if metadata.has_vendor_prefixes() {
			modifier |= SemanticModifier::Vendor;
		}

		modifier
	}
}

#[visitor]
impl Visit for TokenHighlighter {
	fn enter_node(&mut self, query: VisitNode) {
		let node_id = query.node_id.unwrap();
		let modifier = SemanticModifier::from(&query.self_metadata());
		let kind = SemanticKind::from_node_id(node_id).unwrap_or(SemanticKind::None);

		if !modifier.is_none() || kind != SemanticKind::None {
			self.insert(query.span(), kind, modifier);
		}
	}

	// Visit style rules to specifically mark the curlies as Punctuation
	fn visit_style_rule<'a>(&mut self, rule: &StyleRule<'a>) {
		self.insert(rule.rule.block.open_curly.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
		if let Some(close) = rule.rule.block.close_curly {
			self.insert(close.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
		}
	}

	// Visit Declarations to mark the name and colon
	fn enter_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(
		&mut self,
		property: &Declaration<'a, T, CssMetadata>,
		_query: css_ast::VisitNode,
	) {
		let metadata = property.metadata();
		let modifier = SemanticModifier::from(&metadata);
		self.insert(property.name.to_span(), SemanticKind::Declaration, modifier);
		self.insert(property.colon.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
	}

	// Visit PropertyRule to mark the AtKeyword and the Prelude
	fn visit_property_rule<'a>(&mut self, property: &PropertyRule<'a>) {
		self.insert(property.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
		self.insert(property.prelude.to_span(), SemanticKind::Declaration, SemanticModifier::Custom);
	}

	// Visit Color nodes to decorate with the swatch
	fn visit_color<'b>(&mut self, color: &Color<'b>) {
		let metadata = color.metadata();
		let modifier = SemanticModifier::from(&metadata);
		if let Some(bg) = color.to_chromashift() {
			let swatch = SemanticDecoration::BackgroundColor(bg.into());
			self.insert_with_decoration(color.to_span(), SemanticKind::StyleValueColor, modifier, swatch);
		} else {
			self.insert(color.to_span(), SemanticKind::StyleValueColor, modifier);
		}
	}

	// Special case: CSSInt doesn't implement QueryableNode (has visit(skip))
	fn visit_css_int(&mut self, int: &CSSInt) {
		self.insert(int.to_span(), SemanticKind::StyleValueNumber, SemanticModifier::none());
	}
}
