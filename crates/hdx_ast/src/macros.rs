macro_rules! length_percentage_struct {
	($name: ident) => {
		#[derive(Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $name {
			length: $crate::css::values::units::LengthPercentage,
		}

		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				self.length.write_css(sink)
			}
		}
	}
}
pub(crate) use length_percentage_struct;

macro_rules! positive_length_percentage_property {
	($name: ident) => {
		$crate::length_percentage_struct!($name);
		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<hdx_parser::Spanned<Self>> {
				use hdx_lexer::Token;
				use $crate::css::values::units::LengthPercentage;
				let span = parser.span();
				match parser.cur() {
					Token::Number(n, _) if n == 0.0 => Ok(Self { length: LengthPercentage::Zero }.spanned(span)),
					Token::Dimension(n, unit, _) => {
						if n < 0.0 {
							Err(hdx_parser::diagnostics::NumberNotNegative(n, span))?
						}
						if let Some(length) = LengthPercentage::new(n.into(), unit.clone()) {
							Ok(Self { length }.spanned(span))
						} else {
							Err(hdx_parser::diagnostics::UnexpectedDimension(unit, span))?
						}
					}
					token => hdx_parser::unexpected!(parser, token),
				}
			}
		}
	};
}
pub(crate) use positive_length_percentage_property;

macro_rules! length_percentage_property {
	($name: ident) => {
		$crate::length_percentage_struct!($name);
		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<hdx_parser::Spanned<Self>> {
				use hdx_lexer::Token;
				use $crate::css::values::units::LengthPercentage;
				let span = parser.span();
				match parser.cur() {
					Token::Number(n, _) if n == 0.0 => Ok(Self { length: LengthPercentage::Zero }.spanned(span)),
					Token::Dimension(n, unit, _) => {
						if let Some(length) = LengthPercentage::new(n.into(), unit.clone()) {
							Ok(Self { length }.spanned(span))
						} else {
							Err(hdx_parser::diagnostics::UnexpectedDimension(unit, span))?
						}
					}
					token => hdx_parser::unexpected!(parser, token),
				}
			}
		}
	};
}
pub(crate) use length_percentage_property;
