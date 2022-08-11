use flate2::read::{DeflateEncoder, GzEncoder, ZlibEncoder};
use flate2::Compression;
use std::io::{Read, Result};
use yozuk_sdk::prelude::*;

pub struct Algorithm {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub compress: fn(&mut InputStream) -> Result<Vec<u8>>,
}

pub const ENTRIES: &[Algorithm] = &[
    Algorithm {
        name: "Zlib",
        keywords: &["zlib"],
        compress: |stream| {
            let mut z = ZlibEncoder::new(stream, Compression::fast());
            let mut v = vec![];
            z.read_to_end(&mut v).map(|_| v)
        },
    },
    Algorithm {
        name: "Gzip",
        keywords: &["gzip", "gz"],
        compress: |stream| {
            let mut z = GzEncoder::new(stream, Compression::fast());
            let mut v = vec![];
            z.read_to_end(&mut v).map(|_| v)
        },
    },
    Algorithm {
        name: "Deflate",
        keywords: &["deflate"],
        compress: |stream| {
            let mut z = DeflateEncoder::new(stream, Compression::fast());
            let mut v = vec![];
            z.read_to_end(&mut v).map(|_| v)
        },
    },
];
