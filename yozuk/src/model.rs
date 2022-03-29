use super::{skill, FeatureLabeler, Tagger};
use anyhow::{bail, Result};
use bytes::Bytes;
use std::{
    io::{Cursor, Write},
    mem,
    ops::Range,
};
use yozuk_sdk::prelude::*;

pub struct ModelSet {
    pub(crate) data: Bytes,
    pub(crate) keys: Vec<String>,
    pub(crate) ranges: Vec<Range<usize>>,
    pub(crate) header_len: usize,
}

impl ModelSet {
    pub fn from_data<T: Into<Bytes>>(data: T) -> Result<Self> {
        let data = data.into();

        let digest = skill::skills_digest();
        let offset = data.len().saturating_sub(mem::size_of_val(&digest));

        if data[offset..] != digest[..] {
            bail!("Model digest mismatched");
        }

        let mut cursor = Cursor::new(&data);
        let keys = bincode::deserialize_from(&mut cursor)?;
        let ranges = bincode::deserialize_from(&mut cursor)?;
        let header_len = cursor.position() as _;
        Ok(Self {
            data,
            keys,
            ranges,
            header_len,
        })
    }

    pub fn get(&self, key: &str) -> Option<ModelEntry> {
        self.get_index(key)
            .map(|index| self.ranges[index].clone())
            .filter(|range| !range.is_empty())
            .map(|range| {
                self.data
                    .slice(range.start + self.header_len..range.end + self.header_len)
            })
            .map(ModelEntry::new)
    }

    pub fn get_index(&self, key: &str) -> Option<usize> {
        self.keys
            .binary_search_by(|entry| entry.as_str().cmp(key))
            .ok()
    }

    pub fn write<W: Write>(&self, mut dst: W) -> bincode::Result<()> {
        bincode::serialize_into(&mut dst, &self.keys)?;
        bincode::serialize_into(&mut dst, &self.ranges)?;
        dst.write_all(&self.data)?;
        dst.write_all(&skill::skills_digest())?;
        Ok(())
    }
}

pub struct ModelEntry {
    tagger: Tagger,
}

impl ModelEntry {
    pub fn new(data: Bytes) -> Self {
        Self {
            tagger: Tagger::new(data),
        }
    }

    pub fn tag<I, T>(&self, features: I) -> Vec<String>
    where
        I: AsRef<[T]>,
        T: AsRef<[Feature]>,
    {
        self.tagger.tag(features)
    }

    pub fn tag_tokens(&self, labeler: &FeatureLabeler, tokens: &[Token]) -> Vec<Token> {
        let features = labeler.label_features(tokens);
        let tags = self.tag(&features);

        tokens
            .iter()
            .zip(tags.into_iter())
            .map(|(token, tag)| Token {
                tag: if token.tag.is_empty() {
                    tag
                } else {
                    token.tag.clone()
                },
                ..token.clone()
            })
            .collect()
    }
}
