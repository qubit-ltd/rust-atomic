/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use qubit_atomic::Atomic;
use std::sync::Arc;
use std::thread;

const EPSILON: f32 = 1e-6;

#[test]
fn test_new() {
    let atomic = Atomic::<f32>::new(std::f32::consts::PI);
    assert!((atomic.load() - std::f32::consts::PI).abs() < EPSILON);
}

#[test]
fn test_default() {
    let atomic = Atomic::<f32>::default();
    assert_eq!(atomic.load(), 0.0);
}

#[test]
fn test_from() {
    let atomic = Atomic::<f32>::from(2.71);
    assert!((atomic.load() - 2.71).abs() < EPSILON);
}

#[test]
fn test_get_set() {
    let atomic = Atomic::<f32>::new(0.0);
    atomic.store(std::f32::consts::PI);
    assert!((atomic.load() - std::f32::consts::PI).abs() < EPSILON);
    atomic.store(-2.5);
    assert!((atomic.load() - (-2.5)).abs() < EPSILON);
}

#[test]
fn test_swap() {
    let atomic = Atomic::<f32>::new(1.0);
    let old = atomic.swap(2.0);
    assert!((old - 1.0).abs() < EPSILON);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_success() {
    let atomic = Atomic::<f32>::new(1.0);
    assert!(atomic.compare_set(1.0, 2.0).is_ok());
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_failure() {
    let atomic = Atomic::<f32>::new(1.0);
    match atomic.compare_set(1.5, 2.0) {
        Ok(_) => panic!("Should fail"),
        Err(actual) => assert!((actual - 1.0).abs() < EPSILON),
    }
    assert!((atomic.load() - 1.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange() {
    let atomic = Atomic::<f32>::new(1.0);
    let prev = atomic.compare_and_exchange(1.0, 2.0);
    assert!((prev - 1.0).abs() < EPSILON);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_add() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_add(5.5);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.5).abs() < EPSILON);
}

#[test]
fn test_sub() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_sub(3.5);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 6.5).abs() < EPSILON);
}

#[test]
fn test_mul() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_mul(2.5);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 25.0).abs() < EPSILON);
}

#[test]
fn test_div() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_div(2.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 5.0).abs() < EPSILON);
}

#[test]
fn test_get_and_update() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_update(|x| x * 2.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 20.0).abs() < EPSILON);
}

#[test]
fn test_concurrent_add() {
    let sum = Arc::new(Atomic::<f32>::new(0.0));
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
    fn test_atomic(atomic: &Atomic<f32>) {
        atomic.store(std::f32::consts::PI);
        assert!((atomic.load() - std::f32::consts::PI).abs() < EPSILON);
        let old = atomic.swap(2.71);
        assert!((old - std::f32::consts::PI).abs() < EPSILON);
    }

    let atomic = Atomic::<f32>::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_set_weak() {
    fn test_atomic(atomic: &Atomic<f32>) {
        atomic.store(1.0);
        assert!(atomic.compare_set_weak(1.0, 2.0).is_ok());
        assert!((atomic.load() - 2.0).abs() < EPSILON);
    }

    let atomic = Atomic::<f32>::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_exchange_weak() {
    fn test_atomic(atomic: &Atomic<f32>) {
        atomic.store(1.0);
        let prev = atomic.compare_and_exchange_weak(1.0, 2.0);
        assert!((prev - 1.0).abs() < EPSILON);
        assert!((atomic.load() - 2.0).abs() < EPSILON);
    }

    let atomic = Atomic::<f32>::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_fetch_update() {
    fn test_atomic(atomic: &Atomic<f32>) {
        atomic.store(10.0);
        let old = atomic.fetch_update(|x| x * 2.0);
        assert!((old - 10.0).abs() < EPSILON);
        assert!((atomic.load() - 20.0).abs() < EPSILON);
    }

    let atomic = Atomic::<f32>::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_debug_display() {
    let atomic = Atomic::<f32>::new(std::f32::consts::PI);
    let debug_str = format!("{:?}", atomic);
    assert!(debug_str.contains("3.14"));
    let display_str = format!("{}", atomic);
    assert!(display_str.contains("3.14"));
}

#[test]
fn test_negative_values() {
    let atomic = Atomic::<f32>::new(-10.5);
    assert!((atomic.load() - (-10.5)).abs() < EPSILON);
    atomic.fetch_add(5.5);
    assert!((atomic.load() - (-5.0)).abs() < EPSILON);
}

#[test]
fn test_zero() {
    let atomic = Atomic::<f32>::new(0.0);
    assert_eq!(atomic.load(), 0.0);
    atomic.fetch_add(1.0);
    assert!((atomic.load() - 1.0).abs() < EPSILON);
}

#[test]
fn test_infinity() {
    let atomic = Atomic::<f32>::new(f32::INFINITY);
    assert_eq!(atomic.load(), f32::INFINITY);
    atomic.store(f32::NEG_INFINITY);
    assert_eq!(atomic.load(), f32::NEG_INFINITY);
}

#[test]
fn test_compare_and_set_weak() {
    let atomic = Atomic::<f32>::new(1.0);
    assert!(atomic.compare_set_weak(1.0, 2.0).is_ok());
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak() {
    let atomic = Atomic::<f32>::new(1.0);
    let prev = atomic.compare_and_exchange_weak(1.0, 2.0);
    assert!((prev - 1.0).abs() < EPSILON);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_inner() {
    use std::sync::atomic::Ordering;

    let atomic = Atomic::<f32>::new(1.0);
    let bits = atomic.inner().load(Ordering::Relaxed);
    assert_eq!(f32::from_bits(bits), 1.0);

    atomic.inner().store(2.0f32.to_bits(), Ordering::Release);
    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_inner_cas() {
    use std::sync::atomic::Ordering;

    let atomic = Atomic::<f32>::new(1.0);
    let current_bits = atomic.inner().load(Ordering::Relaxed);
    let new_bits = 2.0f32.to_bits();

    atomic
        .inner()
        .compare_exchange(current_bits, new_bits, Ordering::AcqRel, Ordering::Acquire)
        .unwrap();

    assert!((atomic.load() - 2.0).abs() < EPSILON);
}

#[test]
fn test_nan() {
    let atomic = Atomic::<f32>::new(f32::NAN);
    assert!(atomic.load().is_nan());
}

#[test]
fn test_sub_negative() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_sub(-5.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_mul_negative() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_mul(-2.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - (-20.0)).abs() < EPSILON);
}

#[test]
fn test_div_by_zero() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_div(0.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!(atomic.load().is_infinite());
}

#[test]
fn test_compare_and_set_failure_returns_actual() {
    let atomic = Atomic::<f32>::new(1.0);
    match atomic.compare_set(2.0, 3.0) {
        Ok(_) => panic!("Should fail"),
        Err(actual) => assert!((actual - 1.0).abs() < EPSILON),
    }
}

#[test]
fn test_concurrent_mul() {
    let value = Arc::new(Atomic::<f32>::new(1.0));
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
    let value = Arc::new(Atomic::<f32>::new(1024.0));
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
fn test_compare_and_set_weak_in_loop() {
    let atomic = Atomic::<f32>::new(0.0);
    let mut current = atomic.load();
    for i in 0..10 {
        loop {
            match atomic.compare_set_weak(current, (i + 1) as f32) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
        current = (i + 1) as f32;
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak_in_loop() {
    let atomic = Atomic::<f32>::new(0.0);
    let mut current = atomic.load();
    for i in 0..10 {
        loop {
            let prev = atomic.compare_and_exchange_weak(current, (i + 1) as f32);
            if (prev - current).abs() < EPSILON {
                break;
            }
            current = prev;
        }
        current = (i + 1) as f32;
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_add_zero() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_add(0.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_sub_zero() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_sub(0.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_mul_one() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_mul(1.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_div_one() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_div(1.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_display() {
    let atomic = Atomic::<f32>::new(std::f32::consts::PI);
    let display_str = format!("{}", atomic);
    assert!(display_str.contains("3.14"));
}

#[test]
fn test_debug_false() {
    let atomic = Atomic::<f32>::new(0.0);
    let debug_str = format!("{:?}", atomic);
    assert!(debug_str.contains("0"));
}

#[test]
fn test_trait_atomic_comprehensive() {
    fn test_atomic(atomic: &Atomic<f32>) {
        atomic.store(5.0);
        assert!((atomic.load() - 5.0).abs() < EPSILON);

        let old = atomic.swap(10.0);
        assert!((old - 5.0).abs() < EPSILON);

        assert!(atomic.compare_set(10.0, 15.0).is_ok());
        assert_eq!(atomic.compare_and_exchange(15.0, 20.0), 15.0);
    }

    let atomic = Atomic::<f32>::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_get_and_update_identity() {
    let atomic = Atomic::<f32>::new(42.0);
    let old = atomic.fetch_update(|x| x);
    assert!((old - 42.0).abs() < EPSILON);
    assert!((atomic.load() - 42.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_failure_path() {
    let atomic = Atomic::<f32>::new(10.0);
    // Try to CAS with wrong current value
    match atomic.compare_set(5.0, 15.0) {
        Ok(_) => panic!("Should have failed"),
        Err(actual) => assert!((actual - 10.0).abs() < EPSILON),
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_failure_path() {
    let atomic = Atomic::<f32>::new(10.0);
    let prev = atomic.compare_and_exchange(5.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_set_weak_failure_path() {
    let atomic = Atomic::<f32>::new(10.0);
    match atomic.compare_set_weak(5.0, 15.0) {
        Ok(_) => panic!("Should have failed"),
        Err(actual) => assert!((actual - 10.0).abs() < EPSILON),
    }
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak_failure_path() {
    let atomic = Atomic::<f32>::new(10.0);
    let prev = atomic.compare_and_exchange_weak(5.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 10.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_success_path() {
    let atomic = Atomic::<f32>::new(10.0);
    let prev = atomic.compare_and_exchange(10.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_compare_and_exchange_weak_success_path() {
    let atomic = Atomic::<f32>::new(10.0);
    let prev = atomic.compare_and_exchange_weak(10.0, 15.0);
    assert!((prev - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_concurrent_add_high_contention() {
    let atomic = Arc::new(Atomic::<f32>::new(0.0));
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
    let atomic = Arc::new(Atomic::<f32>::new(1000.0));
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
    let atomic = Arc::new(Atomic::<f32>::new(100.0));
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
fn test_concurrent_mul_extreme_contention() {
    let atomic = Arc::new(Atomic::<f32>::new(1.0));
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
    let atomic = Arc::new(Atomic::<f32>::new(1000000.0));
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
fn test_concurrent_get_and_update_contention() {
    let atomic = Arc::new(Atomic::<f32>::new(0.0));
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
// Numeric operation tests
// ============================================================================

#[test]
fn test_atomic_number_fetch_add() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_add(5.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_add_negative() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_add(-3.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 7.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_sub() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_sub(3.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 7.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_sub_negative() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_sub(-5.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - 15.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_mul() {
    let atomic = Atomic::<f32>::new(3.0);
    let old = atomic.fetch_mul(4.0);
    assert!((old - 3.0).abs() < EPSILON);
    assert!((atomic.load() - 12.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_mul_by_zero() {
    let atomic = Atomic::<f32>::new(5.0);
    let old = atomic.fetch_mul(0.0);
    assert!((old - 5.0).abs() < EPSILON);
    assert!((atomic.load() - 0.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_mul_by_negative() {
    let atomic = Atomic::<f32>::new(3.0);
    let old = atomic.fetch_mul(-2.0);
    assert!((old - 3.0).abs() < EPSILON);
    assert!((atomic.load() - (-6.0)).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_div() {
    let atomic = Atomic::<f32>::new(12.0);
    let old = atomic.fetch_div(4.0);
    assert!((old - 12.0).abs() < EPSILON);
    assert!((atomic.load() - 3.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_div_by_negative() {
    let atomic = Atomic::<f32>::new(12.0);
    let old = atomic.fetch_div(-4.0);
    assert!((old - 12.0).abs() < EPSILON);
    assert!((atomic.load() - (-3.0)).abs() < EPSILON);
}

#[test]
fn test_atomic_number_fetch_div_fractional() {
    let atomic = Atomic::<f32>::new(10.0);
    let old = atomic.fetch_div(3.0);
    assert!((old - 10.0).abs() < EPSILON);
    assert!((atomic.load() - (10.0 / 3.0)).abs() < EPSILON);
}

#[test]
fn test_atomic_number_operations_chain() {
    let atomic = Atomic::<f32>::new(10.0);

    // 10.0 + 5.0 = 15.0
    atomic.fetch_add(5.0);
    assert!((atomic.load() - 15.0).abs() < EPSILON);

    // 15.0 * 2.0 = 30.0
    atomic.fetch_mul(2.0);
    assert!((atomic.load() - 30.0).abs() < EPSILON);

    // 30.0 - 10.0 = 20.0
    atomic.fetch_sub(10.0);
    assert!((atomic.load() - 20.0).abs() < EPSILON);

    // 20.0 / 4.0 = 5.0
    atomic.fetch_div(4.0);
    assert!((atomic.load() - 5.0).abs() < EPSILON);
}

#[test]
fn test_atomic_number_concurrent_operations() {
    let atomic = Arc::new(Atomic::<f32>::new(0.0));
    let mut handles = vec![];

    // Start multiple threads for concurrent operations
    for i in 0..10 {
        let atomic = Arc::clone(&atomic);
        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                atomic.fetch_add(1.0);
            } else {
                atomic.fetch_sub(0.5);
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
