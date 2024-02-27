#[cfg(feature = "serde")]
use serde::Serialize;

mod margin_trim;
mod margin;
mod padding;

pub use margin_trim::*;
pub use margin::*;
pub use padding::*;
