use crate::{Span, ToSpan};
use allocator_api2::alloc::Allocator;
use csskit_arena::{Box, Vec};

impl<'a, T: ToSpan, A: Allocator> ToSpan for Box<'a, T, A> {
	fn to_span(&self) -> Span {
		(**self).to_span()
	}
}

impl<'a, T: ToSpan, A: Allocator> ToSpan for Vec<'a, T, A> {
	fn to_span(&self) -> Span {
		self[..].to_span()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::SourceOffset;
	use csskit_arena::Arena;

	#[test]
	fn empty_vec_does_not_widen_span() {
		let arena = Arena::default();
		let empty: Vec<Span> = Vec::new_in(&arena);
		let span = Span::new(SourceOffset(5), SourceOffset(9));
		assert_eq!(empty.to_span(), Span::DUMMY);
		assert_eq!(span + empty.to_span(), span);
	}

	#[test]
	fn vec_spans_its_items() {
		let arena = Arena::default();
		let mut items = Vec::new_in(&arena);
		items.push(Span::new(SourceOffset(5), SourceOffset(9)));
		items.push(Span::new(SourceOffset(12), SourceOffset(20)));
		assert_eq!(items.to_span(), Span::new(SourceOffset(5), SourceOffset(20)));
	}
}
