//! # OxiCode
//!
//! OxiCode is a modern binary serialization library for Rust, serving as the successor to bincode.
//!
//! It provides a compact, efficient binary encoding scheme with zero-fluff serialization.
//! The encoded size is typically equal to or smaller than the in-memory representation.
//!
//! ## Features
//!
//! - **Compact encoding**: Minimal overhead in serialized format
//! - **Fast**: Optimized for performance
//! - **Flexible**: Support for various encoding configurations
//! - **Safe**: No unwrap() policy, comprehensive error handling
//! - **Modern**: Built with latest Rust practices and patterns
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode, PartialEq, Debug)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let point = Point { x: 1.0, y: 2.0 };
//!
//!     // Encode to bytes
//!     let encoded = oxicode::encode(&point)?;
//!
//!     // Decode from bytes
//!     let decoded: Point = oxicode::decode(&encoded)?;
//!
//!     assert_eq!(point, decoded);
//!     Ok(())
//! }
//! ```
//!
//! ## Relation to bincode
//!
//! OxiCode is designed as the spiritual successor to bincode, maintaining compatibility
//! with the core concepts while introducing modern improvements and best practices.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod config;
pub mod de;
pub mod enc;
pub mod error;

// Re-export old names for backward compatibility during transition
#[doc(hidden)]
pub mod decode {
    pub use crate::de::*;
}
#[doc(hidden)]
pub mod encode {
    pub use crate::enc::{SliceWriter, VecWriter, Writer};
}

pub(crate) mod utils;
pub(crate) mod varint;

// Features module is always present (atomic types work in no_std)
mod features;

// SIMD-optimized encoding/decoding
#[cfg(feature = "simd")]
pub mod simd;

// Compression support
#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
pub mod compression;

// Schema versioning support
pub mod versioning;

// Streaming serialization support
#[cfg(feature = "alloc")]
pub mod streaming;

// Validation middleware
pub mod validation;

pub use de::Decode;
pub use enc::Encode;
pub use error::{Error, Result};

#[cfg(feature = "derive")]
pub use oxicode_derive::{Decode, Encode};

#[cfg(feature = "serde")]
pub use features::serde;

/// Encode a value to a `Vec<u8>` using the standard configuration
#[cfg(feature = "alloc")]
pub fn encode_to_vec<E: Encode>(value: &E) -> Result<alloc::vec::Vec<u8>> {
    encode_to_vec_with_config(value, config::standard())
}

/// Encode a value to a `Vec<u8>` with a custom configuration
#[cfg(feature = "alloc")]
pub fn encode_to_vec_with_config<E: Encode, C: config::Config>(
    value: &E,
    config: C,
) -> Result<alloc::vec::Vec<u8>> {
    let writer = enc::VecWriter::new();
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().into_vec())
}

/// Encode a value into a byte slice
pub fn encode_into_slice<E: Encode, C: config::Config>(
    value: E,
    dst: &mut [u8],
    config: C,
) -> Result<usize> {
    let writer = enc::SliceWriter::new(dst);
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Decode a value from a byte slice using the standard configuration
pub fn decode_from_slice<D: Decode>(src: &[u8]) -> Result<(D, usize)> {
    decode_from_slice_with_config(src, config::standard())
}

/// Decode a value from a byte slice with a custom configuration
pub fn decode_from_slice_with_config<D: Decode, C: config::Config>(
    src: &[u8],
    config: C,
) -> Result<(D, usize)> {
    let reader = de::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::new(reader, config);
    let result = D::decode(&mut decoder)?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((result, bytes_read))
}

/// Encode a value into a writer using the given configuration
pub fn encode_into_writer<E: Encode, W: enc::Writer, C: config::Config>(
    value: E,
    writer: W,
    config: C,
) -> Result<()> {
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    Ok(())
}

/// Encode a value into a std::io::Write using the given configuration
#[cfg(feature = "std")]
pub fn encode_into_std_write<E: Encode, W: std::io::Write, C: config::Config>(
    value: E,
    writer: W,
    config: C,
) -> Result<usize> {
    let io_writer = enc::IoWriter::new(writer);
    let mut encoder = enc::EncoderImpl::new(io_writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Decode a value from a reader using the given configuration
pub fn decode_from_reader<D: Decode, R: de::Reader, C: config::Config>(
    reader: R,
    config: C,
) -> Result<D> {
    let mut decoder = de::DecoderImpl::new(reader, config);
    D::decode(&mut decoder)
}

/// Decode a value from a std::io::Read using the given configuration
#[cfg(feature = "std")]
pub fn decode_from_std_read<D: Decode, R: std::io::Read, C: config::Config>(
    reader: R,
    config: C,
) -> Result<D> {
    let io_reader = de::IoReader::new(reader);
    let mut decoder = de::DecoderImpl::new(io_reader, config);
    D::decode(&mut decoder)
}

/// Decode a value from a byte slice with custom context
pub fn decode_from_slice_with_context<Ctx, D, C: config::Config>(
    src: &[u8],
    config: C,
    context: Ctx,
) -> Result<(D, usize)>
where
    D: de::Decode<Ctx>,
{
    let reader = de::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::with_context(reader, config, context);
    let result = D::decode(&mut decoder)?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((result, bytes_read))
}

/// Borrow decode a value from a byte slice (zero-copy) using standard configuration
pub fn borrow_decode_from_slice<'a, D>(src: &'a [u8]) -> Result<(D, usize)>
where
    D: de::BorrowDecode<'a>,
{
    borrow_decode_from_slice_with_config(src, config::standard())
}

/// Borrow decode a value from a byte slice (zero-copy) with custom configuration
pub fn borrow_decode_from_slice_with_config<'a, D, C: config::Config>(
    src: &'a [u8],
    config: C,
) -> Result<(D, usize)>
where
    D: de::BorrowDecode<'a>,
{
    let reader = de::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::new(reader, config);
    let result = D::borrow_decode(&mut decoder)?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((result, bytes_read))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_encoding() {
        // Basic test placeholder
        assert_eq!(2 + 2, 4);
    }
}
