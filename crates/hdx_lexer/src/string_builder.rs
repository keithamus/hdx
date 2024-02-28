// Copied from https://github.com/mozilla-spidermonkey/jsparagus/blob/master/crates/parser/src/lexer.rs#L2256

use bumpalo::collections::String;

use crate::Lexer;

pub struct AutoCow<'a> {
	pub start: &'a str,
	pub value: Option<String<'a>>,
}

impl<'a> AutoCow<'a> {
	pub fn new(lexer: &Lexer<'a>) -> Self {
		let start = lexer.remaining();
		AutoCow { start, value: None }
	}

	// Push a char that matches lexer.chars().next()
	pub fn push_matching(&mut self, c: char) {
		if let Some(text) = &mut self.value {
			text.push(c);
		}
	}

	// Push a different character than lexer.chars().next().
	// force_allocation_without_current_ascii_char must be called before this.
	pub fn push_different(&mut self, c: char) {
		debug_assert!(self.value.is_some());
		self.value.as_mut().unwrap().push(c);
	}

	// Force allocation of a String, excluding the current ASCII character.
	pub fn force_allocation_without_current_ascii_char(&mut self, lexer: &'_ Lexer<'a>) {
		if self.value.is_some() {
			return;
		}
		self.value = if self.start.len() == lexer.remaining().len() {
			Some(String::from_str_in("", lexer.allocator))
		} else {
			Some(String::from_str_in(&self.start[..self.start.len() - lexer.remaining().len() - 1], lexer.allocator))
		};
	}

	pub fn finish(mut self, lexer: &Lexer<'a>) -> &'a str {
		match self.value.take() {
			Some(s) => s.into_bump_str(),
			None => &self.start[..self.start.len() - lexer.remaining().len()],
		}
	}

	// Just like finish, but without pushing current char.
	pub fn finish_without_push(mut self, lexer: &Lexer<'a>) -> &'a str {
		match self.value.take() {
			Some(s) => s.into_bump_str(),
			None => &self.start[..self.start.len() - lexer.remaining().len() - 1],
		}
	}
}
