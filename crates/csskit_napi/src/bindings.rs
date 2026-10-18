//! The napi-rs surface for the CSS AST object model.
//!
//! Compiled only under the `napi` feature so the default workspace build links without a Node
//! runtime.

use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::ast::{Ast, Diagnostic, QueryMatch};
use css_ast::visit::{NodeId, NodeKey, NodeVisitor, VisitNode};
use visit_flow::{VisitFlow, VisitFlowExt};

// The object API is one JS class for each AST node kind. `Node` is the only native class. The JS
// side declares `class X extends Node` for each kind and sets the prototype of each instance from
// its `kindId`. A kind crosses the boundary as its kebab-case tag.

/// A visitor callback. A returned string is a command: `'skip'` or `'skip-children'` prunes the
/// subtree, `'stop'` ends the traversal. All other values descend.
type VisitFn<'a> = Function<'a, Reference<Node>, Either<String, Unknown<'a>>>;

/// Sends the JS `enter` and `exit` callbacks to a [`NodeVisitor`] walk. Each callback gets a real [`Node`].
struct JsSink<'a> {
	ast: &'a Ast,
	env: Env,
	root: Reference<Node>,
	enter: Option<VisitFn<'a>>,
	exit: Option<VisitFn<'a>>,
	/// If set, only this node and its structural descendants call back.
	gate: Option<NodeKey>,
	err: Option<napi::Error>,
}

impl JsSink<'_> {
	fn dispatch(&mut self, exit: bool, node: VisitNode) -> VisitFlow {
		if self.err.is_some() {
			return VisitFlow::STOP;
		}
		let Some(id) = node.node_id else { return VisitFlow::DESCEND };
		let node_key = node.key().expect("queryable nodes have identity");
		let span = node.span();
		let (start, end) = (span.start().0, span.end().0);
		if let Some(gate) = self.gate
			&& !self.ast.contains(gate, node_key)
		{
			return VisitFlow::DESCEND;
		}
		let flow = match if exit { self.exit.as_ref() } else { self.enter.as_ref() } {
			None => VisitFlow::DESCEND,
			Some(cb) => {
				let node = match self.root.clone(self.env).and_then(|root| {
					Node { owner: Owner::Descendant(root), node_id: id, node_key: Some(node_key), start, end }
						.into_reference(self.env)
				}) {
					Ok(node) => node,
					Err(e) => {
						self.err = Some(e);
						return VisitFlow::STOP;
					}
				};
				match cb.call(node) {
					Ok(Either::A(cmd)) => match cmd.as_str() {
						"skip" | "skip-children" | "skipChildren" => VisitFlow::SKIP_CHILDREN,
						"stop" => VisitFlow::STOP,
						_ => VisitFlow::DESCEND,
					},
					Ok(Either::B(_)) => VisitFlow::DESCEND,
					Err(e) => {
						self.err = Some(e);
						VisitFlow::STOP
					}
				}
			}
		};
		if exit && self.gate == Some(node_key) { VisitFlow::STOP } else { flow }
	}
}

impl NodeVisitor for JsSink<'_> {
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		let Some(gate) = self.gate else { return VisitFlow::DESCEND };
		let Some(node_key) = node.key() else { return VisitFlow::DESCEND };
		if self.ast.contains(node_key, gate) || self.ast.contains(gate, node_key) {
			VisitFlow::DESCEND
		} else {
			VisitFlow::SKIP_CHILDREN
		}
	}

	fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
		self.dispatch(false, node)
	}

	fn exit_node(&mut self, node: VisitNode) -> VisitFlow {
		self.dispatch(true, node)
	}
}

/// Tells if a [`Node`] owns the parse, or points to the root that owns it.
enum Owner {
	Root(Ast),
	Descendant(Reference<Node>),
}

/// A node in a parsed AST.
///
/// The root owns the arena and the source. All other nodes hold a reference to the root, thus a node
/// keeps the parse alive.
#[napi]
pub struct Node {
	owner: Owner,
	node_id: NodeId,
	node_key: Option<NodeKey>,
	start: u32,
	end: u32,
}

impl Node {
	fn ast(&self) -> &Ast {
		match &self.owner {
			Owner::Root(ast) => ast,
			Owner::Descendant(root) => root.ast(),
		}
	}

	fn node_key(&self) -> Option<NodeKey> {
		self.node_key.or_else(|| self.ast().root_key())
	}

	/// A reference to the root node. Descendants point to the root, never to each other.
	fn root_ref(&self, this: Reference<Node>, env: Env) -> Result<Reference<Node>> {
		match &self.owner {
			Owner::Root(_) => Ok(this),
			Owner::Descendant(root) => root.clone(env),
		}
	}

	fn descendants(&self, selector: &str) -> Result<impl Iterator<Item = QueryMatch>> {
		self.ast().descendants_of(self.node_key(), selector).map_err(napi::Error::from_reason)
	}
}

#[napi]
impl Node {
	/// The kind of this node, as an index into the [`node_kinds`] table.
	#[napi(getter)]
	pub fn kind_id(&self) -> u32 {
		self.node_id as u32
	}

	#[napi(getter)]
	pub fn start(&self) -> u32 {
		self.start
	}

	#[napi(getter)]
	pub fn end(&self) -> u32 {
		self.end
	}

	/// The full source text given to `parse`.
	#[napi(getter)]
	pub fn source(&self) -> &str {
		self.ast().source()
	}

	/// The source text of this node.
	#[napi(getter)]
	pub fn text(&self) -> &str {
		self.ast().text_at(self.start, self.end)
	}

	/// The diagnostics of the parse. Includes diagnostics from outside this subtree.
	#[napi(getter)]
	pub fn diagnostics(&self) -> Vec<Diagnostic> {
		self.ast().diagnostics().to_vec()
	}

	/// True if this node matches `selector`.
	#[napi]
	pub fn matches(&self, selector: String) -> Result<bool> {
		self.ast().matches(self.node_key(), &selector).map_err(napi::Error::from_reason)
	}

	/// Every descendant that matches `selector`, for example `style-rule *[name=color]`.
	#[napi]
	pub fn query_selector_all(&self, this: Reference<Node>, env: Env, selector: String) -> Result<Vec<Node>> {
		let root = self.root_ref(this, env)?;
		self.descendants(&selector)?
			.map(|m| {
				Ok(Node {
					owner: Owner::Descendant(root.clone(env)?),
					node_id: m.node_id,
					node_key: Some(m.node_key),
					start: m.start,
					end: m.end,
				})
			})
			.collect()
	}

	/// The first descendant that matches `selector`, or `null`.
	#[napi]
	pub fn query_selector(&self, this: Reference<Node>, env: Env, selector: String) -> Result<Option<Node>> {
		let Some(m) = self.descendants(&selector)?.next() else { return Ok(None) };
		let root = self.root_ref(this, env)?;
		Ok(Some(Node {
			owner: Owner::Descendant(root),
			node_id: m.node_id,
			node_key: Some(m.node_key),
			start: m.start,
			end: m.end,
		}))
	}

	/// Runs the Rust visitor over this subtree. Calls the JS `enter` and `exit` callbacks.
	#[napi]
	pub fn accept(&self, this: Reference<Node>, env: Env, enter: Option<VisitFn>, exit: Option<VisitFn>) -> Result<()> {
		let root = self.root_ref(this, env)?;
		let gate = matches!(self.owner, Owner::Descendant(_)).then(|| self.node_key()).flatten();
		let mut sink = JsSink { ast: self.ast(), env, root, enter, exit, gate, err: None };
		self.ast().walk(&mut sink);
		match sink.err {
			Some(e) => Err(e),
			None => Ok(()),
		}
	}
}

/// Parses `source` as the node kind with this `tag`.
///
/// Each JS class carries its tag as a static. A tag also survives minification, but a class name
/// does not.
#[napi]
pub fn parse(source: String, tag: String) -> Result<Node> {
	let id =
		NodeId::from_tag_name(&tag).ok_or_else(|| napi::Error::from_reason(format!("{tag} is not a node kind")))?;
	let ast =
		Ast::parse(source, id).ok_or_else(|| napi::Error::from_reason(format!("{tag} cannot be parsed on its own")))?;
	let (start, end) = ast.span();
	Ok(Node { owner: Owner::Root(ast), node_id: id, node_key: None, start, end })
}

/// The tag of each AST node kind, indexed by [`Node::kind_id`].
#[napi]
pub fn node_kinds() -> Vec<&'static str> {
	NodeId::all_variants().map(NodeId::tag_name).collect()
}
