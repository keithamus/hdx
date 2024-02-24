mod cursor;
pub mod diagnostics;
mod macros;
mod parser;
mod span;
mod traits;

pub use miette::{Error, Result};
pub use oxc_allocator::{Box, Vec};
pub use parser::*;
pub use span::*;
pub use traits::*;
