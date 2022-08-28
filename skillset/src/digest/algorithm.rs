use digest::{Digest, FixedOutputReset};

pub const ENTRIES: &[AlgorithmEntry] = &[
    AlgorithmEntry {
        name: "MD4",
        keywords: &["md4"],
        init: || Box::new(DigestEntry::<md4::Md4>::new()),
        multihash_prefix: Some(&[0xd4]),
    },
    AlgorithmEntry {
        name: "MD5",
        keywords: &["md5"],
        init: || Box::new(DigestEntry::<md5::Md5>::new()),
        multihash_prefix: Some(&[0xd5]),
    },
    AlgorithmEntry {
        name: "SHA-1",
        keywords: &["sha1", "sha-1"],
        init: || Box::new(DigestEntry::<sha1::Sha1>::new()),
        multihash_prefix: Some(&[0x11]),
    },
    AlgorithmEntry {
        name: "SHA-224",
        keywords: &["sha-224", "sha224", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha224>::new()),
        multihash_prefix: Some(&[0x10, 0x13]),
    },
    AlgorithmEntry {
        name: "SHA-256",
        keywords: &["sha-256", "sha256", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha256>::new()),
        multihash_prefix: Some(&[0x12]),
    },
    AlgorithmEntry {
        name: "SHA-384",
        keywords: &["sha-384", "sha384", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha384>::new()),
        multihash_prefix: Some(&[0x20]),
    },
    AlgorithmEntry {
        name: "SHA-512",
        keywords: &["sha-512", "sha512", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha512>::new()),
        multihash_prefix: Some(&[0x13]),
    },
    AlgorithmEntry {
        name: "SHA-512/224",
        keywords: &["sha-512/224", "sha512-224", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha512_224>::new()),
        multihash_prefix: Some(&[0x10, 0x14]),
    },
    AlgorithmEntry {
        name: "SHA-512/256",
        keywords: &["sha-512/256", "sha512-256", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha512_256>::new()),
        multihash_prefix: Some(&[0x10, 0x15]),
    },
    AlgorithmEntry {
        name: "SHA-3-224",
        keywords: &["sha3-224", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_224>::new()),
        multihash_prefix: Some(&[0x17]),
    },
    AlgorithmEntry {
        name: "SHA-3-256",
        keywords: &["sha3-256", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_256>::new()),
        multihash_prefix: Some(&[0x16]),
    },
    AlgorithmEntry {
        name: "SHA-3-384",
        keywords: &["sha3-384", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_384>::new()),
        multihash_prefix: Some(&[0x15]),
    },
    AlgorithmEntry {
        name: "SHA-3-512",
        keywords: &["sha3-512", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_512>::new()),
        multihash_prefix: Some(&[0x14]),
    },
    AlgorithmEntry {
        name: "CRC-32/ISO-HDLC",
        keywords: &[
            "crc32",
            "crc-32",
            "crc32-iso",
            "crc-32-iso",
            "crc-32-iso-hdlc",
            "crc-32/iso-hdlc",
            "crc32/iso-hdlc",
            "crc32/iso",
        ],
        init: || {
            Box::new(Crc32Entry::new(crc_all::Crc::<u32>::new(
                0x04c11db7, 32, 0xffffffff, 0xffffffff, true,
            )))
        },
        multihash_prefix: None,
    },
    AlgorithmEntry {
        name: "BLAKE2-S-256",
        keywords: &["blake2s256", "blake2"],
        init: || Box::new(DigestEntry::<blake2::Blake2s256>::new()),
        multihash_prefix: Some(&[0xb2, 0x60]),
    },
    AlgorithmEntry {
        name: "BLAKE2-B-512",
        keywords: &["blake2b512", "blake2"],
        init: || Box::new(DigestEntry::<blake2::Blake2b512>::new()),
        multihash_prefix: Some(&[0xb2, 0x40]),
    },
    AlgorithmEntry {
        name: "BLAKE3",
        keywords: &["blake3"],
        init: || Box::new(DigestEntry::<blake3::Hasher>::new()),
        multihash_prefix: Some(&[0x1e]),
    },
];

pub struct AlgorithmEntry {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub init: fn() -> Box<dyn Algorithm>,
    pub multihash_prefix: Option<&'static [u8]>,
}

pub trait Algorithm {
    fn update(&mut self, data: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
}

pub struct DigestEntry<T>(T);

impl<T> DigestEntry<T>
where
    T: Digest,
{
    pub fn new() -> Self {
        Self(T::new())
    }
}

impl<T> Algorithm for DigestEntry<T>
where
    T: Digest + FixedOutputReset,
{
    fn update(&mut self, data: &[u8]) {
        Digest::update(&mut self.0, data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0.finalize_reset().to_vec()
    }
}

pub struct Crc32Entry(crc_all::Crc<u32>);

impl Crc32Entry {
    pub fn new(alg: crc_all::Crc<u32>) -> Self {
        Self(alg)
    }
}

impl Algorithm for Crc32Entry {
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0.finish().to_be_bytes().to_vec()
    }
}
