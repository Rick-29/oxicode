//! Encode/Decode implementations for std-dependent types

use crate::{
    de::{read::Reader, Decode, Decoder},
    enc::{write::Writer, Encode, Encoder},
    error::Error,
};
use std::{
    collections::{HashMap, HashSet},
    ffi::{CStr, CString},
    hash::{BuildHasher, Hash},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// ===== HashMap<K, V> =====

impl<K, V, S> Encode for HashMap<K, V, S>
where
    K: Encode,
    V: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for (key, value) in self.iter() {
            key.encode(encoder)?;
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<K, V, S> Decode for HashMap<K, V, S>
where
    K: Decode + Eq + Hash,
    V: Decode,
    S: BuildHasher + Default,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut map = HashMap::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            let key = K::decode(decoder)?;
            let value = V::decode(decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}

// ===== HashSet<T> =====

impl<T, S> Encode for HashSet<T, S>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T, S> Decode for HashSet<T, S>
where
    T: Decode + Eq + Hash,
    S: BuildHasher + Default,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;

        let mut set = HashSet::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            set.insert(T::decode(decoder)?);
        }
        Ok(set)
    }
}

// ===== Mutex<T> =====

impl<T: Encode> Encode for Mutex<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let guard = self.lock().map_err(|_| Error::Custom {
            message: "Mutex poisoned",
        })?;
        (*guard).encode(encoder)
    }
}

impl<T: Decode> Decode for Mutex<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Mutex::new(T::decode(decoder)?))
    }
}

// ===== RwLock<T> =====

impl<T: Encode> Encode for RwLock<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let guard = self.read().map_err(|_| Error::Custom {
            message: "RwLock poisoned",
        })?;
        (*guard).encode(encoder)
    }
}

impl<T: Decode> Decode for RwLock<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(RwLock::new(T::decode(decoder)?))
    }
}

// ===== Duration =====

impl Encode for Duration {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.as_secs().encode(encoder)?;
        self.subsec_nanos().encode(encoder)
    }
}

impl Decode for Duration {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let secs = u64::decode(decoder)?;
        let nanos = u32::decode(decoder)?;

        // Validate nanos < 1_000_000_000
        if nanos >= 1_000_000_000 {
            return Err(Error::InvalidDuration { secs, nanos });
        }

        Ok(Duration::new(secs, nanos))
    }
}

// ===== SystemTime =====

impl Encode for SystemTime {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                0u8.encode(encoder)?;
                duration.encode(encoder)
            }
            Err(e) => {
                1u8.encode(encoder)?;
                e.duration().encode(encoder)
            }
        }
    }
}

impl Decode for SystemTime {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        let duration = Duration::decode(decoder)?;

        match variant {
            0 => Ok(UNIX_EPOCH + duration),
            1 => Ok(UNIX_EPOCH - duration),
            _ => Err(Error::InvalidData {
                message: "Invalid SystemTime variant",
            }),
        }
    }
}

// ===== Path & PathBuf =====

impl Encode for Path {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let os_str = self.as_os_str();
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let bytes = os_str.as_bytes();
            (bytes.len() as u64).encode(encoder)?;
            encoder.writer().write(bytes)
        }
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            let wide: Vec<u16> = os_str.encode_wide().collect();
            (wide.len() as u64).encode(encoder)?;
            for code_unit in wide {
                code_unit.encode(encoder)?;
            }
            Ok(())
        }
        #[cfg(not(any(unix, windows)))]
        {
            // Fallback: convert to string lossy
            let string = os_str.to_string_lossy();
            string.encode(encoder)
        }
    }
}

impl Encode for PathBuf {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.as_path().encode(encoder)
    }
}

impl Decode for PathBuf {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        #[cfg(unix)]
        {
            use std::ffi::OsStr;
            use std::os::unix::ffi::OsStrExt;

            let len = u64::decode(decoder)? as usize;
            decoder.claim_bytes_read(len)?;

            let mut bytes = alloc::vec![0u8; len];
            decoder.reader().read(&mut bytes)?;

            Ok(PathBuf::from(OsStr::from_bytes(&bytes)))
        }
        #[cfg(windows)]
        {
            use std::ffi::OsString;
            use std::os::windows::ffi::OsStringExt;

            let len = u64::decode(decoder)? as usize;
            let mut wide = alloc::vec![0u16; len];
            for code_unit in &mut wide {
                *code_unit = u16::decode(decoder)?;
            }

            Ok(PathBuf::from(OsString::from_wide(&wide)))
        }
        #[cfg(not(any(unix, windows)))]
        {
            let string = String::decode(decoder)?;
            Ok(PathBuf::from(string))
        }
    }
}

// ===== IpAddr =====

impl Encode for IpAddr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            IpAddr::V4(addr) => {
                0u8.encode(encoder)?;
                addr.encode(encoder)
            }
            IpAddr::V6(addr) => {
                1u8.encode(encoder)?;
                addr.encode(encoder)
            }
        }
    }
}

impl Decode for IpAddr {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(IpAddr::V4(Ipv4Addr::decode(decoder)?)),
            1 => Ok(IpAddr::V6(Ipv6Addr::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid IpAddr variant",
            }),
        }
    }
}

// ===== Ipv4Addr =====

impl Encode for Ipv4Addr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.writer().write(&self.octets())
    }
}

impl Decode for Ipv4Addr {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut octets = [0u8; 4];
        decoder.reader().read(&mut octets)?;
        Ok(Ipv4Addr::from(octets))
    }
}

// ===== Ipv6Addr =====

impl Encode for Ipv6Addr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.writer().write(&self.octets())
    }
}

impl Decode for Ipv6Addr {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut octets = [0u8; 16];
        decoder.reader().read(&mut octets)?;
        Ok(Ipv6Addr::from(octets))
    }
}

// ===== SocketAddr =====

impl Encode for SocketAddr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            SocketAddr::V4(addr) => {
                0u8.encode(encoder)?;
                addr.encode(encoder)
            }
            SocketAddr::V6(addr) => {
                1u8.encode(encoder)?;
                addr.encode(encoder)
            }
        }
    }
}

impl Decode for SocketAddr {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(SocketAddr::V4(SocketAddrV4::decode(decoder)?)),
            1 => Ok(SocketAddr::V6(SocketAddrV6::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid SocketAddr variant",
            }),
        }
    }
}

// ===== SocketAddrV4 =====

impl Encode for SocketAddrV4 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.ip().encode(encoder)?;
        self.port().encode(encoder)
    }
}

impl Decode for SocketAddrV4 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let ip = Ipv4Addr::decode(decoder)?;
        let port = u16::decode(decoder)?;
        Ok(SocketAddrV4::new(ip, port))
    }
}

// ===== SocketAddrV6 =====

impl Encode for SocketAddrV6 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.ip().encode(encoder)?;
        self.port().encode(encoder)?;
        self.flowinfo().encode(encoder)?;
        self.scope_id().encode(encoder)
    }
}

impl Decode for SocketAddrV6 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let ip = Ipv6Addr::decode(decoder)?;
        let port = u16::decode(decoder)?;
        let flowinfo = u32::decode(decoder)?;
        let scope_id = u32::decode(decoder)?;
        Ok(SocketAddrV6::new(ip, port, flowinfo, scope_id))
    }
}

// ===== CString =====

impl Encode for CString {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode as bytes without the null terminator
        let bytes = self.as_bytes();
        (bytes.len() as u64).encode(encoder)?;
        encoder.writer().write(bytes)
    }
}

impl Decode for CString {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let len = u64::decode(decoder)? as usize;
        decoder.claim_bytes_read(len)?;

        let mut bytes = alloc::vec![0u8; len];
        decoder.reader().read(&mut bytes)?;

        // Verify no null bytes in the middle
        if bytes.contains(&0) {
            return Err(Error::Custom {
                message: "CString contains null byte",
            });
        }

        CString::new(bytes).map_err(|_| Error::Custom {
            message: "CString contains null byte",
        })
    }
}

// ===== CStr =====

impl Encode for CStr {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let bytes = self.to_bytes();
        (bytes.len() as u64).encode(encoder)?;
        encoder.writer().write(bytes)
    }
}
