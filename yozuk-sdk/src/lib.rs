#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub use bytes::Bytes;

pub mod args;
pub mod env;
pub mod feature;
pub mod output;
pub mod prelude;
pub mod skill;
pub mod token;

mod serde_bytes;
