//! Decoding functionality

use crate::error::{Error, Result};

/// Reader trait for decoding values
pub trait Reader {
    /// Read into the provided buffer
    ///
    /// This should completely fill the buffer or return an error
    fn read(&mut self, bytes: &mut [u8]) -> Result<()>;
}

/// Reader implementation that reads from a byte slice
pub struct SliceReader<'a> {
    /// The slice being read from (public for internal use)
    pub(crate) slice: &'a [u8],
}

impl<'a> SliceReader<'a> {
    /// Create a new SliceReader
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice }
    }

    /// Get the remaining bytes in the slice
    pub fn remaining(&self) -> &'a [u8] {
        self.slice
    }
}

impl<'a> Reader for SliceReader<'a> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<()> {
        let len = bytes.len();
        if self.slice.len() < len {
            return Err(Error::UnexpectedEnd {
                additional: len - self.slice.len(),
            });
        }
        bytes.copy_from_slice(&self.slice[..len]);
        self.slice = &self.slice[len..];
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_reader() {
        let data = [0x42, 0x43, 0x44, 0x45];
        let mut reader = SliceReader::new(&data);

        let mut buf = [0u8; 1];
        reader.read(&mut buf).expect("Failed to read");
        assert_eq!(buf[0], 0x42);
        assert_eq!(reader.remaining().len(), 3);

        let mut buf = [0u8; 2];
        reader.read(&mut buf).expect("Failed to read");
        assert_eq!(buf, [0x43, 0x44]);
        assert_eq!(reader.remaining().len(), 1);

        let mut buf = [0u8; 2];
        assert!(reader.read(&mut buf).is_err()); // Not enough bytes
    }
}
