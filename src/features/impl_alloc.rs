//! Encode/Decode implementations for alloc-dependent types

use crate::{
    de::{read::Reader, BorrowDecode, BorrowDecoder, BorrowReader, Decode, Decoder},
    enc::{write::Writer, Encode, Encoder},
    error::Error,
};
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
};

// ===== Vec<T> =====

impl<T: Encode> Encode for Vec<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode length first
        (self.len() as u64).encode(encoder)?;
        // Encode each element
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        // Claim memory for the container
        decoder.claim_container_read::<T>(len)?;

        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(decoder)?);
        }
        Ok(vec)
    }
}

// ===== String =====

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.as_str().encode(encoder)
    }
}

impl Encode for str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode byte length first
        (self.len() as u64).encode(encoder)?;
        // Encode UTF-8 bytes
        encoder.writer().write(self.as_bytes())
    }
}

impl Encode for &str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (*self).encode(encoder)
    }
}

impl Encode for &[u8] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode length first
        (self.len() as u64).encode(encoder)?;
        // Encode bytes
        encoder.writer().write(self)
    }
}

impl Decode for String {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        // Claim bytes
        decoder.claim_bytes_read(len)?;

        let mut bytes = alloc::vec![0u8; len];
        decoder.reader().read(&mut bytes)?;

        String::from_utf8(bytes).map_err(|e| Error::Utf8 {
            inner: e.utf8_error(),
        })
    }
}

// ===== Box<T> =====

impl<T: Encode> Encode for Box<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Box<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Box::new(T::decode(decoder)?))
    }
}

// ===== Box<[T]> =====

impl<T: Encode> Encode for Box<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Box<[T]> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}

// ===== Box<str> =====

impl Encode for Box<str> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl Decode for Box<str> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let string = String::decode(decoder)?;
        Ok(string.into_boxed_str())
    }
}

// ===== Cow<'a, T> =====

impl<T: Encode + ToOwned + ?Sized> Encode for Cow<'_, T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<'a, T> Decode for Cow<'a, T>
where
    T: ToOwned,
    T::Owned: Decode,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cow::Owned(T::Owned::decode(decoder)?))
    }
}

// ===== Rc<T> =====

impl<T: Encode> Encode for Rc<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Rc<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Rc::new(T::decode(decoder)?))
    }
}

// ===== Rc<[T]> =====

impl<T: Encode> Encode for Rc<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Rc<[T]> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(Rc::from(vec.into_boxed_slice()))
    }
}

// ===== Rc<str> =====

impl Encode for Rc<str> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl Decode for Rc<str> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let string = String::decode(decoder)?;
        Ok(Rc::from(string.into_boxed_str()))
    }
}

// ===== Arc<T> =====

impl<T: Encode> Encode for Arc<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Arc<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Arc::new(T::decode(decoder)?))
    }
}

// ===== Arc<[T]> =====

impl<T: Encode> Encode for Arc<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: Decode> Decode for Arc<[T]> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let vec = Vec::<T>::decode(decoder)?;
        Ok(Arc::from(vec.into_boxed_slice()))
    }
}

// ===== Arc<str> =====

impl Encode for Arc<str> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl Decode for Arc<str> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let string = String::decode(decoder)?;
        Ok(Arc::from(string.into_boxed_str()))
    }
}

// ===== BTreeMap<K, V> =====

impl<K: Encode, V: Encode> Encode for BTreeMap<K, V> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for (key, value) in self.iter() {
            key.encode(encoder)?;
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut map = BTreeMap::new();
        for _ in 0..len {
            let key = K::decode(decoder)?;
            let value = V::decode(decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

// ===== BTreeSet<T> =====

impl<T: Encode> Encode for BTreeSet<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> Decode for BTreeSet<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut set = BTreeSet::new();
        for _ in 0..len {
            set.insert(T::decode(decoder)?);
        }
        Ok(set)
    }
}

// ===== BinaryHeap<T> =====

impl<T: Encode> Encode for BinaryHeap<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> Decode for BinaryHeap<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut heap = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            heap.push(T::decode(decoder)?);
        }
        Ok(heap)
    }
}

// ===== VecDeque<T> =====

impl<T: Encode> Encode for VecDeque<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: Decode> Decode for VecDeque<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut deque = VecDeque::with_capacity(len);
        for _ in 0..len {
            deque.push_back(T::decode(decoder)?);
        }
        Ok(deque)
    }
}

// ===== BorrowDecode for &[u8] (zero-copy) =====

impl<'de> BorrowDecode<'de> for &'de [u8] {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        use crate::de::Decode;
        let len = u64::decode(decoder)? as usize;
        decoder.claim_bytes_read(len)?;

        let bytes = decoder.borrow_reader().take_bytes(len)?;
        Ok(bytes)
    }
}

// ===== BorrowDecode for &str (zero-copy) =====

impl<'de> BorrowDecode<'de> for &'de str {
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        use crate::de::Decode;
        let len = u64::decode(decoder)? as usize;
        decoder.claim_bytes_read(len)?;

        let bytes = decoder.borrow_reader().take_bytes(len)?;

        // Validate UTF-8
        core::str::from_utf8(bytes).map_err(|e| Error::Utf8 { inner: e })
    }
}
