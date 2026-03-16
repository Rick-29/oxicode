//! LZ4 compression implementation using oxiarc-lz4 (pure Rust).
//!
//! LZ4 is an extremely fast compression algorithm with good compression ratios.
//! It's ideal for real-time applications where decompression speed is critical.
//!
//! Decompression speed: ~4 GB/s on modern hardware.

use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Default maximum decompression output size (256 MB).
/// This prevents decompression bombs from consuming excessive memory.
const MAX_DECOMPRESSED_SIZE: usize = 256 * 1024 * 1024;

/// Compress data using LZ4 frame format.
#[cfg(feature = "alloc")]
pub fn compress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    oxiarc_lz4::compress(data).map_err(|_| Error::Custom {
        message: "LZ4 compression error",
    })
}

/// Decompress LZ4-compressed data (frame format).
#[cfg(feature = "alloc")]
pub fn decompress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    oxiarc_lz4::decompress(data, MAX_DECOMPRESSED_SIZE).map_err(|_| Error::InvalidData {
        message: "LZ4 decompression error",
    })
}

/// Compress data into a pre-allocated buffer.
/// Returns the number of bytes written.
#[allow(dead_code)]
#[cfg(feature = "alloc")]
pub fn compress_into(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    let compressed = oxiarc_lz4::compress(src).map_err(|_| Error::Custom {
        message: "LZ4 compression failed",
    })?;

    if dst.len() < compressed.len() {
        return Err(Error::UnexpectedEnd {
            additional: compressed.len() - dst.len(),
        });
    }

    dst[..compressed.len()].copy_from_slice(&compressed);
    Ok(compressed.len())
}

/// Get the maximum compressed size for a given input size.
/// LZ4 worst case is approximately input_size + (input_size / 255) + 16 + frame overhead.
#[allow(dead_code)]
pub fn max_compressed_size(input_size: usize) -> usize {
    // LZ4 frame overhead (header + end mark + optional checksum) + worst-case block expansion
    // Conservative estimate matching LZ4 spec
    input_size + (input_size / 255) + 32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_compress_decompress() {
        let data = b"Hello, World! This is a test of LZ4 compression.";
        let compressed = compress(data).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_empty_data() {
        let data: &[u8] = b"";
        let compressed = compress(data).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data, decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_large_data() {
        // Create repetitive data that compresses well
        let data: alloc::vec::Vec<u8> = (0..100000).map(|i| (i % 256) as u8).collect();
        let compressed = compress(&data).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data, decompressed);

        // Should have actually compressed
        assert!(compressed.len() < data.len());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_incompressible_data() {
        // Random-ish data that doesn't compress well
        let data: alloc::vec::Vec<u8> = (0..1000).map(|i| ((i * 17 + 31) % 256) as u8).collect();
        let compressed = compress(&data).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_max_compressed_size() {
        let size = max_compressed_size(1000);
        // Should be larger than input (overhead + worst case)
        assert!(size > 1000);
    }
}
