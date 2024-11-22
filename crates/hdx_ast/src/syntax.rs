use crate::css::units::CSSFloat;
use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Kind, PairWise, QuoteStyle};
use hdx_parser::{
	diagnostics, AtRule as AtRuleTrait, Block as BlockTrait, Parse, Parser, QualifiedRule as QualifiedRuleTrait,
	Result as ParserResult, Span, Spanned, State, Vec, T,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ComponentValues<'a>(pub Vec<'a, Spanned<ComponentValue<'a>>>);

impl<'a> Parse<'a> for ComponentValues<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = p.new_vec();
		if let Some(token) = p.peek_with::<T![Whitespace]>(Include::Whitespace) {
			p.hop(token);
		}
		loop {
			if p.at_end() {
				break;
			}
			if p.is(State::Nested) && p.peek::<T![RightCurly]>().is_some() {
				break;
			}
			if p.is(State::StopOnSemicolon) && p.peek::<T![;]>().is_some() {
				break;
			}
			if p.is(State::StopOnComma) && p.peek::<T![,]>().is_some() {
				break;
			}
			values.push(p.parse_spanned::<ComponentValue>()?);
		}
		Ok(Self(values))
	}
}

impl<'a> WriteCss<'a> for ComponentValues<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		for value in &self.0 {
			value.write_css(sink)?;
		}
		Ok(())
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
// A compatible "Token" per CSS grammar, subsetted to the tokens possibly
// rendered by ComponentValue (so no pairwise, function tokens, etc).
#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(SimpleBlock<'a>),
	Function(Function<'a>),
	Whitespace,
	Number(CSSFloat),
	Dimension(CSSFloat, Atom),
	Ident(Atom),
	AtKeyword(Atom),
	Hash(Atom),
	String(&'a str, QuoteStyle),
	Url(&'a str, QuoteStyle),
	Delim(char),
	Colon,
	Semicolon,
	Comma,
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = p.peek_with::<T![Any]>(Include::Whitespace).unwrap();
		Ok(match token.kind() {
			Kind::LeftCurly | Kind::LeftSquare | Kind::LeftParen => {
				let old_state = p.set_state(State::Nested);
				p.parse::<SimpleBlock>()
					.inspect_err(|_| {
						p.set_state(old_state);
					})
					.map(|b| {
						p.set_state(old_state);
						Self::SimpleBlock(b)
					})?
			}
			Kind::Function => Self::Function(p.parse::<Function>()?),
			Kind::Whitespace => {
				p.hop(token);
				Self::Whitespace
			}
			Kind::Number => {
				p.hop(token);
				Self::Number(p.parse_number(token).into())
			}
			Kind::Dimension => {
				p.hop(token);
				Self::Dimension(p.parse_number(token).into(), p.parse_atom(token))
			}
			Kind::Ident => {
				p.hop(token);
				Self::Ident(p.parse_atom(token))
			}
			Kind::AtKeyword => {
				p.hop(token);
				Self::AtKeyword(p.parse_atom(token))
			}
			Kind::Hash => {
				p.hop(token);
				Self::Hash(p.parse_atom(token))
			}
			Kind::String => {
				p.hop(token);
				Self::String(p.parse_str(token), token.quote_style())
			}
			Kind::Url => {
				p.hop(token);
				Self::Url(p.parse_str(token), token.quote_style())
			}
			Kind::Delim => {
				p.hop(token);
				Self::Delim(token.char().unwrap())
			}
			Kind::Colon => {
				p.hop(token);
				Self::Colon
			}
			Kind::Semicolon => {
				p.hop(token);
				Self::Semicolon
			}
			Kind::Comma => {
				p.hop(token);
				Self::Comma
			}
			_ => Err(diagnostics::Unexpected(token, token.span()))?,
		})
	}
}

impl<'a> WriteCss<'a> for ComponentValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::SimpleBlock(b) => b.write_css(sink),
			Self::Function(f) => f.write_css(sink),
			Self::Ident(name) => sink.write_str(name),
			Self::AtKeyword(name) => {
				sink.write_char('@')?;
				sink.write_str(name)
			}
			Self::Hash(hash) => {
				sink.write_char('#')?;
				sink.write_str(hash)
			}
			Self::String(string, quote) => sink.write_with_quotes(string, *quote, false),
			Self::Url(url, quote) => {
				atom!("url").write_css(sink)?;
				sink.write_char('(')?;
				sink.write_with_quotes(url.as_ref(), *quote, true)?;
				sink.write_char(')')
			}
			Self::Delim(ch) => sink.write_char(*ch),
			Self::Number(n) => sink.write_str(&format!("{}", n)),
			Self::Dimension(n, unit) => {
				sink.write_str(&format!("{}", n))?;
				sink.write_str(unit)
			}
			Self::Whitespace => sink.write_char(' '),
			Self::Colon => sink.write_char(':'),
			Self::Semicolon => sink.write_char(';'),
			Self::Comma => sink.write_char(','),
		}
	}
}

#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct SimpleBlock<'a> {
	pub pairwise: PairWise,
	pub values: Vec<'a, Spanned<ComponentValue<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let pair = p.parse::<T![PairWise]>()?.to_pairwise().unwrap();
		let mut values = p.new_vec();
		loop {
			if p.at_end() {
				break;
			}
			if let Some(token) = p.peek::<T![PairWise]>() {
				if token.to_pairwise() == Some(pair) && token.kind() == pair.end() {
					p.hop(token);
					break;
				}
			}
			values.push(p.parse_spanned::<ComponentValue>()?);
		}
		Ok(Self { values, pairwise: pair })
	}
}

impl<'a> WriteCss<'a> for SimpleBlock<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self.pairwise {
			PairWise::Square => sink.write_char('[')?,
			PairWise::Curly => sink.write_char('{')?,
			PairWise::Paren => sink.write_char('(')?,
		}
		for value in &self.values {
			value.write_css(sink)?;
		}
		match self.pairwise {
			PairWise::Square => sink.write_char(']')?,
			PairWise::Curly => sink.write_char('}')?,
			PairWise::Paren => sink.write_char(')')?,
		}
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum Rule<'a> {
	AtRule(AtRule<'a>),
	QualifiedRule(QualifiedRule<'a>),
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![AtKeyword]>().is_some() {
			return p.parse::<AtRule>().map(Self::AtRule);
		}
		p.parse::<QualifiedRule>().map(Self::QualifiedRule)
	}
}

impl<'a> WriteCss<'a> for Rule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::AtRule(value) => value.write_css(sink),
			Self::QualifiedRule(value) => value.write_css(sink),
		}
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Block<'a> {
	pub declarations: Vec<'a, Spanned<Declaration<'a>>>,
	pub rules: Vec<'a, Spanned<Rule<'a>>>,
}

impl<'a> Parse<'a> for Block<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (declarations, rules) = Self::parse_block(p)?;
		Ok(Self { declarations, rules })
	}
}

impl<'a> BlockTrait<'a> for Block<'a> {
	type Declaration = Declaration<'a>;
	type Rule = Rule<'a>;
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Declaration<'a> {
	pub name: Atom,
	pub value: Spanned<ComponentValues<'a>>,
	pub important: bool,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-declaration
impl<'a> Parse<'a> for Declaration<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Ident]>()?;
		p.parse::<T![:]>()?;
		let old_state = p.set_state(State::StopOnSemicolon | State::Nested);
		let mut value = p.parse_spanned::<ComponentValues>().inspect_err(|_| {
			p.set_state(old_state);
		})?;
		p.set_state(old_state);
		let mut iter = value.node.0.iter_mut();
		let important =
			matches!(iter.nth_back(1), Some(Spanned { node: ComponentValue::Ident(atom!("important")), .. }))
				&& matches!(iter.nth_back(2), Some(Spanned { node: ComponentValue::Delim('!'), .. }));
		let name = p.parse_atom(token);
		Ok(Self { name, value, important })
	}
}

impl<'a> WriteCss<'a> for Declaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.name.write_css(sink)?;
		sink.write_char(':')?;
		sink.write_whitespace()?;
		self.value.write_css(sink)?;
		if self.important {
			sink.write_whitespace()?;
			sink.write_char('!')?;
			atom!("important").write_css(sink)?;
		}
		sink.write_char(';')?;
		Ok(())
	}
}

impl<'a> WriteCss<'a> for Block<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('{')?;
		sink.write_newline()?;
		sink.indent();
		for decl in &self.declarations {
			sink.write_indent()?;
			decl.write_css(sink)?;
			sink.write_newline()?;
		}
		for rule in &self.rules {
			sink.write_indent()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_char('}')
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct AtRule<'a> {
	pub name: Atom,
	pub prelude: Spanned<ComponentValues<'a>>,
	pub block: Spanned<Block<'a>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
impl<'a> Parse<'a> for AtRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = p.peek::<T![AtKeyword]>();
		let (prelude_opt, block_opt) = Self::parse_at_rule(p, None)?;
		let prelude =
			prelude_opt.unwrap_or_else(|| Spanned { node: ComponentValues(p.new_vec()), span: Span::dummy() });
		let block = block_opt.unwrap_or_else(|| Spanned {
			node: Block { declarations: p.new_vec(), rules: p.new_vec() },
			span: Span::dummy(),
		});
		let name = p.parse_atom(token.unwrap());
		Ok(Self { name, prelude, block })
	}
}

impl<'a> AtRuleTrait<'a> for AtRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
}

impl<'a> WriteCss<'a> for AtRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('@')?;
		sink.write_str(self.name.as_ref())?;
		self.prelude.write_css(sink)?;
		sink.write_whitespace()?;
		self.block.write_css(sink)?;
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct QualifiedRule<'a> {
	pub prelude: Spanned<ComponentValues<'a>>,
	pub block: Spanned<Block<'a>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
impl<'a> Parse<'a> for QualifiedRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (prelude, block) = Self::parse_qualified_rule(p)?;
		Ok(Self { prelude, block })
	}
}

pub struct BadDeclaration;
// https://drafts.csswg.org/css-syntax-3/#consume-the-remnants-of-a-bad-declaration
impl<'a> Parse<'a> for BadDeclaration {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		// To consume the remnants of a bad declaration from a token stream input, given a bool nested:
		//
		// Process input:
		loop {
			let token = p.peek::<T![Any]>().unwrap();
			dbg!(p.at_end(), token);
			//
			// <eof-token>
			// <semicolon-token>
			//
			//     Discard a token from input, and return nothing.
			if p.at_end() || token.kind() == Kind::Semicolon {
				p.hop(token);
				return Ok(Self);
			}
			// <}-token>
			//
			//     If nested is true, return nothing. Otherwise, discard a token.
			if token.kind() == Kind::RightCurly {
				if p.is(State::Nested) {
					return Ok(Self);
				} else {
					p.hop(token);
				}
			}
			// anything else
			//
			//     Consume a component value from input, and do nothing.
			//
			p.parse::<ComponentValue>()?;
		}
	}
}

impl<'a> QualifiedRuleTrait<'a> for QualifiedRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
	type BadDeclaration = BadDeclaration;
}

impl<'a> WriteCss<'a> for QualifiedRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.prelude.write_css(sink)?;
		sink.write_whitespace()?;
		self.block.write_css(sink)?;
		Ok(())
	}
}

#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Function<'a> {
	pub name: Atom,
	pub values: Vec<'a, Spanned<ComponentValue<'a>>>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for Function<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Function]>()?;
		let mut values = p.new_vec();
		loop {
			if p.at_end() {
				break;
			}
			if p.peek::<T![RightParen]>().is_some() {
				break;
			}
			values.push(p.parse_spanned::<ComponentValue>()?);
		}
		p.parse::<T![RightParen]>()?;
		let name = p.parse_atom(token);
		Ok(Self { name, values })
	}
}

impl<'a> WriteCss<'a> for Function<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str(self.name.as_ref())?;
		sink.write_char('(')?;
		for value in &self.values {
			value.write_css(sink)?;
		}
		sink.write_char(')')
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ComponentValues, 32);
		assert_size!(ComponentValue, 48);
		assert_size!(SimpleBlock, 40);
		assert_size!(Function, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ComponentValue, "foo");
		// assert_parse!(Function, "one(two)");
		// assert_parse!(SimpleBlock, "[foo]");
		// assert_parse!(SimpleBlock, "(one two three)");
		// assert_parse!(SimpleBlock, "(one(two))");
		// assert_parse!(SimpleBlock, "{one(two)}");
		// assert_parse!(SimpleBlock, "{}");
		// assert_parse!(SimpleBlock, "{foo}");
		// assert_parse!(SimpleBlock, "{foo:bar}");
		// assert_parse!(Function, "one((two)three)");
		// assert_parse!(ComponentValues, "a b c d");
		// assert_parse!(ComponentValues, "body {color: black }");
		// assert_parse!(ComponentValues, "body ");
		// assert_parse!(Block, "{\n}");
		// assert_parse!(Block, "{\n\tfoo: bar;\n}");
		// assert_parse!(Block, "{\n\tfoo: bar;\n\tbaz: bing;\n}");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Block, "{\n\tfoo: bar;\n\tbaz:bing;\n}", "{foo:bar;baz:bing;}");
	}
}
