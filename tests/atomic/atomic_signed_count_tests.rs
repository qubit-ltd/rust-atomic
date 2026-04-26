/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use std::sync::Arc;
use std::thread;

use qubit_atomic::AtomicSignedCount;

#[test]
fn test_new_get() {
    let counter = AtomicSignedCount::new(-42);
    assert_eq!(counter.get(), -42);
}

#[test]
fn test_zero() {
    let counter = AtomicSignedCount::zero();
    assert_eq!(counter.get(), 0);
    assert!(counter.is_zero());
}

#[test]
fn test_default() {
    let counter = AtomicSignedCount::default();
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_from() {
    let counter = AtomicSignedCount::from(-9);
    assert_eq!(counter.get(), -9);
}

#[test]
fn test_sign_checks() {
    let positive = AtomicSignedCount::new(1);
    let zero = AtomicSignedCount::zero();
    let negative = AtomicSignedCount::new(-1);

    assert!(positive.is_positive());
    assert!(!positive.is_zero());
    assert!(!positive.is_negative());

    assert!(zero.is_zero());
    assert!(!zero.is_positive());
    assert!(!zero.is_negative());

    assert!(negative.is_negative());
    assert!(!negative.is_zero());
    assert!(!negative.is_positive());
}

#[test]
fn test_inc_returns_new_value() {
    let counter = AtomicSignedCount::zero();

    assert_eq!(counter.inc(), 1);
    assert_eq!(counter.inc(), 2);
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_dec_returns_new_value() {
    let counter = AtomicSignedCount::zero();

    assert_eq!(counter.dec(), -1);
    assert_eq!(counter.dec(), -2);
    assert_eq!(counter.get(), -2);
}

#[test]
fn test_add_positive_delta_returns_new_value() {
    let counter = AtomicSignedCount::new(2);

    assert_eq!(counter.add(3), 5);
    assert_eq!(counter.get(), 5);
}

#[test]
fn test_add_negative_delta_returns_new_value() {
    let counter = AtomicSignedCount::new(2);

    assert_eq!(counter.add(-5), -3);
    assert_eq!(counter.get(), -3);
}

#[test]
fn test_try_add_success() {
    let counter = AtomicSignedCount::new(-2);

    assert_eq!(counter.try_add(5), Some(3));
    assert_eq!(counter.get(), 3);
}

#[test]
fn test_try_add_overflow_keeps_value() {
    let counter = AtomicSignedCount::new(isize::MAX);

    assert_eq!(counter.try_add(1), None);
    assert_eq!(counter.get(), isize::MAX);
}

#[test]
fn test_try_add_underflow_keeps_value() {
    let counter = AtomicSignedCount::new(isize::MIN);

    assert_eq!(counter.try_add(-1), None);
    assert_eq!(counter.get(), isize::MIN);
}

#[test]
#[should_panic(expected = "atomic signed counter out of range")]
fn test_add_overflow_panics() {
    let counter = AtomicSignedCount::new(isize::MAX);
    counter.inc();
}

#[test]
#[should_panic(expected = "atomic signed counter out of range")]
fn test_add_underflow_panics() {
    let counter = AtomicSignedCount::new(isize::MIN);
    counter.add(-1);
}

#[test]
fn test_sub_positive_delta_returns_new_value() {
    let counter = AtomicSignedCount::new(2);

    assert_eq!(counter.sub(5), -3);
    assert_eq!(counter.get(), -3);
}

#[test]
fn test_sub_negative_delta_returns_new_value() {
    let counter = AtomicSignedCount::new(2);

    assert_eq!(counter.sub(-5), 7);
    assert_eq!(counter.get(), 7);
}

#[test]
fn test_try_sub_success() {
    let counter = AtomicSignedCount::new(2);

    assert_eq!(counter.try_sub(5), Some(-3));
    assert_eq!(counter.get(), -3);
}

#[test]
fn test_try_sub_overflow_keeps_value() {
    let counter = AtomicSignedCount::new(isize::MAX);

    assert_eq!(counter.try_sub(-1), None);
    assert_eq!(counter.get(), isize::MAX);
}

#[test]
fn test_try_sub_underflow_keeps_value() {
    let counter = AtomicSignedCount::new(isize::MIN);

    assert_eq!(counter.try_sub(1), None);
    assert_eq!(counter.get(), isize::MIN);
}

#[test]
#[should_panic(expected = "atomic signed counter out of range")]
fn test_sub_overflow_panics() {
    let counter = AtomicSignedCount::new(isize::MAX);
    counter.sub(-1);
}

#[test]
#[should_panic(expected = "atomic signed counter out of range")]
fn test_sub_underflow_panics() {
    let counter = AtomicSignedCount::new(isize::MIN);
    counter.dec();
}

#[test]
fn test_debug() {
    let counter = AtomicSignedCount::new(-42);

    assert_eq!(format!("{counter:?}"), "AtomicSignedCount { value: -42 }");
}

#[test]
fn test_display() {
    let counter = AtomicSignedCount::new(-42);

    assert_eq!(format!("{counter}"), "-42");
}

#[test]
fn test_concurrent_add() {
    const THREAD_COUNT: usize = 8;
    const ITERATIONS: usize = 1_000;

    let counter = Arc::new(AtomicSignedCount::zero());
    let mut handles = Vec::with_capacity(THREAD_COUNT);

    for _ in 0..THREAD_COUNT {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS {
                counter.add(1);
            }
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("signed counter add thread should not panic");
    }

    assert_eq!(counter.get(), (THREAD_COUNT * ITERATIONS) as isize);
}

#[test]
fn test_concurrent_add_and_sub() {
    const THREAD_COUNT: usize = 8;
    const ITERATIONS: usize = 1_000;

    let counter = Arc::new(AtomicSignedCount::zero());
    let mut handles = Vec::with_capacity(THREAD_COUNT * 2);

    for _ in 0..THREAD_COUNT {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS {
                counter.add(1);
            }
        }));
    }

    for _ in 0..THREAD_COUNT {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS {
                counter.sub(1);
            }
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("signed counter worker thread should not panic");
    }

    assert_eq!(counter.get(), 0);
    assert!(counter.is_zero());
}
