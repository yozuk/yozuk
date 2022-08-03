use rand::rngs::ThreadRng;
use rand::Rng;

pub const ENTRIES: &[AlgorithmEntry] = &[
    AlgorithmEntry {
        name: "Bcrypt",
        keywords: &["bcrypt"],
        init: || Box::new(Bcrypt),
    },
    AlgorithmEntry {
        name: "Scrypt",
        keywords: &["scrypt"],
        init: || Box::new(Scrypt),
    },
    AlgorithmEntry {
        name: "Argon2id",
        keywords: &["argon2", "argon2id"],
        init: || Box::new(Argon2id),
    },
    AlgorithmEntry {
        name: "Argon2i",
        keywords: &["argon2i"],
        init: || Box::new(Argon2i),
    },
    AlgorithmEntry {
        name: "Argon2d",
        keywords: &["argon2d"],
        init: || Box::new(Argon2d),
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
        use scrypt::password_hash::{PasswordHasher, SaltString};
        let salt = SaltString::generate(rng);
        scrypt::Scrypt
            .hash_password(password, &salt)
            .map(|hash| hash.to_string())
            .unwrap_or_default()
    }
}

struct Argon2i;

impl Algorithm for Argon2i {
    fn hash_default(&self, password: &[u8], rng: &mut ThreadRng) -> String {
        use argon2::password_hash::{PasswordHasher, SaltString};
        let salt = SaltString::generate(rng);
        argon2::Argon2::new(
            argon2::Algorithm::Argon2i,
            Default::default(),
            Default::default(),
        )
        .hash_password(password, &salt)
        .map(|hash| hash.to_string())
        .unwrap_or_default()
    }
}

struct Argon2d;

impl Algorithm for Argon2d {
    fn hash_default(&self, password: &[u8], rng: &mut ThreadRng) -> String {
        use argon2::password_hash::{PasswordHasher, SaltString};
        let salt = SaltString::generate(rng);
        argon2::Argon2::new(
            argon2::Algorithm::Argon2d,
            Default::default(),
            Default::default(),
        )
        .hash_password(password, &salt)
        .map(|hash| hash.to_string())
        .unwrap_or_default()
    }
}

struct Argon2id;

impl Algorithm for Argon2id {
    fn hash_default(&self, password: &[u8], rng: &mut ThreadRng) -> String {
        use argon2::password_hash::{PasswordHasher, SaltString};
        let salt = SaltString::generate(rng);
        argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            Default::default(),
            Default::default(),
        )
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
