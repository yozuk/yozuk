mod base64;
mod calc;
mod dice;
mod digest;
mod english;
mod lipsum;
mod mediatype;
mod nanoid;
mod numeric;
mod punycode;
mod smalltalk;
mod uuid;
mod version;

use yozuk_sdk::prelude::*;

#[derive(Clone)]
pub struct NamedSkillEntry {
    pub key: &'static str,
    pub entry: SkillEntry,
}

const MODEL_ROOT_ID: &[u8] = b"GbZvfkHw3RogOcwf542tr";

#[macro_export]
macro_rules! skills {
    ( $([ $x:ident, $y:literal ],)* ) => {
        pub const SKILLS: &[NamedSkillEntry] = &[
            $(
                #[cfg(feature = $y)]
                NamedSkillEntry {
                    key: $y,
                    entry: $x::ENTRY
                },
            )*
        ];

        pub const fn skills_digest() -> [u8; 20] {
            use const_sha1::ConstBuffer;
            let sha_sum = ConstBuffer::from_slice(MODEL_ROOT_ID);
            $(
                let sha = ConstBuffer::new();
                #[cfg(feature = $y)]
                let sha = sha.push_slice($x::ENTRY.model_id);
                let sha = sha.push_slice(&const_sha1::sha1(&sha_sum).bytes());
                let sha_sum = ConstBuffer::from_slice(&const_sha1::sha1(&sha).bytes());
            )*
            const_sha1::sha1(&sha_sum).bytes()
        }
    };
}

skills!(
    [base64, "yozuk-skill-base64"],
    [calc, "yozuk-skill-calc"],
    [dice, "yozuk-skill-dice"],
    [digest, "yozuk-skill-digest"],
    [english, "yozuk-skill-english"],
    [lipsum, "yozuk-skill-lipsum"],
    [nanoid, "yozuk-skill-nanoid"],
    [numeric, "yozuk-skill-numeric"],
    [smalltalk, "yozuk-skill-smalltalk"],
    [mediatype, "yozuk-skill-mediatype"],
    [punycode, "yozuk-skill-punycode"],
    [uuid, "yozuk-skill-uuid"],
    [version, "yozuk-skill-version"],
);