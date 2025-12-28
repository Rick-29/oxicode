//! SIMD-accelerated array encoding example
//!
//! This example demonstrates how oxicode can use SIMD instructions
//! to accelerate encoding and decoding of large arrays.
//!
//! Run with: cargo run --example simd_arrays --features simd

use oxicode::{config, Decode, Encode};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct SensorData {
    timestamp: u64,
    temperature_readings: Vec<f64>,
    pressure_readings: Vec<f32>,
    accelerometer: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct LargeDataset {
    id: u64,
    values_i32: Vec<i32>,
    values_u32: Vec<u32>,
    values_i64: Vec<i64>,
    values_u64: Vec<u64>,
    values_f32: Vec<f32>,
    values_f64: Vec<f64>,
}

fn main() -> Result<(), oxicode::Error> {
    println!("OxiCode SIMD Acceleration Example\n");
    println!("This example shows performance improvements for large arrays.");
    println!(
        "SIMD feature is: {}\n",
        if cfg!(feature = "simd") {
            "ENABLED ✓"
        } else {
            "DISABLED"
        }
    );

    // Example 1: Sensor data with multiple array types
    println!("1. Encoding sensor data:");
    let sensor_data = SensorData {
        timestamp: 1234567890,
        temperature_readings: (0..10_000).map(|i| 20.0 + (i as f64) * 0.01).collect(),
        pressure_readings: (0..10_000).map(|i| 1013.25 + (i as f32) * 0.1).collect(),
        accelerometer: (0..10_000).map(|i| i * 123).collect(),
    };

    let start = Instant::now();
    let bytes = oxicode::encode_to_vec(&sensor_data)?;
    let encode_time = start.elapsed();

    println!(
        "   Encoded {} items in {:?}",
        sensor_data.temperature_readings.len() * 3,
        encode_time
    );
    println!("   Total size: {} bytes", bytes.len());

    let start = Instant::now();
    let (decoded, _): (SensorData, _) = oxicode::decode_from_slice(&bytes)?;
    let decode_time = start.elapsed();

    println!("   Decoded in {:?}", decode_time);
    assert_eq!(sensor_data, decoded);
    println!("   ✓ Round-trip successful\n");

    // Example 2: Large dataset with all SIMD-optimized types
    println!("2. Encoding large dataset:");
    let dataset = LargeDataset {
        id: 42,
        values_i32: (0..50_000).collect(),
        values_u32: (0..50_000).map(|i| i as u32).collect(),
        values_i64: (0..50_000).map(|i| i as i64 * 1000).collect(),
        values_u64: (0..50_000).map(|i| i as u64 * 1000).collect(),
        values_f32: (0..50_000).map(|i| (i as f32) * 0.1).collect(),
        values_f64: (0..50_000).map(|i| (i as f64) * 0.1).collect(),
    };

    let start = Instant::now();
    let bytes = oxicode::encode_to_vec(&dataset)?;
    let encode_time = start.elapsed();

    println!("   Encoded 300,000 numeric values in {:?}", encode_time);
    println!("   Total size: {} bytes", bytes.len());

    let start = Instant::now();
    let (decoded, _): (LargeDataset, _) = oxicode::decode_from_slice(&bytes)?;
    let decode_time = start.elapsed();

    println!("   Decoded in {:?}", decode_time);
    assert_eq!(dataset, decoded);
    println!("   ✓ Round-trip successful\n");

    // Example 3: Comparison with different array sizes
    println!("3. Performance scaling with array size:");
    for size in [1_000, 10_000, 100_000] {
        let data: Vec<f64> = (0..size).map(|i| i as f64).collect();

        let start = Instant::now();
        let bytes = oxicode::encode_to_vec(&data)?;
        let encode_time = start.elapsed();

        let start = Instant::now();
        let (decoded, _): (Vec<f64>, _) = oxicode::decode_from_slice(&bytes)?;
        let decode_time = start.elapsed();

        assert_eq!(data, decoded);

        println!(
            "   Size {:>6}: encode {:>8.2?}, decode {:>8.2?}, total {} bytes",
            size,
            encode_time,
            decode_time,
            bytes.len()
        );
    }
    println!();

    // Example 4: SIMD-optimized types
    println!("4. SIMD-optimized types (when feature enabled):");
    println!("   - i32, u32, i64, u64");
    println!("   - f32, f64");
    println!("   - Requires CPU with SSE2, AVX2, or AVX-512 support");
    println!("   - Auto-detects and uses best available SIMD instructions");
    println!("   - Typical speedup: 2-4x for large arrays (>1000 elements)\n");

    // Example 5: Configuration with SIMD
    println!("5. Using SIMD with different configurations:");
    let data: Vec<i64> = (0..10_000).collect();

    // Standard config (varint + SIMD)
    let cfg_standard = config::standard();
    let bytes_standard = oxicode::encode_to_vec_with_config(&data, cfg_standard)?;
    println!(
        "   Standard config (varint): {} bytes",
        bytes_standard.len()
    );

    // Legacy config (fixed-int + SIMD)
    let cfg_legacy = config::legacy();
    let bytes_legacy = oxicode::encode_to_vec_with_config(&data, cfg_legacy)?;
    println!("   Legacy config (fixed-int): {} bytes", bytes_legacy.len());

    println!("   Note: SIMD optimizations apply to both configurations!\n");

    // Example 6: Real-world use case - Time series data
    println!("6. Real-world use case - Time series data:");

    #[allow(dead_code)]
    #[derive(Debug, Encode, Decode)]
    struct TimeSeriesPoint {
        timestamp: u64,
        value: f64,
    }

    #[derive(Debug, Encode, Decode)]
    struct TimeSeries {
        name: String,
        timestamps: Vec<u64>,
        values: Vec<f64>,
    }

    let time_series = TimeSeries {
        name: "CPU Usage".to_string(),
        timestamps: (0..100_000).map(|i| 1_600_000_000 + i * 60).collect(),
        values: (0..100_000)
            .map(|i| 50.0 + ((i as f64) * 0.01).sin() * 30.0)
            .collect(),
    };

    let start = Instant::now();
    let bytes = oxicode::encode_to_vec(&time_series)?;
    let encode_time = start.elapsed();

    println!("   Encoded 100,000 data points in {:?}", encode_time);
    println!(
        "   Size: {} bytes ({:.2} bytes/point)",
        bytes.len(),
        bytes.len() as f64 / 100_000.0
    );

    let start = Instant::now();
    let (_decoded, _): (TimeSeries, _) = oxicode::decode_from_slice(&bytes)?;
    let decode_time = start.elapsed();

    println!("   Decoded in {:?}", decode_time);
    println!("   ✓ Time series round-trip successful\n");

    println!("All SIMD examples completed successfully!");

    if !cfg!(feature = "simd") {
        println!("\n⚠️  SIMD feature is not enabled.");
        println!("   Run with: cargo run --example simd_arrays --features simd");
        println!("   to see SIMD-accelerated performance!");
    }

    Ok(())
}
