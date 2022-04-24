use anyhow::Result;
use bytes::Bytes;
use std::{
    io::{Cursor, Write},
    ops::Range,
};

pub struct ModelSet {
    data: Bytes,
    keys: Vec<(String, Range<usize>)>,
    header_len: usize,
}

impl ModelSet {
    pub fn new<D, I>(data: D, keys: I) -> Self
    where
        D: Into<Bytes>,
        I: IntoIterator<Item = (String, Range<usize>)>,
    {
        Self {
            data: data.into(),
            keys: keys.into_iter().collect(),
            header_len: 0,
        }
    }

    pub fn from_data<T: Into<Bytes>>(data: T) -> Result<Self> {
        let data = data.into();

        let mut cursor = Cursor::new(&data);
        let keys = bincode::deserialize_from(&mut cursor)?;
        let header_len = cursor.position() as _;
        Ok(Self {
            data,
            keys,
            header_len,
        })
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        self.get_index(key)
            .map(|index| self.keys[index].1.clone())
            .filter(|range| !range.is_empty())
            .map(|range| {
                self.data
                    .slice(range.start + self.header_len..range.end + self.header_len)
            })
    }

    pub fn get_index(&self, key: &str) -> Option<usize> {
        self.keys
            .binary_search_by(|entry| entry.0.as_str().cmp(key))
            .ok()
    }

    pub fn write<W: Write>(&self, mut dst: W) -> bincode::Result<()> {
        bincode::serialize_into(&mut dst, &self.keys)?;
        dst.write_all(&self.data)?;
        Ok(())
    }
}
