/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Atomic Reference Example
//!
//! Demonstrates using atomic references for lock-free data structures.

use qubit_atomic::AtomicRef;
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone)]
struct Config {
    version: u32,
    name: String,
    value: i32,
}

fn main() {
    println!("=== Atomic Reference Example ===\n");

    // Example 1: Basic reference operations
    println!("1. Basic Reference Operations:");
    let config = Arc::new(Config {
        version: 1,
        name: "initial".to_string(),
        value: 100,
    });
    let atomic_config = AtomicRef::new(config.clone());

    println!("   Initial config: {:?}", atomic_config.load());

    let new_config = Arc::new(Config {
        version: 2,
        name: "updated".to_string(),
        value: 200,
    });
    atomic_config.store(new_config);

    println!("   Updated config: {:?}", atomic_config.load());

    // Example 2: Compare-and-swap
    println!("\n2. Compare-and-Swap:");
    let config = Arc::new(Config {
        version: 1,
        name: "v1".to_string(),
        value: 10,
    });
    let atomic_config = AtomicRef::new(config.clone());

    let current = atomic_config.load();
    let new_config = Arc::new(Config {
        version: 2,
        name: "v2".to_string(),
        value: 20,
    });

    match atomic_config.compare_set(&current, new_config) {
        Ok(_) => println!("   CAS succeeded: {:?}", atomic_config.load()),
        Err(actual) => println!("   CAS failed: {:?}", actual),
    }

    // Example 3: Functional updates
    println!("\n3. Functional Updates:");
    let config = Arc::new(Config {
        version: 1,
        name: "counter".to_string(),
        value: 0,
    });
    let atomic_config = AtomicRef::new(config);

    let old = atomic_config.fetch_update(|current| {
        Arc::new(Config {
            version: current.version + 1,
            name: current.name.clone(),
            value: current.value + 10,
        })
    });

    println!("   Old config: {:?}", old);
    println!("   New config: {:?}", atomic_config.load());

    // Example 4: Multi-threaded updates
    println!("\n4. Multi-threaded Updates:");
    let config = Arc::new(Config {
        version: 0,
        name: "shared".to_string(),
        value: 0,
    });
    let atomic_config = Arc::new(AtomicRef::new(config));
    let mut handles = vec![];

    for i in 0..10 {
        let atomic_config = atomic_config.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                atomic_config.fetch_update(|current| {
                    Arc::new(Config {
                        version: current.version + 1,
                        name: current.name.clone(),
                        value: current.value + 1,
                    })
                });
            }
            println!("   Thread {} completed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_config = atomic_config.load();
    println!("   Final config: {:?}", final_config);
    println!("   Expected version: 100, actual: {}", final_config.version);
    println!("   Expected value: 100, actual: {}", final_config.value);

    println!("\n=== Example completed ===");
}
