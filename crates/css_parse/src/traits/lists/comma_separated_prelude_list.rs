use crate::{Parse, Parser, Result, T};
use bumpalo::collections::Vec;
use css_lexer::KindSet;

/// This trait can be used for AST nodes representing an At-Rule prelude that consists of multiple items separated with
/// commas.
///
/// The [CommaSeparatedPreludeList::PreludeItem] must be defined (representing the `<prelude-item>`), which will be the
/// item that is parsed between the commas. Error tolerance is provided for the commas, and so the return [Vec] will
/// have [Options][Option] of [T![,]][crate::token_macros::comma]. Parsing will stop once
/// [CommaSeparatedPreludeList::STOP_TOKENS] is reached.
///
/// The effective grammar for this trait is:
///
/// ```md
/// <comma-separated-prelude-list>
///  │├─╭─ <prelude-item> ─╮─ "," ─╭─╮─ "{" ─╭─┤│
///     │                  ╰───────╯ ├─ ";" ─╯
///     ╰────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-at-rule-list
pub trait CommaSeparatedPreludeList<'a>: Sized + Parse<'a> {
	type PreludeItem: Parse<'a>;
	const STOP_TOKENS: KindSet = KindSet::LEFT_CURLY_OR_SEMICOLON;

	fn parse_prelude_list(p: &mut Parser<'a>) -> Result<Vec<'a, (Self::PreludeItem, Option<T![,]>)>> {
		let mut items = Vec::new_in(p.bump());
		loop {
			let item = p.parse::<Self::PreludeItem>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			items.push((item, comma));
			if p.peek_next() == Self::STOP_TOKENS {
				return Ok(items);
			}
		}
	}
}
