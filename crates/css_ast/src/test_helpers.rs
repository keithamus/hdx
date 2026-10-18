#[cfg(feature = "visitable")]
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VisitEvent {
	Enter(&'static str),
	Exit(&'static str),
}

#[cfg(feature = "visitable")]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TestVisitor {
	pub visits: Vec<&'static str>,
	pub events: Vec<VisitEvent>,
	stack: Vec<&'static str>,
}

#[cfg(feature = "visitable")]
impl TestVisitor {
	pub fn new() -> Self {
		Self { visits: vec![], events: vec![], stack: vec![] }
	}

	pub(crate) fn handle_visit(&mut self, type_name: &'static str) {
		self.visits.push(type_name);
		self.events.push(VisitEvent::Enter(type_name));
		self.stack.push(type_name);
	}

	pub(crate) fn handle_exit(&mut self, type_name: &'static str) {
		self.events.push(VisitEvent::Exit(type_name));
		if let Some(last) = self.stack.pop() {
			if last != type_name {
				panic!("Unbalanced visit/exit: expected to exit {:?} but got exit {:?}", last, type_name);
			}
		} else {
			panic!("Exit {:?} called without matching visit", type_name);
		}
	}

	pub fn validate_balanced(&self) {
		if !self.stack.is_empty() {
			panic!("Unbalanced visit/exit: {} nodes were visited but not exited: {:?}", self.stack.len(), self.stack);
		}
	}
}

#[cfg(feature = "visitable")]
macro_rules! visit_mut_trait {
	( $(
		$name: ident$(<$($gen:tt),+>)?($obj: ident$(<$($ogen:tt),+>)?),
	)+ ) => {
		impl $crate::VisitMut for TestVisitor {
			$(
				fn $name$(<$($gen),+>)?(&mut self, _rule: &mut $crate::$obj$(<$($ogen),+>)?) {
					// Only handle visits for visit_* methods, not exit_* methods
					let name = stringify!($name);
					if name.starts_with("visit_") {
						self.handle_visit(stringify!($obj));
					} else if name.starts_with("exit_") {
						self.handle_exit(stringify!($obj));
					}
				}
			)+
		}
	}
}
#[cfg(feature = "visitable")]
include!(concat!(env!("OUT_DIR"), "/css_apply_visit_methods.rs"));
#[cfg(feature = "visitable")]
apply_visit_methods!(visit_mut_trait);

#[macro_export]
#[cfg(feature = "visitable")]
macro_rules! assert_visits {
	($source: expr, $parse_type: ty $(, $visit_type: ty)* $(,)?) => {{
		use css_parse::{Arena, Parser};
		use css_lexer::Lexer;
		use $crate::VisitableMut;

		let alloc = Arena::default();
		let source_text = $source;
		let lexer = Lexer::new(&$crate::CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let result = parser.parse_entirely::<$parse_type>();
		if !result.errors.is_empty() {
			panic!("\n\nParse {:?} failed. Saw error {:?}", source_text, result.errors[0]);
		}
		let mut parsed = result.output.unwrap();

		let mut visitor = $crate::test_helpers::TestVisitor::new();
		parsed.accept_mut(&mut visitor);

		// Validate that all visit calls have matching exit calls
		visitor.validate_balanced();

		let actual_visits = visitor.visits.as_slice();
		let expected_visits = vec![ stringify!($parse_type), $( stringify!($visit_type) ),* ];

		if actual_visits != expected_visits {
			panic!(
				"\n\nVisit assertion failed for {:?}:\n\nActual visits: {:?}\nExpected visits: {:?}",
				source_text,
				actual_visits,
				expected_visits,
			);
		}
	}};
}

/// Parse `$source` as `$ty`, run a [`ControlFlowTestVisitor`] over it, and assert the
/// `entered`/`exited` node sequences (and optionally the resulting [`VisitFlow`]).
///
/// Node names are bare idents; the macro prefixes them with `NodeId::`. The visitor is
/// configured with either `descend` (no control flow), a `callback: |id| ...` returning
/// a [`VisitFlow`], or a `span_filter: <Span>`.
#[cfg(feature = "visitable")]
macro_rules! assert_visit_flow {
	($source:expr, $ty:ty, $visitor:expr,
		entered: [$($enter:ident),* $(,)?],
		exited: [$($exit:ident),* $(,)?]
		$(, result: $result:pat)? $(,)?
	) => {{
		use $crate::Visitable;
		use $crate::NodeId;
		let alloc = ::css_parse::Arena::new();
		let lexer = css_lexer::Lexer::new(&$crate::CssAtomSet::ATOMS, $source);
		let mut parser = css_parse::Parser::new(&alloc, $source, lexer);
		let parsed = parser.parse_entirely::<$ty>().output.expect("parse failed");
		let mut v = $visitor;
		let _result = parsed.accept(&mut v);
		assert_eq!(v.entered(), vec![$(NodeId::$enter),*], "entered mismatch for {:?}", $source);
		assert_eq!(v.exited(), vec![$(NodeId::$exit),*], "exited mismatch for {:?}", $source);
		$(assert!(matches!(_result, $result), "result mismatch for {:?}: got {:?}", $source, _result);)?
	}};
}
#[cfg(feature = "visitable")]
pub(crate) use assert_visit_flow;

#[cfg(feature = "css_feature_data")]
#[macro_export]
macro_rules! assert_feature_id {
	($source: expr, $ty: ty, $id: literal) => {{
		use css_lexer::Lexer;
		use css_parse::{Arena, Parser};
		let alloc = Arena::default();
		let source_text = $source;
		let lexer = Lexer::new(&$crate::CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let result = parser.parse_entirely::<$ty>();
		if !result.errors.is_empty() {
			panic!("\n\nParse {:?} failed. Saw error {:?}", source_text, result.errors[0]);
		}
		assert_eq!(result.output.unwrap().to_css_feature().unwrap().id, $id);
	}};
}

/// A visitor for testing `VisitFlow` control: `Stop`, `SkipChildren`, and span filtering.
///
/// Records enter/exit events for every queryable node visited. The `on_visit` callback
/// controls what `VisitFlow` is returned from `enter_node`, allowing tests to
/// simulate early termination or child-skipping without writing a bespoke visitor each time.
#[cfg(feature = "visitable")]
use visit_flow::{VisitFlow, VisitFlowExt};

#[cfg(feature = "visitable")]
pub(crate) struct ControlFlowTestVisitor {
	pub events: Vec<(bool, crate::NodeId)>, // (is_enter, node_id)
	span_filter: Option<css_lexer::Span>,
	on_visit: Box<dyn Fn(crate::NodeId) -> VisitFlow>,
}

#[cfg(feature = "visitable")]
impl ControlFlowTestVisitor {
	/// Records all nodes without any control flow.
	pub fn new() -> Self {
		Self::with_callback(|_| VisitFlow::DESCEND)
	}

	pub fn with_callback(on_visit: impl Fn(crate::NodeId) -> VisitFlow + 'static) -> Self {
		Self { events: Vec::new(), span_filter: None, on_visit: Box::new(on_visit) }
	}

	/// Only visit nodes whose span overlaps `span`.
	pub fn with_span_filter(span: css_lexer::Span) -> Self {
		Self { events: Vec::new(), span_filter: Some(span), on_visit: Box::new(|_| VisitFlow::DESCEND) }
	}

	pub fn entered(&self) -> Vec<crate::NodeId> {
		self.events.iter().filter_map(|(enter, id)| enter.then_some(*id)).collect()
	}

	pub fn exited(&self) -> Vec<crate::NodeId> {
		self.events.iter().filter_map(|(enter, id)| (!enter).then_some(*id)).collect()
	}
}

#[cfg(feature = "visitable")]
impl crate::Visit for ControlFlowTestVisitor {}

#[cfg(feature = "visitable")]
impl crate::NodeVisitor for ControlFlowTestVisitor {
	fn consider_node(&self, node: crate::VisitNode) -> VisitFlow {
		if let Some(filter_span) = self.span_filter {
			let span = node.span();
			// Prune if no overlap: span must share at least one byte with filter_span.
			if span.start() >= filter_span.end() || filter_span.start() >= span.end() {
				return VisitFlow::SKIP_CHILDREN;
			}
		}
		VisitFlow::DESCEND
	}

	fn enter_node(&mut self, node: crate::VisitNode) -> VisitFlow {
		let node_id = node.node_id.unwrap();
		self.events.push((true, node_id));
		(self.on_visit)(node_id)
	}

	fn exit_node(&mut self, query: crate::VisitNode) -> VisitFlow {
		self.events.push((false, query.node_id.unwrap()));
		VisitFlow::DESCEND
	}
}

#[cfg(all(test, feature = "visitable"))]
mod tests {
	use super::*;
	use crate::{Color, VisitMut, VisitableMut};
	use css_parse::{Arena, Parse, Parser};

	// Test visitor that intentionally skips exit calls
	struct UnbalancedVisitor {
		visitor: TestVisitor,
	}

	impl UnbalancedVisitor {
		fn new() -> Self {
			Self { visitor: TestVisitor::new() }
		}
	}

	impl VisitMut for UnbalancedVisitor {
		fn visit_color(&mut self, _: &mut Color) {
			self.visitor.handle_visit("Color");
			// Intentionally NOT calling handle_exit
		}
		// Note: exit_color is not overridden, so it won't call handle_exit
	}

	#[test]
	#[should_panic(expected = "Unbalanced visit/exit: 1 nodes were visited but not exited")]
	fn test_unbalanced_visitor_panics() {
		use css_lexer::Lexer;

		let alloc = Arena::new();
		let input = "red";
		let lexer = Lexer::new(&crate::CssAtomSet::ATOMS, input);
		let mut parser = Parser::new(&alloc, input, lexer);

		let mut color = Color::parse(&mut parser).unwrap();
		let mut visitor = UnbalancedVisitor::new();
		color.accept_mut(&mut visitor);

		// This should panic because we visited but didn't exit
		visitor.visitor.validate_balanced();
	}

	#[test]
	fn test_balanced_visitor_succeeds() {
		use css_lexer::Lexer;

		let alloc = Arena::new();
		let input = "red";
		let lexer = Lexer::new(&crate::CssAtomSet::ATOMS, input);
		let mut parser = Parser::new(&alloc, input, lexer);

		let mut color = Color::parse(&mut parser).unwrap();
		let mut visitor = TestVisitor::new();
		color.accept_mut(&mut visitor);

		// This should succeed - the auto-generated implementation is balanced
		visitor.validate_balanced();

		// Verify we got both enter and exit events
		assert!(visitor.events.len() >= 2);
		assert!(matches!(visitor.events[0], VisitEvent::Enter("Color")));
		assert!(matches!(visitor.events[visitor.events.len() - 1], VisitEvent::Exit("Color")));
	}
}
