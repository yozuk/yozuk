use rand::rngs::ThreadRng;
use rand::Rng;
use scrypt::password_hash::{PasswordHasher, SaltString};

pub const ENTRIES: &[AlgorithmEntry] = &[
    AlgorithmEntry {
        name: "bcrypt",
        keywords: &["bcrypt"],
        init: || Box::new(Bcrypt),
    },
    AlgorithmEntry {
        name: "scrypt",
        keywords: &["scrypt"],
        init: || Box::new(Scrypt),
    },
];

struct Bcrypt;

impl Algorithm for Bcrypt {
    fn hash_default(&self, password: &[u8], rng: &mut ThreadRng) -> String {
        bcrypt::hash_with_salt(password, bcrypt::DEFAULT_COST, rng.gen())
            .map(|hash| hash.to_string())
            .unwrap_or_default()
    }
}

struct Scrypt;

impl Algorithm for Scrypt {
    fn hash_default(&self, password: &[u8], rng: &mut ThreadRng) -> String {
        let salt = SaltString::generate(rng);
        scrypt::Scrypt
            .hash_password(password, &salt)
            .map(|hash| hash.to_string())
            .unwrap_or_default()
    }
}

pub struct AlgorithmEntry {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub init: fn() -> Box<dyn Algorithm>,
}

pub trait Algorithm {
    fn hash_default(&self, password: &[u8], rng: &mut ThreadRng) -> String;
}
