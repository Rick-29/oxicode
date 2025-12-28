//! Encoding benchmarks comparing oxicode to bincode

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

// Benchmark data structures
mod oxicode_types {
    use oxicode::{Decode, Encode};

    #[derive(Encode, Decode)]
    pub struct SmallStruct {
        pub id: u32,
        pub value: i32,
    }

    #[derive(Encode, Decode)]
    pub struct MediumStruct {
        pub id: u64,
        pub name: String,
        pub values: Vec<i32>,
        pub active: bool,
    }

    #[allow(dead_code)]
    #[derive(Encode, Decode)]
    pub struct LargeStruct {
        pub id: u64,
        pub title: String,
        pub description: String,
        pub tags: Vec<String>,
        pub data: Vec<u8>,
        pub metadata: Vec<(String, String)>,
    }
}

mod bincode_types {
    use bincode::{Decode, Encode};

    #[derive(Encode, Decode)]
    pub struct SmallStruct {
        pub id: u32,
        pub value: i32,
    }

    #[derive(Encode, Decode)]
    pub struct MediumStruct {
        pub id: u64,
        pub name: String,
        pub values: Vec<i32>,
        pub active: bool,
    }

    #[allow(dead_code)]
    #[derive(Encode, Decode)]
    pub struct LargeStruct {
        pub id: u64,
        pub title: String,
        pub description: String,
        pub tags: Vec<String>,
        pub data: Vec<u8>,
        pub metadata: Vec<(String, String)>,
    }
}

fn bench_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitives");

    // u32 encoding
    group.bench_function("oxicode_u32", |b| {
        let value = 42u32;
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&value)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_u32", |b| {
        let value = 42u32;
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&value), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // i64 encoding (varint)
    group.bench_function("oxicode_i64", |b| {
        let value = -12345i64;
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&value)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_i64", |b| {
        let value = -12345i64;
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&value), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    group.finish();
}

fn bench_collections(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections");

    // Vec<u32>
    let vec_data = vec![1u32, 2, 3, 4, 5, 10, 20, 30, 40, 50];
    group.throughput(Throughput::Bytes((vec_data.len() * 4) as u64));

    group.bench_function("oxicode_vec", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&vec_data)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_vec", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&vec_data), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // String
    let string_data = "Hello, OxiCode! This is a test string with some content. 🦀".to_string();
    group.throughput(Throughput::Bytes(string_data.len() as u64));

    group.bench_function("oxicode_string", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&string_data)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_string", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&string_data), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    group.finish();
}

fn bench_structs(c: &mut Criterion) {
    let mut group = c.benchmark_group("structs");

    // Small struct
    let oxi_small = oxicode_types::SmallStruct {
        id: 123,
        value: -456,
    };
    let bin_small = bincode_types::SmallStruct {
        id: 123,
        value: -456,
    };

    group.bench_function("oxicode_small_struct", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&oxi_small)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_small_struct", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&bin_small), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // Medium struct
    let oxi_medium = oxicode_types::MediumStruct {
        id: 999,
        name: "benchmark_data".to_string(),
        values: vec![1, 2, 3, 4, 5],
        active: true,
    };
    let bin_medium = bincode_types::MediumStruct {
        id: 999,
        name: "benchmark_data".to_string(),
        values: vec![1, 2, 3, 4, 5],
        active: true,
    };

    group.bench_function("oxicode_medium_struct", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&oxi_medium)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_medium_struct", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&bin_medium), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_primitives, bench_collections, bench_structs);
criterion_main!(benches);
