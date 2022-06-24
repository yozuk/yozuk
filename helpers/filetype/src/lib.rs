use chardetng::EncodingDetector;

pub fn is_utf8_text<T>(data: T) -> bool
where
    T: AsRef<[u8]>,
{
    let mut detector = EncodingDetector::new();
    detector.feed(data.as_ref(), true);
    detector.guess(None, true) == encoding_rs::UTF_8
}
