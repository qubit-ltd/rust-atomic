/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use qubit_atomic::{Atomic, AtomicRef};
use std::sync::Arc;

// Test that supported atomic values expose the common primitive API correctly.
#[test]
fn test_atomic_trait_bool() {
    fn test_atomic(atomic: &Atomic<bool>) {
        atomic.store(true);
        assert!(atomic.load());
        let old = atomic.swap(false);
        assert!(old);
        assert!(!atomic.load());
    }

    let atomic = Atomic::<bool>::new(false);
    test_atomic(&atomic);
}

#[test]
fn test_atomic_trait_integers() {
    fn test_atomic(atomic: &Atomic<i32>) {
        atomic.store(42);
        assert_eq!(atomic.load(), 42);
        let old = atomic.swap(100);
        assert_eq!(old, 42);
        assert_eq!(atomic.load(), 100);

        assert!(atomic.compare_set(100, 200).is_ok());
        assert_eq!(atomic.load(), 200);

        let prev = atomic.compare_and_exchange(200, 300);
        assert_eq!(prev, 200);
        assert_eq!(atomic.load(), 300);
    }

    let atomic = Atomic::<i32>::new(0);
    test_atomic(&atomic);
}

#[test]
fn test_atomic_trait_floats() {
    fn test_atomic(atomic: &Atomic<f32>) {
        atomic.store(std::f32::consts::PI);
        assert!((atomic.load() - std::f32::consts::PI).abs() < 1e-6);
        let old = atomic.swap(2.71);
        assert!((old - std::f32::consts::PI).abs() < 1e-6);
    }

    let atomic = Atomic::<f32>::new(0.0);
    test_atomic(&atomic);
}

#[test]
fn test_atomic_trait_ref() {
    fn test_atomic(atomic: &AtomicRef<i32>) {
        atomic.store(Arc::new(42));
        assert_eq!(*atomic.load(), 42);
        let old = atomic.swap(Arc::new(100));
        assert_eq!(*old, 42);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

// Test common atomic methods through a helper function.
fn test_atomic_methods_via_helper(atomic: &Atomic<i32>) {
    atomic.store(10);
    assert_eq!(atomic.load(), 10);

    let old = atomic.swap(20);
    assert_eq!(old, 10);
    assert_eq!(atomic.load(), 20);

    assert!(atomic.compare_set(20, 30).is_ok());
    assert_eq!(atomic.load(), 30);

    let prev = atomic.compare_and_exchange(30, 40);
    assert_eq!(prev, 30);
    assert_eq!(atomic.load(), 40);
}

// Test numeric atomic methods through a helper function.
fn test_atomic_number_methods_via_helper(atomic: &Atomic<i32>) {
    let old = atomic.fetch_add(1);
    assert_eq!(old, 0);

    let old = atomic.fetch_add(1);
    assert_eq!(old, 1);

    let old = atomic.fetch_sub(1);
    assert_eq!(old, 2);

    let old = atomic.fetch_sub(1);
    assert_eq!(old, 1);

    let old = atomic.fetch_add(10);
    assert_eq!(old, 0);

    let old = atomic.fetch_add(5);
    assert_eq!(old, 10); // returns old value
}

#[test]
fn test_all_traits_via_generic() {
    let atomic = Atomic::<i32>::new(0);
    test_atomic_methods_via_helper(&atomic);

    let atomic = Atomic::<i32>::new(0);
    test_atomic_methods_via_helper(&atomic);

    let atomic = Atomic::<i32>::new(0);
    test_atomic_number_methods_via_helper(&atomic);
}

// Test integer-specific operations.
#[test]
fn test_atomic_integer_trait_i8() {
    let atomic = Atomic::<i8>::new(0);
    assert_eq!(atomic.fetch_inc(), 0); // returns old value
    assert_eq!(atomic.load(), 1);

    assert_eq!(atomic.fetch_add(5), 1); // returns old value
    assert_eq!(atomic.load(), 6);

    assert_eq!(atomic.fetch_dec(), 6); // returns old value
    assert_eq!(atomic.load(), 5);

    atomic.store(0b0101);
    atomic.fetch_and(0b0011);
    assert_eq!(atomic.load(), 0b0001);
}

#[test]
fn test_atomic_integer_trait_u16() {
    let atomic = Atomic::<u16>::new(0);
    assert_eq!(atomic.fetch_inc(), 0); // returns old value
    assert_eq!(atomic.fetch_add(10), 1); // returns old value
    assert_eq!(atomic.fetch_sub(5), 11); // returns old value
    assert_eq!(atomic.load(), 6);

    atomic.fetch_max(20);
    assert_eq!(atomic.load(), 20);

    atomic.fetch_min(10);
    assert_eq!(atomic.load(), 10);
}

#[test]
fn test_atomic_integer_trait_i32() {
    let atomic = Atomic::<i32>::new(0);
    // fetch_accumulate returns old value
    let old = atomic.fetch_accumulate(5, |a, b| a + b);
    assert_eq!(old, 0); // old value
    assert_eq!(atomic.load(), 5); // new value

    let old2 = atomic.fetch_accumulate(10, |a, b| a * b);
    assert_eq!(old2, 5); // old value
    assert_eq!(atomic.load(), 50); // new value
}

#[test]
fn test_atomic_integer_trait_i64() {
    let atomic = Atomic::<i64>::new(0);
    atomic.fetch_inc();
    atomic.fetch_add(99);
    assert_eq!(atomic.load(), 100);

    atomic.fetch_or(0b1111);
    assert_eq!(atomic.load() & 0b1111, 0b1111);
}

#[test]
fn test_atomic_integer_trait_usize() {
    let atomic = Atomic::<usize>::new(0);
    for _ in 0..10 {
        atomic.fetch_inc();
    }
    assert_eq!(atomic.load(), 10);

    atomic.fetch_xor(0b1010);
    // Result depends on platform, just check it doesn't panic
    let _ = atomic.load();
}

// Test that common and numeric methods are available together.
#[test]
fn test_all_traits_i32() {
    fn test_all(atomic: &Atomic<i32>) {
        atomic.store(10);
        assert_eq!(atomic.load(), 10);

        let _old = atomic.fetch_update(|x| x + 5);
        assert_eq!(atomic.load(), 15);

        let _old = atomic.fetch_add(1);
        assert_eq!(atomic.load(), 16);
    }

    let atomic = Atomic::<i32>::new(0);
    test_all(&atomic);
}

// Test Send and Sync traits
#[test]
fn test_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Atomic<bool>>();
    assert_sync::<Atomic<bool>>();

    assert_send::<Atomic<i32>>();
    assert_sync::<Atomic<i32>>();

    assert_send::<Atomic<u64>>();
    assert_sync::<Atomic<u64>>();

    assert_send::<Atomic<f32>>();
    assert_sync::<Atomic<f32>>();

    assert_send::<Atomic<f64>>();
    assert_sync::<Atomic<f64>>();

    assert_send::<AtomicRef<i32>>();
    assert_sync::<AtomicRef<i32>>();
}

// Test Default trait
#[test]
fn test_default_trait() {
    let atomic_bool = Atomic::<bool>::default();
    assert!(!atomic_bool.load());

    let atomic_i32 = Atomic::<i32>::default();
    assert_eq!(atomic_i32.load(), 0);

    let atomic_f64 = Atomic::<f64>::default();
    assert_eq!(atomic_f64.load(), 0.0);
}

// Test From trait
#[test]
fn test_from_trait() {
    let atomic_bool = Atomic::<bool>::from(true);
    assert!(atomic_bool.load());

    let atomic_i32 = Atomic::<i32>::from(42);
    assert_eq!(atomic_i32.load(), 42);

    let atomic_f32 = Atomic::<f32>::from(std::f32::consts::PI);
    assert!((atomic_f32.load() - std::f32::consts::PI).abs() < 1e-6);
}

// Test Debug and Display traits
#[test]
fn test_debug_display_traits() {
    let atomic_bool = Atomic::<bool>::new(true);
    assert!(format!("{:?}", atomic_bool).contains("true"));
    assert_eq!(format!("{}", atomic_bool), "true");

    let atomic_i32 = Atomic::<i32>::new(42);
    assert!(format!("{:?}", atomic_i32).contains("42"));
    assert_eq!(format!("{}", atomic_i32), "42");

    let atomic_f64 = Atomic::<f64>::new(std::f64::consts::PI);
    assert!(format!("{:?}", atomic_f64).contains("3.14"));
    assert!(format!("{}", atomic_f64).contains("3.14"));
}
