/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Atomic Boolean Flag Example
//!
//! Demonstrates using atomic booleans for thread synchronization.

use qubit_atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Atomic Boolean Flag Example ===\n");

    // Example 1: Simple flag
    println!("1. Simple Flag:");
    let flag = AtomicBool::new(false);
    println!("   Initial value: {}", flag.load());

    flag.store(true);
    println!("   After set(true): {}", flag.load());

    flag.fetch_not();
    println!("   After negate: {}", flag.load());

    // Example 2: One-time initialization
    println!("\n2. One-time Initialization:");
    let initialized = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    for i in 0..5 {
        let initialized = initialized.clone();
        let handle = thread::spawn(move || {
            if initialized.set_if_false(true).is_ok() {
                println!("   Thread {} performed initialization", i);
                thread::sleep(Duration::from_millis(100));
            } else {
                println!("   Thread {} skipped (already initialized)", i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("   Final state: initialized = {}", initialized.load());

    // Example 3: Producer-Consumer signaling
    println!("\n3. Producer-Consumer Signaling:");
    let ready = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicBool::new(false));

    let ready_clone = ready.clone();
    let data_clone = data.clone();

    // Producer thread
    let producer = thread::spawn(move || {
        println!("   Producer: preparing data...");
        thread::sleep(Duration::from_millis(100));
        data_clone.store(true);
        ready_clone.store(true);
        println!("   Producer: data ready!");
    });

    // Consumer thread
    let consumer = thread::spawn(move || {
        println!("   Consumer: waiting for data...");
        while !ready.load() {
            thread::yield_now();
        }
        println!("   Consumer: received data = {}", data.load());
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    // Example 4: Toggle operations
    println!("\n4. Toggle Operations:");
    let flag = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    for i in 0..10 {
        let flag = flag.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                flag.fetch_not();
            }
            println!("   Thread {} completed 10 toggles", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("   Final state: {} (after 100 toggles)", flag.load());

    println!("\n=== Example completed ===");
}
