use clap::Parser;
use hdx_parser::Allocator;
use hdx_writer::{BaseCssWriter, WriteCss};

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

	let source_text = std::fs::read_to_string(args.input.first().unwrap()).unwrap();
	let allocator = Allocator::default();
	let result = hdx_parser::Parser::new(
		&allocator,
		source_text.as_str(),
		hdx_parser::ParserOptions::default(),
	)
	.parse();
	{
		let start = std::time::Instant::now();
		let mut str = String::new();
		let mut writer = BaseCssWriter::new(&mut str, args.minify);
		if let Some(stylesheet) = &result.output {
			stylesheet.write_css(&mut writer).unwrap();
			if let Some(file) = args.output {
				std::fs::write(file, str.as_bytes()).unwrap();
			} else {
				println!("{}", str);
				eprintln!("Slurped up CSS in {:?}! Neat!", start.elapsed());
			}
		} else {
			eprintln!("{:?}", result.errors);
		}
	}
}
