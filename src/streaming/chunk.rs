//! Chunk format for streaming data.

use crate::{Error, Result};

/// Magic bytes for chunk header: "OXIS" (OXIcode Stream)
pub const CHUNK_MAGIC: [u8; 4] = [0x4F, 0x58, 0x49, 0x53];

/// Chunk type indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ChunkType {
    /// Data chunk containing encoded items.
    Data = 0,
    /// Final chunk indicating end of stream.
    End = 1,
    /// Metadata chunk (version info, etc.).
    Metadata = 2,
}

impl ChunkType {
    /// Parse from byte.
    fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(ChunkType::Data),
            1 => Some(ChunkType::End),
            2 => Some(ChunkType::Metadata),
            _ => None,
        }
    }
}

/// Header for each chunk in the stream.
///
/// Format (13 bytes):
/// - Bytes 0-3: Magic "OXIS"
/// - Byte 4: Chunk type
/// - Bytes 5-8: Payload length (u32, little-endian)
/// - Bytes 9-12: Item count in this chunk (u32, little-endian)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkHeader {
    /// Type of this chunk.
    pub chunk_type: ChunkType,

    /// Length of payload in bytes.
    pub payload_len: u32,

    /// Number of items in this chunk.
    pub item_count: u32,
}

impl ChunkHeader {
    /// Header size in bytes.
    pub const SIZE: usize = 13;

    /// Create a new data chunk header.
    #[inline]
    pub fn data(payload_len: u32, item_count: u32) -> Self {
        Self {
            chunk_type: ChunkType::Data,
            payload_len,
            item_count,
        }
    }

    /// Create an end-of-stream chunk header.
    #[inline]
    pub fn end() -> Self {
        Self {
            chunk_type: ChunkType::End,
            payload_len: 0,
            item_count: 0,
        }
    }

    /// Create a metadata chunk header.
    #[inline]
    pub fn metadata(payload_len: u32) -> Self {
        Self {
            chunk_type: ChunkType::Metadata,
            payload_len,
            item_count: 0,
        }
    }

    /// Check if this is an end chunk.
    #[inline]
    pub fn is_end(&self) -> bool {
        matches!(self.chunk_type, ChunkType::End)
    }

    /// Check if this is a data chunk.
    #[inline]
    pub fn is_data(&self) -> bool {
        matches!(self.chunk_type, ChunkType::Data)
    }

    /// Convert to bytes.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];

        bytes[0..4].copy_from_slice(&CHUNK_MAGIC);
        bytes[4] = self.chunk_type as u8;
        bytes[5..9].copy_from_slice(&self.payload_len.to_le_bytes());
        bytes[9..13].copy_from_slice(&self.item_count.to_le_bytes());

        bytes
    }

    /// Parse from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < Self::SIZE {
            return Err(Error::UnexpectedEnd {
                additional: Self::SIZE - data.len(),
            });
        }

        // Check magic
        if data[0..4] != CHUNK_MAGIC {
            return Err(Error::InvalidData {
                message: "invalid chunk magic",
            });
        }

        // Parse chunk type
        let chunk_type = ChunkType::from_byte(data[4]).ok_or(Error::InvalidData {
            message: "invalid chunk type",
        })?;

        // Parse lengths
        let payload_len = u32::from_le_bytes([data[5], data[6], data[7], data[8]]);
        let item_count = u32::from_le_bytes([data[9], data[10], data[11], data[12]]);

        Ok(Self {
            chunk_type,
            payload_len,
            item_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_header_roundtrip() {
        let header = ChunkHeader::data(1024, 10);
        let bytes = header.to_bytes();
        let parsed = ChunkHeader::from_bytes(&bytes).expect("parse failed");

        assert_eq!(header, parsed);
    }

    #[test]
    fn test_end_chunk() {
        let header = ChunkHeader::end();
        assert!(header.is_end());
        assert!(!header.is_data());

        let bytes = header.to_bytes();
        let parsed = ChunkHeader::from_bytes(&bytes).expect("parse failed");
        assert!(parsed.is_end());
    }

    #[test]
    fn test_data_chunk() {
        let header = ChunkHeader::data(500, 5);
        assert!(header.is_data());
        assert!(!header.is_end());
        assert_eq!(header.payload_len, 500);
        assert_eq!(header.item_count, 5);
    }

    #[test]
    fn test_metadata_chunk() {
        let header = ChunkHeader::metadata(256);
        assert!(!header.is_data());
        assert!(!header.is_end());
        assert_eq!(header.payload_len, 256);
    }

    #[test]
    fn test_invalid_magic() {
        let mut bytes = ChunkHeader::data(100, 1).to_bytes();
        bytes[0] = 0xFF; // Corrupt magic

        let result = ChunkHeader::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_header_size() {
        assert_eq!(ChunkHeader::SIZE, 13);
    }
}
