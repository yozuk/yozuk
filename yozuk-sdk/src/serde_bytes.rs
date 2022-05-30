use bytes::Bytes;
use std::borrow::Cow;
use std::str;

pub fn serialize_bytes<S>(data: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serde::Serialize::serialize(&base64::encode(data), serializer)
}

pub fn serialize_bytes_vec<S>(vec: &[Bytes], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
    for data in vec {
        seq.serialize_element(&base64::encode(data))?;
    }
    seq.end()
}

pub fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let data: Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
    Ok(Bytes::from(
        base64::decode(data.as_ref()).map_err(Error::custom)?,
    ))
}

pub fn deserialize_bytes_vec<'de, D>(deserializer: D) -> Result<Vec<Bytes>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let vec: Cow<'de, [Cow<'de, str>]> = serde::Deserialize::deserialize(deserializer)?;
    let mut bytes: Vec<Bytes> = Vec::with_capacity(vec.len());
    for data in vec.as_ref() {
        bytes.push(Bytes::from(
            base64::decode(data.as_ref()).map_err(Error::custom)?,
        ));
    }
    Ok(bytes)
}
