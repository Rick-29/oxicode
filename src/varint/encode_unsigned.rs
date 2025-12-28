//! Variable-length encoding for unsigned integers

use super::{SINGLE_BYTE_MAX, U128_BYTE, U16_BYTE, U32_BYTE, U64_BYTE};
use crate::{config::Endianness, encode::Writer, error::Result};

/// Encode a u16 value as a varint
pub fn varint_encode_u16<W: Writer>(writer: &mut W, endian: Endianness, val: u16) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as _ {
        writer.write(&[val as u8])
    } else {
        writer.write(&[U16_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&val.to_be_bytes()),
            Endianness::Little => writer.write(&val.to_le_bytes()),
        }
    }
}

/// Encode a u32 value as a varint
pub fn varint_encode_u32<W: Writer>(writer: &mut W, endian: Endianness, val: u32) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as _ {
        writer.write(&[val as u8])
    } else if val <= u16::MAX as _ {
        writer.write(&[U16_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&(val as u16).to_be_bytes()),
            Endianness::Little => writer.write(&(val as u16).to_le_bytes()),
        }
    } else {
        writer.write(&[U32_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&val.to_be_bytes()),
            Endianness::Little => writer.write(&val.to_le_bytes()),
        }
    }
}

/// Encode a u64 value as a varint
pub fn varint_encode_u64<W: Writer>(writer: &mut W, endian: Endianness, val: u64) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as _ {
        writer.write(&[val as u8])
    } else if val <= u16::MAX as _ {
        writer.write(&[U16_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&(val as u16).to_be_bytes()),
            Endianness::Little => writer.write(&(val as u16).to_le_bytes()),
        }
    } else if val <= u32::MAX as _ {
        writer.write(&[U32_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&(val as u32).to_be_bytes()),
            Endianness::Little => writer.write(&(val as u32).to_le_bytes()),
        }
    } else {
        writer.write(&[U64_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&val.to_be_bytes()),
            Endianness::Little => writer.write(&val.to_le_bytes()),
        }
    }
}

/// Encode a u128 value as a varint
pub fn varint_encode_u128<W: Writer>(writer: &mut W, endian: Endianness, val: u128) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as _ {
        writer.write(&[val as u8])
    } else if val <= u16::MAX as _ {
        writer.write(&[U16_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&(val as u16).to_be_bytes()),
            Endianness::Little => writer.write(&(val as u16).to_le_bytes()),
        }
    } else if val <= u32::MAX as _ {
        writer.write(&[U32_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&(val as u32).to_be_bytes()),
            Endianness::Little => writer.write(&(val as u32).to_le_bytes()),
        }
    } else if val <= u64::MAX as _ {
        writer.write(&[U64_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&(val as u64).to_be_bytes()),
            Endianness::Little => writer.write(&(val as u64).to_le_bytes()),
        }
    } else {
        writer.write(&[U128_BYTE])?;
        match endian {
            Endianness::Big => writer.write(&val.to_be_bytes()),
            Endianness::Little => writer.write(&val.to_le_bytes()),
        }
    }
}

/// Encode a usize value as a varint (encoded as u64)
pub fn varint_encode_usize<W: Writer>(
    writer: &mut W,
    endian: Endianness,
    val: usize,
) -> Result<()> {
    varint_encode_u64(writer, endian, val as u64)
}
