#![forbid(unsafe_code)]

pub const MODEL_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model.crfsuite"));
