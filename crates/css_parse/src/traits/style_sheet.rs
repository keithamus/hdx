use crate::{Parse, Parser, Result, T};
use bumpalo::collections::Vec;
use css_lexer::Kind;

/// This trait provides an implementation for parsing a [StyleSheet][1].
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#parse-stylesheet
///
/// It does not implement [Parse], but provides `parse_stylesheet(&mut Parser<'a>) -> Result<...>`, which can make
/// for a trivial [Parse] implementation. The type [StyleSheet::Rule] must be defined, and represents any Rule allowed
/// in a style sheet, which is the only top level item of the stylesheet.
///
/// StyleSheets are special in that they must discard CdcOrCdo tokens.
///
/// The steps `parse_stylesheet` takes can be defined as:
///
/// ```md
/// <style-sheet>
///  │├─╭─ (Discard <cdcorcdo-token>) ─ <rule> ─╮─┤│
///     ╰───────────────────────────────────────╯
/// ```
///
pub trait StyleSheet<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_stylesheet(p: &mut Parser<'a>) -> Result<Vec<'a, Self::Rule>> {
		let mut rules: Vec<'a, Self::Rule> = Vec::new_in(p.bump());
		loop {
			p.parse_if_peek::<T![CdcOrCdo]>()?;
			// need to peek as last tokens may be whitespace.
			if p.at_end() || p.peek_next() == Kind::Eof {
				return Ok(rules);
			}
			rules.push(p.parse::<Self::Rule>()?);
		}
	}
}
