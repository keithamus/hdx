use crate::css::units::CSSFloat;
use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Kind, PairWise, QuoteStyle};
use hdx_parser::{
	diagnostics, AtRule as AtRuleTrait, Block as BlockTrait, Parse, Parser, QualifiedRule as QualifiedRuleTrait,
	Result as ParserResult, Span, Spanned, State, Token, Vec,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
#[derive(PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ComponentValues<'a>(pub Vec<'a, Spanned<ComponentValue<'a>>>);

impl<'a> Parse<'a> for ComponentValues<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = parser.new_vec();
		if let Some(token) = parser.peek_with::<Token![Whitespace]>(Include::Whitespace) {
			parser.hop(token);
		}
		loop {
			if parser.at_end() {
				break;
			}
			if parser.is(State::Nested) && parser.peek::<Token![RightCurly]>().is_some() {
				break;
			}
			if parser.is(State::StopOnSemicolon) && parser.peek::<Token![;]>().is_some() {
				break;
			}
			if parser.is(State::StopOnComma) && parser.peek::<Token![,]>().is_some() {
				break;
			}
			values.push(ComponentValue::parse_spanned(parser)?);
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.peek_with::<Token![Any]>(Include::Whitespace).unwrap();
		Ok(match token.kind() {
			Kind::LeftCurly | Kind::LeftSquare | Kind::LeftParen => {
				let old_state = parser.set_state(State::Nested);
				parser
					.parse::<SimpleBlock>()
					.inspect_err(|_| {
						parser.set_state(old_state);
					})
					.map(|b| {
						parser.set_state(old_state);
						Self::SimpleBlock(b)
					})?
			}
			Kind::Function => Self::Function(Function::parse(parser)?),
			Kind::Whitespace => {
				parser.hop(token);
				Self::Whitespace
			}
			Kind::Number => {
				parser.hop(token);
				Self::Number(parser.parse_number(token).into())
			}
			Kind::Dimension => {
				parser.hop(token);
				Self::Dimension(parser.parse_number(token).into(), parser.parse_atom(token))
			}
			Kind::Ident => {
				parser.hop(token);
				Self::Ident(parser.parse_atom(token))
			}
			Kind::AtKeyword => {
				parser.hop(token);
				Self::AtKeyword(parser.parse_atom(token))
			}
			Kind::Hash => {
				parser.hop(token);
				Self::Hash(parser.parse_atom(token))
			}
			Kind::String => {
				parser.hop(token);
				Self::String(parser.parse_str(token), token.quote_style())
			}
			Kind::Url => {
				parser.hop(token);
				Self::Url(parser.parse_str(token), token.quote_style())
			}
			Kind::Delim => {
				parser.hop(token);
				Self::Delim(token.char().unwrap())
			}
			Kind::Colon => {
				parser.hop(token);
				Self::Colon
			}
			Kind::Semicolon => {
				parser.hop(token);
				Self::Semicolon
			}
			Kind::Comma => {
				parser.hop(token);
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let pair = parser.parse::<Token![PairWise]>()?.to_pairwise().unwrap();
		let mut values = parser.new_vec();
		loop {
			if parser.at_end() {
				break;
			}
			if let Some(token) = parser.peek::<Token![PairWise]>() {
				if token.to_pairwise() == Some(pair) && token.kind() == pair.end() {
					parser.hop(token);
					break;
				}
			}
			values.push(parser.parse_spanned::<ComponentValue>()?);
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if parser.peek::<Token![AtKeyword]>().is_some() {
			return parser.parse::<AtRule>().map(Self::AtRule);
		}
		parser.parse::<QualifiedRule>().map(Self::QualifiedRule)
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (declarations, rules) = Self::parse_block(parser)?;
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *parser.parse::<Token![Ident]>()?;
		parser.parse::<Token![:]>()?;
		let old_state = parser.set_state(State::StopOnSemicolon | State::Nested);
		let mut value = parser.parse_spanned::<ComponentValues>().inspect_err(|_| {
			parser.set_state(old_state);
		})?;
		parser.set_state(old_state);
		let mut iter = value.node.0.iter_mut();
		let important =
			matches!(iter.nth_back(1), Some(Spanned { node: ComponentValue::Ident(atom!("important")), .. }))
				&& matches!(iter.nth_back(2), Some(Spanned { node: ComponentValue::Delim('!'), .. }));
		let name = parser.parse_atom(token);
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.peek::<Token![AtKeyword]>();
		let (prelude_opt, block_opt) = Self::parse_at_rule(parser, None)?;
		let prelude =
			prelude_opt.unwrap_or_else(|| Spanned { node: ComponentValues(parser.new_vec()), span: Span::dummy() });
		let block = block_opt.unwrap_or_else(|| Spanned {
			node: Block { declarations: parser.new_vec(), rules: parser.new_vec() },
			span: Span::dummy(),
		});
		let name = parser.parse_atom(token.unwrap());
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (prelude, block) = Self::parse_qualified_rule(parser)?;
		Ok(Self { prelude, block })
	}
}

impl<'a> QualifiedRuleTrait<'a> for QualifiedRule<'a> {
	type Block = Block<'a>;
	type Prelude = ComponentValues<'a>;
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *parser.parse::<Token![Function]>()?;
		let mut values = parser.new_vec();
		loop {
			if parser.at_end() {
				break;
			}
			if parser.peek::<Token![RightParen]>().is_some() {
				break;
			}
			values.push(ComponentValue::parse_spanned(parser)?);
		}
		parser.parse::<Token![RightParen]>()?;
		let name = parser.parse_atom(token);
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
