#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub use bytes::Bytes;

pub mod args;
pub mod block;
pub mod display;
pub mod encoding;
pub mod env;
pub mod feature;
pub mod highlight;
pub mod metadata;
pub mod model;
pub mod output;
pub mod prelude;
pub mod preprocessor;
pub mod skill;
pub mod stream;
pub mod token;
pub mod tokenizer;
pub mod user;

mod serde_bytes;
