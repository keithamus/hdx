use crate::{diagnostics, Build, Parser, Peek, Result};

/// This trait allows AST nodes to construct themselves from a mutable [Parser] instance.
///
/// Nodes that implement this trait are entitled to consume any number of [Cursors][css_lexer::Cursor] from [Parser] in
/// order to construct themselves. They may also consume some amount of tokens and still return an [Err] - there is no
/// need to try and reset the [Parser] state on failure ([Parser::try_parse()] exists for this reason).
///
/// When wanting to parse child nodes, implementations should _not_ call [Parse::parse()] directly. Instead - call
/// [Parser::parse<T>()]. Other convenience methods such as [Parser::parse_if_peek<T>()] and [Parser::try_parse<T>()]
/// exist.
///
/// Any node implementing [Parse::parse()] gets [Parse::try_parse()] for free. It's unlikely that nodes can come up with
/// a more efficient algorithm than the provided one, so it is not worth re-implementing [Parse::try_parse()].
///
/// If a Node can construct itself from a single [Cursor][css_lexer::Cursor] it should instead implement
/// [Peek][crate::Peek] and [Build][crate::Build], which will provide [Parse] for free.
pub trait Parse<'a>: Sized {
	fn parse(p: &mut Parser<'a>) -> Result<Self>;

	fn try_parse(p: &mut Parser<'a>) -> Result<Self> {
		let checkpoint = p.checkpoint();
		Self::parse(p).inspect_err(|_| p.rewind(checkpoint))
	}
}

impl<'a, T> Parse<'a> for T
where
	T: Sized + Peek<'a> + Build<'a>,
{
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self::build(p, c))
		} else {
			let c = p.next();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}
