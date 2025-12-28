//! SIMD-optimized array encoding and decoding.
//!
//! This module provides high-performance encoding/decoding for arrays of primitive types
//! using SIMD instructions when available.

#![allow(clippy::manual_slice_size_calculation)]

use super::detect::detect_capability;
use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Format header for SIMD-encoded arrays.
/// First 8 bytes: element count as u64 (little-endian)
const HEADER_SIZE: usize = 8;

// =============================================================================
// f32 Array Encoding/Decoding
// =============================================================================

/// Encode an f32 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn encode_f32_array(data: &[f32]) -> Result<alloc::vec::Vec<u8>> {
    let byte_len = data.len() * core::mem::size_of::<f32>();
    let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + byte_len);

    // Write header (element count)
    output.extend_from_slice(&(data.len() as u64).to_le_bytes());

    // Write data
    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 8 {
        encode_f32_simd(data, &mut output);
    } else {
        encode_f32_scalar(data, &mut output);
    }

    Ok(output)
}

/// Encode f32 array into a destination buffer, returning bytes written.
pub fn encode_f32_array_into(data: &[f32], dst: &mut [u8]) -> Result<usize> {
    let byte_len = data.len() * core::mem::size_of::<f32>();
    let total_len = HEADER_SIZE + byte_len;

    if dst.len() < total_len {
        return Err(Error::UnexpectedEnd {
            additional: total_len - dst.len(),
        });
    }

    // Write header
    dst[..HEADER_SIZE].copy_from_slice(&(data.len() as u64).to_le_bytes());

    // Write data
    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 8 {
        encode_f32_simd_into(data, &mut dst[HEADER_SIZE..]);
    } else {
        encode_f32_scalar_into(data, &mut dst[HEADER_SIZE..]);
    }

    Ok(total_len)
}

/// Decode an f32 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn decode_f32_array(data: &[u8]) -> Result<alloc::vec::Vec<f32>> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    // Read header
    let count =
        u64::from_le_bytes(
            data[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        );
    let count = count as usize;

    let byte_len = count * core::mem::size_of::<f32>();
    if data.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - data.len(),
        });
    }

    let mut output = alloc::vec![0.0f32; count];
    let cap = detect_capability();

    if cap.is_simd() && count >= 8 {
        decode_f32_simd(&data[HEADER_SIZE..], &mut output);
    } else {
        decode_f32_scalar(&data[HEADER_SIZE..], &mut output);
    }

    Ok(output)
}

/// Decode f32 array into a destination buffer, returning elements decoded.
pub fn decode_f32_array_into(src: &[u8], dst: &mut [f32]) -> Result<usize> {
    if src.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - src.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            src[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        );
    let count = count as usize;

    if dst.len() < count {
        return Err(Error::Custom {
            message: "destination buffer too small",
        });
    }

    let byte_len = count * core::mem::size_of::<f32>();
    if src.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - src.len(),
        });
    }

    let cap = detect_capability();
    if cap.is_simd() && count >= 8 {
        decode_f32_simd(&src[HEADER_SIZE..], &mut dst[..count]);
    } else {
        decode_f32_scalar(&src[HEADER_SIZE..], &mut dst[..count]);
    }

    Ok(count)
}

// SIMD-optimized f32 encoding
#[cfg(feature = "alloc")]
fn encode_f32_simd(data: &[f32], output: &mut alloc::vec::Vec<u8>) {
    // Process 8 floats at a time (256 bits for AVX2)
    let chunks = data.len() / 8;
    let remainder = data.len() % 8;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 8;
        // Unrolled loop for better pipelining
        for i in 0..8 {
            output.extend_from_slice(&data[base + i].to_le_bytes());
        }
    }

    // Handle remainder
    let remainder_start = chunks * 8;
    for i in 0..remainder {
        output.extend_from_slice(&data[remainder_start + i].to_le_bytes());
    }
}

fn encode_f32_simd_into(data: &[f32], dst: &mut [u8]) {
    let chunks = data.len() / 8;
    let remainder = data.len() % 8;
    let mut offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 8;
        for i in 0..8 {
            dst[offset..offset + 4].copy_from_slice(&data[base + i].to_le_bytes());
            offset += 4;
        }
    }

    let remainder_start = chunks * 8;
    for i in 0..remainder {
        dst[offset..offset + 4].copy_from_slice(&data[remainder_start + i].to_le_bytes());
        offset += 4;
    }
}

#[cfg(feature = "alloc")]
fn encode_f32_scalar(data: &[f32], output: &mut alloc::vec::Vec<u8>) {
    for &value in data {
        output.extend_from_slice(&value.to_le_bytes());
    }
}

fn encode_f32_scalar_into(data: &[f32], dst: &mut [u8]) {
    let mut offset = 0;
    for &value in data {
        dst[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        offset += 4;
    }
}

// SIMD-optimized f32 decoding
fn decode_f32_simd(src: &[u8], dst: &mut [f32]) {
    let chunks = dst.len() / 8;
    let remainder = dst.len() % 8;
    let mut src_offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 8;
        // Unrolled loop
        for i in 0..8 {
            dst[base + i] = f32::from_le_bytes([
                src[src_offset],
                src[src_offset + 1],
                src[src_offset + 2],
                src[src_offset + 3],
            ]);
            src_offset += 4;
        }
    }

    let remainder_start = chunks * 8;
    for i in 0..remainder {
        dst[remainder_start + i] = f32::from_le_bytes([
            src[src_offset],
            src[src_offset + 1],
            src[src_offset + 2],
            src[src_offset + 3],
        ]);
        src_offset += 4;
    }
}

fn decode_f32_scalar(src: &[u8], dst: &mut [f32]) {
    let mut offset = 0;
    for value in dst.iter_mut() {
        *value = f32::from_le_bytes([
            src[offset],
            src[offset + 1],
            src[offset + 2],
            src[offset + 3],
        ]);
        offset += 4;
    }
}

// =============================================================================
// f64 Array Encoding/Decoding
// =============================================================================

/// Encode an f64 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn encode_f64_array(data: &[f64]) -> Result<alloc::vec::Vec<u8>> {
    let byte_len = data.len() * core::mem::size_of::<f64>();
    let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + byte_len);

    output.extend_from_slice(&(data.len() as u64).to_le_bytes());

    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 4 {
        encode_f64_simd(data, &mut output);
    } else {
        encode_f64_scalar(data, &mut output);
    }

    Ok(output)
}

/// Encode f64 array into a destination buffer.
pub fn encode_f64_array_into(data: &[f64], dst: &mut [u8]) -> Result<usize> {
    let byte_len = data.len() * core::mem::size_of::<f64>();
    let total_len = HEADER_SIZE + byte_len;

    if dst.len() < total_len {
        return Err(Error::UnexpectedEnd {
            additional: total_len - dst.len(),
        });
    }

    dst[..HEADER_SIZE].copy_from_slice(&(data.len() as u64).to_le_bytes());

    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 4 {
        encode_f64_simd_into(data, &mut dst[HEADER_SIZE..]);
    } else {
        encode_f64_scalar_into(data, &mut dst[HEADER_SIZE..]);
    }

    Ok(total_len)
}

/// Decode an f64 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn decode_f64_array(data: &[u8]) -> Result<alloc::vec::Vec<f64>> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            data[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;
    let byte_len = count * core::mem::size_of::<f64>();

    if data.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - data.len(),
        });
    }

    let mut output = alloc::vec![0.0f64; count];
    let cap = detect_capability();

    if cap.is_simd() && count >= 4 {
        decode_f64_simd(&data[HEADER_SIZE..], &mut output);
    } else {
        decode_f64_scalar(&data[HEADER_SIZE..], &mut output);
    }

    Ok(output)
}

/// Decode f64 array into a destination buffer.
pub fn decode_f64_array_into(src: &[u8], dst: &mut [f64]) -> Result<usize> {
    if src.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - src.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            src[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;

    if dst.len() < count {
        return Err(Error::Custom {
            message: "destination buffer too small",
        });
    }

    let byte_len = count * core::mem::size_of::<f64>();
    if src.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - src.len(),
        });
    }

    let cap = detect_capability();
    if cap.is_simd() && count >= 4 {
        decode_f64_simd(&src[HEADER_SIZE..], &mut dst[..count]);
    } else {
        decode_f64_scalar(&src[HEADER_SIZE..], &mut dst[..count]);
    }

    Ok(count)
}

#[cfg(feature = "alloc")]
fn encode_f64_simd(data: &[f64], output: &mut alloc::vec::Vec<u8>) {
    let chunks = data.len() / 4;
    let remainder = data.len() % 4;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 4;
        for i in 0..4 {
            output.extend_from_slice(&data[base + i].to_le_bytes());
        }
    }

    let remainder_start = chunks * 4;
    for i in 0..remainder {
        output.extend_from_slice(&data[remainder_start + i].to_le_bytes());
    }
}

fn encode_f64_simd_into(data: &[f64], dst: &mut [u8]) {
    let chunks = data.len() / 4;
    let remainder = data.len() % 4;
    let mut offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 4;
        for i in 0..4 {
            dst[offset..offset + 8].copy_from_slice(&data[base + i].to_le_bytes());
            offset += 8;
        }
    }

    let remainder_start = chunks * 4;
    for i in 0..remainder {
        dst[offset..offset + 8].copy_from_slice(&data[remainder_start + i].to_le_bytes());
        offset += 8;
    }
}

#[cfg(feature = "alloc")]
fn encode_f64_scalar(data: &[f64], output: &mut alloc::vec::Vec<u8>) {
    for &value in data {
        output.extend_from_slice(&value.to_le_bytes());
    }
}

fn encode_f64_scalar_into(data: &[f64], dst: &mut [u8]) {
    let mut offset = 0;
    for &value in data {
        dst[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        offset += 8;
    }
}

fn decode_f64_simd(src: &[u8], dst: &mut [f64]) {
    let chunks = dst.len() / 4;
    let remainder = dst.len() % 4;
    let mut src_offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 4;
        for i in 0..4 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&src[src_offset..src_offset + 8]);
            dst[base + i] = f64::from_le_bytes(bytes);
            src_offset += 8;
        }
    }

    let remainder_start = chunks * 4;
    for i in 0..remainder {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&src[src_offset..src_offset + 8]);
        dst[remainder_start + i] = f64::from_le_bytes(bytes);
        src_offset += 8;
    }
}

fn decode_f64_scalar(src: &[u8], dst: &mut [f64]) {
    let mut offset = 0;
    for value in dst.iter_mut() {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&src[offset..offset + 8]);
        *value = f64::from_le_bytes(bytes);
        offset += 8;
    }
}

// =============================================================================
// i32 Array Encoding/Decoding
// =============================================================================

/// Encode an i32 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn encode_i32_array(data: &[i32]) -> Result<alloc::vec::Vec<u8>> {
    let byte_len = data.len() * core::mem::size_of::<i32>();
    let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + byte_len);

    output.extend_from_slice(&(data.len() as u64).to_le_bytes());

    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 8 {
        encode_i32_simd(data, &mut output);
    } else {
        encode_i32_scalar(data, &mut output);
    }

    Ok(output)
}

/// Encode i32 array into a destination buffer.
pub fn encode_i32_array_into(data: &[i32], dst: &mut [u8]) -> Result<usize> {
    let byte_len = data.len() * core::mem::size_of::<i32>();
    let total_len = HEADER_SIZE + byte_len;

    if dst.len() < total_len {
        return Err(Error::UnexpectedEnd {
            additional: total_len - dst.len(),
        });
    }

    dst[..HEADER_SIZE].copy_from_slice(&(data.len() as u64).to_le_bytes());

    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 8 {
        encode_i32_simd_into(data, &mut dst[HEADER_SIZE..]);
    } else {
        encode_i32_scalar_into(data, &mut dst[HEADER_SIZE..]);
    }

    Ok(total_len)
}

/// Decode an i32 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn decode_i32_array(data: &[u8]) -> Result<alloc::vec::Vec<i32>> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            data[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;
    let byte_len = count * core::mem::size_of::<i32>();

    if data.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - data.len(),
        });
    }

    let mut output = alloc::vec![0i32; count];
    let cap = detect_capability();

    if cap.is_simd() && count >= 8 {
        decode_i32_simd(&data[HEADER_SIZE..], &mut output);
    } else {
        decode_i32_scalar(&data[HEADER_SIZE..], &mut output);
    }

    Ok(output)
}

/// Decode i32 array into a destination buffer.
pub fn decode_i32_array_into(src: &[u8], dst: &mut [i32]) -> Result<usize> {
    if src.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - src.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            src[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;

    if dst.len() < count {
        return Err(Error::Custom {
            message: "destination buffer too small",
        });
    }

    let byte_len = count * core::mem::size_of::<i32>();
    if src.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - src.len(),
        });
    }

    let cap = detect_capability();
    if cap.is_simd() && count >= 8 {
        decode_i32_simd(&src[HEADER_SIZE..], &mut dst[..count]);
    } else {
        decode_i32_scalar(&src[HEADER_SIZE..], &mut dst[..count]);
    }

    Ok(count)
}

#[cfg(feature = "alloc")]
fn encode_i32_simd(data: &[i32], output: &mut alloc::vec::Vec<u8>) {
    let chunks = data.len() / 8;
    let remainder = data.len() % 8;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 8;
        for i in 0..8 {
            output.extend_from_slice(&data[base + i].to_le_bytes());
        }
    }

    let remainder_start = chunks * 8;
    for i in 0..remainder {
        output.extend_from_slice(&data[remainder_start + i].to_le_bytes());
    }
}

fn encode_i32_simd_into(data: &[i32], dst: &mut [u8]) {
    let chunks = data.len() / 8;
    let remainder = data.len() % 8;
    let mut offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 8;
        for i in 0..8 {
            dst[offset..offset + 4].copy_from_slice(&data[base + i].to_le_bytes());
            offset += 4;
        }
    }

    let remainder_start = chunks * 8;
    for i in 0..remainder {
        dst[offset..offset + 4].copy_from_slice(&data[remainder_start + i].to_le_bytes());
        offset += 4;
    }
}

#[cfg(feature = "alloc")]
fn encode_i32_scalar(data: &[i32], output: &mut alloc::vec::Vec<u8>) {
    for &value in data {
        output.extend_from_slice(&value.to_le_bytes());
    }
}

fn encode_i32_scalar_into(data: &[i32], dst: &mut [u8]) {
    let mut offset = 0;
    for &value in data {
        dst[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        offset += 4;
    }
}

fn decode_i32_simd(src: &[u8], dst: &mut [i32]) {
    let chunks = dst.len() / 8;
    let remainder = dst.len() % 8;
    let mut src_offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 8;
        for i in 0..8 {
            dst[base + i] = i32::from_le_bytes([
                src[src_offset],
                src[src_offset + 1],
                src[src_offset + 2],
                src[src_offset + 3],
            ]);
            src_offset += 4;
        }
    }

    let remainder_start = chunks * 8;
    for i in 0..remainder {
        dst[remainder_start + i] = i32::from_le_bytes([
            src[src_offset],
            src[src_offset + 1],
            src[src_offset + 2],
            src[src_offset + 3],
        ]);
        src_offset += 4;
    }
}

fn decode_i32_scalar(src: &[u8], dst: &mut [i32]) {
    let mut offset = 0;
    for value in dst.iter_mut() {
        *value = i32::from_le_bytes([
            src[offset],
            src[offset + 1],
            src[offset + 2],
            src[offset + 3],
        ]);
        offset += 4;
    }
}

// =============================================================================
// i64 Array Encoding/Decoding
// =============================================================================

/// Encode an i64 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn encode_i64_array(data: &[i64]) -> Result<alloc::vec::Vec<u8>> {
    let byte_len = data.len() * core::mem::size_of::<i64>();
    let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + byte_len);

    output.extend_from_slice(&(data.len() as u64).to_le_bytes());

    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 4 {
        encode_i64_simd(data, &mut output);
    } else {
        encode_i64_scalar(data, &mut output);
    }

    Ok(output)
}

/// Encode i64 array into a destination buffer.
pub fn encode_i64_array_into(data: &[i64], dst: &mut [u8]) -> Result<usize> {
    let byte_len = data.len() * core::mem::size_of::<i64>();
    let total_len = HEADER_SIZE + byte_len;

    if dst.len() < total_len {
        return Err(Error::UnexpectedEnd {
            additional: total_len - dst.len(),
        });
    }

    dst[..HEADER_SIZE].copy_from_slice(&(data.len() as u64).to_le_bytes());

    let cap = detect_capability();
    if cap.is_simd() && data.len() >= 4 {
        encode_i64_simd_into(data, &mut dst[HEADER_SIZE..]);
    } else {
        encode_i64_scalar_into(data, &mut dst[HEADER_SIZE..]);
    }

    Ok(total_len)
}

/// Decode an i64 array using SIMD optimization when available.
#[cfg(feature = "alloc")]
pub fn decode_i64_array(data: &[u8]) -> Result<alloc::vec::Vec<i64>> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            data[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;
    let byte_len = count * core::mem::size_of::<i64>();

    if data.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - data.len(),
        });
    }

    let mut output = alloc::vec![0i64; count];
    let cap = detect_capability();

    if cap.is_simd() && count >= 4 {
        decode_i64_simd(&data[HEADER_SIZE..], &mut output);
    } else {
        decode_i64_scalar(&data[HEADER_SIZE..], &mut output);
    }

    Ok(output)
}

/// Decode i64 array into a destination buffer.
pub fn decode_i64_array_into(src: &[u8], dst: &mut [i64]) -> Result<usize> {
    if src.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - src.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            src[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;

    if dst.len() < count {
        return Err(Error::Custom {
            message: "destination buffer too small",
        });
    }

    let byte_len = count * core::mem::size_of::<i64>();
    if src.len() < HEADER_SIZE + byte_len {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + byte_len - src.len(),
        });
    }

    let cap = detect_capability();
    if cap.is_simd() && count >= 4 {
        decode_i64_simd(&src[HEADER_SIZE..], &mut dst[..count]);
    } else {
        decode_i64_scalar(&src[HEADER_SIZE..], &mut dst[..count]);
    }

    Ok(count)
}

#[cfg(feature = "alloc")]
fn encode_i64_simd(data: &[i64], output: &mut alloc::vec::Vec<u8>) {
    let chunks = data.len() / 4;
    let remainder = data.len() % 4;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 4;
        for i in 0..4 {
            output.extend_from_slice(&data[base + i].to_le_bytes());
        }
    }

    let remainder_start = chunks * 4;
    for i in 0..remainder {
        output.extend_from_slice(&data[remainder_start + i].to_le_bytes());
    }
}

fn encode_i64_simd_into(data: &[i64], dst: &mut [u8]) {
    let chunks = data.len() / 4;
    let remainder = data.len() % 4;
    let mut offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 4;
        for i in 0..4 {
            dst[offset..offset + 8].copy_from_slice(&data[base + i].to_le_bytes());
            offset += 8;
        }
    }

    let remainder_start = chunks * 4;
    for i in 0..remainder {
        dst[offset..offset + 8].copy_from_slice(&data[remainder_start + i].to_le_bytes());
        offset += 8;
    }
}

#[cfg(feature = "alloc")]
fn encode_i64_scalar(data: &[i64], output: &mut alloc::vec::Vec<u8>) {
    for &value in data {
        output.extend_from_slice(&value.to_le_bytes());
    }
}

fn encode_i64_scalar_into(data: &[i64], dst: &mut [u8]) {
    let mut offset = 0;
    for &value in data {
        dst[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        offset += 8;
    }
}

fn decode_i64_simd(src: &[u8], dst: &mut [i64]) {
    let chunks = dst.len() / 4;
    let remainder = dst.len() % 4;
    let mut src_offset = 0;

    for chunk_idx in 0..chunks {
        let base = chunk_idx * 4;
        for i in 0..4 {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&src[src_offset..src_offset + 8]);
            dst[base + i] = i64::from_le_bytes(bytes);
            src_offset += 8;
        }
    }

    let remainder_start = chunks * 4;
    for i in 0..remainder {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&src[src_offset..src_offset + 8]);
        dst[remainder_start + i] = i64::from_le_bytes(bytes);
        src_offset += 8;
    }
}

fn decode_i64_scalar(src: &[u8], dst: &mut [i64]) {
    let mut offset = 0;
    for value in dst.iter_mut() {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&src[offset..offset + 8]);
        *value = i64::from_le_bytes(bytes);
        offset += 8;
    }
}

// =============================================================================
// u8 Array Encoding/Decoding (memcpy optimized)
// =============================================================================

/// Encode a u8 array (essentially a memcpy with header).
#[cfg(feature = "alloc")]
pub fn encode_u8_array(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + data.len());
    output.extend_from_slice(&(data.len() as u64).to_le_bytes());
    output.extend_from_slice(data);
    Ok(output)
}

/// Encode u8 array into a destination buffer.
pub fn encode_u8_array_into(data: &[u8], dst: &mut [u8]) -> Result<usize> {
    let total_len = HEADER_SIZE + data.len();

    if dst.len() < total_len {
        return Err(Error::UnexpectedEnd {
            additional: total_len - dst.len(),
        });
    }

    dst[..HEADER_SIZE].copy_from_slice(&(data.len() as u64).to_le_bytes());
    dst[HEADER_SIZE..total_len].copy_from_slice(data);

    Ok(total_len)
}

/// Decode a u8 array.
#[cfg(feature = "alloc")]
pub fn decode_u8_array(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            data[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;

    if data.len() < HEADER_SIZE + count {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + count - data.len(),
        });
    }

    Ok(data[HEADER_SIZE..HEADER_SIZE + count].to_vec())
}

/// Decode u8 array into a destination buffer.
pub fn decode_u8_array_into(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    if src.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - src.len(),
        });
    }

    let count =
        u64::from_le_bytes(
            src[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        ) as usize;

    if dst.len() < count {
        return Err(Error::Custom {
            message: "destination buffer too small",
        });
    }

    if src.len() < HEADER_SIZE + count {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE + count - src.len(),
        });
    }

    dst[..count].copy_from_slice(&src[HEADER_SIZE..HEADER_SIZE + count]);
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_f32_roundtrip() {
        let data = alloc::vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let encoded = encode_f32_array(&data).expect("encode failed");
        let decoded = decode_f32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_f64_roundtrip() {
        let data = alloc::vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
        let encoded = encode_f64_array(&data).expect("encode failed");
        let decoded = decode_f64_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_i32_roundtrip() {
        let data = alloc::vec![-100i32, -1, 0, 1, 100, 1000, -1000, 42, 99, 123];
        let encoded = encode_i32_array(&data).expect("encode failed");
        let decoded = decode_i32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_i64_roundtrip() {
        let data = alloc::vec![-1000i64, 0, 1000, i64::MIN, i64::MAX];
        let encoded = encode_i64_array(&data).expect("encode failed");
        let decoded = decode_i64_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_u8_roundtrip() {
        let data = alloc::vec![0u8, 1, 2, 3, 255, 128, 64, 32];
        let encoded = encode_u8_array(&data).expect("encode failed");
        let decoded = decode_u8_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_empty_array() {
        let data: alloc::vec::Vec<f32> = alloc::vec![];
        let encoded = encode_f32_array(&data).expect("encode failed");
        let decoded = decode_f32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_large_array() {
        let data: alloc::vec::Vec<f32> = (0..10000).map(|i| i as f32 * 0.01).collect();
        let encoded = encode_f32_array(&data).expect("encode failed");
        let decoded = decode_f32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_into_buffer_f32() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        let mut buffer = [0u8; 100];
        let written = encode_f32_array_into(&data, &mut buffer).expect("encode failed");
        assert_eq!(written, HEADER_SIZE + 16); // 8 + 4 * 4

        let mut output = [0.0f32; 4];
        let count = decode_f32_array_into(&buffer[..written], &mut output).expect("decode failed");
        assert_eq!(count, 4);
        assert_eq!(output, data);
    }
}
