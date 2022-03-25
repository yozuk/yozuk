use mediatype::MediaTypeBuf;
use std::io::{Read, Result};

const HEADER_LENGTH: usize = 1024;

pub struct InputStream {
    reader: Box<dyn Read + Send + Sync>,
    header: Option<Box<[u8]>>,
    offset: usize,
    media_type: MediaTypeBuf,
}

impl InputStream {
    pub fn new<T, M>(reader: T, media_type: M) -> Self
    where
        T: 'static + Read + Send + Sync,
        M: Into<MediaTypeBuf>,
    {
        Self {
            reader: Box::new(reader),
            header: None,
            offset: 0,
            media_type: media_type.into(),
        }
    }

    pub fn read_header(&mut self) -> Result<&[u8]> {
        if self.header.is_none() {
            let mut header = vec![0; HEADER_LENGTH];
            let len = self.reader.read(&mut header)?;
            header.resize(len, 0);
            self.header = Some(header.into_boxed_slice());
        }
        Ok(self.header())
    }

    pub fn header(&self) -> &[u8] {
        if let Some(header) = &self.header {
            header
        } else {
            &[]
        }
    }

    pub fn media_type(&self) -> &MediaTypeBuf {
        &self.media_type
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let header_remain = &self.header()[self.offset..];
        if header_remain.is_empty() {
            self.reader.read(buf)
        } else {
            let len = header_remain.len().min(buf.len());
            buf[..len].copy_from_slice(&header_remain[..len]);
            self.offset += len;
            Ok(len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mediatype::media_type;
    use std::iter;

    struct DataReader {
        data: Vec<u8>,
        offset: usize,
    }

    impl Read for DataReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let data_remain = &self.data[self.offset..];
            let len = data_remain.len().min(buf.len());
            buf[..len].copy_from_slice(&data_remain[..len]);
            self.offset += len;
            Ok(len)
        }
    }

    #[test]
    fn input_stream() {
        let data = iter::repeat(0)
            .enumerate()
            .map(|(_, i)| (i % 0xff) as u8)
            .take(HEADER_LENGTH * 2)
            .collect::<Vec<_>>();

        let mut stream = InputStream::new(
            DataReader {
                data: data.clone(),
                offset: 0,
            },
            media_type!(APPLICATION / OCTET_STREAM),
        );

        assert_eq!(stream.read_header().unwrap(), &data[..HEADER_LENGTH]);
        assert_eq!(stream.header(), &data[..HEADER_LENGTH]);

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, data);
    }

    #[test]
    fn input_stream_small_data() {
        let data = iter::repeat(0)
            .enumerate()
            .map(|(_, i)| (i % 0xff) as u8)
            .take(HEADER_LENGTH / 2)
            .collect::<Vec<_>>();

        let mut stream = InputStream::new(
            DataReader {
                data: data.clone(),
                offset: 0,
            },
            media_type!(APPLICATION / OCTET_STREAM),
        );

        assert_eq!(stream.read_header().unwrap(), &data);
        assert_eq!(stream.header(), &data);

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, data);
    }
}
