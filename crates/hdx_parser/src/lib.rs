#![cfg_attr(not(target_arch = "wasm32"), feature(portable_simd))]
#![feature(slice_as_chunks)]

mod css;
mod cursor;
mod diagnostics;

pub use hdx_ast::{css::stylesheet::CSSStyleSheet, Spanned, Unit};
pub use hdx_atom::{atom, Atom, Atomizable};
pub use hdx_lexer::{Kind, Lexer, PairWise, Span, Token};
pub use miette::{Error, Result};
pub use oxc_allocator::Allocator;
pub(crate) use oxc_allocator::Vec;

pub trait Parse<'a>: Sized {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>>;

	fn spanned(self, span: Span) -> Spanned<Self> {
		Spanned { node: self, span }
	}
}

#[derive(Debug, Default)]
pub enum SourceType {
	#[default]
	CSS,
	SCSS,
}

pub struct Parser<'a> {
	lexer: Lexer<'a>,

	pub source_type: SourceType,

	sloppy: bool,

	warnings: std::vec::Vec<Error>,

	errors: std::vec::Vec<Error>,

	token: Token,

	prev_span: Span,

	allocator: &'a Allocator,
}

#[derive(Debug, Default)]
pub struct ParserOptions {
	source_type: SourceType,
	sloppy: bool,
}

pub struct ParserReturn<T> {
	pub output: Option<T>,
	pub errors: std::vec::Vec<Error>,
	pub warnings: std::vec::Vec<Error>,
	pub panicked: bool,
}

impl<'a> Parser<'a> {
	/// Create a new parser
	pub fn new(allocator: &'a Allocator, source_text: &'a str, options: ParserOptions) -> Self {
		Self {
			lexer: Lexer::new(allocator, source_text),
			source_type: options.source_type,
			sloppy: options.sloppy,
			warnings: std::vec::Vec::new(),
			errors: std::vec::Vec::new(),
			token: Token::default(),
			prev_span: Span::dummy(),
			allocator,
			// state: ParserState::new(allocator),
			// ctx: Self::default_context(source_type),
		}
	}

	#[inline]
	pub fn new_vec<T>(&self) -> crate::Vec<'a, T> {
		crate::Vec::new_in(self.allocator)
	}

	#[inline]
	pub fn boxup<T>(&self, value: T) -> oxc_allocator::Box<'a, T> {
		oxc_allocator::Box(self.allocator.alloc(value))
	}

	pub fn parse(self) -> ParserReturn<Spanned<CSSStyleSheet<'a>>> {
		self.parse_entirely_with::<CSSStyleSheet<'a>>()
	}

	pub fn parse_entirely_with<T: Parse<'a>>(mut self) -> ParserReturn<Spanned<T>> {
		self.advance();
		let (output, panicked) = match T::parse(&mut self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		self.skip_trivia();
		if !self.at(Kind::Eof) {
			let span = self.cur().span;
			loop {
				if self.at(Kind::Eof) {
					break;
				}
				self.advance()
			}
			self.errors.push(diagnostics::ExpectedEnd(span.until(self.cur().span)).into());
		}
		ParserReturn { output, warnings: self.warnings, errors: self.errors, panicked }
	}

	pub fn parse_with<T: Parse<'a>>(mut self) -> ParserReturn<Spanned<T>> {
		self.advance();
		let (output, panicked) = match T::parse(&mut self) {
			Ok(output) => (Some(output), false),
			Err(error) => {
				self.errors.push(error);
				(None, true)
			}
		};
		ParserReturn { output, warnings: self.warnings, errors: self.errors, panicked }
	}

	fn parse_comma_list_of<T: Parse<'a>>(&mut self) -> Result<oxc_allocator::Vec<'a, Spanned<T>>> {
		let mut vec = self.new_vec();
		let mut last_kind;
		loop {
			vec.push(T::parse(self)?);
			match self.cur().kind {
				Kind::Comma => {
					self.expect(Kind::Comma)?;
					last_kind = Kind::Comma;
				}
				k => {
					last_kind = k;
					break;
				}
			}
		}
		if last_kind == Kind::Comma {
			let warn: Error = diagnostics::WarnTrailing(self.cur().kind, self.cur().span).into();
			if !self.sloppy {
				Err(warn)?;
			}
		}
		Ok(vec)
	}
}

#[cfg(test)]
mod test {
	use oxc_allocator::Allocator;

	use crate::{Parser, ParserOptions};

	#[test]
	fn smoke_basic_error() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "c", ParserOptions::default());
		let parser_return = parser.parse();
		assert_eq!(parser_return.warnings.len(), 1);
		assert_eq!(parser_return.errors.len(), 1);
		assert!(parser_return.output.is_none());
		println!("{:?}", parser_return.warnings.get(0).unwrap());
		println!("{:?}", parser_return.errors.get(0).unwrap());
	}
}
