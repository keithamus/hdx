use std::{collections::HashSet, env, path::Path};

use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, Searcher};

pub fn kebab(str: String) -> String {
	let mut kebab = String::new();
	for (i, ch) in str.char_indices() {
		if i > 0 && ch.is_uppercase() {
			kebab.push('-');
		}
		kebab.push(ch.to_ascii_lowercase());
	}
	kebab
}

fn main() {
	println!("cargo::rerun-if-changed=build.rs");

	let matcher = RegexMatcher::new_line_matcher(
		"(atom!\\(\"|atomizable\\(\"|suffix = \"|https://drafts.csswg.org/css[^/]+/#|#\\[value\\(\")([^\"\\)]+)(?: \"\\)\\])?",
	)
	.unwrap();
	let mut matches = HashSet::new();
	matches.insert("%".to_owned());
	for entry in glob("../**/*.rs").unwrap() {
		// for entry in glob("../**/values/ui/mod.rs").unwrap() {
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
						dbg!(&line, &line[captures.get(0).unwrap()]);
						let start = &line[captures.get(1).unwrap()];
						let capture = &line[captures.get(2).unwrap()];
						dbg!(&start, &capture);
						if start == "#[value(\"" {
							let keywords = capture
								.split(" | ")
								.map(|part| {
									part.trim()
										.split(" ")
										.next()
										.unwrap_or("")
										.trim_start_matches('[')
										.trim_end_matches(']')
										.trim_end_matches('?')
										.trim_end_matches('(')
										.trim_start_matches(' ')
										.trim()
								})
								.filter(|part| !(part.is_empty() || part.starts_with('<')))
								.collect::<Vec<&str>>();
							for keyword in keywords {
								if keyword.chars().all(|c| c == '-' || char::is_alphanumeric(c)) {
									matches.insert(keyword.to_owned());
								}
							}
						} else if capture.chars().all(|c| c == '-' || c == '_' || char::is_alphanumeric(c)) {
							matches.insert(capture.to_owned());
						}
						true
					})?;
					Ok(true)
				}),
			)
			.unwrap();
	}

	let matcher = RegexMatcher::new_line_matcher("^pub (?:struct|enum) (\\w*)").unwrap();
	for entry in glob("../hdx_ast/src/css/values/*/mod.rs").unwrap() {
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
						let capture = &line[captures.get(1).unwrap()];
						if !capture.is_empty() {
							let mut atom_name = kebab(capture.to_string());
							if atom_name.starts_with("webkit") {
								atom_name = format!("-{}", atom_name);
							}
							matches.insert(atom_name);
						}
						true
					})?;
					Ok(true)
				}),
			)
			.unwrap();
	}

	string_cache_codegen::AtomType::new("Atom", "atom!")
		.atoms(matches)
		.write_to_file(&Path::new(&env::var("OUT_DIR").unwrap()).join("hdx_atom.rs"))
		.unwrap();
}
