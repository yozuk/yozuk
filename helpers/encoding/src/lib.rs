#![forbid(unsafe_code)]
#![deny(clippy::all)]

use yozuk_sdk::encoding::*;
use yozuk_sdk::prelude::*;

#[derive(Debug)]
pub struct EncodingPreprocessor {
    encodings: Vec<RawEncoding>,
}

impl EncodingPreprocessor {
    pub fn new<I>(encodings: I) -> Self
    where
        I: IntoIterator<Item = RawEncoding>,
    {
        Self {
            encodings: encodings.into_iter().collect(),
        }
    }
}

impl Preprocessor for EncodingPreprocessor {
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token> {
        input
            .into_iter()
            .map(|t| {
                let bytes = t.as_str().as_bytes();
                let decoded = self.encodings.iter().find_map(|enc| match enc {
                    RawEncoding::Base64 if is_like_base64(bytes) => {
                        base64::decode_config(t.as_str(), base64::STANDARD)
                            .ok()
                            .map(|data| (enc, data))
                    }
                    RawEncoding::Base64Url if is_like_base64(bytes) => {
                        base64::decode_config(t.as_str(), base64::URL_SAFE)
                            .ok()
                            .map(|data| (enc, data))
                    }
                    RawEncoding::Hex if is_like_hex(bytes) => {
                        hex::decode(t.as_str()).ok().map(|data| (enc, data))
                    }
                    _ => None,
                });
                if let Some((enc, data)) = decoded {
                    Token {
                        data: data.into(),
                        raw_encoding: Some(*enc),
                        ..t
                    }
                } else {
                    t
                }
            })
            .collect()
    }
}

pub fn is_like_base64(data: &[u8]) -> bool {
    let invalid = data.iter().any(|&c| {
        !(b'a'..=b'z').contains(&c)
            && !(b'A'..=b'Z').contains(&c)
            && !(b'0'..=b'9').contains(&c)
            && c != b'+'
            && c != b'/'
            && c != b'-'
            && c != b'_'
            && c != b'='
    });
    if invalid {
        return false;
    }
    let mut score = 0;
    score += data.iter().any(|c| (b'a'..=b'f').contains(c)) as u8;
    score += data.iter().any(|c| (b'A'..=b'F').contains(c)) as u8;
    score += data.iter().any(|c| (b'g'..=b'z').contains(c)) as u8;
    score += data.iter().any(|c| (b'G'..=b'Z').contains(c)) as u8;
    score += data.iter().any(|c| (b'0'..=b'9').contains(c)) as u8;
    score += data
        .iter()
        .any(|&c| c == b'+' || c == b'/' || c == b'-' || c == b'_' || c == b'=') as u8;
    score >= 4
}

pub fn is_like_hex(data: &[u8]) -> bool {
    if data.len() < 16 {
        return false;
    }
    let mut upper = 0;
    let mut lower = 0;
    let mut number = false;
    for c in data {
        match c {
            b'a'..=b'f' => lower = 1,
            b'A'..=b'F' => upper = 1,
            b'0'..=b'9' => number = true,
            _ => return false,
        }
    }
    (lower + upper) == 1 && number
}
