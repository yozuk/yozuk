#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub const MODEL_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model.crfsuite"));
