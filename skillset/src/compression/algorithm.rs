use flate2::read::ZlibDecoder;
use std::io::{Read, Result};
use yozuk_sdk::prelude::*;

pub struct Algorithm {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub compress: fn(&mut InputStream) -> Result<Vec<u8>>,
}

pub const ENTRIES: &[Algorithm] = &[Algorithm {
    name: "Zlib",
    keywords: &["zlib"],
    compress: |stream| {
        let mut z = ZlibDecoder::new(stream);
        let mut v = vec![];
        z.read_to_end(&mut v).map(|_| v)
    },
}];
