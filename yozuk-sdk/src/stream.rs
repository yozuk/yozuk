use std::io::{Read, Result};

const HEADER_LENGTH: usize = 1024;

pub struct InputStream {
    reader: Box<dyn Read + Send>,
    header: Box<[u8]>,
    offset: usize,
}

impl InputStream {
    pub fn new<T>(mut reader: T) -> Result<Self>
    where
        T: 'static + Read + Send,
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
            buf[..len].copy_from_slice(header_remain);
            self.offset += len;
            Ok(len)
        }
    }
}
