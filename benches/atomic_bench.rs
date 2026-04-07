/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Atomic Performance Benchmarks
//!
//! Benchmarks for atomic operations to measure performance.

use qubit_atomic::AtomicI32;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Atomic Performance Benchmarks ===\n");

    // Benchmark 1: Single-threaded increment
    println!("1. Single-threaded Increment (1,000,000 operations):");
    let counter = AtomicI32::new(0);
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        counter.fetch_inc();
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );

    // Benchmark 2: Multi-threaded increment
    println!("\n2. Multi-threaded Increment (10 threads, 100,000 ops each):");
    let counter = Arc::new(AtomicI32::new(0));
    let start = std::time::Instant::now();
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                counter.fetch_inc();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );
    println!("   Final value: {}", counter.load());

    // Benchmark 3: Compare-and-swap
    println!("\n3. Compare-and-Swap (1,000,000 operations):");
    let counter = AtomicI32::new(0);
    let start = std::time::Instant::now();
    for i in 0..1_000_000 {
        while counter.compare_set(i, i + 1).is_err() {
            // Retry on failure
        }
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );

    // Benchmark 4: Functional update
    println!("\n4. Functional Update (1,000,000 operations):");
    let counter = AtomicI32::new(0);
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        counter.fetch_update(|x| x + 1);
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        1_000_000.0 / duration.as_secs_f64()
    );

    // Benchmark 5: Read operations
    println!("\n5. Read Operations (10,000,000 operations):");
    let counter = AtomicI32::new(42);
    let start = std::time::Instant::now();
    let mut sum = 0i64;
    for _ in 0..10_000_000 {
        sum += counter.load() as i64;
    }
    let duration = start.elapsed();
    println!("   Time: {:?}", duration);
    println!(
        "   Operations/sec: {:.2}",
        10_000_000.0 / duration.as_secs_f64()
    );
    println!("   Sum: {} (to prevent optimization)", sum);

    println!("\n=== Benchmarks completed ===");
}
