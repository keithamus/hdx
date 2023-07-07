//! CSS Token Kinds

use std::fmt;

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Kind {
    Undetermined,
    #[default]
    Eof,
    Comment,
    Ident,
    Function,
    AtKeyword,
    Hash,
    String,
    BadString,
    Url,
    BadUrl,
    Delim,
    Number,
    Percentage,
    Dimension,
    Whitespace,
    Cdo,
    Cdc,
    Colon,
    Semicolon,
    Comma,
    LeftSquare,
    RightSquare,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
}

#[allow(clippy::enum_glob_use)]
use self::Kind::*;

impl Kind {
    pub fn is_eof(&self) -> bool {
        matches!(self, Eof)
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, Whitespace | Comment)
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Number | Percentage | Dimension)
    }

    pub fn is_function_like(&self) -> bool {
        matches!(self, Url | Function)
    }

    pub fn is_bad(&self) -> bool {
        matches!(self, BadUrl | BadString)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn display() {
        assert_eq!(format!("{}", Kind::Eof), "Eof");
        assert_eq!(format!("{}", Kind::BadString), "BadString");
    }
}
