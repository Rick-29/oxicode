//! Decode implementations for tuples (1-16 elements)

use super::{Decode, Decoder};
use crate::error::Error;

// Implement Decode for tuples up to 16 elements
// Following bincode's pattern of direct implementations (not macros)

impl<T0: Decode> Decode for (T0,) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::decode(decoder)?,))
    }
}

impl<T0: Decode, T1: Decode> Decode for (T0, T1) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((T0::decode(decoder)?, T1::decode(decoder)?))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode> Decode for (T0, T1, T2) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
        ))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode> Decode for (T0, T1, T2, T3) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
        ))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode, T4: Decode> Decode for (T0, T1, T2, T3, T4) {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
        ))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode, T4: Decode, T5: Decode> Decode
    for (T0, T1, T2, T3, T4, T5)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
        ))
    }
}

impl<T0: Decode, T1: Decode, T2: Decode, T3: Decode, T4: Decode, T5: Decode, T6: Decode> Decode
    for (T0, T1, T2, T3, T4, T5, T6)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
        T13: Decode,
    > Decode for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
            T13::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
        T13: Decode,
        T14: Decode,
    > Decode
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
    )
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
            T13::decode(decoder)?,
            T14::decode(decoder)?,
        ))
    }
}

impl<
        T0: Decode,
        T1: Decode,
        T2: Decode,
        T3: Decode,
        T4: Decode,
        T5: Decode,
        T6: Decode,
        T7: Decode,
        T8: Decode,
        T9: Decode,
        T10: Decode,
        T11: Decode,
        T12: Decode,
        T13: Decode,
        T14: Decode,
        T15: Decode,
    > Decode
    for (
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    )
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
            T4::decode(decoder)?,
            T5::decode(decoder)?,
            T6::decode(decoder)?,
            T7::decode(decoder)?,
            T8::decode(decoder)?,
            T9::decode(decoder)?,
            T10::decode(decoder)?,
            T11::decode(decoder)?,
            T12::decode(decoder)?,
            T13::decode(decoder)?,
            T14::decode(decoder)?,
            T15::decode(decoder)?,
        ))
    }
}
