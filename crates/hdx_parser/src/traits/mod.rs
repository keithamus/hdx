use hdx_atom::Atom;
use hdx_lexer::{Span, Spanned, Token as LexerToken};

mod declarations;
mod rules;
mod selectors;

pub use declarations::*;
pub use rules::*;
pub use selectors::*;

use crate::{diagnostics, parser::Parser, Comparison, Result, State, Vec, T};

impl<'a, T: Parse<'a>> Parse<'a> for Vec<'a, T> {
	fn parse(p: &mut Parser<'a>) -> Result<Vec<'a, T>> {
		loop {
			let mut vec = p.new_vec();
			if let Ok(t) = p.parse::<T>() {
				vec.push(t);
			} else {
				return Ok(vec);
			}
		}
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for Spanned<T> {
	fn parse(p: &mut Parser<'a>) -> Result<Spanned<T>> {
		T::parse_spanned(p)
	}
}

pub trait Parse<'a>: Sized {
	fn parse(p: &mut Parser<'a>) -> Result<Self>;

	fn try_parse(p: &mut Parser<'a>) -> Result<Self> {
		let checkpoint = p.checkpoint();
		Self::parse(p).inspect_err(|_| p.rewind(checkpoint))
	}

	fn parse_spanned(p: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let start = p.offset();
		let node = Self::parse(p)?;
		Ok(Spanned { node, span: Span::new(start, p.offset()) })
	}

	fn try_parse_spanned(p: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let start = p.offset();
		let node = Self::try_parse(p)?;
		Ok(Spanned { node, span: Span::new(start, p.offset()) })
	}
}

pub trait Peek<'a>: Sized {
	fn peek(p: &Parser<'a>) -> Option<LexerToken>;
}

pub trait Block<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type Rule: Parse<'a>;

	// https://drafts.csswg.org/css-syntax-3/#consume-block-contents
	fn parse_block(p: &mut Parser<'a>) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::Rule>>)> {
		p.parse::<T![LeftCurly]>()?;
		let mut declarations = p.new_vec();
		let mut rules = p.new_vec();
		loop {
			if p.at_end() {
				break;
			}
			p.parse::<T![;]>().ok();
			if p.parse::<T![RightCurly]>().is_ok() {
				break;
			}
			let old_state = p.set_state(State::Nested);
			if p.peek::<T![AtKeyword]>().is_some() {
				rules.push(p.parse_spanned::<Self::Rule>().inspect_err(|_| {
					p.set_state(old_state);
				})?);
			} else {
				let checkpoint = p.checkpoint();

				if let Ok(decl) = p.parse_spanned::<Self::Declaration>() {
					declarations.push(decl);
				} else {
					p.rewind(checkpoint);
					rules.push(p.parse_spanned::<Self::Rule>()?);
				}
			}
			p.set_state(old_state);
		}
		Ok((declarations, rules))
	}
}

pub trait StyleSheet<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_stylesheet(p: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		let mut rules: Vec<'a, Spanned<Self::Rule>> = p.new_vec();
		loop {
			if p.at_end() {
				return Ok(rules);
			}
			p.parse::<T![CdcOrCdo]>().ok();
			if let Ok(rule) = p.parse_spanned::<Self::Rule>() {
				rules.push(rule)
			}
		}
	}
}

pub trait DiscreteMediaFeature<'a>: Sized + Default {
	fn parse_media_feature_value(p: &mut Parser<'a>) -> Result<Self>;

	fn parse_descrete_media_feature(name: Atom, p: &mut Parser<'a>) -> Result<Self> {
		let token = *p.parse::<T![Ident]>()?;
		let atom = p.parse_atom_lower(token);
		if atom != name {
			Err(diagnostics::ExpectedIdentOf(name, atom, token.span()))?
		}
		if let Some(token) = p.peek::<T![:]>() {
			p.hop(token);
			Ok(Self::parse_media_feature_value(p)?)
		} else {
			Ok(Self::default())
		}
	}
}

pub trait RangedMediaFeature<'a>: Sized {
	type Type: Parse<'a>;

	fn new(left: (Comparison, Self::Type), right: Option<(Comparison, Self::Type)>, legacy: bool) -> Self;

	fn parse_ranged_media_feature(name: Atom, p: &mut Parser<'a>) -> Result<Self> {
		let left = if let Some(token) = p.peek::<T![Ident]>() {
			p.hop(token);
			let mut legacy = false;
			let legacy_cmp = match p.parse_atom_lower(token) {
				atom if atom == name => {
					legacy = p.peek::<T![:]>().is_some();
					Comparison::Equal
				}
				atom if atom.strip_prefix("max-").unwrap_or("") == name.as_ref() => {
					legacy = true;
					Comparison::GreaterThanEqual
				}
				atom if atom.strip_prefix("min-").unwrap_or("") == name.as_ref() => {
					legacy = true;
					Comparison::LessThanEqual
				}
				atom => Err(diagnostics::ExpectedIdentOf(name, atom, token.span()))?,
			};
			if legacy {
				p.parse::<T![:]>()?;
				return Ok(Self::new((legacy_cmp, p.parse::<Self::Type>()?), None, true));
			} else {
				let cmp = p.parse::<Comparison>()?;
				return Ok(Self::new((cmp, p.parse::<Self::Type>()?), None, false));
			}
		} else {
			p.parse::<Self::Type>()?
		};
		let offset = p.offset();
		let left_cmp = p.parse::<Comparison>()?;
		let token = *p.parse::<T![Ident]>()?;
		let atom = p.parse_atom_lower(token);
		if atom != name {
			Err(diagnostics::ExpectedIdentOf(name, atom, token.span()))?
		}
		if p.peek::<T![Delim]>().is_none() {
			return Ok(Self::new((left_cmp, left), None, false));
		}
		let right_cmp = p.parse::<Comparison>()?;
		if left_cmp == Comparison::Equal && right_cmp == Comparison::Equal {
			Err(diagnostics::UnexpectedMediaRangeComparisonEqualsTwice(Span::new(offset, p.offset())))?
		}
		Ok(Self::new((left_cmp, left), Some((right_cmp, p.parse::<Self::Type>()?)), false))
	}
}
