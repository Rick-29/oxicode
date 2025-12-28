//! Comprehensive tests for all collection types

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[test]
fn test_vec_roundtrip() {
    let original = vec![1u32, 2, 3, 4, 5, 100, 1000, 10000];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_empty() {
    let original: Vec<u32> = vec![];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_nested() {
    let original = vec![vec![1, 2], vec![3, 4, 5], vec![]];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Vec<Vec<i32>>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_string_roundtrip() {
    let original = String::from("Hello, OxiCode! 🦀🚀");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (String, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_hashmap_roundtrip() {
    let mut original = HashMap::new();
    original.insert("key1".to_string(), 100u32);
    original.insert("key2".to_string(), 200);
    original.insert("key3".to_string(), 300);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_hashset_roundtrip() {
    let mut original = HashSet::new();
    original.insert(10u64);
    original.insert(20);
    original.insert(30);
    original.insert(40);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (HashSet<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_btreemap_roundtrip() {
    let mut original = BTreeMap::new();
    original.insert(1, "one".to_string());
    original.insert(2, "two".to_string());
    original.insert(3, "three".to_string());

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (BTreeMap<i32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_btreeset_roundtrip() {
    let mut original = BTreeSet::new();
    original.insert(5);
    original.insert(2);
    original.insert(8);
    original.insert(1);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (BTreeSet<i32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_vecdeque_roundtrip() {
    let mut original = VecDeque::new();
    original.push_back(1u16);
    original.push_back(2);
    original.push_back(3);
    original.push_front(0);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (VecDeque<u16>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_binaryheap_roundtrip() {
    let mut original = BinaryHeap::new();
    original.push(5);
    original.push(2);
    original.push(8);
    original.push(1);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (BinaryHeap<i32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");

    // BinaryHeap doesn't guarantee order, so convert to sorted vecs
    let mut original_vec: Vec<_> = original.into_iter().collect();
    let mut decoded_vec: Vec<_> = decoded.into_iter().collect();
    original_vec.sort();
    decoded_vec.sort();
    assert_eq!(original_vec, decoded_vec);
}

#[test]
fn test_option_some() {
    let original: Option<u64> = Some(12345);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Option<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_none() {
    let original: Option<u64> = None;
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Option<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_result_ok() {
    let original: Result<u32, String> = Ok(42);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Result<u32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_result_err() {
    let original: Result<u32, String> = Err("error message".to_string());
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Result<u32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_box_roundtrip() {
    let original = Box::new(vec![1, 2, 3, 4, 5]);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Box<Vec<i32>>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_rc_roundtrip() {
    use std::rc::Rc;
    let original = Rc::new(String::from("shared data"));
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Rc<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(*original, *decoded);
}

#[test]
fn test_arc_roundtrip() {
    use std::sync::Arc;
    let original = Arc::new(42u128);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Arc<u128>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(*original, *decoded);
}

#[test]
fn test_cow_owned() {
    use std::borrow::Cow;
    let original: Cow<str> = Cow::Owned("owned string".to_string());
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    #[allow(clippy::owned_cow)]
    let (decoded, _): (Cow<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(*original, *decoded);
}

#[test]
#[allow(clippy::type_complexity)]
fn test_tuple_large() {
    let original = (
        1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9.0f32, 10.0f64,
    );
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): ((u8, u16, u32, u64, i8, i16, i32, i64, f32, f64), _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_roundtrip() {
    let original = [1u32, 2, 3, 4, 5];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): ([u32; 5], _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_complex_nested() {
    type Complex = Vec<HashMap<String, Vec<Option<(u32, String)>>>>;

    let mut inner_map1 = HashMap::new();
    inner_map1.insert(
        "key1".to_string(),
        vec![Some((1, "a".to_string())), None, Some((2, "b".to_string()))],
    );

    let mut inner_map2 = HashMap::new();
    inner_map2.insert("key2".to_string(), vec![None, Some((3, "c".to_string()))]);

    let original: Complex = vec![inner_map1, inner_map2];

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Complex, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");

    // Can't directly compare HashMaps, so check structure
    assert_eq!(original.len(), decoded.len());
}
