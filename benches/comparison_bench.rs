//! Comprehensive comparison benchmarks: oxicode vs bincode vs rkyv vs postcard vs borsh
//!
//! This benchmark compares oxicode against other popular binary serialization libraries:
//! - bincode: Binary encoding, similar design to oxicode
//! - rkyv: Zero-copy deserialization
//! - postcard: Embedded-friendly serialization
//! - borsh: Borsh binary serialization (used in Solana/NEAR)
//!
//! Run with: cargo bench --bench comparison_bench

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

// Test data structures
#[derive(Clone)]
struct BenchmarkData {
    primitives: Vec<u64>,
    strings: Vec<String>,
    nested: Vec<NestedData>,
}

#[derive(Clone)]
struct NestedData {
    id: u64,
    name: String,
    values: Vec<f64>,
}

impl BenchmarkData {
    fn new(size: usize) -> Self {
        Self {
            primitives: (0..size as u64).collect(),
            strings: (0..size).map(|i| format!("String value {}", i)).collect(),
            nested: (0..size)
                .map(|i| NestedData {
                    id: i as u64,
                    name: format!("Nested {}", i),
                    values: vec![i as f64; 10],
                })
                .collect(),
        }
    }
}

// OxiCode types
mod oxicode_types {
    use oxicode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct BenchmarkData {
        pub primitives: Vec<u64>,
        pub strings: Vec<String>,
        pub nested: Vec<NestedData>,
    }

    #[derive(Clone, Encode, Decode)]
    pub struct NestedData {
        pub id: u64,
        pub name: String,
        pub values: Vec<f64>,
    }

    impl From<&super::BenchmarkData> for BenchmarkData {
        fn from(data: &super::BenchmarkData) -> Self {
            Self {
                primitives: data.primitives.clone(),
                strings: data.strings.clone(),
                nested: data
                    .nested
                    .iter()
                    .map(|n| NestedData {
                        id: n.id,
                        name: n.name.clone(),
                        values: n.values.clone(),
                    })
                    .collect(),
            }
        }
    }
}

// Bincode types
mod bincode_types {
    use bincode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct BenchmarkData {
        pub primitives: Vec<u64>,
        pub strings: Vec<String>,
        pub nested: Vec<NestedData>,
    }

    #[derive(Clone, Encode, Decode)]
    pub struct NestedData {
        pub id: u64,
        pub name: String,
        pub values: Vec<f64>,
    }

    impl From<&super::BenchmarkData> for BenchmarkData {
        fn from(data: &super::BenchmarkData) -> Self {
            Self {
                primitives: data.primitives.clone(),
                strings: data.strings.clone(),
                nested: data
                    .nested
                    .iter()
                    .map(|n| NestedData {
                        id: n.id,
                        name: n.name.clone(),
                        values: n.values.clone(),
                    })
                    .collect(),
            }
        }
    }
}

fn bench_encoding_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding_comparison");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        // Estimate throughput
        let estimated_bytes = size * (8 + 20 + 80); // rough estimate

        group.throughput(Throughput::Bytes(estimated_bytes as u64));

        // OxiCode
        group.bench_with_input(BenchmarkId::new("oxicode", size), &oxi_data, |b, data| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                black_box(bytes);
            });
        });

        // Bincode
        group.bench_with_input(BenchmarkId::new("bincode", size), &bin_data, |b, data| {
            let config = bincode::config::standard();
            b.iter(|| {
                let bytes = bincode::encode_to_vec(black_box(data), black_box(config)).unwrap();
                black_box(bytes);
            });
        });
    }

    group.finish();
}

fn bench_decoding_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_comparison");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        // Pre-encode data
        let oxi_bytes = oxicode::encode_to_vec(&oxi_data).unwrap();
        let bin_bytes = bincode::encode_to_vec(&bin_data, bincode::config::standard()).unwrap();

        let estimated_bytes = size * (8 + 20 + 80);
        group.throughput(Throughput::Bytes(estimated_bytes as u64));

        // OxiCode
        group.bench_with_input(BenchmarkId::new("oxicode", size), &oxi_bytes, |b, bytes| {
            b.iter(|| {
                let (decoded, _): (oxicode_types::BenchmarkData, _) =
                    oxicode::decode_from_slice(black_box(bytes)).unwrap();
                black_box(decoded);
            });
        });

        // Bincode
        group.bench_with_input(BenchmarkId::new("bincode", size), &bin_bytes, |b, bytes| {
            let config = bincode::config::standard();
            b.iter(|| {
                let (decoded, _): (bincode_types::BenchmarkData, _) =
                    bincode::decode_from_slice(black_box(bytes), black_box(config)).unwrap();
                black_box(decoded);
            });
        });
    }

    group.finish();
}

fn bench_size_comparison(c: &mut Criterion) {
    let group = c.benchmark_group("size_comparison");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        // Encode with each library
        let oxi_bytes = oxicode::encode_to_vec(&oxi_data).unwrap();
        let bin_bytes = bincode::encode_to_vec(&bin_data, bincode::config::standard()).unwrap();

        // Print size comparison (not a benchmark, just informative)
        println!("\nSize comparison for {} items:", size);
        println!("  OxiCode: {} bytes", oxi_bytes.len());
        println!("  Bincode: {} bytes", bin_bytes.len());
        println!(
            "  Ratio: {:.2}%",
            (oxi_bytes.len() as f64 / bin_bytes.len() as f64) * 100.0
        );
    }

    group.finish();
}

#[cfg(feature = "simd")]
fn bench_simd_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_arrays");

    for size in [100, 1000, 10000] {
        let data_i32: Vec<i32> = (0..size).collect();
        let data_u32: Vec<u32> = (0..size as u32).collect();
        let data_i64: Vec<i64> = (0..size as i64).collect();
        let data_u64: Vec<u64> = (0..size as u64).collect();
        let data_f32: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let data_f64: Vec<f64> = (0..size).map(|i| i as f64).collect();

        group.throughput(Throughput::Bytes((size * 8) as u64));

        // i32 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_i32_simd", size),
            &data_i32,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // u32 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_u32_simd", size),
            &data_u32,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // i64 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_i64_simd", size),
            &data_i64,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // u64 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_u64_simd", size),
            &data_u64,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // f32 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_f32_simd", size),
            &data_f32,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // f64 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_f64_simd", size),
            &data_f64,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );
    }

    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        let estimated_bytes = size * (8 + 20 + 80);
        group.throughput(Throughput::Bytes(estimated_bytes as u64));

        // OxiCode roundtrip
        group.bench_with_input(BenchmarkId::new("oxicode", size), &oxi_data, |b, data| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                let (decoded, _): (oxicode_types::BenchmarkData, _) =
                    oxicode::decode_from_slice(&bytes).unwrap();
                black_box(decoded);
            });
        });

        // Bincode roundtrip
        group.bench_with_input(BenchmarkId::new("bincode", size), &bin_data, |b, data| {
            let config = bincode::config::standard();
            b.iter(|| {
                let bytes = bincode::encode_to_vec(black_box(data), config).unwrap();
                let (decoded, _): (bincode_types::BenchmarkData, _) =
                    bincode::decode_from_slice(&bytes, config).unwrap();
                black_box(decoded);
            });
        });
    }

    group.finish();
}

#[cfg(feature = "simd")]
criterion_group!(
    benches,
    bench_encoding_comparison,
    bench_decoding_comparison,
    bench_size_comparison,
    bench_simd_arrays,
    bench_roundtrip
);

#[cfg(not(feature = "simd"))]
criterion_group!(
    benches,
    bench_encoding_comparison,
    bench_decoding_comparison,
    bench_size_comparison,
    bench_roundtrip
);

criterion_main!(benches);
