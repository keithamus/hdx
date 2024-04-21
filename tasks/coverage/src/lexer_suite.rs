use std::{
	io::{Error, ErrorKind},
	path::Path,
};

use bumpalo::Bump;
use console::Style;
use hdx_lexer::{Include, Lexer, Token};
use serde::Serialize;
use serde_json::to_string_pretty;
use similar::{ChangeTag, TextDiff};

use crate::AppArgs;

pub enum TestResult {
	ToRun,
	Passed,
	Skipped,
	Failed(Box<str>, Box<str>),
}

/// A Test Suite is responsible for reading code from a repository
pub trait LexerSuite<T: LexerCase> {
	fn new(args: &AppArgs) -> Self;

	fn get_tests(&mut self, args: &AppArgs) -> Vec<T>;

	fn run(&mut self, args: &AppArgs) -> Result<(), Error> {
		let cases = self.get_tests(args);
		let mut fail_count = 0;
		for mut case in cases {
			let mut result = TestResult::ToRun;
			if let Some(filter) = &args.filter {
				filter.as_str().split(',').for_each(|f| {
					if !case.name().contains(f) {
						result = TestResult::Skipped;
					}
				})
			}
			if !matches!(result, TestResult::Skipped) {
				println!("executing {}", case.name());
				result = case.execute(args);
			}
			match result {
				TestResult::ToRun => {
					fail_count += 1;
					print!("X");
				}
				TestResult::Passed => {
					println!("{} {}", Style::new().green().apply_to("✔"), case.name());
				}
				TestResult::Skipped => {
					fail_count += 1;
					print!("S");
				}
				TestResult::Failed(old, new) => {
					fail_count += 1;
					println!("{} {}", Style::new().red().apply_to("✘ FAILED"), case.name());
					if args.diff {
						println!("{}", Style::new().red().apply_to("- actual"));
						println!("{}", Style::new().green().apply_to("+ expected"));
						let diff = TextDiff::from_lines(&*old, &*new);
						for change in diff.iter_all_changes() {
							let (sign, style) = match change.tag() {
								ChangeTag::Delete => ("-", Style::new().red()),
								ChangeTag::Insert => ("+", Style::new().green()),
								ChangeTag::Equal => (" ", Style::new()),
							};
							print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
						}
					}
				}
			}
		}

		if args.filter.is_none() {
			// self.snapshot_errors(&report).unwrap();
		}
		if fail_count > 0 {
			println!("{} {}", Style::new().red().apply_to("✘ FAILED"), fail_count);
			Err(Error::new(ErrorKind::Other, "Test Failed"))?;
		}
		Ok(())
	}
}

/// A Test Case is responsible for interpreting the contents of a file
pub trait LexerCase: Sized {
	type Token: Serialize + PartialEq;

	fn name(&self) -> &str;
	fn source_text(&self) -> &str;
	fn path(&self) -> &Path;
	fn desired(&self) -> &Vec<Self::Token>;
	fn convert_token(&self, start: usize, end: usize, token: &Token) -> Self::Token;
	fn update_desired(&self, tokens: &Vec<Self::Token>);

	/// Execute the parser once and get the test result
	fn execute(&mut self, args: &AppArgs) -> TestResult {
		let allocator = Bump::default();
		let source_text = self.source_text();
		let mut lexer = Lexer::new(&allocator, source_text);
		let mut tokens = vec![];
		let mut pos = 0;
		loop {
			let token = lexer.advance_with(Include::all());
			if token == Token::Eof {
				break;
			}
			tokens.push(self.convert_token(pos, lexer.pos() as usize, &token));
			pos = lexer.pos() as usize;
		}
		if tokens != *self.desired() {
			if args.update {
				self.update_desired(&tokens);
			} else {
				return TestResult::Failed(
					to_string_pretty(&tokens).unwrap_or("".to_string()).into(),
					to_string_pretty(self.desired()).unwrap_or("".to_string()).into(),
				);
			}
		}
		TestResult::Passed
	}
}
