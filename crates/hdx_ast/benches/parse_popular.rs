use bumpalo::Bump;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use glob::glob;
use hdx_ast::css::StyleSheet;
use hdx_parser::{Features, Parser};
use pprof::criterion::{Output, PProfProfiler};
use std::fs::read_to_string;

const FIXTURES_GLOB: &str = "../../tasks/coverage/popular/*.css";

struct TestFile {
	name: String,
	source_text: String,
}

fn get_files() -> Vec<TestFile> {
	let mut files = vec![];
	for source_path in glob(FIXTURES_GLOB).unwrap().flatten() {
		files.push(TestFile {
			name: source_path.file_stem().unwrap().to_str().unwrap().to_owned(),
			source_text: read_to_string(&source_path).unwrap(),
		});
	}
	files
}

fn popular(c: &mut Criterion) {
	let mut group = c.benchmark_group("parse_popular");
	for file in get_files() {
		group.throughput(Throughput::Bytes(file.source_text.len() as u64));
		group.bench_with_input(BenchmarkId::from_parameter(&file.name), &file.source_text, |b, source_text| {
			b.iter_with_large_drop(|| {
				let allocator = Bump::default();
				let _ = Parser::new(&allocator, source_text, Features::default()).parse_entirely_with::<StyleSheet>();

				allocator
			});
		});
	}
	group.finish();
}

criterion_group! {
	name = benches;
	config = Criterion::default()
		.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = popular
}

criterion_main!(benches);
