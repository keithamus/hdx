use std::fmt::Write;
use std::{collections::HashSet, env, fs::write, path::Path};

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

	let matcher = RegexMatcher::new_line_matcher("^pub (?:struct|enum) (\\w*(:?<'a>)?)").unwrap();
	let mut matches = HashSet::new();
	for entry in glob("src/css/values/*/mod.rs").unwrap() {
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
							matches.insert(capture.to_string());
						}
						true
					})?;
					Ok(true)
				}),
			)
			.unwrap();
	}

	let source = format!(
		r"macro_rules! apply_properties {{
			($macro: ident) => {{
				$macro! {{
{}				}}
			}}
		}}",
		matches.iter().fold(String::new(), |mut out, prop| {
			let mut atom_name = kebab(prop.trim_end_matches("<'a>").to_string());
			if atom_name.starts_with("webkit") {
				atom_name = format!("-{}", atom_name);
			}
			let _ = writeln!(out, "\t\t\t\t\t{}: atom!(\"{}\"),", prop, atom_name);
			out
		})
	);

	let _ = write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_apply_properties.rs"), source);
}
