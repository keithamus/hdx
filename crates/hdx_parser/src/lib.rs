mod comparison;
mod cursor;
mod cursor_stream;
pub mod diagnostics;
mod macros;
mod parser;
mod parser_return;
pub mod token_macros;
mod traits;

pub use bumpalo::{boxed::Box, collections::Vec};
pub use comparison::*;
pub use cursor_stream::*;
pub use hdx_lexer::Span;
pub use miette::{Error, Result};
pub use parser::*;
pub use parser_return::*;
pub use traits::*;

#[cfg(test)]
mod test {
	use bumpalo::{collections::String, Bump};

	use crate::*;

	#[test]
	fn smoke_test() {
		let allocator = Bump::default();
		let source_text = "foo bar";
		let mut p = Parser::new(&allocator, source_text, Features::none());
		let foo = p.parse::<T![Ident]>().unwrap();
		let bar = p.parse::<T![Ident]>().unwrap();
		let mut s = CursorStream::new(&allocator);
		foo.to_cursors(&mut s);
		bar.to_cursors(&mut s);
		let mut str = String::new_in(&allocator);
		s.write(source_text, &mut str).unwrap();
		assert_eq!(str, source_text);
	}
}
