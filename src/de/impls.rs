//! Decode implementations for primitive and standard types

use super::{read::Reader, Decode, Decoder};
use crate::{
    config::{Endianness, IntEncoding, InternalEndianConfig, InternalIntEncodingConfig},
    error::Error,
};
use core::marker::PhantomData;

// ===== Unit and PhantomData =====

impl Decode for () {
    fn decode<D: Decoder<Context = ()>>(_: &mut D) -> Result<Self, Error> {
        Ok(())
    }
}

impl<T> Decode for PhantomData<T> {
    fn decode<D: Decoder<Context = ()>>(_: &mut D) -> Result<Self, Error> {
        Ok(PhantomData)
    }
}

// ===== Boolean =====

impl Decode for bool {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match u8::decode(decoder)? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(Error::InvalidBooleanValue(v)),
        }
    }
}

// ===== Unsigned Integers =====

impl Decode for u8 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0])
    }
}

impl Decode for u16 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u16::from_be_bytes(bytes),
                    Endianness::Little => u16::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for u32 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u32::from_be_bytes(bytes),
                    Endianness::Little => u32::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for u64 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u64::from_be_bytes(bytes),
                    Endianness::Little => u64::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for u128 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u128::from_be_bytes(bytes),
                    Endianness::Little => u128::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for usize {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_usize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                let value = match D::C::ENDIAN {
                    Endianness::Big => u64::from_be_bytes(bytes),
                    Endianness::Little => u64::from_le_bytes(bytes),
                };
                Ok(value as usize)
            }
        }
    }
}

// ===== Signed Integers =====

impl Decode for i8 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0] as i8)
    }
}

impl Decode for i16 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i16::from_be_bytes(bytes),
                    Endianness::Little => i16::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for i32 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i32::from_be_bytes(bytes),
                    Endianness::Little => i32::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for i64 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i64::from_be_bytes(bytes),
                    Endianness::Little => i64::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for i128 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i128::from_be_bytes(bytes),
                    Endianness::Little => i128::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for isize {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_isize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                let value = match D::C::ENDIAN {
                    Endianness::Big => i64::from_be_bytes(bytes),
                    Endianness::Little => i64::from_le_bytes(bytes),
                };
                Ok(value as isize)
            }
        }
    }
}

// ===== Floating Point =====

impl Decode for f32 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 4];
        decoder.reader().read(&mut bytes)?;
        Ok(match D::C::ENDIAN {
            Endianness::Big => f32::from_be_bytes(bytes),
            Endianness::Little => f32::from_le_bytes(bytes),
        })
    }
}

impl Decode for f64 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 8];
        decoder.reader().read(&mut bytes)?;
        Ok(match D::C::ENDIAN {
            Endianness::Big => f64::from_be_bytes(bytes),
            Endianness::Little => f64::from_le_bytes(bytes),
        })
    }
}

// ===== Arrays =====

impl<T: Decode, const N: usize> Decode for [T; N] {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        // Arrays don't have length prefix (compile-time known)
        // Use array::try_from_fn when stabilized, for now use unsafe
        let mut result: [core::mem::MaybeUninit<T>; N] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        for item in result.iter_mut() {
            item.write(T::decode(decoder)?);
        }

        // SAFETY: All elements have been initialized
        Ok(unsafe { core::mem::transmute_copy::<_, [T; N]>(&result) })
    }
}

// ===== Option =====

impl<T: Decode> Decode for Option<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(None),
            1 => Ok(Some(T::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Option variant",
            }),
        }
    }
}

// ===== Result =====

impl<T: Decode, U: Decode> Decode for Result<T, U> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u32::decode(decoder)?;
        match variant {
            0 => Ok(Ok(T::decode(decoder)?)),
            1 => Ok(Err(U::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Result variant",
            }),
        }
    }
}

// ===== Cell & RefCell =====

use core::cell::{Cell, RefCell};

impl<T: Decode + Copy> Decode for Cell<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cell::new(T::decode(decoder)?))
    }
}

impl<T: Decode> Decode for RefCell<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(RefCell::new(T::decode(decoder)?))
    }
}

// ===== NonZero types =====

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::error::IntegerType;

macro_rules! impl_decode_nonzero {
    ($nonzero:ty, $inner:ty, $int_type:expr) => {
        impl Decode for $nonzero {
            fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
                let value = <$inner>::decode(decoder)?;
                <$nonzero>::new(value).ok_or(Error::NonZeroTypeIsZero {
                    non_zero_type: $int_type,
                })
            }
        }
    };
}

impl_decode_nonzero!(NonZeroU8, u8, IntegerType::U8);
impl_decode_nonzero!(NonZeroU16, u16, IntegerType::U16);
impl_decode_nonzero!(NonZeroU32, u32, IntegerType::U32);
impl_decode_nonzero!(NonZeroU64, u64, IntegerType::U64);
impl_decode_nonzero!(NonZeroU128, u128, IntegerType::U128);
impl_decode_nonzero!(NonZeroUsize, usize, IntegerType::Usize);
impl_decode_nonzero!(NonZeroI8, i8, IntegerType::I8);
impl_decode_nonzero!(NonZeroI16, i16, IntegerType::I16);
impl_decode_nonzero!(NonZeroI32, i32, IntegerType::I32);
impl_decode_nonzero!(NonZeroI64, i64, IntegerType::I64);
impl_decode_nonzero!(NonZeroI128, i128, IntegerType::I128);
impl_decode_nonzero!(NonZeroIsize, isize, IntegerType::Isize);

// ===== Wrapping & Reverse =====

use core::cmp::Reverse;
use core::num::Wrapping;

impl<T: Decode> Decode for Wrapping<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Wrapping(T::decode(decoder)?))
    }
}

impl<T: Decode> Decode for Reverse<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Reverse(T::decode(decoder)?))
    }
}

// ===== Range types =====

use core::ops::{Bound, Range, RangeInclusive};

impl<T: Decode> Decode for Range<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Range {
            start: T::decode(decoder)?,
            end: T::decode(decoder)?,
        })
    }
}

impl<T: Decode> Decode for RangeInclusive<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let start = T::decode(decoder)?;
        let end = T::decode(decoder)?;
        Ok(RangeInclusive::new(start, end))
    }
}

impl<T: Decode> Decode for Bound<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(T::decode(decoder)?)),
            2 => Ok(Bound::Excluded(T::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Bound variant",
            }),
        }
    }
}

// ===== Character =====

// UTF-8 decoding constants
const CONT_MASK: u8 = 0b0011_1111;

impl Decode for char {
    /// Decode a char from UTF-8 (bincode compatible)
    ///
    /// UTF-8 encoding uses variable 1-4 bytes:
    /// - 0xxxxxxx: 1 byte (ASCII)
    /// - 110xxxxx 10xxxxxx: 2 bytes
    /// - 1110xxxx 10xxxxxx 10xxxxxx: 3 bytes
    /// - 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx: 4 bytes
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        // Read the first byte to determine the length
        let first = u8::decode(decoder)?;

        let code = if first < 0x80 {
            // 1-byte: ASCII
            first as u32
        } else if first < 0xE0 {
            // 2-byte: 110xxxxx 10xxxxxx
            let second = u8::decode(decoder)?;
            if !is_continuation_byte(second) {
                return Err(Error::InvalidCharEncoding([first, second, 0, 0]));
            }
            ((first as u32 & 0x1F) << 6) | (second as u32 & CONT_MASK as u32)
        } else if first < 0xF0 {
            // 3-byte: 1110xxxx 10xxxxxx 10xxxxxx
            let second = u8::decode(decoder)?;
            let third = u8::decode(decoder)?;
            if !is_continuation_byte(second) || !is_continuation_byte(third) {
                return Err(Error::InvalidCharEncoding([first, second, third, 0]));
            }
            ((first as u32 & 0x0F) << 12)
                | ((second as u32 & CONT_MASK as u32) << 6)
                | (third as u32 & CONT_MASK as u32)
        } else {
            // 4-byte: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
            let second = u8::decode(decoder)?;
            let third = u8::decode(decoder)?;
            let fourth = u8::decode(decoder)?;
            if !is_continuation_byte(second)
                || !is_continuation_byte(third)
                || !is_continuation_byte(fourth)
            {
                return Err(Error::InvalidCharEncoding([first, second, third, fourth]));
            }
            ((first as u32 & 0x07) << 18)
                | ((second as u32 & CONT_MASK as u32) << 12)
                | ((third as u32 & CONT_MASK as u32) << 6)
                | (fourth as u32 & CONT_MASK as u32)
        };

        char::from_u32(code).ok_or(Error::InvalidCharEncoding([
            (code >> 24) as u8,
            (code >> 16) as u8,
            (code >> 8) as u8,
            code as u8,
        ]))
    }
}

/// Check if a byte is a UTF-8 continuation byte (10xxxxxx)
#[inline]
const fn is_continuation_byte(byte: u8) -> bool {
    (byte & 0b1100_0000) == 0b1000_0000
}
