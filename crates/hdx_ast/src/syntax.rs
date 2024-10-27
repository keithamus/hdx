use crate::css::units::CSSFloat;
use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Kind, PairWise, QuoteStyle};
use hdx_parser::{
	expect, unexpected, AtRule as AtRuleTrait, Block as BlockTrait, Parse, Parser, QualifiedRule as QualifiedRuleTrait,
	Result as ParserResult, Span, Spanned, State, Vec,
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
		if matches!(parser.peek_with(Include::Whitespace).kind(), Kind::Whitespace) {
			parser.next_with(Include::Whitespace);
		}
		loop {
			match parser.peek_with(Include::Whitespace).kind() {
				Kind::Eof => break,
				Kind::RightCurly if parser.is(State::Nested) => break,
				// ComponentValues can be passed a "stop token" which could be any token.
				// In reality it is only ever called with a comma-token or semicolon-token.
				Kind::Semicolon if parser.is(State::StopOnSemicolon) => break,
				Kind::Comma if parser.is(State::StopOnComma) => break,
				_ => values.push(ComponentValue::parse_spanned(parser)?),
			}
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
		let peek = parser.peek_with(Include::Whitespace);
		Ok(match peek.kind() {
			Kind::LeftCurly | Kind::LeftSquare | Kind::LeftParen => {
				Self::SimpleBlock(SimpleBlock::parse_with_state(parser, State::Nested)?)
			}
			Kind::Function => Self::Function(Function::parse(parser)?),
			Kind::Whitespace => Self::Whitespace,
			Kind::Number => Self::Number(parser.parse_number(peek).into()),
			Kind::Dimension => Self::Dimension(parser.parse_number(peek).into(), parser.parse_atom(peek)),
			Kind::Ident => Self::Ident(parser.parse_atom(peek)),
			Kind::AtKeyword => Self::AtKeyword(parser.parse_atom(peek)),
			Kind::Hash => Self::Hash(parser.parse_atom(peek)),
			Kind::String => Self::String(parser.parse_str(peek), peek.quote_style()),
			Kind::Url => Self::Url(parser.parse_str(peek), peek.quote_style()),
			Kind::Delim => Self::Delim(peek.char().unwrap()),
			Kind::Colon => Self::Colon,
			Kind::Semicolon => Self::Semicolon,
			Kind::Comma => Self::Comma,
			_ => unexpected!(parser, peek),
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
		if let Some(pairwise) = parser.next().to_pairwise() {
			let mut values = parser.new_vec();
			let end_kind = pairwise.end();
			loop {
				match parser.peek_with(Include::Whitespace) {
					t if t.kind() == Kind::Eof => break,
					t if t.kind() == end_kind => break,
					_ => values.push(ComponentValue::parse_spanned(parser)?),
				}
			}
			if parser.next().kind() != pairwise.end() {
				unexpected!(parser)
			}
			Ok(Self { values, pairwise })
		} else {
			unexpected!(parser)
		}
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
		Ok(match parser.next().kind() {
			Kind::AtKeyword => Rule::AtRule(AtRule::parse(parser)?),
			_ => Rule::QualifiedRule(QualifiedRule::parse(parser)?),
		})
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
    let token = parser.next();
		match token.kind() {
			Kind::Ident => {
				expect!(parser.next(), Kind::Colon);
				let mut value =
					ComponentValues::parse_spanned_with_state(parser, State::StopOnSemicolon | State::Nested)?;
				let mut iter = value.node.0.iter_mut();
				let important =
					matches!(iter.nth_back(1), Some(Spanned { node: ComponentValue::Ident(atom!("important")), .. }))
						&& matches!(iter.nth_back(2), Some(Spanned { node: ComponentValue::Delim('!'), .. }));
        let name = parser.parse_atom(token);
				Ok(Self { name, value, important })
			}
			_ => unexpected!(parser, token),
		}
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
		let token = parser.next();
		match token.kind() {
			Kind::AtKeyword => {
				let (prelude_opt, block_opt) = Self::parse_at_rule(parser)?;
				let prelude = prelude_opt
					.unwrap_or_else(|| Spanned { node: ComponentValues(parser.new_vec()), span: Span::dummy() });
				let block = block_opt.unwrap_or_else(|| Spanned {
					node: Block { declarations: parser.new_vec(), rules: parser.new_vec() },
					span: Span::dummy(),
				});
				let name = parser.parse_atom(token);
				Ok(Self { name, prelude, block })
			}
			_ => unexpected!(parser, token),
		}
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
		let token = parser.next();
		match token.kind() {
			Kind::Function => {
				let mut values = parser.new_vec();
				loop {
					match parser.peek().kind() {
						Kind::Eof => break,
						Kind::RightParen => break,
						_ => values.push(ComponentValue::parse_spanned(parser)?),
					}
				}
				expect!(parser.next(), Kind::RightParen);
				let name = parser.parse_atom(token);
				Ok(Self { name, values })
			}
			_ => unexpected!(parser, token),
		}
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
		assert_parse!(Function, "one(two)");
		assert_parse!(SimpleBlock, "[foo]");
		assert_parse!(SimpleBlock, "(one two three)");
		assert_parse!(SimpleBlock, "(one(two))");
		assert_parse!(SimpleBlock, "{one(two)}");
		assert_parse!(SimpleBlock, "{}");
		assert_parse!(SimpleBlock, "{foo}");
		assert_parse!(SimpleBlock, "{foo:bar}");
		assert_parse!(Function, "one((two)three)");
		assert_parse!(ComponentValues, "a b c d");
		assert_parse!(ComponentValues, "body {color: black }");
		assert_parse!(ComponentValues, "body ");
		assert_parse!(Block, "{\n}");
		assert_parse!(Block, "{\n\tfoo: bar;\n}");
		assert_parse!(Block, "{\n\tfoo: bar;\n\tbaz: bing;\n}");
	}

	#[test]
	fn test_minify() {
		assert_minify!(Block, "{\n\tfoo: bar;\n\tbaz:bing;\n}", "{foo:bar;baz:bing;}");
	}
}
