use std::io::{Read, Result};

const HEADER_LENGTH: usize = 1024;

pub struct InputStream {
    reader: Box<dyn Read + Send + Sync>,
    header: Box<[u8]>,
    offset: usize,
}

impl InputStream {
    pub fn new<T>(mut reader: T) -> Result<Self>
    where
        T: 'static + Read + Send + Sync,
    {
        let mut header = vec![0; HEADER_LENGTH];
        let len = reader.read(&mut header)?;
        header.resize(len, 0);
        Ok(Self {
            reader: Box::new(reader),
            header: header.into_boxed_slice(),
            offset: 0,
        })
    }

    pub fn header(&self) -> &[u8] {
        &self.header
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let header_remain = &self.header[self.offset..];
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

        let mut stream = InputStream::new(DataReader {
            data: data.clone(),
            offset: 0,
        })
        .unwrap();

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

        let mut stream = InputStream::new(DataReader {
            data: data.clone(),
            offset: 0,
        })
        .unwrap();

        assert_eq!(stream.header(), &data);

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, data);
    }
}
