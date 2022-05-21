#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod labeler;
mod modelgen;
mod tagger;

#[cfg(feature = "modelgen")]
pub use modelgen::*;

pub use labeler::*;
pub use tagger::*;
