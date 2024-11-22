use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, T};

pub mod charset;
pub mod color_profile;
pub mod container;
pub mod counter_style;
pub mod document;
pub mod font_face;
pub mod font_feature_values;
pub mod font_palette_values;
pub mod import;
pub mod keyframes;
pub mod layer;
pub mod media;
pub mod moz;
pub mod namespace;
pub mod page;
pub mod property;
pub mod scope;
pub mod starting_style;
pub mod supports;
pub mod webkit;

pub use charset::*;
pub use color_profile::*;
pub use container::*;
pub use counter_style::*;
pub use document::*;
pub use font_face::*;
pub use font_feature_values::*;
pub use font_palette_values::*;
pub use import::*;
pub use keyframes::*;
pub use layer::*;
pub use media::*;
pub use moz::*;
pub use namespace::*;
pub use page::*;
pub use property::*;
pub use scope::*;
pub use starting_style::*;
pub use supports::*;
pub use webkit::*;

pub struct NoPreludeAllowed;
impl<'a> Parse<'a> for NoPreludeAllowed {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if parser.peek::<T![LeftCurly]>().is_some() || parser.peek::<T![;]>().is_some() {
			Ok(Self {})
		} else {
			let token = parser.peek::<T![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
	}
}

pub struct NoBlockAllowed;
impl<'a> Parse<'a> for NoBlockAllowed {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if parser.at_end() || parser.peek::<T![;]>().is_some() {
			Ok(Self {})
		} else {
			let token = parser.peek::<T![Any]>().unwrap();
			Err(diagnostics::Unexpected(token, token.span()))?
		}
	}
}
