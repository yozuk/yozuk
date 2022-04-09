use bytes::Bytes;
use serde_derive::{Deserialize, Serialize};
use std::str;

pub fn serialize_bytes<S>(data: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Ok(s) = str::from_utf8(data) {
        serde::Serialize::serialize(&Data::String(s), serializer)
    } else {
        serde::Serialize::serialize(
            &Data::Base64(Base64Data {
                base64: &base64::encode(data),
            }),
            serializer,
        )
    }
}

pub fn serialize_bytes_vec<S>(vec: &[Bytes], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
    for data in vec {
        if let Ok(s) = str::from_utf8(data) {
            seq.serialize_element(&Data::String(s))?;
        } else {
            seq.serialize_element(&Data::Base64(Base64Data {
                base64: &base64::encode(data),
            }))?;
        }
    }
    seq.end()
}

pub fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let data: Data = serde::Deserialize::deserialize(deserializer)?;
    Ok(match data {
        Data::String(str) => Bytes::from(str.to_string()),
        Data::Base64(data) => Bytes::from(base64::decode(data.base64).map_err(Error::custom)?),
        Data::Bytes(data) => data,
    })
}

pub fn deserialize_bytes_vec<'de, D>(deserializer: D) -> Result<Vec<Bytes>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let vec: Vec<Data> = serde::Deserialize::deserialize(deserializer)?;
    let mut bytes: Vec<Bytes> = Vec::with_capacity(vec.len());
    for data in vec {
        bytes.push(match data {
            Data::String(str) => Bytes::from(str.to_string()),
            Data::Base64(data) => Bytes::from(base64::decode(data.base64).map_err(Error::custom)?),
            Data::Bytes(data) => data,
        });
    }
    Ok(bytes)
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Data<'a> {
    String(&'a str),
    Base64(Base64Data<'a>),
    Bytes(Bytes),
}

#[derive(Debug, Serialize, Deserialize)]
struct Base64Data<'a> {
    base64: &'a str,
}
