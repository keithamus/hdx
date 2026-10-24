include!(concat!(env!("OUT_DIR"), "/css_node_kind.rs"));
include!(concat!(env!("OUT_DIR"), "/css_apply_visit_methods.rs"));

use css_parse::Vec;
use css_parse::{
	AttributeOperator, Block, Box, CommaSeparated, Comparison, ComponentValue, ComponentValues, Cursor, Declaration,
	DeclarationGroup, DeclarationList, DeclarationOrBad, DeclarationValue, Either, NoBlockAllowed, NodeMetadata,
	NodeWithMetadata, Optionals2, Optionals3, Optionals4, Optionals5, Parse, Peek, QualifiedRule, RuleList, Span,
	ToCursors, ToSpan, syntax::BadDeclaration, token_macros,
};
use visit_flow::try_visit;

/// The `#[visitor]` attribute: observer methods without a return type
/// auto-descend. Re-exported so visitor consumers need only depend on `css_ast`.
pub use csskit_derives::visitor;
pub use visit_flow::{VisitAction, VisitBreak, VisitFlow, VisitFlowExt};

mod root;
mod visit_node;
pub use root::{ErasedNode, ParsedRoot, parse_root};
pub(crate) use visit_node::QueryNodeData;
pub use visit_node::{NodeKey, VisitNode};

use crate::*;

macro_rules! visit_mut_trait {
	( $(
		$name: ident$(<$($gen:tt),+>)?($obj: ty) [$($tp: ident),*],
	)+ ) => {
		pub trait VisitMut: Sized {
			fn visit_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &mut Declaration<'a, T, CssMetadata>) {}
			fn exit_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &mut Declaration<'a, T, CssMetadata>) {}
			fn visit_bad_declaration<'a>(&mut self, _rule: &mut BadDeclaration<'a>) {}
			fn exit_bad_declaration<'a>(&mut self, _rule: &mut BadDeclaration<'a>) {}
			fn visit_string(&mut self, _str: &mut token_macros::String) {}
			fn exit_string(&mut self, _str: &mut token_macros::String) {}
			fn visit_comparison(&mut self, _comparison: &mut Comparison) {}
			fn exit_comparison(&mut self, _comparison: &mut Comparison) {}
			$(
				fn $name$(<$($gen),+>)?(&mut self, _rule: &mut $obj) {}
			)+
		}
	}
}
apply_visit_methods!(visit_mut_trait);

/// The object-safe core of [`Visit`].
///
/// [`Visit`] has methods that are type-specific. A visitor that needs the node kind and the span can implement just
/// this trait, and can then walk a node whose type is erased, such as an [`ErasedNode`].
///
/// The methods have no defaults, thus an implementation cannot drop one by mistake.
pub trait NodeVisitor {
	/// Called before entering a node.
	///
	/// Return [`VisitFlow::SKIP_CHILDREN`] to prune the node and its entire subtree (it is never entered). Return
	/// [`VisitFlow::STOP`] to halt the whole traversal.
	fn consider_node(&self, node: VisitNode) -> VisitFlow;

	/// Called on entry to every queryable node.
	///
	/// Receives a [`VisitNode`]; per-node metadata and properties are available via its methods. Return
	/// [`VisitFlow::SKIP_CHILDREN`] to skip the typed `visit_*` call and children.
	fn enter_node(&mut self, node: VisitNode) -> VisitFlow;

	/// Called on exit from every queryable node.
	fn exit_node(&mut self, node: VisitNode) -> VisitFlow;
}

impl NodeVisitor for &mut (dyn NodeVisitor + '_) {
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		(**self).consider_node(node)
	}

	fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
		(**self).enter_node(node)
	}

	fn exit_node(&mut self, node: VisitNode) -> VisitFlow {
		(**self).exit_node(node)
	}
}

impl Visit for &mut (dyn NodeVisitor + '_) {}

macro_rules! visit_trait {
	( $(
		$name: ident$(<$($gen:tt),+>)?($obj: ty) [$($tp: ident),*],
	)+ ) => {
		pub trait Visit: NodeVisitor + Sized {

			fn enter_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &Declaration<'a, T, CssMetadata>, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &Declaration<'a, T, CssMetadata>, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn visit_bad_declaration<'a>(&mut self, _rule: &BadDeclaration<'a>) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_bad_declaration<'a>(&mut self, _rule: &BadDeclaration<'a>) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn visit_string(&mut self, _str: &token_macros::String) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_string(&mut self, _str: &token_macros::String) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn visit_comparison(&mut self, _comparison: &Comparison) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_comparison(&mut self, _comparison: &Comparison) -> VisitFlow {
				VisitFlow::DESCEND
			}

			fn visit_feature<T: FeatureMetadata>(&mut self, _node: &T) {}
			fn exit_feature<T: FeatureMetadata>(&mut self, _node: &T) {}

			$(
				fn $name$(<$($gen),+>)?(&mut self, _rule: &$obj) -> VisitFlow where $($tp: Visitable + ToSpan,)* {
					VisitFlow::DESCEND
				}
			)+
		}
	}
}
apply_visit_methods!(visit_trait);

pub trait VisitableMut {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V);
}

pub trait Visitable {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow;
}

/// Marker trait for AST nodes that can be queried with selectors.
///
/// Implemented by `#[derive(Visitable)]` for queryable nodes, and manually for nodes
/// with `get_property` overrides (named at-rules, declarations).
/// Not part of the `Visit` public API - visitors receive a [`VisitNode`] instead.
pub(crate) trait QueryableNode: ToSpan + NodeWithMetadata<CssMetadata> {
	/// Unique identifier for this node type.
	const NODE_ID: NodeId;

	/// Returns a cursor for the given property kind, if the node has that property.
	/// Used by attribute selectors to extract values from nodes.
	///
	/// For `PropertyKind::Name`, returns a cursor to the node's name (e.g., property
	/// name for declarations, animation name for `@keyframes`).
	fn get_property(&self, _kind: PropertyKind) -> Option<Cursor> {
		None
	}

	/// Builds the [`VisitNode`] passed to every `Visit` callback for this node.
	///
	/// Metadata and properties are reached through `&dyn` accessors, so they only cost anything
	/// when a visitor actually reads them - `subtree_metadata` in particular walks the subtree.
	fn visit_node(&self) -> VisitNode<'_>
	where
		Self: Sized,
	{
		VisitNode::new(Self::NODE_ID, self)
	}
}

/// Blanket bridge so any [`QueryableNode`] can be held as a `&dyn` in [`VisitNode`].
impl<T: QueryableNode> QueryNodeData for T {
	#[inline]
	fn span(&self) -> Span {
		ToSpan::to_span(self)
	}

	#[inline]
	fn self_metadata(&self) -> CssMetadata {
		NodeWithMetadata::self_metadata(self)
	}

	#[inline]
	fn subtree_metadata(&self) -> CssMetadata {
		NodeWithMetadata::metadata(self)
	}

	#[inline]
	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		QueryableNode::get_property(self, kind)
	}
}

impl<T> VisitableMut for Option<T>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		if let Some(node) = self {
			node.accept_mut(v)
		}
	}
}

macro_rules! impl_optionals {
	($N:ident, $($T:ident),+) => {
		impl<$($T),*> Visitable for $N<$($T),+>
		where
			$($T: Visitable,)+
		{
			#[allow(non_snake_case)]
			#[allow(unused)]
			fn accept<VI: Visit>(&self, v: &mut VI) -> VisitFlow {
				let $N($($T),+) = self;
				$(try_visit!($T.accept(v));)+;
				VisitFlow::DESCEND
			}
		}

		impl<$($T),*> VisitableMut for $N<$($T),+>
		where
			$($T: VisitableMut,)+
		{
			#[allow(non_snake_case)]
			#[allow(unused)]
			fn accept_mut<VI: VisitMut>(&mut self, v: &mut VI) {
				let $N($($T),+) = self;
				$($T.accept_mut(v);)+;
			}
		}
	};
}

impl_optionals!(Optionals2, T, U);
impl_optionals!(Optionals3, T, U, V);
impl_optionals!(Optionals4, T, U, V, W);
impl_optionals!(Optionals5, T, U, V, W, X);

impl Visitable for token_macros::Ident {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::Ident {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::Function {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::Function {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::Comma {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::Comma {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::LeftParen {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::LeftParen {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::RightParen {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::RightParen {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::Colon {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::Colon {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::Semicolon {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::Semicolon {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for Comparison {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_comparison(self));
		try_visit!(v.exit_comparison(self));
		VisitFlow::DESCEND
	}
}

impl VisitableMut for Comparison {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_comparison(self);
		v.exit_comparison(self);
	}
}

impl Visitable for token_macros::delim::Dash {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::delim::Dash {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::delim::Slash {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::delim::Slash {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::Number {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::Number {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::Any {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::Any {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl Visitable for token_macros::String {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_string(self));
		try_visit!(v.exit_string(self));
		VisitFlow::DESCEND
	}
}

impl VisitableMut for token_macros::String {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_string(self);
		v.exit_string(self);
	}
}

impl<T> Visitable for Option<T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		if let Some(node) = self {
			try_visit!(node.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, T: VisitableMut> VisitableMut for Box<'a, T> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		(**self).accept_mut(v)
	}
}

impl<'a, T: Visitable> Visitable for Box<'a, T> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		(**self).accept(v)
	}
}

impl<'a, T, const MIN: usize> VisitableMut for CommaSeparated<'a, T, MIN>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for (node, _) in self {
			node.accept_mut(v)
		}
	}
}

impl<'a, T, const MIN: usize> Visitable for CommaSeparated<'a, T, MIN>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for (node, _) in self {
			try_visit!(node.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<Left, Right> VisitableMut for Either<Left, Right>
where
	Left: VisitableMut,
	Right: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::Left(t) => t.accept_mut(v),
			Self::Right(t) => t.accept_mut(v),
		}
	}
}

impl<Left, Right> Visitable for Either<Left, Right>
where
	Left: Visitable,
	Right: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		match self {
			Self::Left(t) => t.accept(v),
			Self::Right(t) => t.accept(v),
		}
	}
}

impl<'a, T> VisitableMut for Declaration<'a, T, CssMetadata>
where
	T: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_declaration(self);
		self.value.accept_mut(v);
		v.exit_declaration(self);
	}
}

impl<'a, T> QueryableNode for Declaration<'a, T, CssMetadata>
where
	T: DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	const NODE_ID: NodeId = NodeId::StyleValue;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.name.into()),
			_ => None,
		}
	}

	fn visit_node(&self) -> VisitNode<'_> {
		// Use T::NODE_ID so each declaration type has its own identity.
		VisitNode::new(T::NODE_ID, self)
	}
}

impl<'a, T> Visitable for Declaration<'a, T, CssMetadata>
where
	T: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = self.visit_node();
		if let visit_flow::VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let visit_flow::VisitAction::Descend = visit_flow::try_visit!(v.enter_node(node)) {
			if let visit_flow::VisitAction::Descend = visit_flow::try_visit!(v.enter_declaration::<T>(self, node)) {
				try_visit!(self.value.accept(v));
			}
			try_visit!(v.exit_declaration::<T>(self, node));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl<'a, T> VisitableMut for DeclarationList<'a, T, CssMetadata>
where
	T: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for declaration in &mut self.declarations {
			declaration.accept_mut(v);
		}
	}
}

impl<'a, T> Visitable for DeclarationList<'a, T, CssMetadata>
where
	T: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for declaration in &self.declarations {
			try_visit!(declaration.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, T, M> VisitableMut for RuleList<'a, T, M>
where
	T: VisitableMut + Parse<'a> + ToCursors + ToSpan + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		self.rules.accept_mut(v);
	}
}

impl<'a, T, M> Visitable for RuleList<'a, T, M>
where
	T: Visitable + Parse<'a> + ToCursors + ToSpan + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		self.rules.accept(v)
	}
}

impl<'a, P, D, R> VisitableMut for QualifiedRule<'a, P, D, R, CssMetadata>
where
	P: VisitableMut + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
	R: VisitableMut + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R, CssMetadata>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		self.prelude.accept_mut(v);
		self.block.accept_mut(v);
	}
}

impl<'a, P, D, R> Visitable for QualifiedRule<'a, P, D, R, CssMetadata>
where
	P: Visitable + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
	R: Visitable + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R, CssMetadata>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(self.prelude.accept(v));
		self.block.accept(v)
	}
}

impl<'a, D, R> VisitableMut for Block<'a, D, R, CssMetadata>
where
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
	R: VisitableMut + Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for declaration in &mut self.declarations {
			declaration.accept_mut(v);
		}
		for rule in &mut self.rules {
			rule.accept_mut(v);
		}
	}
}

impl<'a, D, R> Visitable for Block<'a, D, R, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
	R: Visitable + Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for declaration in &self.declarations {
			try_visit!(declaration.accept(v));
		}
		for rule in &self.rules {
			try_visit!(rule.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, T> VisitableMut for Vec<'a, T>
where
	T: VisitableMut,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for node in self {
			node.accept_mut(v);
		}
	}
}

impl<'a, T> Visitable for Vec<'a, T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for node in self {
			try_visit!(node.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut for BadDeclaration<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_bad_declaration(self);
		v.exit_bad_declaration(self);
	}
}

impl<'a> Visitable for BadDeclaration<'a> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_bad_declaration(self));
		try_visit!(v.exit_bad_declaration(self));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut for ComponentValues<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_component_values(self);
		for value in &mut self.values {
			value.accept_mut(v);
		}
		v.exit_component_values(self);
	}
}

impl<'a> QueryableNode for ComponentValues<'a> {
	const NODE_ID: NodeId = NodeId::ComponentValues;
}

impl<'a> Visitable for ComponentValues<'a> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = QueryableNode::visit_node(self);
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(node)) {
			if let VisitAction::Descend = try_visit!(v.visit_component_values(self)) {
				for value in &self.values {
					try_visit!(value.accept(v));
				}
			}
			try_visit!(v.exit_component_values(self));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut for ComponentValue<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_component_value(self);
		match self {
			ComponentValue::SimpleBlock(block) => block.values.accept_mut(v),
			ComponentValue::Function(function) => function.params.accept_mut(v),
			_ => {}
		}
		v.exit_component_value(self);
	}
}

impl<'a> QueryableNode for ComponentValue<'a> {
	const NODE_ID: NodeId = NodeId::ComponentValue;
}

impl<'a> Visitable for ComponentValue<'a> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = QueryableNode::visit_node(self);
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(node)) {
			if let VisitAction::Descend = try_visit!(v.visit_component_value(self)) {
				match self {
					ComponentValue::SimpleBlock(block) => {
						try_visit!(block.values.accept(v));
					}
					ComponentValue::Function(function) => {
						try_visit!(function.params.accept(v));
					}
					_ => {}
				}
			}
			try_visit!(v.exit_component_value(self));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl VisitableMut for AttributeOperator {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_attribute_operator(self);
		v.exit_attribute_operator(self);
	}
}

impl QueryableNode for AttributeOperator {
	const NODE_ID: NodeId = NodeId::AttributeOperator;
}

impl Visitable for AttributeOperator {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = QueryableNode::visit_node(self);
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(node)) {
			try_visit!(v.visit_attribute_operator(self));
			try_visit!(v.exit_attribute_operator(self));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut for Unresolved<'a> {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl<'a> Visitable for Unresolved<'a> {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<D, M> VisitableMut for NoBlockAllowed<D, M> {
	fn accept_mut<V: VisitMut>(&mut self, _: &mut V) {}
}

impl<D, M> Visitable for NoBlockAllowed<D, M> {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a, D> VisitableMut for DeclarationGroup<'a, D, CssMetadata>
where
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for declaration in &mut self.declarations {
			declaration.accept_mut(v)
		}
	}
}

impl<'a, D> Visitable for DeclarationGroup<'a, D, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for declaration in &self.declarations {
			try_visit!(declaration.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, D> VisitableMut for DeclarationOrBad<'a, D, CssMetadata>
where
	D: VisitableMut + DeclarationValue<'a, CssMetadata>,
{
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::Declaration(d) => d.accept_mut(v),
			Self::Bad(b) => b.accept_mut(v),
		}
	}
}

impl<'a, D> Visitable for DeclarationOrBad<'a, D, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		match self {
			Self::Declaration(d) => d.accept(v),
			Self::Bad(b) => b.accept(v),
		}
	}
}

macro_rules! impl_tuple_mut {
    ($($T:ident),*) => {
				impl<$($T),*> VisitableMut for ($($T),*)
        where
            $($T: VisitableMut,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
						fn accept_mut<VI: VisitMut>(&mut self, v: &mut VI) {
                let ($($T),*) = self;
                $($T.accept_mut(v);)*
            }
        }
    };
}

impl_tuple_mut!(T, U);
impl_tuple_mut!(T, U, V);
impl_tuple_mut!(T, U, V, W);
impl_tuple_mut!(T, U, V, W, X);
impl_tuple_mut!(T, U, V, W, X, Y);
impl_tuple_mut!(T, U, V, W, X, Y, Z);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B, C);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B, C, D);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B, C, D, E);

macro_rules! impl_tuple {
    ($($T:ident),*) => {
			impl<$($T),*> Visitable for ($($T),*)
        where
            $($T: Visitable,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
					fn accept<VI: Visit>(&self, v: &mut VI) -> VisitFlow {
                let ($($T),*) = self;
                $(try_visit!($T.accept(v));)*
                VisitFlow::DESCEND
            }
        }
    };
}
impl_tuple!(T, U);
impl_tuple!(T, U, V);
impl_tuple!(T, U, V, W);
impl_tuple!(T, U, V, W, X);
impl_tuple!(T, U, V, W, X, Y);
impl_tuple!(T, U, V, W, X, Y, Z);
impl_tuple!(T, U, V, W, X, Y, Z, A);
impl_tuple!(T, U, V, W, X, Y, Z, A, B);
impl_tuple!(T, U, V, W, X, Y, Z, A, B, C);
impl_tuple!(T, U, V, W, X, Y, Z, A, B, C, D);
impl_tuple!(T, U, V, W, X, Y, Z, A, B, C, D, E);
