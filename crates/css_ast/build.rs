use std::fmt::Write;
use std::io;
use std::str::from_utf8;
use std::{collections::HashSet, env, fs::write, path::Path};

use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::{RegexMatcher, RegexMatcherBuilder};
use grep_searcher::{Searcher, SearcherBuilder, Sink, SinkError, SinkMatch};

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

pub fn snake(str: String) -> String {
	let mut snake = String::new();
	for (i, ch) in str.char_indices() {
		if i > 0 && ch.is_uppercase() {
			snake.push('_');
		}
		snake.push(ch.to_ascii_lowercase());
	}
	snake
}

pub struct NodeMatcher<'a> {
	matcher: &'a RegexMatcher,
	visit_matches: &'a mut HashSet<String>,
	stylevalue_matches: &'a mut HashSet<String>,
}

impl Sink for NodeMatcher<'_> {
	type Error = io::Error;

	fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error> {
		let mut captures = self.matcher.new_captures()?;
		let line = match from_utf8(mat.bytes()) {
			Ok(matched) => matched,
			Err(err) => return Err(io::Error::error_message(err)),
		};
		self.matcher.captures_iter(mat.bytes(), &mut captures, |captures| -> bool {
			dbg!(
				&line,
				&captures,
				captures.get(2).map(|r| &line[r]),
				captures.get(3).map(|r| &line[r]),
				captures.get(4).map(|r| &line[r]),
				captures.get(5).map(|r| &line[r]),
				captures.get(6).map(|r| &line[r])
			);
			let value_or_visit = &line[captures.get(1).unwrap()];
			let capture = &line[captures.get(6).unwrap()];
			if !capture.is_empty() {
				if value_or_visit == "value" {
					self.stylevalue_matches.insert(capture.to_string());
				}
				self.visit_matches.insert(capture.to_string());
			} else {
				dbg!(&line);
				panic!("#[visit] or #[value] on unknown");
			}
			true
		})?;
		Ok(true)
	}
}

fn main() {
	println!("cargo::rerun-if-changed=build.rs");
	use std::time::Instant;
	let now = Instant::now();
	let matcher = RegexMatcherBuilder::new()
		.multi_line(true)
		.dot_matches_new_line(true)
		.ignore_whitespace(true)
		.build(
			r#"
			# match the #[value] or #[visit] attribute
			^\s*\#\[(value|visit)
			# munch the data between the attribute and the definition
			.*?
			(
				# Is this a public definition?
				pub\s*(?:struct|enum)\s*
				|
				# Or one of the parser macros that create public definitions?
				(:?
					keyword_set!|
					pseudo_(?:class|element)!|
					(?:ranged|boolean|discrete)_feature!
				)\(
			)
			# munch any comments/attributes between this and our name (for macros)
			(:?\n?\s*(:?\/\/|\#)[^\n]*)*
			# finally grab the word (plus any lifetime definition)
			\s*(\w*(:?<'a>)?)"#,
		)
		.unwrap();
	let mut visit_matches = HashSet::new();
	let mut stylevalue_matches = HashSet::new();
	let mut searcher = SearcherBuilder::new().line_number(false).multi_line(true).build();
	for entry in glob("src/**/*.rs").unwrap() {
		let str = &entry.as_ref().unwrap().display();
		println!("cargo::rerun-if-changed={}", str);
		let context = NodeMatcher {
			matcher: &matcher,
			visit_matches: &mut visit_matches,
			stylevalue_matches: &mut stylevalue_matches,
		};
		searcher.search_path(&matcher, entry.unwrap(), context).unwrap();
	}

	let source = format!(
		r"#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub enum NodeKind {{
			{}
		}}",
		visit_matches.iter().fold(String::new(), |mut out, prop| {
			let variant_name = prop.trim_end_matches("<'a>");
			writeln!(out, "\t\t\t\t\t{},", variant_name).unwrap();
			out
		})
	);
	let _ = write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_node_kind.rs"), source);
	let source = format!(
		r"macro_rules! apply_visit_methods {{
			($macro: ident) => {{
				$macro! {{
{}				}}
			}}
		}}",
		visit_matches.iter().fold(String::new(), |mut out, prop| {
			let method_name = prop.trim_end_matches("<'a>");
			writeln!(out, "\t\t\t\t\tvisit_{}({}),", snake(method_name.into()), prop).unwrap();
			out
		})
	);
	let _ = write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_apply_visit_methods.rs"), source);

	let source = format!(
		r"macro_rules! apply_properties {{
			($macro: ident) => {{
				$macro! {{
{}				}}
			}}
		}}",
		stylevalue_matches.iter().fold(String::new(), |mut out, prop| {
			let variant_name = prop.trim_end_matches("<'a>").trim_end_matches("StyleValue").to_string();
			let mut variant_str = kebab(variant_name.to_owned());
			if variant_str.starts_with("webkit") {
				variant_str = format!("-{}", variant_str);
			}
			writeln!(out, "\t\t\t\t\t{}: {} = \"{}\",", variant_name, prop, variant_str).unwrap();
			out
		})
	);

	let _ = write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_apply_properties.rs"), source);

	let elapsed = now.elapsed();
	println!("cargo::warning=Built in {:.?}", &elapsed);
}
