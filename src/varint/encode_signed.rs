//! Variable-length encoding for signed integers using zigzag encoding
//!
//! Zigzag encoding maps signed integers to unsigned integers in a way that
//! small absolute values result in small encoded values:
//! - 0 -> 0, -1 -> 1, 1 -> 2, -2 -> 3, 2 -> 4, etc.
//!
//! For negative values, we use: `!n * 2 + 1 = -2n - 1`
//! For positive values, we use: `n * 2`

use super::{varint_encode_u128, varint_encode_u16, varint_encode_u32, varint_encode_u64};
use crate::{config::Endianness, encode::Writer, error::Result};

/// Encode an i16 value as a varint using zigzag encoding
pub fn varint_encode_i16<W: Writer>(writer: &mut W, endian: Endianness, val: i16) -> Result<()> {
    varint_encode_u16(
        writer,
        endian,
        if val < 0 {
            // Zigzag encoding for negative values
            // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 1
            !(val as u16) * 2 + 1
        } else {
            // Positive values are simply doubled
            (val as u16) * 2
        },
    )
}

/// Encode an i32 value as a varint using zigzag encoding
pub fn varint_encode_i32<W: Writer>(writer: &mut W, endian: Endianness, val: i32) -> Result<()> {
    varint_encode_u32(
        writer,
        endian,
        if val < 0 {
            // Zigzag encoding for negative values
            !(val as u32) * 2 + 1
        } else {
            // Positive values are simply doubled
            (val as u32) * 2
        },
    )
}

/// Encode an i64 value as a varint using zigzag encoding
pub fn varint_encode_i64<W: Writer>(writer: &mut W, endian: Endianness, val: i64) -> Result<()> {
    varint_encode_u64(
        writer,
        endian,
        if val < 0 {
            // Zigzag encoding for negative values
            !(val as u64) * 2 + 1
        } else {
            // Positive values are simply doubled
            (val as u64) * 2
        },
    )
}

/// Encode an i128 value as a varint using zigzag encoding
pub fn varint_encode_i128<W: Writer>(writer: &mut W, endian: Endianness, val: i128) -> Result<()> {
    varint_encode_u128(
        writer,
        endian,
        if val < 0 {
            // Zigzag encoding for negative values
            !(val as u128) * 2 + 1
        } else {
            // Positive values are simply doubled
            (val as u128) * 2
        },
    )
}

/// Encode an isize value as a varint (encoded as i64)
pub fn varint_encode_isize<W: Writer>(
    writer: &mut W,
    endian: Endianness,
    val: isize,
) -> Result<()> {
    varint_encode_i64(writer, endian, val as i64)
}
