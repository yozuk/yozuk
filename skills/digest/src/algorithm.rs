use digest::{Digest, FixedOutputReset};

pub const ENTRIES: &[AlgorithmEntry] = &[
    AlgorithmEntry {
        name: "MD5",
        keywords: &["md5"],
        init: || Box::new(DigestEntry::<md5::Md5>::new()),
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
