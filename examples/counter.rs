/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Atomic Counter Example
//!
//! Demonstrates using atomic integers as thread-safe counters.

use qubit_atomic::Atomic;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Atomic Counter Example ===\n");

    // Example 1: Basic counter operations
    println!("1. Basic Counter Operations:");
    let counter = Atomic::<i32>::new(0);
    println!("   Initial value: {}", counter.load());

    counter.fetch_inc();
    println!("   After increment: {}", counter.load());

    counter.fetch_add(5);
    println!("   After adding 5: {}", counter.load());

    counter.fetch_dec();
    println!("   After decrement: {}", counter.load());

    // Example 2: Multi-threaded counter
    println!("\n2. Multi-threaded Counter:");
    let counter = Arc::new(Atomic::<i32>::new(0));
    let num_threads = 10;
    let increments_per_thread = 1000;

    let mut handles = vec![];
    for i in 0..num_threads {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                counter.fetch_inc();
            }
            println!("   Thread {} completed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "   Final count: {} (expected: {})",
        counter.load(),
        num_threads * increments_per_thread
    );

    // Example 3: Compare-and-swap
    println!("\n3. Compare-and-Swap:");
    let counter = Atomic::<i32>::new(10);
    println!("   Initial value: {}", counter.load());

    match counter.compare_set(10, 20) {
        Ok(_) => println!("   CAS succeeded: value is now {}", counter.load()),
        Err(actual) => println!("   CAS failed: actual value was {}", actual),
    }

    match counter.compare_set(10, 30) {
        Ok(_) => println!("   CAS succeeded: value is now {}", counter.load()),
        Err(actual) => println!("   CAS failed: actual value was {}", actual),
    }

    // Example 4: Functional updates
    println!("\n4. Functional Updates:");
    let counter = Atomic::<i32>::new(5);
    println!("   Initial value: {}", counter.load());

    let old = counter.fetch_update(|x| x * 2);
    println!("   After doubling - old: {}, new: {}", old, counter.load());

    let new = counter.fetch_update(|x| x + 10);
    println!("   After adding 10 - new: {}", new);

    // Example 5: Accumulate operations
    println!("\n5. Accumulate Operations:");
    let counter = Atomic::<i32>::new(1);
    println!("   Initial value: {}", counter.load());

    let old = counter.fetch_accumulate(2, |a, b| a * b);
    println!(
        "   After multiplying by 2 - old: {}, new: {}",
        old,
        counter.load()
    );

    let new = counter.fetch_accumulate(3, |a, b| a + b);
    println!("   After adding 3 - new: {}", new);

    println!("\n=== Example completed ===");
}
