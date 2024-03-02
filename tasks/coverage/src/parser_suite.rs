use std::{
	io::{Error, ErrorKind},
	panic::UnwindSafe,
	path::Path,
};

use console::Style;
use hdx_ast::css::StyleSheet;
use hdx_parser::{Features, Parser, Spanned};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource, Report};
use oxc_allocator::Allocator;
use serde::Serialize;
use serde_json::to_string_pretty;
use similar::{ChangeTag, TextDiff};

use crate::AppArgs;

pub enum TestResult {
	ToRun,
	Passed,
	Skipped,
	Errored(Vec<Report>),
	Failed(Box<str>, Box<str>),
}

/// A Test Suite is responsible for reading code from a repository
pub trait ParserSuite<T: ParserCase> {
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
				println!("Running {}", case.name());
				result = case.execute(args);
			}
			match result {
				TestResult::ToRun => {
					fail_count += 1;
				}
				TestResult::Passed => {
					println!("{} {}", Style::new().green().apply_to("✔"), case.name());
				}
				TestResult::Skipped => {
					fail_count += 1;
				}
				TestResult::Errored(errors) => {
					fail_count += 1;
					let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode());
					let mut output = String::new();
					for error in errors {
						let error = error.with_source_code(NamedSource::new(
							case.path().to_str().unwrap(),
							case.source_text().to_owned(),
						));
						handler.render_report(&mut output, error.as_ref()).unwrap();
					}
					println!("{} {}", Style::new().red().apply_to("✘ ERRORED"), case.name());
					println!("{}", output);
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
pub trait ParserCase: Sized + Sync + Send + UnwindSafe {
	type AST: Serialize + PartialEq;

	fn name(&self) -> &str;
	fn source_text(&self) -> &str;
	fn path(&self) -> &Path;
	fn desired(&self) -> &Self::AST;
	fn update_desired(&self, ast: &Self::AST);
	fn convert_ast(&self, ast: &Spanned<StyleSheet>) -> Self::AST;
	fn desired_warnings(&self) -> String;
	fn update_warnings(&self, warnings: String);
	fn parser_options(&self, _args: &AppArgs) -> Features {
		Features::default()
	}

	/// Execute the parser once and get the test result
	fn execute(&mut self, args: &AppArgs) -> TestResult {
		let allocator = Allocator::default();
		let source_text = self.source_text().to_owned();
		let source_path = self.path();
		let parser = Parser::new(&allocator, &source_text, self.parser_options(args));
		let ret = parser.parse_with::<StyleSheet>();
		let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
		let mut warnings = String::new();
		for warn in ret.warnings {
			let warn = warn.with_source_code(NamedSource::new(source_path.to_str().unwrap(), source_text.to_string()));
			let _ = handler.render_report(&mut warnings, warn.as_ref());
		}
		println!("{}", &warnings);
		if !ret.errors.is_empty() {
			return TestResult::Errored(ret.errors);
		}
		let sheet = self.convert_ast(&ret.output.unwrap());
		if *self.desired() != sheet {
			if args.update {
				self.update_desired(&sheet);
			} else {
				return TestResult::Failed(
					to_string_pretty(&sheet).unwrap_or("".to_string()).into(),
					to_string_pretty(self.desired()).unwrap_or("".to_string()).into(),
				);
			}
		}
		if warnings != self.desired_warnings() {
			if args.update {
				self.update_warnings(warnings);
			} else {
				println!("{}", warnings);
				return TestResult::Failed(warnings.into(), self.desired_warnings().into());
			}
		}
		TestResult::Passed
	}
}
