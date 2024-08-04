use std::{env, path::Path};

use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, Searcher};

fn main() {
	println!("cargo::rerun-if-changed=build.rs");

	let matcher = RegexMatcher::new_line_matcher(
		"(atom!\\(\"|atomizable\\(\"|atom = \"|suffix = \"|prefix = \"|rename = \"|#\\[value\\(\")([^\"\\)]+)(?: \"\\)\\])?",
	)
	.unwrap();
	let mut matches = vec![];
	for entry in glob("../**/*.rs").unwrap() {
		let str = &entry.as_ref().unwrap().display();
		println!("cargo::rerun-if-changed={}", str);
		let mut searcher = Searcher::new();
		searcher
			.search_path(
				&matcher,
				entry.unwrap(),
				UTF8(|_lnum, line| {
					let mut captures = matcher.new_captures()?;
					matcher.captures_iter(line.as_bytes(), &mut captures, |captures| -> bool {
						let start = &line[captures.get(1).unwrap()];
						let capture = &line[captures.get(2).unwrap()];
						if start == "#[value(\"" {
							let keywords = capture
								.split("|")
								.map(|part| part.trim().trim_start_matches('[').trim_end_matches(']'))
								.filter(|part| !(part.is_empty() || part.starts_with('<')))
								.collect::<Vec<&str>>();
							println!("cargo::warning={:?}", &keywords);
							for keyword in keywords {
								matches.push(keyword.to_owned());
							}
						} else {
							matches.push(capture.to_owned());
						}
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
		.unwrap();
}
