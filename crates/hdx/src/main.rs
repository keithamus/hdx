use clap::Parser;
use hdx_ast::css::StyleSheet;
use hdx_writer::{BaseCssWriter, WriteCss, OutputOption};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use oxc_allocator::Allocator;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[clap(value_parser)]
	input: Vec<String>,
	#[clap(short, long, value_parser)]
	minify: bool,
	#[clap(short, long, group = "output_file", value_parser)]
	output: Option<String>,
}

fn main() {
	let args = Cli::parse();

	if args.input.is_empty() {
		panic!("Need input file");
	}

	if args.input.len() > 1 {
		todo!("Can't handle multiple files yet")
	}

	let file_name = args.input.first().unwrap();
	let source_text = std::fs::read_to_string(file_name).unwrap();
	let allocator = Allocator::default();
	let result = hdx_parser::Parser::new(&allocator, source_text.as_str(), hdx_parser::Features::default())
		.parse_with::<StyleSheet>();
	{
		let start = std::time::Instant::now();
		let mut str = String::new();
		let opts = if args.minify {
			OutputOption::none()
		} else {
			OutputOption::all()
		};
		let mut writer = BaseCssWriter::new(&mut str, opts);
		if let Some(stylesheet) = &result.output {
			stylesheet.write_css(&mut writer).unwrap();
			if let Some(file) = args.output {
				std::fs::write(file, str.as_bytes()).unwrap();
			} else {
				println!("{}", str);
				eprintln!("Slurped up CSS in {:?}! Neat!", start.elapsed());
			}
		} else {
			let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
			for err in result.errors {
				let mut report = String::new();
				let named = NamedSource::new(file_name, source_text.clone());
				let err = err.with_source_code(named);
				handler.render_report(&mut report, err.as_ref()).unwrap();
				println!("{}", report);
			}
		}
	}
}
