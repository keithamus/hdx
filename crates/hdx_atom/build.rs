use std::{env, path::Path};

use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, Searcher};

fn main() {
	let matcher = RegexMatcher::new_line_matcher(
		"(?:atom!\\(\"|atomizable\\(\"|atom = \"|suffix = \"|prefix = \"|rename = \")([^\"]+)",
	)
	.unwrap();
	let mut matches = vec![];
	for entry in glob("../**/*.rs").unwrap() {
		let mut searcher = Searcher::new();
		searcher
			.search_path(
				&matcher,
				entry.unwrap(),
				UTF8(|_lnum, line| {
					let mut captures = matcher.new_captures()?;
					matcher.captures_iter(line.as_bytes(), &mut captures, |captures| -> bool {
						matches.push(line[captures.get(1).unwrap()].to_owned());
						true
					})?;
					Ok(true)
				}),
			)
			.unwrap();
	}

	matches.sort();

	string_cache_codegen::AtomType::new("Atom", "atom!")
		.atoms(matches)
		.write_to_file(&Path::new(&env::var("OUT_DIR").unwrap()).join("hdx_atom.rs"))
		.unwrap()
}
