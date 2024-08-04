use hdx_atom::atom;
use hdx_lexer::Span;
use hdx_parser::{diagnostics, AtRule, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use super::{KeyframeList, KeyframeName};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct WebkitKeyframes<'a> {
	name: Spanned<KeyframeName>,
	rules: Spanned<KeyframeList<'a>>,
}

impl<'a> Parse<'a> for WebkitKeyframes<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let start = parser.offset();
		match Self::parse_at_rule(parser, Some(atom!("-webkit-keyframes")))? {
			(Some(name), Some(rules)) => Ok(Self { name, rules }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(Span::new(start, parser.offset())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, parser.offset())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, parser.offset())))?,
		}
	}
}

impl<'a> AtRule<'a> for WebkitKeyframes<'a> {
	type Prelude = KeyframeName;
	type Block = KeyframeList<'a>;
}

impl<'a> WriteCss<'a> for WebkitKeyframes<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.indent();
		write_css!(sink, '@', atom!("-webkit-keyframes"), ' ', self.name, (), self.rules);
		sink.dedent();
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WebkitKeyframes, 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WebkitKeyframes, "@-webkit-keyframes foo {}");
		assert_parse!(WebkitKeyframes, "@-webkit-keyframes \"include\" {}");
		assert_parse!(
			WebkitKeyframes,
			"@-webkit-keyframes spin {\n\t0% {\n\t\trotate: 0deg;\n\t}\n\n\t100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
		assert_parse!(
			WebkitKeyframes,
			"@-webkit-keyframes spin {\n\tfrom, 0% {\n\t\trotate: 0deg;\n\t}\n\n\tto, 100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
	}
}
