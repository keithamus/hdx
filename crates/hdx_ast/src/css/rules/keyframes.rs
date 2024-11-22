use hdx_atom::{atom, Atom};
use hdx_lexer::{QuoteStyle, Span};
use hdx_parser::{diagnostics, discard, AtRule, Parse, Parser, Result as ParserResult, Spanned, Vec, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::{properties::Property, units::Percent};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Keyframes<'a> {
	name: Spanned<KeyframeName>,
	rules: Spanned<KeyframeList<'a>>,
}

impl<'a> Parse<'a> for Keyframes<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let start = parser.offset();
		match Self::parse_at_rule(parser, Some(atom!("keyframes")))? {
			(Some(name), Some(rules)) => Ok(Self { name, rules }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(Span::new(start, parser.offset())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, parser.offset())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, parser.offset())))?,
		}
	}
}

impl<'a> AtRule<'a> for Keyframes<'a> {
	type Prelude = KeyframeName;
	type Block = KeyframeList<'a>;
}

impl<'a> WriteCss<'a> for Keyframes<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.indent();
		write_css!(sink, '@', atom!("keyframes"), ' ', self.name, (), self.rules);
		sink.dedent();
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframeName(pub Atom, pub QuoteStyle);

impl KeyframeName {
	fn valid_ident(atom: &Atom) -> bool {
		!matches!(
			atom.to_ascii_lowercase(),
			atom!("default") | atom!("initial") | atom!("inherit") | atom!("unset") | atom!("none")
		)
	}
}

impl<'a> Parse<'a> for KeyframeName {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<T![String]>() {
			parser.hop(token);
			let atom = parser.parse_atom(token);
			return Ok(Self(atom, token.quote_style()));
		}
		let token = *parser.parse::<T![Ident]>()?;
		let atom = parser.parse_atom_lower(token);
		if Self::valid_ident(&atom) {
			Ok(Self(atom, QuoteStyle::None))
		} else {
			Err(diagnostics::UnexpectedIdent(atom, token.span()))?
		}
	}
}

impl<'a> WriteCss<'a> for KeyframeName {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_with_quotes(self.0.as_ref(), self.1, Self::valid_ident(&self.0))
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframeList<'a>(Vec<'a, Spanned<Keyframe<'a>>>);

impl<'a> Parse<'a> for KeyframeList<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		parser.parse::<T![LeftCurly]>()?;
		let mut rules = parser.new_vec();
		loop {
			if discard!(parser, RightCurly) {
				return Ok(Self(rules));
			}
			rules.push(parser.parse_spanned::<Keyframe>()?);
		}
	}
}

impl<'a> WriteCss<'a> for KeyframeList<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('{')?;
		for rule in self.0.iter() {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		sink.write_char('}')
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Keyframe<'a> {
	selector: SmallVec<[KeyframeSelector; 1]>,
	properties: Vec<'a, Spanned<Property<'a>>>,
}

impl<'a> Parse<'a> for Keyframe<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut selector = smallvec![];
		loop {
			selector.push(parser.parse::<KeyframeSelector>()?);
			if parser.at_end() || discard!(parser, LeftCurly) {
				break;
			}
			parser.parse::<T![,]>()?;
		}
		let mut properties = parser.new_vec();
		loop {
			discard!(parser, Semicolon);
			if parser.at_end() || discard!(parser, RightCurly) {
				break;
			}
			properties.push(parser.parse_spanned::<Property>()?);
		}
		Ok(Self { selector, properties })
	}
}

impl<'a> WriteCss<'a> for Keyframe<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_indent()?;
		write_css!(sink, self.selector, (), '{');
		sink.indent();
		let mut iter = self.properties.iter().peekable();
		while let Some(prop) = iter.next() {
			sink.write_newline()?;
			sink.write_indent()?;
			prop.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(';')?;
			} else {
				sink.write_trailing_char(';')?;
			}
		}
		sink.dedent();
		sink.write_newline()?;
		sink.write_indent()?;
		sink.write_char('}')
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum KeyframeSelector {
	From,
	To,
	Percent(Percent),
}

impl<'a> Parse<'a> for KeyframeSelector {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<T![Ident]>() {
			parser.hop(token);
			return match parser.parse_atom_lower(token) {
				atom!("from") => Ok(KeyframeSelector::From),
				atom!("to") => Ok(KeyframeSelector::To),
				atom => Err(diagnostics::UnexpectedIdent(atom, token.span()))?,
			};
		}
		let token = *parser.parse::<T![Dimension]>()?;
		let n = parser.parse_number(token);
		let unit = parser.parse_atom(token);
		if unit == atom!("%") && (0.0..=100.0).contains(&n) {
			Ok(Self::Percent(n.into()))
		} else {
			Err(diagnostics::UnexpectedDimension(unit, token.span()))?
		}
	}
}

impl<'a> WriteCss<'a> for KeyframeSelector {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::To => atom!("to").write_css(sink),
			Self::From => atom!("from").write_css(sink),
			Self::Percent(pc) => pc.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Keyframes, 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Keyframes, "@keyframes foo {}");
		assert_parse!(Keyframes, "@keyframes \"include\" {}");
		assert_parse!(
			Keyframes,
			"@keyframes spin {\n\t0% {\n\t\trotate: 0deg;\n\t}\n\n\t100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
		assert_parse!(
			Keyframes,
			"@keyframes spin {\n\tfrom, 0% {\n\t\trotate: 0deg;\n\t}\n\n\tto, 100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
		assert_parse!(
			Keyframes,
			"@keyframes spin {to{transform:rotate(360deg)}}",
			"@keyframes spin {\n\tto {\n\t\ttransform: rotate(360deg);\n\t}\n}"
		);
		assert_parse!(
			Keyframes,
			"@keyframes x {\n\tto {\n\t\tanimation-timing-function: cubic-bezier(0, 0, 0.2, 1);\n\t}\n}"
		);
	}
}
