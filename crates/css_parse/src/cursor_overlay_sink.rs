use crate::{Arena, Cursor, CursorSink, Kind, SourceCursor, SourceCursorSink, SourceOffset, Span, ToSpan, Vec};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct OverlaySegment<'a> {
	span: Span,
	cursors: Vec<'a, SourceCursor<'a>>,
	kind: OverlayKind,
}

impl<'a> OverlaySegment<'a> {
	pub fn new(span: Span, cursors: Vec<'a, SourceCursor<'a>>, kind: OverlayKind) -> Self {
		Self { span, cursors, kind }
	}

	pub fn start(&self) -> SourceOffset {
		self.span.start()
	}

	pub fn end(&self) -> SourceOffset {
		self.span.end()
	}

	pub fn cursors(&self) -> &[SourceCursor<'a>] {
		&self.cursors
	}

	pub fn is_insertion(&self) -> bool {
		matches!(self.kind, OverlayKind::InsertBefore | OverlayKind::InsertAfter)
	}

	pub fn kind(&self) -> OverlayKind {
		self.kind
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OverlayKind {
	Replace,
	InsertBefore,
	InsertAfter,
}

#[derive(Debug)]
pub struct CursorOverlaySet<'a> {
	segments: Vec<'a, OverlaySegment<'a>>,
}

impl<'a> CursorOverlaySet<'a> {
	pub fn new(alloc: &'a Arena) -> Self {
		Self { segments: Vec::new_in(alloc) }
	}

	pub fn insert(&mut self, span: Span, cursors: Vec<'a, SourceCursor<'a>>) {
		#[cfg(debug_assertions)]
		{
			let has_non_eof = cursors.iter().any(|cursor| cursor.token() != Kind::Eof);
			debug_assert!(has_non_eof || cursors.is_empty(), "Overlay for span {:?} produced no output", span);
		}
		self.push_segment(OverlaySegment::new(span, cursors, OverlayKind::Replace));
	}

	pub fn clear(&mut self) {
		self.segments.clear();
	}

	pub fn has_overlay(&self, span: Span) -> bool {
		let start = &span.start();
		let end = &span.end();
		self.segments
			.iter()
			.any(|segment| !segment.is_insertion() && segment.start() <= *start && *end <= segment.end())
	}

	pub fn push_segment(&mut self, segment: OverlaySegment<'a>) {
		let idx = self.segments.partition_point(|existing| {
			existing.start() < segment.start()
				|| (existing.start() == segment.start() && existing.end() <= segment.end())
		});
		self.segments.insert(idx, segment);
	}

	pub fn segments(&self) -> &[OverlaySegment<'a>] {
		&self.segments
	}
}

/// This is a [CursorSink] that wraps a [SourceCursorSink], while also taking a [CursorOverlaySet]. As [Cursor]s get
/// appended into this sink, it will replay those to the underlying [SourceCursorSink] _unless_ a [CursorOverlaySet]
/// overlaps the [Cursor]'s span, at which point the overlay wil be replayed to the underlying [SourceCursorSink].
/// This Sink is useful for collecting new Cursors (say from an AST) to overlap (or, say, transform) the underlying base
/// Cursors (read: AST). In other words, writing over the top of the source.
///
/// Other than replaying overlays in place of the underyling cursors, no other modifications are made to the Cursors,
/// that is up to the base SourceCursorSink, which can apply additional formatting or logic.
#[derive(Debug)]
pub struct CursorOverlaySink<'a, 'o, T: SourceCursorSink<'a>> {
	source_text: &'a str,
	overlays: &'o CursorOverlaySet<'a>,
	sink: T,
	processed_overlay_ranges: BTreeMap<SourceOffset, SourceOffset>,
	next_segment: usize,
	#[cfg(debug_assertions)]
	seen_eof: bool,
}

impl<'a, 'o, T: SourceCursorSink<'a>> CursorOverlaySink<'a, 'o, T> {
	pub fn new(source_text: &'a str, overlays: &'o CursorOverlaySet<'a>, sink: T) -> Self {
		Self {
			source_text,
			overlays,
			sink,
			processed_overlay_ranges: BTreeMap::new(),
			next_segment: 0,
			#[cfg(debug_assertions)]
			seen_eof: false,
		}
	}

	fn flush_segments_up_to(&mut self, limit: SourceOffset, include_after: bool) {
		let segments = self.overlays.segments();
		while self.next_segment < segments.len() {
			let segment = &segments[self.next_segment];
			if segment.start() > limit {
				break;
			}
			if !include_after && segment.kind() == OverlayKind::InsertAfter && segment.start() == limit {
				break;
			}
			for cursor in segment.cursors() {
				if cursor.token() != Kind::Eof {
					self.sink.append(*cursor);
				}
			}
			if !segment.is_insertion() {
				self.processed_overlay_ranges.insert(segment.start(), segment.end());
			}
			self.next_segment += 1;
		}
	}

	fn cursor_is_consumed(&self, cursor_start: SourceOffset, cursor_end: SourceOffset) -> bool {
		self.processed_overlay_ranges
			.range(..=cursor_start)
			.next_back()
			.is_some_and(|(&range_start, &range_end)| cursor_start >= range_start && cursor_end <= range_end)
	}
}

impl<'a, 'o, T: SourceCursorSink<'a>> SourceCursorSink<'a> for CursorOverlaySink<'a, 'o, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		let span = c.to_span();
		let cursor_start = span.start();
		let cursor_end = span.end();

		self.flush_segments_up_to(cursor_start, false);

		if self.cursor_is_consumed(cursor_start, cursor_end) {
			return;
		}

		self.sink.append(c);

		self.flush_segments_up_to(cursor_end, true);
	}
}

impl<'a, 'o, T: SourceCursorSink<'a>> CursorSink for CursorOverlaySink<'a, 'o, T> {
	fn append(&mut self, c: Cursor) {
		#[cfg(debug_assertions)]
		{
			debug_assert!(!self.seen_eof, "Received cursor after EOF: {:?}", c);
			if c == Kind::Eof {
				self.seen_eof = true;
			}
		}

		SourceCursorSink::append(self, SourceCursor::from(c, c.str_slice(self.source_text)))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::Arena;
	use crate::Vec;
	use crate::{
		ComponentValue, ComponentValues, CursorPrettyWriteSink, CursorToSourceCursorSink, CursorWriteSink,
		EmptyAtomSet, Parser, QuoteStyle, T, ToCursors, ToSpan,
	};
	use css_lexer::Lexer;

	fn snippet_cursors<'a>(alloc: &'a Arena, snippet: &'a str) -> Vec<'a, SourceCursor<'a>> {
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, snippet);
		let mut parser = Parser::new(alloc, snippet, lexer);
		let parsed = parser.parse_entirely::<ComponentValues<'a>>();
		let mut cursors = Vec::new_in(alloc);
		let mut sink = CursorToSourceCursorSink::new(snippet, &mut cursors);
		parsed.to_cursors(&mut sink);
		cursors
	}

	#[test]
	fn test_basic() {
		let source_text = "black white";
		let alloc = Arena::default();
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&alloc, source_text, lexer);
		let output = p.parse_entirely::<(T![Ident], T![Ident])>().output.unwrap();

		let overlay_text = "green";
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, overlay_text);
		let mut p = Parser::new(&alloc, overlay_text, lexer);
		let overlay = p.parse_entirely::<T![Ident]>();
		let mut source_cursors = Vec::new_in(&alloc);
		let mut sink = CursorToSourceCursorSink::new(overlay_text, &mut source_cursors);
		overlay.to_cursors(&mut sink);
		let mut overlays = CursorOverlaySet::new(&alloc);
		overlays.insert(output.1.to_span(), source_cursors);

		let mut str = String::new();
		let mut stream = CursorOverlaySink::new(source_text, &overlays, CursorWriteSink::new(source_text, &mut str));
		output.to_cursors(&mut stream);

		assert_eq!(str, "black green");
	}

	#[test]
	fn test_with_pretty_writer() {
		let source_text = "foo{use:other;}";
		let alloc = Arena::default();
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&alloc, source_text, lexer);
		let output = p.parse_entirely::<Vec<'_, ComponentValue>>().output.unwrap();
		let ComponentValue::SimpleBlock(ref block) = output[1] else { panic!("output[1] was not a block") };

		let overlay_text = "inner{foo: bar;}";
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, overlay_text);
		let mut p = Parser::new(&alloc, overlay_text, lexer);
		let overlay = p.parse_entirely::<Vec<'_, ComponentValue>>();
		let mut source_cursors = Vec::new_in(&alloc);
		let mut sink = CursorToSourceCursorSink::new(overlay_text, &mut source_cursors);
		overlay.to_cursors(&mut sink);
		let mut overlays = CursorOverlaySet::new(&alloc);
		overlays.insert(block.values.to_span(), source_cursors);

		let mut str = String::new();
		let mut stream = CursorOverlaySink::new(
			source_text,
			&overlays,
			CursorPrettyWriteSink::new(source_text, &mut str, None, QuoteStyle::Double),
		);
		output.to_cursors(&mut stream);

		assert_eq!(
			str,
			r#"
foo {
	inner {
		foo: bar;
	}
}
			"#
			.trim()
		);
	}

	#[test]
	fn test_insert_before_and_after() {
		let source_text = "ab";
		let alloc = Arena::default();
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let output = parser.parse_entirely::<Vec<'_, ComponentValue>>().output.unwrap();

		let mut overlays = CursorOverlaySet::new(&alloc);
		overlays.push_segment(OverlaySegment::new(
			Span::new(SourceOffset(0), SourceOffset(0)),
			snippet_cursors(&alloc, "pre"),
			OverlayKind::InsertBefore,
		));
		overlays.push_segment(OverlaySegment::new(
			Span::new(SourceOffset(2), SourceOffset(2)),
			snippet_cursors(&alloc, "post"),
			OverlayKind::InsertAfter,
		));

		let mut str = String::new();
		let mut stream = CursorOverlaySink::new(source_text, &overlays, CursorWriteSink::new(source_text, &mut str));
		output.to_cursors(&mut stream);
		assert_eq!(str, "pre ab post");
	}

	#[test]
	fn test_multiple_inserts_preserve_order() {
		let source_text = "x";
		let alloc = Arena::default();
		let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let output = parser.parse_entirely::<Vec<'_, ComponentValue>>().output.unwrap();

		let mut overlays = CursorOverlaySet::new(&alloc);
		overlays.push_segment(OverlaySegment::new(
			Span::new(SourceOffset(0), SourceOffset(0)),
			snippet_cursors(&alloc, "A"),
			OverlayKind::InsertBefore,
		));
		overlays.push_segment(OverlaySegment::new(
			Span::new(SourceOffset(0), SourceOffset(0)),
			snippet_cursors(&alloc, "B"),
			OverlayKind::InsertBefore,
		));
		overlays.push_segment(OverlaySegment::new(
			Span::new(SourceOffset(1), SourceOffset(1)),
			snippet_cursors(&alloc, "C"),
			OverlayKind::InsertAfter,
		));
		overlays.push_segment(OverlaySegment::new(
			Span::new(SourceOffset(1), SourceOffset(1)),
			snippet_cursors(&alloc, "D"),
			OverlayKind::InsertAfter,
		));

		let mut str = String::new();
		let mut stream = CursorOverlaySink::new(source_text, &overlays, CursorWriteSink::new(source_text, &mut str));
		output.to_cursors(&mut stream);
		assert_eq!(str, "A B x C D");
	}
}
