use crate::{Kind, Token};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindSet(u32);

impl KindSet {
	pub const NONE: KindSet = KindSet(0);
	pub const TRIVIA: KindSet = KindSet::new(&[Kind::Whitespace, Kind::Comment]);
	pub const WHITESPACE: KindSet = KindSet::new(&[Kind::Whitespace]);
	pub const COMMENTS: KindSet = KindSet::new(&[Kind::Comment]);
	pub const RIGHT_CURLY_OR_SEMICOLON: KindSet = KindSet::new(&[Kind::RightCurly, Kind::Semicolon]);
	pub const LEFT_CURLY_OR_SEMICOLON: KindSet = KindSet::new(&[Kind::LeftCurly, Kind::Semicolon]);
	pub const LEFT_CURLY_RIGHT_PAREN_OR_SEMICOLON: KindSet =
		KindSet::new(&[Kind::LeftCurly, Kind::RightParen, Kind::Semicolon]);
	pub const LEFT_CURLY_RIGHT_PAREN_COMMA_OR_SEMICOLON: KindSet =
		KindSet::new(&[Kind::LeftCurly, Kind::RightParen, Kind::Comma, Kind::Semicolon]);

	pub const fn new(kinds: &[Kind]) -> Self {
		let mut u = 0;
		let mut i = 0;
		let len = kinds.len();
		while i < len {
			u |= 1 << (kinds[i] as u8 % 32);
			i += 1;
		}
		Self(u)
	}

	pub const fn add(&self, kind: Kind) -> Self {
		Self(self.0 | (1 << (kind as u8 % 32)))
	}

	pub const fn new_from_tokens(tokens: &[Token]) -> Self {
		let mut u = 0;
		let mut i = 0;
		let len = tokens.len();
		while i < len {
			u |= 1 << (tokens[i].kind_bits() % 32);
			i += 1;
		}
		Self(u)
	}

	pub const fn contains(&self, kind: Kind) -> bool {
		self.0 & (1 << (kind as u8 % 32)) != 0
	}

	pub(crate) const fn contains_bits(&self, kind_bits: u8) -> bool {
		self.0 & (1 << (kind_bits % 32)) != 0
	}
}

#[test]
fn test_kindset_contains() {
	let set = KindSet::new(&[Kind::Eof, Kind::Whitespace, Kind::Comment]);
	assert!(set.contains(Kind::Eof));
	assert!(set.contains(Kind::Whitespace));
	assert!(set.contains(Kind::Comment));
	assert!(!set.contains(Kind::String));
	assert!(!set.contains(Kind::Url));
}
