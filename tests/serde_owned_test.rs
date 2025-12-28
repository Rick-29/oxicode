#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestHeader {
    element_size: usize,
    shape: Vec<usize>,
    total_elements: usize,
    metadata: Option<String>,
}

#[test]
fn test_serde_owned_roundtrip() {
    let header = TestHeader {
        element_size: 8,
        shape: vec![10, 20, 30],
        total_elements: 6000,
        metadata: Some("test data".to_string()),
    };

    // Encode
    let cfg = oxicode::config::standard();
    let bytes = oxicode::serde::encode_to_vec(&header, cfg).unwrap();
    println!("Encoded {} bytes", bytes.len());
    println!("Bytes: {:?}", &bytes[..bytes.len().min(50)]);

    // Decode with owned
    let result = oxicode::serde::decode_owned_from_slice::<TestHeader, _>(&bytes, cfg);
    match result {
        Ok((decoded, len)) => {
            println!("Decoded {} bytes", len);
            assert_eq!(header, decoded);
            assert_eq!(len, bytes.len());
        }
        Err(e) => {
            println!("Decode error: {:?}", e);
            panic!("Failed to decode");
        }
    }
}

#[test]
fn test_serde_owned_simple_struct() {
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Simple {
        a: usize,
        b: Vec<usize>,
    }

    let simple = Simple {
        a: 42,
        b: vec![1, 2, 3],
    };

    let cfg = oxicode::config::standard();
    let bytes = oxicode::serde::encode_to_vec(&simple, cfg).unwrap();
    let (decoded, _): (Simple, usize) =
        oxicode::serde::decode_owned_from_slice(&bytes, cfg).unwrap();

    assert_eq!(simple, decoded);
}

#[test]
fn test_serde_owned_vec_usize() {
    let data = vec![1usize, 2, 3, 4, 5];

    let cfg = oxicode::config::standard();
    let bytes = oxicode::serde::encode_to_vec(&data, cfg).unwrap();

    let (decoded, _): (Vec<usize>, usize) =
        oxicode::serde::decode_owned_from_slice(&bytes, cfg).unwrap();

    assert_eq!(data, decoded);
}
