//! Variable-length decoding for signed integers using zigzag decoding
//!
//! Zigzag decoding is the inverse of zigzag encoding:
//! - Even values (n) are decoded as: n / 2
//! - Odd values (n) are decoded as: -(n + 1) / 2
//!
//! This can be simplified to: (n >> 1) ^ -(n & 1)

use super::{varint_decode_u128, varint_decode_u16, varint_decode_u32, varint_decode_u64};
use crate::{config::Endianness, decode::Reader, error::Result};

/// Decode an i16 value from a varint using zigzag decoding
pub fn varint_decode_i16<R: Reader>(reader: &mut R, endian: Endianness) -> Result<i16> {
    let value = varint_decode_u16(reader, endian)?;
    // Zigzag decoding: (n >> 1) ^ -(n & 1)
    Ok(((value >> 1) as i16) ^ (-((value & 1) as i16)))
}

/// Decode an i32 value from a varint using zigzag decoding
pub fn varint_decode_i32<R: Reader>(reader: &mut R, endian: Endianness) -> Result<i32> {
    let value = varint_decode_u32(reader, endian)?;
    // Zigzag decoding: (n >> 1) ^ -(n & 1)
    Ok(((value >> 1) as i32) ^ (-((value & 1) as i32)))
}

/// Decode an i64 value from a varint using zigzag decoding
pub fn varint_decode_i64<R: Reader>(reader: &mut R, endian: Endianness) -> Result<i64> {
    let value = varint_decode_u64(reader, endian)?;
    // Zigzag decoding: (n >> 1) ^ -(n & 1)
    Ok(((value >> 1) as i64) ^ (-((value & 1) as i64)))
}

/// Decode an i128 value from a varint using zigzag decoding
pub fn varint_decode_i128<R: Reader>(reader: &mut R, endian: Endianness) -> Result<i128> {
    let value = varint_decode_u128(reader, endian)?;
    // Zigzag decoding: (n >> 1) ^ -(n & 1)
    Ok(((value >> 1) as i128) ^ (-((value & 1) as i128)))
}

/// Decode an isize value from a varint (encoded as i64)
pub fn varint_decode_isize<R: Reader>(reader: &mut R, endian: Endianness) -> Result<isize> {
    let value = varint_decode_i64(reader, endian)?;

    // Check if the value fits in isize
    if value < isize::MIN as i64 || value > isize::MAX as i64 {
        return Err(crate::error::Error::InvalidData {
            message: "isize value out of range for this platform",
        });
    }

    Ok(value as isize)
}
