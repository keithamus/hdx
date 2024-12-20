#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "kind", content = "value"))]
pub enum WhitespaceStyle {
	#[default]
	None,
	Space = 0b001,                                 // ' '
	Tab = 0b010,                                   // '\t'
	Newline = 0b011,                               // '\n'
	NewlineUsingCarriageReturn = 0b100,            // '\r'
	NewlineUsingCarriageReturnAndLineFeed = 0b101, // '\r\n'
	NewlineUsingFormFeed = 0b110,                  // '\u{c}'
}

impl WhitespaceStyle {
	#[inline]
	pub const fn is_newline(&self) -> bool {
		matches!(
			self,
			Self::Newline
				| Self::NewlineUsingCarriageReturn
				| Self::NewlineUsingCarriageReturnAndLineFeed
				| Self::NewlineUsingFormFeed
		)
	}

	pub(crate) const fn from_bits(bits: u8) -> Self {
		match bits {
			0b001 => Self::Space,
			0b010 => Self::Tab,
			0b011 => Self::Newline,
			0b100 => Self::NewlineUsingCarriageReturn,
			0b101 => Self::NewlineUsingCarriageReturnAndLineFeed,
			0b110 => Self::NewlineUsingFormFeed,
			_ => Self::None,
		}
	}

	pub const fn as_str(&self) -> &str {
		match self {
			Self::None => "",
			Self::Space => " ",
			Self::Tab => "\t",
			Self::Newline => "\n",
			Self::NewlineUsingCarriageReturn => "\r",
			Self::NewlineUsingCarriageReturnAndLineFeed => "\r\n",
			Self::NewlineUsingFormFeed => "\u{c}",
		}
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<WhitespaceStyle>(), 1);
}

#[test]
fn test_from_bits() {
	assert_eq!(WhitespaceStyle::from_bits(WhitespaceStyle::None as u8), WhitespaceStyle::None);
	assert_eq!(WhitespaceStyle::from_bits(WhitespaceStyle::Space as u8), WhitespaceStyle::Space);
	assert_eq!(WhitespaceStyle::from_bits(WhitespaceStyle::Tab as u8), WhitespaceStyle::Tab);
	assert_eq!(WhitespaceStyle::from_bits(WhitespaceStyle::Newline as u8), WhitespaceStyle::Newline);
	assert_eq!(
		WhitespaceStyle::from_bits(WhitespaceStyle::NewlineUsingCarriageReturn as u8),
		WhitespaceStyle::NewlineUsingCarriageReturn
	);
	assert_eq!(
		WhitespaceStyle::from_bits(WhitespaceStyle::NewlineUsingCarriageReturnAndLineFeed as u8),
		WhitespaceStyle::NewlineUsingCarriageReturnAndLineFeed
	);
	assert_eq!(
		WhitespaceStyle::from_bits(WhitespaceStyle::NewlineUsingFormFeed as u8),
		WhitespaceStyle::NewlineUsingFormFeed
	);
}
