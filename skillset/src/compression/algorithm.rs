use flate2::write::{DeflateEncoder, GzEncoder, ZlibEncoder};
use flate2::Compression;
use std::io::Write;

pub struct Algorithm {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub compressor: fn() -> Box<dyn Compressor>,
}

pub const ENTRIES: &[Algorithm] = &[
    Algorithm {
        name: "Zlib",
        keywords: &["zlib"],
        compressor: || Box::new(ZlibCompressor::new()),
    },
    Algorithm {
        name: "Gzip",
        keywords: &["gzip", "gz"],
        compressor: || Box::new(GzipCompressor::new()),
    },
    Algorithm {
        name: "Deflate",
        keywords: &["deflate"],
        compressor: || Box::new(DeflateCompressor::new()),
    },
    Algorithm {
        name: "Snappy",
        keywords: &["snappy"],
        compressor: || Box::new(SnappyCompressor::new()),
    },
];

pub trait Compressor {
    fn update(&mut self, data: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
}

struct ZlibCompressor(Option<ZlibEncoder<Vec<u8>>>);

impl ZlibCompressor {
    fn new() -> Self {
        Self(Some(ZlibEncoder::new(Vec::new(), Compression::fast())))
    }
}

impl Compressor for ZlibCompressor {
    fn update(&mut self, data: &[u8]) {
        if let Some(inner) = &mut self.0 {
            inner.write_all(data).unwrap();
        }
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0
            .take()
            .and_then(|inner| inner.finish().ok())
            .unwrap_or_default()
    }
}

struct GzipCompressor(Option<GzEncoder<Vec<u8>>>);

impl GzipCompressor {
    fn new() -> Self {
        Self(Some(GzEncoder::new(Vec::new(), Compression::fast())))
    }
}

impl Compressor for GzipCompressor {
    fn update(&mut self, data: &[u8]) {
        if let Some(inner) = &mut self.0 {
            inner.write_all(data).unwrap();
        }
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0
            .take()
            .and_then(|inner| inner.finish().ok())
            .unwrap_or_default()
    }
}

struct DeflateCompressor(Option<DeflateEncoder<Vec<u8>>>);

impl DeflateCompressor {
    fn new() -> Self {
        Self(Some(DeflateEncoder::new(Vec::new(), Compression::fast())))
    }
}

impl Compressor for DeflateCompressor {
    fn update(&mut self, data: &[u8]) {
        if let Some(inner) = &mut self.0 {
            inner.write_all(data).unwrap();
        }
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0
            .take()
            .and_then(|inner| inner.finish().ok())
            .unwrap_or_default()
    }
}

struct SnappyCompressor(Option<snap::write::FrameEncoder<Vec<u8>>>);

impl SnappyCompressor {
    fn new() -> Self {
        Self(Some(snap::write::FrameEncoder::new(Vec::new())))
    }
}

impl Compressor for SnappyCompressor {
    fn update(&mut self, data: &[u8]) {
        if let Some(inner) = &mut self.0 {
            inner.write_all(data).unwrap();
        }
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0
            .take()
            .and_then(|inner| inner.into_inner().ok())
            .unwrap_or_default()
    }
}
