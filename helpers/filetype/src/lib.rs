use chardetng::EncodingDetector;
use file_format::FileFormat;
use mediatype::{media_type, MediaType, MediaTypeBuf};

pub fn is_utf8_text<T>(data: T) -> bool
where
    T: AsRef<[u8]>,
{
    let mut detector = EncodingDetector::new();
    detector.feed(data.as_ref(), true);
    detector.guess(None, true) == encoding_rs::UTF_8
}

pub fn guess_media_type<T>(data: T) -> MediaTypeBuf
where
    T: AsRef<[u8]>,
{
    let format = FileFormat::from_bytes(data.as_ref());
    match format {
        FileFormat::ArbitraryBinaryData if is_utf8_text(data) => media_type!(TEXT / PLAIN).into(),
        FileFormat::ArbitraryBinaryData => media_type!(APPLICATION / OCTET_STREAM).into(),
        _ => format.media_type().parse().unwrap(),
    }
}

pub fn get_file_extension<T>(media_type: &MediaType) -> Option<&'static str> {
    mime2ext::mime2ext(media_type.to_string())
}
