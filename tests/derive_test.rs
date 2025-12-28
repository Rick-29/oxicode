//! Tests for derive macros

use oxicode::{config, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Named {
    name: String,
    age: u32,
    active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Tuple(u32, String, bool);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Unit;

#[derive(Debug, PartialEq, Encode, Decode)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Generic<T> {
    value: T,
    count: usize,
}

#[test]
fn test_struct_named_fields() {
    let point = Point { x: 1.5, y: 2.5 };

    let encoded = oxicode::encode_to_vec(&point).expect("Failed to encode");
    let (decoded, _): (Point, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(point, decoded);
}

#[test]
fn test_struct_with_string() {
    let named = Named {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };

    let encoded = oxicode::encode_to_vec(&named).expect("Failed to encode");
    let (decoded, _): (Named, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(named, decoded);
}

#[test]
fn test_tuple_struct() {
    let tuple = Tuple(42, String::from("hello"), false);

    let encoded = oxicode::encode_to_vec(&tuple).expect("Failed to encode");
    let (decoded, _): (Tuple, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(tuple, decoded);
}

#[test]
fn test_unit_struct() {
    let unit = Unit;

    let encoded = oxicode::encode_to_vec(&unit).expect("Failed to encode");
    let (decoded, _): (Unit, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(unit, decoded);
}

#[test]
fn test_enum_unit_variant() {
    let msg = Message::Quit;

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_enum_struct_variant() {
    let msg = Message::Move { x: 10, y: 20 };

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_enum_tuple_variant() {
    let msg = Message::Write(String::from("test"));

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_enum_multiple_fields() {
    let msg = Message::ChangeColor(255, 128, 64);

    let encoded = oxicode::encode_to_vec(&msg).expect("Failed to encode");
    let (decoded, _): (Message, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(msg, decoded);
}

#[test]
fn test_generic_struct() {
    let generic = Generic {
        value: 42u64,
        count: 10,
    };

    let encoded = oxicode::encode_to_vec(&generic).expect("Failed to encode");
    let (decoded, _): (Generic<u64>, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(generic, decoded);
}

#[test]
fn test_generic_with_vec() {
    let generic = Generic {
        value: vec![1, 2, 3, 4, 5],
        count: 5,
    };

    let encoded = oxicode::encode_to_vec(&generic).expect("Failed to encode");
    let (decoded, _): (Generic<Vec<i32>>, _) =
        oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(generic, decoded);
}

#[test]
fn test_nested_structs() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        value: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        inner: Inner,
        name: String,
    }

    let outer = Outer {
        inner: Inner { value: 123 },
        name: String::from("nested"),
    };

    let encoded = oxicode::encode_to_vec(&outer).expect("Failed to encode");
    let (decoded, _): (Outer, _) = oxicode::decode_from_slice(&encoded).expect("Failed to decode");

    assert_eq!(outer, decoded);
}

#[test]
fn test_config_legacy() {
    let point = Point { x: std::f32::consts::PI, y: std::f32::consts::E };

    let config = config::legacy();
    let encoded = oxicode::encode_to_vec_with_config(&point, config).expect("Failed to encode");
    let (decoded, _): (Point, _) =
        oxicode::decode_from_slice_with_config(&encoded, config).expect("Failed to decode");

    assert_eq!(point, decoded);
}
