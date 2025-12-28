//! Encoding functionality

use crate::error::Result;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Writer trait for encoding values
pub trait Writer {
    /// Write bytes to the writer
    ///
    /// This should write all bytes or return an error
    fn write(&mut self, bytes: &[u8]) -> Result<()>;
}

/// Writer implementation that writes to a Vec<u8>
#[cfg(feature = "alloc")]
pub struct VecWriter {
    buffer: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl VecWriter {
    /// Create a new VecWriter
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    /// Create a new VecWriter with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// Get the inner Vec<u8>
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    /// Get a reference to the inner buffer
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }
}

#[cfg(feature = "alloc")]
impl Default for VecWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl Writer for VecWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }
}

/// Writer implementation that writes to a byte slice
pub struct SliceWriter<'a> {
    slice: &'a mut [u8],
    index: usize,
}

impl<'a> SliceWriter<'a> {
    /// Create a new SliceWriter
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self { slice, index: 0 }
    }

    /// Get the number of bytes written
    pub fn bytes_written(&self) -> usize {
        self.index
    }
}

impl<'a> Writer for SliceWriter<'a> {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        let len = bytes.len();
        if self.index + len > self.slice.len() {
            return Err(crate::error::Error::UnexpectedEnd { additional: len });
        }
        self.slice[self.index..self.index + len].copy_from_slice(bytes);
        self.index += len;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_vec_writer() {
        let mut writer = VecWriter::new();
        writer.write(&[0x42]).expect("Failed to write");
        assert_eq!(writer.as_slice(), &[0x42]);

        writer.write(&[0x43, 0x44]).expect("Failed to write");
        assert_eq!(writer.as_slice(), &[0x42, 0x43, 0x44]);
    }

    #[test]
    fn test_slice_writer() {
        let mut buffer = [0u8; 10];
        {
            let mut writer = SliceWriter::new(&mut buffer);

            writer.write(&[0x42]).expect("Failed to write");
            assert_eq!(writer.bytes_written(), 1);

            writer.write(&[0x43, 0x44]).expect("Failed to write");
            assert_eq!(writer.bytes_written(), 3);

            // Fill the rest
            writer
                .write(&[0xFF; 7])
                .expect("Failed to write remaining");
            assert_eq!(writer.bytes_written(), 10);

            // Should fail - buffer full
            assert!(writer.write(&[0x00]).is_err());
        }

        // Check the buffer after writer is dropped
        assert_eq!(buffer[0], 0x42);
        assert_eq!(&buffer[..3], &[0x42, 0x43, 0x44]);
    }
}
