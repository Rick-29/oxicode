//! Tests for zero-copy BorrowDecode

use oxicode::config;

#[test]
fn test_borrow_decode_str() {
    let original = "Hello, OxiCode! 🦀";

    // Encode to bytes (&str implements Encode)
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");

    // Borrow decode (zero-copy)
    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);

    // Verify it's actually borrowing from the input
    // Length is encoded as varint - for "Hello, OxiCode! 🦀" (21 bytes), varint uses 1 byte
    assert_eq!(decoded.as_ptr(), unsafe { bytes.as_ptr().add(1) });
}

#[test]
fn test_borrow_decode_bytes() {
    let original: &[u8] = &[1, 2, 3, 4, 5, 255, 0, 128];

    // Encode to bytes (&[u8] implements Encode)
    let encoded = oxicode::encode_to_vec(&original).expect("Failed to encode");

    // Borrow decode (zero-copy)
    let (decoded, _): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(original, decoded);

    // Verify it's actually borrowing (8 bytes because length=8, varint uses 1 byte)
    assert_eq!(decoded.as_ptr(), unsafe { encoded.as_ptr().add(1) });
}

#[test]
fn test_borrow_decode_str_with_config() {
    let original = "Test with legacy config";

    let config = config::legacy();
    let bytes = oxicode::encode_to_vec_with_config(&original, config).expect("Failed to encode");

    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice_with_config(&bytes, config).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_empty_str() {
    let original = "";

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_empty_bytes() {
    let original: &[u8] = &[];

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_unicode() {
    let original = "こんにちは世界 🌍🚀✨";

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&bytes).expect("Failed to decode");

    assert_eq!(original, decoded);
}

#[test]
fn test_borrow_decode_invalid_utf8() {
    // Manually create invalid UTF-8 using standard config (varint)
    let invalid_bytes: Vec<u8> = {
        let mut buf = Vec::new();
        // Encode length as varint: 4 bytes = 0x04 (single byte)
        buf.push(4);
        // Invalid UTF-8 sequence
        buf.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]);
        buf
    };

    let result = oxicode::borrow_decode_from_slice::<&str>(&invalid_bytes);
    assert!(result.is_err(), "Expected UTF-8 validation error");
}
