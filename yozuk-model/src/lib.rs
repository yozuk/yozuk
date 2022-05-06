#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod labeler;
mod modelgen;

#[cfg(feature = "modelgen")]
pub use modelgen::*;

pub use labeler::*;
