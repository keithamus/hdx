use crate::{diagnostics, parser::Parser, syntax::BangImportant, Parse, Result, T};
use css_lexer::Cursor;

/// This trait provides an implementation for parsing a [Declaration][1].
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#parse-declaration
///
/// It does not implement [Parse], but provides `parse_declaration(&mut Parser<'a>) -> Result<...>`, which can make
/// for a trivial [Parse] implementation. The type [Declaration::DeclarationValue] must be defined, and represents the
/// `<declaration-value>` token(s). The grammar of `<declaration-value>` isn't defined here - it'll be dependant on the
/// property name. Consequently, [Declaration::DeclarationValue] must implement the [DeclarationValue] trait, which
/// must provide the `parse_declaration_value(&mut Parser<'a>, Cursor) -> Result<Self>` method - the [Cursor] given to
/// said method represents the Ident of the property name, so it can be reasoned about in order to dispatch to the right
/// declaration value parsing step.
///
/// Also provided is a [Declaration::valid_property()] method. It defaults to returning `true`, which means
/// all property-ids are valid. If implementing a set of declarations where ony limited property-ids are valid (such as
/// the declarations allowed by an at-rule) then it might be worthwhile changing this to sometimes return `false`, which
/// will cause `parse_declaration` to error early without having to do too much backtracking.
///
/// The steps `parse_declaration` takes can be defined as:
///
/// ```md
/// <property-id>
///  │├─ <ident> ─┤│
///
/// <declaration>
///  │├─ <property-id> ─ ":" ─ <declaration-value> ──╮─────────────────────────────╭─┤│
///                                                  ╰─ "!" ─ <ident "important"> ─╯
/// ```
///
pub trait Declaration<'a>: Sized + Parse<'a> {
	type DeclarationValue: DeclarationValue<'a>;

	fn valid_property(_p: &Parser, _c: Cursor) -> bool {
		true
	}

	fn parse_declaration(
		p: &mut Parser<'a>,
	) -> Result<(T![Ident], T![:], Self::DeclarationValue, Option<BangImportant>)> {
		let name = p.parse::<T![Ident]>()?;
		let c: Cursor = name.into();
		if !Self::valid_property(p, c) {
			Err(diagnostics::UnknownDeclaration(c.into()))?;
		}
		let colon = p.parse::<T![:]>()?;
		let value = Self::DeclarationValue::parse_declaration_value(p, c)?;
		let important = p.parse_if_peek::<BangImportant>()?;
		Ok((name, colon, value, important))
	}
}

pub trait DeclarationValue<'a>: Sized {
	fn parse_declaration_value(p: &mut Parser<'a>, name: Cursor) -> Result<Self>;
}
