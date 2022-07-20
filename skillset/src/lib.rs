#![forbid(unsafe_code)]
#![deny(clippy::all)]

use yozuk_sdk::prelude::*;

const MODEL_ROOT_ID: &[u8] = b"wX1dpA9hksOooO4DGfMNp";

#[macro_export]
macro_rules! skills {
    ( $([ $x:ident, $y:literal ],)* ) => {
        $(
            #[cfg(feature = $y)]
            mod $x;
        )*

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
    [lipsum, "yozuk-skill-lipsum"],
    [nanoid, "yozuk-skill-nanoid"],
    [numeric, "yozuk-skill-numeric"],
    [smalltalk, "yozuk-skill-smalltalk"],
    [mediatype, "yozuk-skill-mediatype"],
    [punycode, "yozuk-skill-punycode"],
    [uuid, "yozuk-skill-uuid"],
    [version, "yozuk-skill-version"],
    [color, "yozuk-skill-color"],
    [unit, "yozuk-skill-unit"],
    [geo, "yozuk-skill-geo"],
    [password, "yozuk-skill-password"],
    [urlencode, "yozuk-skill-urlencode"],
    [bech32, "yozuk-skill-bech32"],
    [prime, "yozuk-skill-prime"],
    [jwt, "yozuk-skill-jwt"],
    [blurhash, "yozuk-skill-blurhash"],
    [qrcode, "yozuk-skill-qrcode"],
    [consts, "yozuk-skill-consts"],
    [time, "yozuk-skill-time"],
    [ip, "yozuk-skill-ip"],
    [msgpack, "yozuk-skill-msgpack"],
    [unicode, "yozuk-skill-unicode"],
);
