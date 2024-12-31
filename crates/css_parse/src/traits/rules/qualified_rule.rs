use crate::{diagnostics, Parse, Parser, Result, State, T};
use css_lexer::{Kind, KindSet};

// A QualifiedRule represents a block with a prelude which may contain other rules.
// Examples of QualifiedRules are StyleRule, KeyframeRule (no s!).
pub trait QualifiedRule<'a>: Sized + Parse<'a> {
	// Prelude MAY implement PreludeList if it accepts multiple values.
	type Prelude: Parse<'a>;
	// Ideally Block would either implement QualifiedRuleList/DeclarationList/RuleList/DeclarationRuleList; but there is
	// no way to enforce that with Rust so it just has to implement Parse.
	type Block: Parse<'a>;

	// QualifiedRules must be able to consume a bad declaration, for when
	// a custom property like declaration is encountered.
	type BadDeclaration: Parse<'a>;

	// QualifiedRules must be able to consume a block from their input when encountering
	// a custom property like declaration that doesn't end but opens a `{` block. This
	// is implemented as parsing the existing block as that' simplifies downstream logic
	// but consumers of this trait can instead opt to implement an optimised version of
	// this which doesn't build up an AST and just throws away tokens.
	fn consume_block(p: &mut Parser<'a>) {
		p.parse::<Self::Block>().ok();
	}

	// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
	fn parse_qualified_rule(p: &mut Parser<'a>) -> Result<(Self::Prelude, Self::Block)> {
		// Let rule be a new qualified rule with its prelude, declarations, and child rules all initially set to empty lists.

		// Process input:

		// <EOF-token>
		// stop token (if passed)
		//   This is a parse error. Return nothing.
		if p.at_end() {
			Err(diagnostics::UnexpectedEnd())?
		}

		// <}-token>
		//   This is a parse error. If nested is true, return nothing. Otherwise, consume a token and append the result to rule’s prelude.
		if p.is(State::Nested) && p.peek::<T!['}']>() {
			Err(diagnostics::UnexpectedCloseCurly(p.peek_n(1).into()))?;
		}

		// <{-token>
		//	If the first two non-<whitespace-token> values of rule’s prelude are an <ident-token> whose value starts with "--" followed by a <colon-token>, then:
		let checkpoint = p.checkpoint();
		if p.peek::<T![DashedIdent]>() {
			p.parse::<T![DashedIdent]>().ok();
			if p.peek::<T![:]>() {
				// If nested is true, consume the remnants of a bad declaration from input, with nested set to true, and return nothing.
				if p.is(State::Nested) {
					p.rewind(checkpoint);
					p.parse::<Self::BadDeclaration>()?;
					Err(diagnostics::BadDeclaration(checkpoint.into()))?
				// If nested is false, consume a block from input, and return nothing.
				} else {
					Self::consume_block(p);
					Err(diagnostics::BadDeclaration(checkpoint.into()))?
				}
			}
			p.rewind(checkpoint);
		}

		// Set the StopOn Curly to signify to prelude parsers that they shouldn't consume beyond the curly
		let old_stop = p.set_stop(KindSet::new(&[Kind::LeftCurly]));
		let prelude = p.parse::<Self::Prelude>();
		p.set_stop(old_stop);
		let prelude = prelude?;

		// Otherwise, consume a block from input, and let child rules be the result.
		// If the first item of child rules is a list of declarations,
		// remove it from child rules and assign it to rule’s declarations.
		// If any remaining items of child rules are lists of declarations,
		// replace them with nested declarations rules containing the list as its sole child.
		// Assign child rules to rule’s child rules.
		Ok((prelude, p.parse::<Self::Block>()?))
	}
}
