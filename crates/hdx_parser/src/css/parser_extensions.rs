use core::fmt::Debug;

use oxc_allocator::Vec;

use crate::{atom, diagnostics, Atom, Kind, Parse, Parser, Result, Spanned, Token};

impl<'a> Parser<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
	pub(crate) fn parse_qualified_rule<
		T,
		Prelude: Parse<'a> + 'a,
		Rule: Parse<'a> + 'a,
		Decl: Parse<'a> + 'a,
		F,
	>(
		&mut self,
		stop_token: Option<Kind>,
		nested: bool,
		finalize: F,
	) -> Result<T>
	where
		F: FnOnce(
			&mut Parser<'a>,
			Option<Spanned<Prelude>>,
			Vec<'a, Spanned<Rule>>,
			Vec<'a, Spanned<Decl>>,
		) -> Result<T>,
	{
		let span = self.cur().span;
		let mut prelude = None;
		loop {
			match self.cur().kind {
				Kind::Eof => Err(diagnostics::Unexpected(Kind::Eof, span.up_to(&self.cur().span)))?,
				Kind::RightCurly => {
					if nested {
						Err(diagnostics::Unexpected(self.cur().kind, self.cur().span))?
					}
					prelude = Some(Prelude::parse(self)?);
				}
				Kind::LeftCurly => {
					return self.parse_block(
						|parser: &mut Parser<'a>,
						 rules: Vec<'a, Spanned<Rule>>,
						 decls: Vec<'a, Spanned<Decl>>| { finalize(parser, prelude, rules, decls) },
					);
				}
				c => {
					if let Some(k) = stop_token {
						if c == k {
							self.advance();
							Err(diagnostics::Unexpected(self.cur().kind, self.cur().span))?
						}
					}
					if prelude.is_some() {
						Err(diagnostics::Unexpected(self.cur().kind, self.cur().span))?
					}
					prelude = Some(Prelude::parse(self)?);
					self.skip_trivia();
				}
			}
		}
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
	pub(crate) fn parse_at_rule<
		T,
		Prelude: Parse<'a> + 'a,
		Rule: Parse<'a> + 'a,
		Decl: Parse<'a> + 'a,
		F,
	>(
		&mut self,
		expected_name: Option<Atom>,
		finalize: F,
	) -> Result<T>
	where
		F: FnOnce(
			&mut Parser<'a>,
			Atom,
			Option<Spanned<Prelude>>,
			Vec<'a, Spanned<Rule>>,
			Vec<'a, Spanned<Decl>>,
		) -> Result<T>,
	{
		let name = self.expect_at_keyword()?;
		if let Some(exp_name) = expected_name {
			if !exp_name.eq_ignore_ascii_case(&name) {
				Err(diagnostics::ExpectedIdent(name.clone(), exp_name, self.token.span))?;
			}
		}
		let mut prelude = None;
		loop {
			match self.cur().kind {
				Kind::Semicolon | Kind::Eof => {
					self.advance();
					return finalize(self, name, prelude, self.new_vec(), self.new_vec());
				}
				Kind::RightCurly => {
					let result = finalize(self, name, prelude, self.new_vec(), self.new_vec());
					if result.is_ok() {
						self.advance()
					}
					return result;
				}
				Kind::LeftCurly => {
					dbg!(self.cur());
					return self.parse_block(
						|parser: &mut Parser<'a>,
						 rules: Vec<'a, Spanned<Rule>>,
						 decls: Vec<'a, Spanned<Decl>>| {
							finalize(parser, name, prelude, rules, decls)
						},
					);
				}
				_ => {
					prelude = Some(Prelude::parse(self)?);
				}
			}
		}
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-block
	pub(crate) fn parse_block<T, Rule: Parse<'a> + 'a, Decl: Parse<'a> + 'a, F>(
		&mut self,
		finalize: F,
	) -> Result<T>
	where
		F: FnOnce(&mut Parser<'a>, Vec<'a, Spanned<Rule>>, Vec<'a, Spanned<Decl>>) -> Result<T>,
	{
		let mut decls = self.new_vec();
		let mut rules = self.new_vec();
		self.expect(Kind::LeftCurly)?;
		loop {
			match self.cur().kind {
				Kind::Whitespace | Kind::Semicolon => {
					self.advance();
				}
				Kind::Eof | Kind::RightCurly => {
					self.next_token();
					let res = finalize(self, rules, decls);
					self.skip_trivia();
					return res;
				}
				Kind::AtKeyword => {
					rules.push(Rule::parse(self)?);
				}
				_ => {
					let checkpoint = self.checkpoint();
					match Decl::parse(self) {
						Ok(decl) => decls.push(decl),
						Err(_) => {
							self.rewind(checkpoint);
							rules.push(Rule::parse(self)?);
						}
					}
				}
			}
		}
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-declaration
	pub(crate) fn parse_declaration<T, Value: Debug + Parse<'a> + 'a, F>(
		&mut self,
		expected_name: Option<Atom>,
		finalize: F,
	) -> Result<T>
	where
		F: FnOnce(&mut Parser<'a>, &Token, Spanned<Value>, bool) -> Result<T>,
	{
		let span = self.cur().span;
		let name_token = self.cur().clone();
		let ident = self.expect_ident()?;
		if let Some(name) = expected_name {
			if ident != name {
				Err(diagnostics::ExpectedIdent(name, ident.clone(), span))?;
			}
		}
		if ident.starts_with("--") {
			Err(diagnostics::Unimplemented(span))?;
		}
		if ident == atom!("unicode-range") {
			Err(diagnostics::Unimplemented(span))?;
		}
		self.expect(Kind::Colon)
			.map_err(|_| diagnostics::BadDeclaration(span.up_to(&self.cur().span)))?;
		let value = Value::parse(self)?;
		let mut important = false;
		loop {
			match self.cur().kind {
				Kind::Semicolon => {
					// Swallow the last semi and break
					self.advance();
					break;
				}
				Kind::Delim => {
					if self.cur().value.as_char().unwrap() == '!'
						&& self.peek().matches_ignore_case(&atom!("important"))
					{
						important = true;
						self.advance();
						self.advance();
					} else {
						break;
					}
				}
				_ => break,
			}
		}
		finalize(self, &name_token, value, important)
	}
}
