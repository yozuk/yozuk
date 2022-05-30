#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub use bytes::Bytes;

pub mod args;
pub mod block;
pub mod display;
pub mod env;
pub mod feature;
pub mod i18n;
pub mod metadata;
pub mod model;
pub mod output;
pub mod prelude;
pub mod skill;
pub mod stream;
pub mod token;
pub mod tokenizer;

mod serde_bytes;
