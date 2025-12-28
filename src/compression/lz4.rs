//! LZ4 compression implementation.
//!
//! LZ4 is an extremely fast compression algorithm with good compression ratios.
//! It's ideal for real-time applications where decompression speed is critical.
//!
//! Decompression speed: ~4 GB/s on modern hardware.

use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Compress data using LZ4.
#[cfg(feature = "alloc")]
pub fn compress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    Ok(lz4_flex::compress_prepend_size(data))
}

/// Decompress LZ4-compressed data.
#[cfg(feature = "alloc")]
pub fn decompress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    lz4_flex::decompress_size_prepended(data).map_err(|_| Error::InvalidData {
        message: "LZ4 decompression error",
    })
}

/// Compress data into a pre-allocated buffer.
/// Returns the number of bytes written.
#[allow(dead_code)]
pub fn compress_into(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    // lz4_flex requires prepending the size, so we do it manually
    if dst.len() < 4 {
        return Err(Error::UnexpectedEnd {
            additional: 4 - dst.len(),
        });
    }

    let size_bytes = (src.len() as u32).to_le_bytes();
    dst[..4].copy_from_slice(&size_bytes);

    let max_compressed = lz4_flex::block::get_maximum_output_size(src.len());
    if dst.len() < 4 + max_compressed {
        // Try compression and see if it fits
        #[cfg(feature = "alloc")]
        {
            let compressed = lz4_flex::compress(src);
            if dst.len() < 4 + compressed.len() {
                return Err(Error::UnexpectedEnd {
                    additional: 4 + compressed.len() - dst.len(),
                });
            }
            dst[4..4 + compressed.len()].copy_from_slice(&compressed);
            return Ok(4 + compressed.len());
        }
        #[cfg(not(feature = "alloc"))]
        {
            return Err(Error::UnexpectedEnd {
                additional: 4 + max_compressed - dst.len(),
            });
        }
    }

    let compressed_size =
        lz4_flex::compress_into(src, &mut dst[4..]).map_err(|_| Error::Custom {
            message: "LZ4 compression failed",
        })?;

    Ok(4 + compressed_size)
}

/// Get the maximum compressed size for a given input size.
#[allow(dead_code)]
pub fn max_compressed_size(input_size: usize) -> usize {
    // 4 bytes for size prefix + maximum compressed size
    4 + lz4_flex::block::get_maximum_output_size(input_size)
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
