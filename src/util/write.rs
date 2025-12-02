use std::fmt::{Error, Result, Write};

pub struct SliceWriter<'s> {
    slice: &'s mut [u8],
    len: usize,
}

impl<'s> SliceWriter<'s> {
    pub fn new(slice: &'s mut [u8]) -> Self {
        Self { slice, len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.slice.len()
    }

    pub fn remaining(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn into_str(self) -> &'s str {
        std::str::from_utf8(&self.slice[0..self.len]).unwrap()
    }
}

impl Write for SliceWriter<'_> {
    fn write_str(&mut self, s: &str) -> Result {
        let bytes = s.as_bytes();
        if bytes.len() > self.remaining() {
            return Err(Error);
        }

        let end = self.len() + bytes.len();
        self.slice[self.len..end].copy_from_slice(bytes);
        self.len = end;
        Ok(())
    }
}
