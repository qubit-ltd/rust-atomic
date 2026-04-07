/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use qubit_atomic::atomic::{
    Atomic,
    AtomicF64,
    AtomicNumber,
};
use std::sync::Arc;
use std::thread;

const EPSILON: f64 = 1e-10;

#[test]
fn test_new() {
    let atomic = AtomicF64::new(std::f64::consts::PI);
    assert!((atomic.load() - std::f64::consts::PI).abs() < EPSILON);
}

#[test]
fn test_default() {
    let atomic = AtomicF64::default();
    assert_eq!(atomic.load(), 0.0);
}

#[test]
fn test_from() {
    let atomic = AtomicF64::from(std::f64::consts::E);
    assert!((atomic.load() - std::f64::consts::E).abs() < EPSILON);
}

#[test]
fn test_get_set() {
    let atomic = AtomicF64::new(0.0);
    atomic.store(std::f64::consts::PI);
    assert!((atomic.load() - std::f64::consts::PI).abs() < EPSILON);
    atomic.store(-2.5);
    assert!((atomic.load() - (-2.5)).abs() < EPSILON);
}

#[test]
fn test_swap() {
    let atomic = AtomicF64::new(1.0);
    let old = atomic.swap(2.0);
    assert!((old - 1.0).abs() < EPSILON);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_success() {
    let atomic = AtomicF64::new(1.0);
    assert!(atomic.compare_set(1.0, 2.0).is_ok());
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_failure() {
    let atomic = AtomicF64::new(1.0);
    match atomic.compare_set(1.5, 2.0) {
        Ok(_) => panic!("Should fail"),
        Err(actual) => assert!((actual - 1.0).abs() < EPSILON),
    }
    assert!((atomic.load() - 1.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange() {
    let atomic = AtomicF64::new(1.0);
    let prev = atomic.compare_exchange(1.0, 2.0);
    assert!((prev - 1.0).abs() < EPSILON);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_add() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_add(5.5);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.5).abs() < EPSILON);
}

#[test]
fn test_sub() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_sub(3.5);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 6.5).abs() < EPSILON);
}

#[test]
fn test_mul() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_mul(2.5);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 25.0).abs() < EPSILON);
}

#[test]
fn test_div() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_div(2.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 5.0).abs() < EPSILON);
}

#[test]
fn test_get_and_update() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_update(|x| x * 2.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 20.0).abs() < EPSILON);
}

#[test]
fn test_concurrent_add() {
    let sum = Arc::new(AtomicF64::new(0.0));
    let mut handles = vec![];

    for _ in 0..10 {
        let sum = sum.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                sum.fetch_add(0.01);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Due to floating point precision, result may not be exactly 10.0
    let result = sum.load();
    assert!((result - 10.0).abs() < 0.01);
}

#[test]
fn test_trait_atomic() {
    fn test_atomic<T: Atomic<Value = f64>>(atomic: &T) {
        atomic.store(std::f64::consts::PI);
        assert!((atomic.load() - std::f64::consts::PI).abs() < EPSILON);
        let old = atomic.swap(std::f64::consts::E);
        assert!((old - std::f64::consts::PI).abs() < EPSILON);
    }

    let atomic = AtomicF64::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_set_weak() {
    fn test_atomic<T: Atomic<Value = f64>>(atomic: &T) {
        atomic.store(1.0);
        assert!(atomic.compare_set_weak(1.0, 2.0).is_ok());
        assert!((atomic.load() - 2.0).abs() < EPSILON);
    }

    let atomic = AtomicF64::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_exchange_weak() {
    fn test_atomic<T: Atomic<Value = f64>>(atomic: &T) {
        atomic.store(1.0);
        let prev = atomic.compare_exchange_weak(1.0, 2.0);
        assert!((prev - 1.0).abs() < EPSILON);
        assert!((atomic.load() - 2.0).abs() < EPSILON);
    }

    let atomic = AtomicF64::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_fetch_update() {
    fn test_atomic<T: Atomic<Value = f64>>(atomic: &T) {
        atomic.store(10.0);
        let old = atomic.fetch_update(|x| x * 2.0);
        assert!((old - 10.0).abs() < EPSILON);
        assert!((atomic.load() - 20.0).abs() < EPSILON);
    }

    let atomic = AtomicF64::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_debug_display() {
    let atomic = AtomicF64::new(std::f64::consts::PI);
    let debug_str = format!("{:?}", atomic);
    assert!(debug_str.contains("3.14"));
    let display_str = format!("{}", atomic);
    assert!(display_str.contains("3.14"));
}

#[test]
fn test_negative_values() {
    let atomic = AtomicF64::new(-10.5);
    assert!((atomic.load() - (-10.5)).abs() < EPSILON);
    atomic.fetch_add(5.5);
    assert!((atomic.load() - (-5.0)).abs() < EPSILON);
}

#[test]
fn test_zero() {
    let atomic = AtomicF64::new(0.0);
    assert_eq!(atomic.load(), 0.0);
    atomic.fetch_add(1.0);
    assert!((atomic.load() - 1.0).abs() < EPSILON);
}

#[test]
fn test_infinity() {
    let atomic = AtomicF64::new(f64::INFINITY);
    assert_eq!(atomic.load(), f64::INFINITY);
    atomic.store(f64::NEG_INFINITY);
    assert_eq!(atomic.load(), f64::NEG_INFINITY);
}

#[test]
fn test_high_precision() {
    let atomic = AtomicF64::new(1.23456789012345);
    assert!((atomic.load() - 1.23456789012345).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_weak() {
    let atomic = AtomicF64::new(1.0);
    assert!(atomic.compare_set_weak(1.0, 2.0).is_ok());
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak() {
    let atomic = AtomicF64::new(1.0);
    let prev = atomic.compare_and_exchange_weak(1.0, 2.0);
    assert!((prev - 1.0).abs() < EPSILON);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_inner() {
    use std::sync::atomic::Ordering;

    let atomic = AtomicF64::new(1.0);
    let bits = atomic.inner().load(Ordering::Relaxed);
    assert_eq!(f64::from_bits(bits), 1.0);

    atomic.inner().store(2.0f64.to_bits(), Ordering::Release);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_inner_cas() {
    use std::sync::atomic::Ordering;

    let atomic = AtomicF64::new(1.0);
    let current_bits = atomic.inner().load(Ordering::Relaxed);
    let new_bits = 2.0f64.to_bits();

    atomic
        .inner()
        .compare_exchange(current_bits, new_bits, Ordering::AcqRel, Ordering::Acquire)
        .unwrap();

    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_nan() {
    let atomic = AtomicF64::new(f64::NAN);
    assert!(atomic.load().is_nan());
}

#[test]
fn test_sub_negative() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_sub(-5.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_mul_negative() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_mul(-2.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - (-20.0)).abs() < EPSILON);
}

#[test]
fn test_div_by_zero() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_div(0.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!(atomic.load().is_infinite());
}

#[test]
fn test_compare_and_set_failure_returns_actual() {
    let atomic = AtomicF64::new(1.0);
    match atomic.compare_set(2.0, 3.0) {
        Ok(_) => panic!("Should fail"),
        Err(actual) => assert!((actual - 1.0).abs() < EPSILON),
    }
}

#[test]
fn test_concurrent_mul() {
    let value = Arc::new(AtomicF64::new(1.0));
    let mut handles = vec![];

    for _ in 0..5 {
        let value = value.clone();
        let handle = thread::spawn(move || {
            value.fetch_mul(2.0);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be 2^5 = 32
    let result = value.load();
    assert!((result - 32.0).abs() < 0.01);
}

#[test]
fn test_concurrent_div() {
    let value = Arc::new(AtomicF64::new(1024.0));
    let mut handles = vec![];

    for _ in 0..5 {
        let value = value.clone();
        let handle = thread::spawn(move || {
            value.fetch_div(2.0);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be 1024 / 2^5 = 32
    let result = value.load();
    assert!((result - 32.0).abs() < 0.01);
}

#[test]
fn test_concurrent_sub() {
    let value = Arc::new(AtomicF64::new(1000.0));
    let mut handles = vec![];

    for _ in 0..10 {
        let value = value.clone();
        let handle = thread::spawn(move || {
            value.fetch_sub(10.0);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be around 900
    let result = value.load();
    assert!((result - 900.0).abs() < 1.0);
}

#[test]
fn test_compare_and_set_weak_in_loop() {
    let atomic = AtomicF64::new(0.0);
    let mut current = atomic.load();
    for i in 0..10 {
        loop {
            match atomic.compare_set_weak(current, (i + 1) as f64) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
        current = (i + 1) as f64;
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak_in_loop() {
    let atomic = AtomicF64::new(0.0);
    let mut current = atomic.load();
    for i in 0..10 {
        loop {
            let prev = atomic.compare_and_exchange_weak(current, (i + 1) as f64);
            if (prev - current).abs() < EPSILON {
                break;
            }
            current = prev;
        }
        current = (i + 1) as f64;
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_add_zero() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_add(0.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_sub_zero() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_sub(0.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_mul_one() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_mul(1.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_div_one() {
    let atomic = AtomicF64::new(10.0);
    let old = atomic.fetch_div(1.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_display() {
    let atomic = AtomicF64::new(std::f64::consts::PI);
    let display_str = format!("{}", atomic);
    assert!(display_str.contains("3.14"));
}

#[test]
fn test_debug_false() {
    let atomic = AtomicF64::new(0.0);
    let debug_str = format!("{:?}", atomic);
    assert!(debug_str.contains("0"));
}

// TODO: Rewrite after UpdatableAtomic removal
// #[test]
// fn test_trait_updatable_atomic_comprehensive() {
//     // Test needs to be rewritten for fetch_update
// }

#[test]
fn test_trait_atomic_comprehensive() {
    fn test_atomic<T: Atomic<Value = f64>>(atomic: &T) {
        atomic.store(5.0);
        assert!((atomic.load() - 5.0).abs() < EPSILON);

        let old = atomic.swap(10.0);
        assert!((old - 5.0).abs() < EPSILON);

        assert!(atomic.compare_set(10.0, 15.0).is_ok());
        assert_eq!(atomic.compare_exchange(15.0, 20.0), 15.0);
    }

    let atomic = AtomicF64::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_get_and_update_identity() {
    let atomic = AtomicF64::new(42.0);
    let old = atomic.fetch_update(|x| x);
    assert!((old - 42.0).abs() < EPSILON);
    assert!((atomic.load() - 42.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_failure_path() {
    let atomic = AtomicF64::new(10.0);
    // Try to CAS with wrong current value
    match atomic.compare_set(5.0, 15.0) {
        Ok(_) => panic!("Should have failed"),
        Err(actual) => assert!((actual - 10.0).abs() < EPSILON),
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_failure_path() {
    let atomic = AtomicF64::new(10.0);
    let prev = atomic.compare_exchange(5.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_weak_failure_path() {
    let atomic = AtomicF64::new(10.0);
    match atomic.compare_set_weak(5.0, 15.0) {
        Ok(_) => panic!("Should have failed"),
        Err(actual) => assert!((actual - 10.0).abs() < EPSILON),
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak_failure_path() {
    let atomic = AtomicF64::new(10.0);
    let prev = atomic.compare_and_exchange_weak(5.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_success_path() {
    let atomic = AtomicF64::new(10.0);
    let prev = atomic.compare_exchange(10.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak_success_path() {
    let atomic = AtomicF64::new(10.0);
    let prev = atomic.compare_and_exchange_weak(10.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_concurrent_add_high_contention() {
    let atomic = Arc::new(AtomicF64::new(0.0));
    let mut handles = vec![];

    // High contention: many threads adding simultaneously
    for _ in 0..20 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..50 {
                atomic.fetch_add(0.1);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be around 100.0 (20 * 50 * 0.1)
    let result = atomic.load();
    assert!((result - 100.0).abs() < 0.5);
}

#[test]
fn test_concurrent_sub_high_contention() {
    let atomic = Arc::new(AtomicF64::new(1000.0));
    let mut handles = vec![];

    // High contention: many threads subtracting simultaneously
    for _ in 0..20 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                atomic.fetch_sub(1.0);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be around 800.0 (1000 - 20 * 10)
    let result = atomic.load();
    assert!((result - 800.0).abs() < 1.0);
}

#[test]
fn test_concurrent_mul_and_div() {
    let atomic = Arc::new(AtomicF64::new(100.0));
    let mut handles = vec![];

    // Some threads multiply, some divide
    for i in 0..10 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                atomic.fetch_mul(1.1);
            } else {
                atomic.fetch_div(1.1);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be close to original (5 muls and 5 divs)
    let result = atomic.load();
    assert!(result > 50.0 && result < 200.0);
}

#[test]
fn test_concurrent_sub_extreme_contention() {
    let atomic = Arc::new(AtomicF64::new(10000.0));
    let mut handles = vec![];

    // Very high contention: 30 threads, each doing 20 subtractions
    for _ in 0..30 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                atomic.fetch_sub(0.1);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be 10000.0 - 30 * 20 * 0.1 = 9940.0
    let result = atomic.load();
    assert!((result - 9940.0).abs() < 1.0);
}

#[test]
fn test_concurrent_mul_extreme_contention() {
    let atomic = Arc::new(AtomicF64::new(1.0));
    let mut handles = vec![];

    // Very high contention: 30 threads, each doing 20 multiplications
    for _ in 0..30 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                atomic.fetch_mul(1.001);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be greater than 1.0
    let result = atomic.load();
    assert!(result > 1.0);
}

#[test]
fn test_concurrent_div_extreme_contention() {
    let atomic = Arc::new(AtomicF64::new(1000000.0));
    let mut handles = vec![];

    // Very high contention: 30 threads, each doing 20 divisions
    for _ in 0..30 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                atomic.fetch_div(1.001);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be less than original
    let result = atomic.load();
    assert!(result < 1000000.0);
}

#[test]
fn test_concurrent_get_and_update_extreme_contention() {
    let atomic = Arc::new(AtomicF64::new(0.0));
    let mut handles = vec![];

    // Very high contention: 30 threads, each doing 20 updates
    for _ in 0..30 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                atomic.fetch_update(|x| x + 0.1);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be 30 * 20 * 0.1 = 60.0
    let result = atomic.load();
    assert!((result - 60.0).abs() < 1.0);
}

#[test]
fn test_concurrent_get_and_update_contention() {
    let atomic = Arc::new(AtomicF64::new(0.0));
    let mut handles = vec![];

    for _ in 0..10 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                atomic.fetch_update(|x| x + 0.5);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be 10 * 20 * 0.5 = 100.0
    let result = atomic.load();
    assert!((result - 100.0).abs() < 0.5);
}

// ============================================================================
// AtomicNumber trait tests
// ============================================================================

#[test]
fn test_atomic_number_fetch_add() {
    let atomic = AtomicF64::new(10.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_add(&atomic, 5.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_add_negative() {
    let atomic = AtomicF64::new(10.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_add(&atomic, -3.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 7.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_sub() {
    let atomic = AtomicF64::new(10.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_sub(&atomic, 3.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 7.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_sub_negative() {
    let atomic = AtomicF64::new(10.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_sub(&atomic, -5.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_mul() {
    let atomic = AtomicF64::new(3.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_mul(&atomic, 4.0);
    assert!((old - 3.0).abs() < EPSILON);
    assert!((atomic.load() - 12.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_mul_by_zero() {
    let atomic = AtomicF64::new(5.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_mul(&atomic, 0.0);
    assert!((old - 5.0).abs() < EPSILON);
    assert!((atomic.load() - 0.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_mul_by_negative() {
    let atomic = AtomicF64::new(3.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_mul(&atomic, -2.0);
    assert!((old - 3.0).abs() < EPSILON);
    assert!((atomic.load() - (-6.0)).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_div() {
    let atomic = AtomicF64::new(12.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_div(&atomic, 4.0);
    assert!((old - 12.0).abs() < EPSILON);
    assert!((atomic.load() - 3.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_div_by_negative() {
    let atomic = AtomicF64::new(12.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_div(&atomic, -4.0);
    assert!((old - 12.0).abs() < EPSILON);
    assert!((atomic.load() - (-3.0)).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_div_fractional() {
    let atomic = AtomicF64::new(10.0);
    let old = <AtomicF64 as AtomicNumber>::fetch_div(&atomic, 3.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - (10.0 / 3.0)).abs() < EPSILON);
}

#[test]
fn test_atomic_number_operations_chain() {
    let atomic = AtomicF64::new(10.0);

    // 10.0 + 5.0 = 15.0
    <AtomicF64 as AtomicNumber>::fetch_add(&atomic, 5.0);
    assert!((atomic.load() - 15.0).abs() < EPSILON);

    // 15.0 * 2.0 = 30.0
    <AtomicF64 as AtomicNumber>::fetch_mul(&atomic, 2.0);
    assert!((atomic.load() - 30.0).abs() < EPSILON);

    // 30.0 - 10.0 = 20.0
    <AtomicF64 as AtomicNumber>::fetch_sub(&atomic, 10.0);
    assert!((atomic.load() - 20.0).abs() < EPSILON);

    // 20.0 / 4.0 = 5.0
    <AtomicF64 as AtomicNumber>::fetch_div(&atomic, 4.0);
    assert!((atomic.load() - 5.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_concurrent_operations() {
    let atomic = Arc::new(AtomicF64::new(0.0));
    let mut handles = vec![];

    // Start multiple threads for concurrent operations
    for i in 0..10 {
        let atomic = Arc::clone(&atomic);
        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                <AtomicF64 as AtomicNumber>::fetch_add(&atomic, 1.0);
            } else {
                <AtomicF64 as AtomicNumber>::fetch_sub(&atomic, 0.5);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 5 threads add 1.0, 5 threads subtract 0.5, result should be 5.0 - 2.5 = 2.5
    let result = atomic.load();
    assert!((result - 2.5).abs() < EPSILON);
}
