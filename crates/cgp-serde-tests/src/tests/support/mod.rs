//! Helpers that run a value through a context and a Serde format, so each test states only its
//! input and the exact output it expects.

mod json;
mod postcard;
mod ron;

pub use json::*;
pub use postcard::*;
pub use ron::*;
