//! Variable-length decoding for unsigned integers

use super::{SINGLE_BYTE_MAX, U128_BYTE, U16_BYTE, U32_BYTE, U64_BYTE};
use crate::{
    config::Endianness,
    decode::Reader,
    error::{Error, IntegerType, Result},
};

/// Helper function for invalid varint discriminant errors
#[inline(never)]
#[cold]
fn invalid_varint_discriminant<T>(expected: IntegerType, found: IntegerType) -> Result<T> {
    Err(Error::InvalidIntegerType { expected, found })
}

/// Decode a u16 value from a varint
pub fn varint_decode_u16<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u16> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u16),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(bytes),
                Endianness::Little => u16::from_le_bytes(bytes),
            })
        }
        U32_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U32),
        U64_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U16, IntegerType::Reserved),
    }
}

/// Decode a u32 value from a varint
pub fn varint_decode_u32<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u32> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u32),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(bytes) as u32,
                Endianness::Little => u16::from_le_bytes(bytes) as u32,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u32::from_be_bytes(bytes),
                Endianness::Little => u32::from_le_bytes(bytes),
            })
        }
        U64_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U32, IntegerType::Reserved),
    }
}

/// Decode a u64 value from a varint
pub fn varint_decode_u64<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u64> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u64),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(bytes) as u64,
                Endianness::Little => u16::from_le_bytes(bytes) as u64,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u32::from_be_bytes(bytes) as u64,
                Endianness::Little => u32::from_le_bytes(bytes) as u64,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u64::from_be_bytes(bytes),
                Endianness::Little => u64::from_le_bytes(bytes),
            })
        }
        U128_BYTE => invalid_varint_discriminant(IntegerType::U64, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U64, IntegerType::Reserved),
    }
}

/// Decode a u128 value from a varint
pub fn varint_decode_u128<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u128> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u128),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(bytes) as u128,
                Endianness::Little => u16::from_le_bytes(bytes) as u128,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u32::from_be_bytes(bytes) as u128,
                Endianness::Little => u32::from_le_bytes(bytes) as u128,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u64::from_be_bytes(bytes) as u128,
                Endianness::Little => u64::from_le_bytes(bytes) as u128,
            })
        }
        U128_BYTE => {
            let mut bytes = [0u8; 16];
            reader.read(&mut bytes)?;
            Ok(match endian {
                Endianness::Big => u128::from_be_bytes(bytes),
                Endianness::Little => u128::from_le_bytes(bytes),
            })
        }
        _ => invalid_varint_discriminant(IntegerType::U128, IntegerType::Reserved),
    }
}

/// Decode a usize value from a varint (encoded as u64)
pub fn varint_decode_usize<R: Reader>(reader: &mut R, endian: Endianness) -> Result<usize> {
    let value = varint_decode_u64(reader, endian)?;

    // Check if the value fits in usize
    if value > usize::MAX as u64 {
        return Err(Error::InvalidData {
            message: "usize value too large for this platform",
        });
    }

    Ok(value as usize)
}
