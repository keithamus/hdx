mod lexer_suite;
mod parser_suite;
mod popular_parser;
mod popular_tokens;
mod postcss_parser;
mod postcss_tokens;
mod romainmenke;

use std::path::PathBuf;

use crate::{
	lexer_suite::LexerSuite, parser_suite::ParserSuite, popular_parser::PopularParserTestSuite,
	popular_tokens::PopularTokensTestSuite, postcss_parser::PostCSSParserTestSuite,
	postcss_tokens::PostCSSTokenizerTestSuite, romainmenke::CSSTokenizerTestSuite,
};

/// # Panics
/// Invalid Project Root
pub fn project_root() -> PathBuf {
	project_root::get_project_root().unwrap()
}

#[derive(Debug, Default)]
pub struct AppArgs {
	pub filter: Option<String>,
	pub update: bool,
	pub diff: bool,
}

impl AppArgs {
	pub fn run_all(&self) {
		self.run_lexer();
		self.run_parser();
	}

	pub fn run_lexer(&self) {
		println!("romainmenke/css-tokenizer-tests");
		let csstokenizer_res = CSSTokenizerTestSuite::new(self).run(self);
		println!("PostCSS Token Tests");
		let postcss_res = PostCSSTokenizerTestSuite::new(self).run(self);
		println!("Popular Token Tests");
		let popular_res = PopularTokensTestSuite::new(self).run(self);
		csstokenizer_res.unwrap();
		postcss_res.unwrap();
		popular_res.unwrap();
	}

	pub fn run_parser(&self) {
		println!("Running Parser Tests");
		println!("PostCSS Parser Tests");
		let postcss_res = PostCSSParserTestSuite::new(self).run(self);
		println!("Popular Parser Tests");
		let popular_res = PopularParserTestSuite::new(self).run(self);
		postcss_res.unwrap();
		popular_res.unwrap();
	}
}

#[test]
#[cfg(any(coverage, coverage_nightly))]
fn test() {
	let args = AppArgs { filter: None, update: false, diff: false };
	args.run_all()
}
