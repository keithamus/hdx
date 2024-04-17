use std::{fs::read_to_string, path::PathBuf, time::Duration};

use criterion::{BenchmarkId, Criterion, Throughput};
use glob::glob;
use hdx_ast::css::StyleSheet;
use hdx_lexer::Lexer;
use hdx_parser::{Parser, Features};
use hdx_writer::{BaseCssWriter, WriteCss, OutputOption};
// use hdx_parser::{Parser, Features};
// use hdx_writer::{BaseCssWriter, WriteCss};
use bumpalo::Bump;

/// # Panics
/// Invalid Project Root
pub fn project_root() -> PathBuf {
	project_root::get_project_root().unwrap()
}

#[derive(Debug, Default)]
pub struct AppArgs {
	pub filter: Option<String>,
	pub save: Option<String>,
}

const FIXTURES_GLOB: &str = "tasks/coverage/popular/*.css";

struct TestFile {
	name: String,
	source_text: String,
}

impl AppArgs {
	pub fn run_all(&self) {
		self.run_lexer();
		// self.run_parser();
		// self.run_minifier();
	}

	fn get_files(&self) -> Vec<TestFile> {
		let mut files = vec![];
		for source_path in glob(FIXTURES_GLOB).unwrap().flatten() {
			files.push(TestFile {
				name: source_path.file_stem().unwrap().to_str().unwrap().to_owned(),
				source_text: read_to_string(&source_path).unwrap(),
			});
		}
		files
	}

	pub fn run_lexer(&self) {
		let measurement_time = Duration::new(/* seconds */ 15, 0);
		let mut criterion = Criterion::default().with_plots().measurement_time(measurement_time);
		if let Some(baseline) = &self.save {
			criterion = criterion.save_baseline(baseline.into());
		}
		let mut group = criterion.benchmark_group("lexer");
		for file in &self.get_files() {
			group.throughput(Throughput::Bytes(file.source_text.len() as u64));
			group.bench_with_input(BenchmarkId::from_parameter(&file.name), &file.source_text, |b, source_text| {
				b.iter_with_large_drop(|| {
					// Include the allocator drop time to make time measurement consistent.
					// Otherwise the allocator will allocate huge memory chunks (by power of two)
					// from the system allocator, which makes time measurement unequal during long
					// runs.
					let allocator = Bump::default();
					let mut lexer = Lexer::new(&allocator, source_text);
					loop {
						if lexer.advance() == hdx_lexer::Token::Eof {
							break;
						}
					}
					allocator
				});
			});
		}
		group.finish();
		drop(criterion);
	}

	pub fn run_parser(&self) {
		let measurement_time = Duration::new(/* seconds */ 15, 0);
		let mut criterion = Criterion::default().with_plots().measurement_time(measurement_time);
		if let Some(baseline) = &self.save {
			criterion = criterion.save_baseline(baseline.into());
		}
		let mut group = criterion.benchmark_group("parser");
		for file in &self.get_files() {
			group.throughput(Throughput::Bytes(file.source_text.len() as u64));
			group.bench_with_input(BenchmarkId::from_parameter(&file.name), &file.source_text, |b, source_text| {
				b.iter_with_large_drop(|| {
					let allocator = Allocator::default();
					let _ = Parser::new(&allocator, source_text, Features::default()).parse_with::<StyleSheet>();
					allocator
				});
			});
		}
		group.finish();
		drop(criterion);
	}

	pub fn run_minifier(&self) {
		let measurement_time = Duration::new(/* seconds */ 15, 0);
		let mut criterion = Criterion::default().with_plots().measurement_time(measurement_time);
		if let Some(baseline) = &self.save {
			criterion = criterion.save_baseline(baseline.into());
		}
		let mut group = criterion.benchmark_group("minify");
		for file in &self.get_files() {
			group.throughput(Throughput::Bytes(file.source_text.len() as u64));
			group.bench_with_input(BenchmarkId::from_parameter(&file.name), &file.source_text, |b, source_text| {
				b.iter_with_large_drop(|| {
					let allocator = Allocator::default();
					let result =
						Parser::new(&allocator, source_text.as_str(), Features::default()).parse_with::<StyleSheet>();
					{
						let mut string = String::new();
						let mut writer = BaseCssWriter::new(&mut string, OutputOption::none());
						if let Some(stylesheet) = &result.output {
							let _ = stylesheet.write_css(&mut writer).unwrap().to_owned();
						}
					}
					// TODO:
					// Figure out how to drop allocator without borrow checker
					// complaining.
				});
			});
		}
		group.finish();
		drop(criterion);
	}
}

#[test]
#[cfg(benchmark)]
fn test() {
	let args = AppArgs { filter: None, update: false, diff: false };
	args.run_all()
}
