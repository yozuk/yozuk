use digest::{Digest, FixedOutputReset};

pub const ENTRIES: &[AlgorithmEntry] = &[
    AlgorithmEntry {
        name: "MD2",
        keywords: &["md2"],
        init: || Box::new(DigestEntry::<md2::Md2>::new()),
    },
    AlgorithmEntry {
        name: "MD4",
        keywords: &["md4"],
        init: || Box::new(DigestEntry::<md4::Md4>::new()),
    },
    AlgorithmEntry {
        name: "MD5",
        keywords: &["md5"],
        init: || Box::new(DigestEntry::<md5::Md5>::new()),
    },
    AlgorithmEntry {
        name: "RIPEMD-160",
        keywords: &["ripemd-160", "ripemd160", "ripemd"],
        init: || Box::new(DigestEntry::<ripemd::Ripemd160>::new()),
    },
    AlgorithmEntry {
        name: "RIPEMD-256",
        keywords: &["ripemd-256", "ripemd256", "ripemd"],
        init: || Box::new(DigestEntry::<ripemd::Ripemd256>::new()),
    },
    AlgorithmEntry {
        name: "RIPEMD-320",
        keywords: &["ripemd-320", "ripemd320", "ripemd"],
        init: || Box::new(DigestEntry::<ripemd::Ripemd320>::new()),
    },
    AlgorithmEntry {
        name: "Shabal192",
        keywords: &["shabal-192", "shabal192", "shabal"],
        init: || Box::new(DigestEntry::<shabal::Shabal192>::new()),
    },
    AlgorithmEntry {
        name: "Shabal224",
        keywords: &["shabal-224", "shabal224", "shabal"],
        init: || Box::new(DigestEntry::<shabal::Shabal224>::new()),
    },
    AlgorithmEntry {
        name: "Shabal256",
        keywords: &["shabal-256", "shabal256", "shabal"],
        init: || Box::new(DigestEntry::<shabal::Shabal256>::new()),
    },
    AlgorithmEntry {
        name: "Shabal384",
        keywords: &["shabal-384", "shabal384", "shabal"],
        init: || Box::new(DigestEntry::<shabal::Shabal384>::new()),
    },
    AlgorithmEntry {
        name: "Shabal512",
        keywords: &["shabal-512", "shabal512", "shabal"],
        init: || Box::new(DigestEntry::<shabal::Shabal512>::new()),
    },
    AlgorithmEntry {
        name: "SHA-1",
        keywords: &["sha1", "sha-1"],
        init: || Box::new(DigestEntry::<sha1::Sha1>::new()),
    },
    AlgorithmEntry {
        name: "SHA-224",
        keywords: &["sha-224", "sha224", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha224>::new()),
    },
    AlgorithmEntry {
        name: "SHA-256",
        keywords: &["sha-256", "sha256", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha256>::new()),
    },
    AlgorithmEntry {
        name: "SHA-384",
        keywords: &["sha-384", "sha384", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha384>::new()),
    },
    AlgorithmEntry {
        name: "SHA-512",
        keywords: &["sha-512", "sha512", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha512>::new()),
    },
    AlgorithmEntry {
        name: "SHA-512/224",
        keywords: &["sha-512/224", "sha512-224", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha512_224>::new()),
    },
    AlgorithmEntry {
        name: "SHA-512/256",
        keywords: &["sha-512/256", "sha512-256", "sha2", "sha-2"],
        init: || Box::new(DigestEntry::<sha2::Sha512_256>::new()),
    },
    AlgorithmEntry {
        name: "SHA-3-224",
        keywords: &["sha3-224", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_224>::new()),
    },
    AlgorithmEntry {
        name: "SHA-3-256",
        keywords: &["sha3-256", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_256>::new()),
    },
    AlgorithmEntry {
        name: "SHA-3-384",
        keywords: &["sha3-384", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_384>::new()),
    },
    AlgorithmEntry {
        name: "SHA-3-512",
        keywords: &["sha3-512", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Sha3_512>::new()),
    },
    AlgorithmEntry {
        name: "Keccak-224",
        keywords: &["keccak-224", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Keccak224>::new()),
    },
    AlgorithmEntry {
        name: "Keccak-256",
        keywords: &["keccak-256", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Keccak256>::new()),
    },
    AlgorithmEntry {
        name: "Keccak-384",
        keywords: &["keccak-384", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Keccak384>::new()),
    },
    AlgorithmEntry {
        name: "Keccak-512",
        keywords: &["keccak-512", "sha3", "sha-3"],
        init: || Box::new(DigestEntry::<sha3::Keccak512>::new()),
    },
];

pub struct AlgorithmEntry {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub init: fn() -> Box<dyn Algorithm>,
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
