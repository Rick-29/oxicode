//! Serde compatibility layer
//!
//! This module provides compatibility with types that implement serde's
//! `Serialize` and `Deserialize` traits but not oxicode's native traits.
//!
//! # Example
//!
//! ```ignore
//! use oxicode::serde::{encode_to_vec, decode_from_slice, Compat};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! let point = Point { x: 1.0, y: 2.0 };
//! let bytes = encode_to_vec(&point, oxicode::config::standard())?;
//! let (decoded, _) = decode_from_slice::<Point>(&bytes, oxicode::config::standard())?;
//! assert_eq!(point, decoded);
//! ```

mod compat;
mod de;
mod ser;

pub use compat::{BorrowCompat, Compat};

use crate::{config::Config, error::Error};

/// Encode a serde Serialize type to a `Vec<u8>`
#[cfg(feature = "alloc")]
pub fn encode_to_vec<T, C>(value: &T, config: C) -> Result<alloc::vec::Vec<u8>, Error>
where
    T: serde::Serialize,
    C: Config,
{
    let writer = crate::enc::VecWriter::new();
    let mut encoder = crate::enc::EncoderImpl::new(writer, config);
    let serializer = ser::Serializer::new(&mut encoder);
    value
        .serialize(serializer)
        .map_err(|e| Error::Custom { message: e.msg })?;
    Ok(encoder.into_writer().into_vec())
}

/// Encode a serde Serialize type into a byte slice
pub fn encode_into_slice<T, C>(value: &T, dst: &mut [u8], config: C) -> Result<usize, Error>
where
    T: serde::Serialize,
    C: Config,
{
    let writer = crate::enc::SliceWriter::new(dst);
    let mut encoder = crate::enc::EncoderImpl::new(writer, config);
    let serializer = ser::Serializer::new(&mut encoder);
    value
        .serialize(serializer)
        .map_err(|e| Error::Custom { message: e.msg })?;
    Ok(encoder.into_writer().bytes_written())
}

/// Decode a serde Deserialize type from a byte slice (borrowed)
pub fn decode_from_slice<'a, T, C>(src: &'a [u8], config: C) -> Result<(T, usize), Error>
where
    T: serde::Deserialize<'a>,
    C: Config,
{
    let reader = crate::de::SliceReader::new(src);
    let mut decoder = crate::de::DecoderImpl::new(reader, config);
    let deserializer = de::Deserializer::new(&mut decoder);
    let value = T::deserialize(deserializer).map_err(|e| Error::Custom { message: e.msg })?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((value, bytes_read))
}

/// Decode an owned serde DeserializeOwned type from a byte slice
pub fn decode_owned_from_slice<T, C>(src: &[u8], config: C) -> Result<(T, usize), Error>
where
    T: serde::de::DeserializeOwned,
    C: Config,
{
    let reader = crate::de::SliceReader::new(src);
    let mut decoder = crate::de::DecoderImpl::new(reader, config);
    let deserializer = de::Deserializer::new(&mut decoder);
    let value = T::deserialize(deserializer).map_err(|e| Error::Custom { message: e.msg })?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((value, bytes_read))
}

/// Decode a serde Deserialize type from a std::io::Read
#[cfg(feature = "std")]
pub fn decode_from_std_read<T, R, C>(reader: R, config: C) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
    C: Config,
{
    let io_reader = crate::de::IoReader::new(reader);
    let mut decoder = crate::de::DecoderImpl::new(io_reader, config);
    let deserializer = de::Deserializer::new(&mut decoder);
    T::deserialize(deserializer).map_err(|e| Error::Custom { message: e.msg })
}
