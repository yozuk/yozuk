use flate2::write::{
    DeflateDecoder, DeflateEncoder, GzDecoder, GzEncoder, ZlibDecoder, ZlibEncoder,
};
use flate2::Compression;
use std::io::{Read, Result, Write};

pub struct Algorithm {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub compressor: fn() -> Box<dyn Compressor>,
    pub test_header: fn(&[u8]) -> bool,
    pub decompressor: fn() -> Box<dyn Decompressor>,
}

pub const ENTRIES: &[Algorithm] = &[
    Algorithm {
        name: "Zlib",
        keywords: &["zlib"],
        compressor: || Box::new(ZlibCompressor::new()),
        test_header: check_zlib_header,
        decompressor: || Box::new(ZlibDecompressor::new()),
    },
    Algorithm {
        name: "Gzip",
        keywords: &["gzip", "gz"],
        compressor: || Box::new(GzipCompressor::new()),
        test_header: |header| header.starts_with(&[0x1f, 0x8b, 0x08]),
        decompressor: || Box::new(GzipDecompressor::new()),
    },
    Algorithm {
        name: "Deflate",
        keywords: &["deflate"],
        compressor: || Box::new(DeflateCompressor::new()),
        test_header: |_| false,
        decompressor: || Box::new(DeflateDecompressor::new()),
    },
    Algorithm {
        name: "Snappy",
        keywords: &["snappy"],
        compressor: || Box::new(SnappyCompressor::new()),
        test_header: |_| false,
        decompressor: || Box::new(SnappyDecompressor::new()),
    },
];

pub trait Compressor {
    fn update(&mut self, data: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
}

pub trait Decompressor {
    fn update(&mut self, data: &[u8]) -> Result<()>;
    fn finalize(&mut self) -> Result<Vec<u8>>;
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

fn check_zlib_header(header: &[u8]) -> bool {
    if let &[cmf, flg, ..] = header {
        let cm = cmf >> 4;
        let cinfo = cmf & 0b1111;
        if cm == 0x8 && cinfo <= 7 {
            let check = cmf as u16 * 256 + flg as u16;
            return check % 31 == 0;
        }
    }
    false
}

struct ZlibDecompressor(Option<ZlibDecoder<Vec<u8>>>);

impl ZlibDecompressor {
    fn new() -> Self {
        Self(Some(ZlibDecoder::new(Vec::new())))
    }
}

impl Decompressor for ZlibDecompressor {
    fn update(&mut self, data: &[u8]) -> Result<()> {
        if let Some(inner) = &mut self.0 {
            inner.write_all(data)?;
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        let inner = self.0.take().unwrap();
        inner.finish()
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

struct GzipDecompressor(Option<GzDecoder<Vec<u8>>>);

impl GzipDecompressor {
    fn new() -> Self {
        Self(Some(GzDecoder::new(Vec::new())))
    }
}

impl Decompressor for GzipDecompressor {
    fn update(&mut self, data: &[u8]) -> Result<()> {
        if let Some(inner) = &mut self.0 {
            inner.write_all(data)?;
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        let inner = self.0.take().unwrap();
        inner.finish()
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

struct DeflateDecompressor(Option<DeflateDecoder<Vec<u8>>>);

impl DeflateDecompressor {
    fn new() -> Self {
        Self(Some(DeflateDecoder::new(Vec::new())))
    }
}

impl Decompressor for DeflateDecompressor {
    fn update(&mut self, data: &[u8]) -> Result<()> {
        if let Some(inner) = &mut self.0 {
            inner.write_all(data)?;
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        let inner = self.0.take().unwrap();
        inner.finish()
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

struct SnappyDecompressor(Vec<u8>);

impl SnappyDecompressor {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl Decompressor for SnappyDecompressor {
    fn update(&mut self, data: &[u8]) -> Result<()> {
        self.0.extend_from_slice(data);
        Ok(())
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        let data: &[u8] = &self.0;
        let mut buf = Vec::new();
        let mut decoder = snap::read::FrameDecoder::new(data);
        let _ = decoder.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
