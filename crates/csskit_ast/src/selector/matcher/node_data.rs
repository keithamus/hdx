use super::PropertyValues;
use css_ast::CssMetadata;
use css_ast::VisitNode;
use css_ast::visit::{NodeId, NodeKey};
use css_lexer::Span;

/// Node-specific data captured during visit (independent of sibling position).
#[derive(Clone, Copy)]
pub(crate) struct NodeData {
	pub(crate) node_id: NodeId,
	pub(crate) node_key: NodeKey,
	pub(crate) span: Span,
	pub(crate) metadata: CssMetadata,
	pub(crate) properties: PropertyValues,
}

impl NodeData {
	#[inline]
	pub(crate) fn from_query(node: VisitNode) -> Self {
		let node_id = node.node_id.expect("NodeData only stores queryable nodes");
		let node_key = node.key().expect("queryable nodes have identity");
		Self {
			node_id,
			node_key,
			span: node.span(),
			metadata: node.self_metadata(),
			properties: PropertyValues::from_query(&node),
		}
	}
}
