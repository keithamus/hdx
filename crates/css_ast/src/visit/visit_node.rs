use css_lexer::Span;
use css_parse::Cursor;

use crate::{CssMetadata, PropertyKind};

use super::NodeId;

pub(crate) trait QueryNodeData {
	/// The source span this node covers.
	fn span(&self) -> Span;
	/// Metadata for *this node only* (no subtree aggregation).
	fn self_metadata(&self) -> CssMetadata;
	/// Metadata for this node and its entire subtree.
	fn subtree_metadata(&self) -> CssMetadata;
	/// Returns a cursor for the given property kind, if the node has that property.
	fn get_property(&self, kind: PropertyKind) -> Option<Cursor>;
}

/// Identity of one node within a parsed tree.
///
/// Keys remain valid while the tree lives. They are opaque and meaningful only among nodes from
/// the same tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeKey {
	address: usize,
	node_id: NodeId,
}

impl NodeKey {
	fn of<T: ?Sized>(value: &T, node_id: NodeId) -> Self {
		Self { address: (std::ptr::from_ref(value) as *const ()).addr(), node_id }
	}
}

/// The single node view passed to every [`Visit`](super::Visit) callback.
#[derive(Clone, Copy)]
pub struct VisitNode<'n> {
	span: Span,
	pub node_id: Option<NodeId>,
	source: Option<&'n dyn QueryNodeData>,
}

impl<'n> VisitNode<'n> {
	/// Construct for a queryable node. For Nodes without `NodeId` or meta see [VisitNode::new_transparent].
	#[inline]
	pub(crate) fn new(node_id: NodeId, source: &'n dyn QueryNodeData) -> Self {
		Self { span: Span::DUMMY, node_id: Some(node_id), source: Some(source) }
	}

	/// Construct for a "transparent node" (no `NodeId`, no meta).
	#[inline]
	pub fn new_transparent(span: Span) -> Self {
		Self { span, node_id: None, source: None }
	}

	/// The source span this node covers.
	#[inline]
	pub fn span(&self) -> Span {
		self.source.map_or(self.span, QueryNodeData::span)
	}

	/// The identity of this queryable node, or `None` for a transparent node.
	#[inline]
	pub fn key(&self) -> Option<NodeKey> {
		self.source.zip(self.node_id).map(|(source, node_id)| NodeKey::of(source, node_id))
	}

	/// Aggregated metadata for this node *and its entire subtree*.
	#[inline]
	pub fn subtree_metadata(&self) -> CssMetadata {
		self.source.map(QueryNodeData::subtree_metadata).unwrap_or_default()
	}

	/// Metadata for *this node only* (no subtree aggregation).
	#[inline]
	pub fn self_metadata(&self) -> CssMetadata {
		self.source.map(QueryNodeData::self_metadata).unwrap_or_default()
	}

	/// Retrieve a named property from this node, if present.
	#[inline]
	pub fn property(&self, kind: PropertyKind) -> Option<Cursor> {
		self.source.and_then(|s| s.get_property(kind))
	}
}
