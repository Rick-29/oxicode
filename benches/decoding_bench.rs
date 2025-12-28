//! Decoding benchmarks comparing oxicode to bincode

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

fn bench_primitive_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitive_decode");

    // u32 decoding
    let value = 42u32;
    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_u32_decode", |b| {
        b.iter(|| {
            let (decoded, _): (u32, _) = oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_u32_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (u32, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    // i64 decoding (zigzag)
    let value = -12345i64;
    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_i64_decode", |b| {
        b.iter(|| {
            let (decoded, _): (i64, _) = oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_i64_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (i64, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_string_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_decode");

    let value = "Hello, OxiCode! This is a test string with some content. 🦀".to_string();
    group.throughput(Throughput::Bytes(value.len() as u64));

    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(&value, bincode::config::standard()).unwrap();

    // Regular decode (with allocation)
    group.bench_function("oxicode_string_decode", |b| {
        b.iter(|| {
            let (decoded, _): (String, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_string_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (String, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    // Zero-copy decode (no allocation)
    group.bench_function("oxicode_str_borrow_decode", |b| {
        b.iter(|| {
            let (decoded, _): (&str, _) =
                oxicode::borrow_decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_str_borrow_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (&str, _) =
                bincode::borrow_decode_from_slice(black_box(&bin_bytes), black_box(config))
                    .unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_vec_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_decode");

    let value = vec![1u32; 1000];
    group.throughput(Throughput::Bytes((value.len() * 4) as u64));

    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(&value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_vec_decode", |b| {
        b.iter(|| {
            let (decoded, _): (Vec<u32>, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_vec_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (Vec<u32>, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_primitive_decode,
    bench_string_decode,
    bench_vec_decode
);
criterion_main!(benches);
