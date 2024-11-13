#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "kind", content = "value"))]
pub enum CommentStyle {
	#[default]
	Block = 0b000, // Standard: /* */
	BlockStar = 0b001,    // Standard but with two stars: /** */
	BlockBang = 0b010,    // Standard but with an excalamation: /*! */
	BlockPound = 0b011,   // Standard but with a hash: /*# */
	BlockHeading = 0b100, // Standard but with a dash or equals: /*= */ or /*- */
	Single = 0b101,       // Non-standard two slashes '//'
	SingleStar = 0b110,   // Non-standard two slashes and a star '//*'
	SingleBang = 0b111,   // Non-standard two slashes and a star '//!'
}

impl CommentStyle {
	#[inline]
	pub fn is_block(&self) -> bool {
		matches!(self, Self::Block | Self::BlockStar | Self::BlockBang | Self::BlockPound | Self::BlockHeading)
	}

	#[inline]
	pub fn is_non_standard(&self) -> bool {
		matches!(self, Self::Single | Self::SingleStar | Self::SingleBang)
	}

	#[inline]
	pub fn retain(&self) -> bool {
		matches!(self, Self::Single | Self::SingleStar | Self::SingleBang)
	}

	pub(crate) fn from_bits(bits: u8) -> Option<Self> {
		match bits {
			0b000 => Some(Self::Block),
			0b001 => Some(Self::BlockStar),
			0b010 => Some(Self::BlockBang),
			0b011 => Some(Self::BlockPound),
			0b100 => Some(Self::BlockHeading),
			0b101 => Some(Self::Single),
			0b110 => Some(Self::SingleStar),
			0b111 => Some(Self::SingleBang),
			_ => None,
		}
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<CommentStyle>(), 1);
}

#[test]
fn test_from_bits() {
	assert_eq!(CommentStyle::from_bits(CommentStyle::Block as u8), Some(CommentStyle::Block));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockStar as u8), Some(CommentStyle::BlockStar));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockBang as u8), Some(CommentStyle::BlockBang));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockPound as u8), Some(CommentStyle::BlockPound));
	assert_eq!(CommentStyle::from_bits(CommentStyle::BlockHeading as u8), Some(CommentStyle::BlockHeading));
	assert_eq!(CommentStyle::from_bits(CommentStyle::Single as u8), Some(CommentStyle::Single));
	assert_eq!(CommentStyle::from_bits(CommentStyle::SingleStar as u8), Some(CommentStyle::SingleStar));
	assert_eq!(CommentStyle::from_bits(CommentStyle::SingleBang as u8), Some(CommentStyle::SingleBang));
}
