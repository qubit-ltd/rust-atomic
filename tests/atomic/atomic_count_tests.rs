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

use qubit_atomic::AtomicCount;

#[test]
fn test_new_get() {
    let counter = AtomicCount::new(42);
    assert_eq!(counter.get(), 42);
}

#[test]
fn test_zero() {
    let counter = AtomicCount::zero();
    assert_eq!(counter.get(), 0);
    assert!(counter.is_zero());
}

#[test]
fn test_default() {
    let counter = AtomicCount::default();
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_from() {
    let counter = AtomicCount::from(9);
    assert_eq!(counter.get(), 9);
}

#[test]
fn test_is_zero() {
    let counter = AtomicCount::new(0);
    assert!(counter.is_zero());

    counter.inc();
    assert!(!counter.is_zero());
}

#[test]
fn test_is_positive() {
    let counter = AtomicCount::new(0);
    assert!(!counter.is_positive());

    counter.inc();
    assert!(counter.is_positive());
}

#[test]
fn test_inc_returns_new_value() {
    let counter = AtomicCount::zero();

    assert_eq!(counter.inc(), 1);
    assert_eq!(counter.inc(), 2);
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_add_returns_new_value() {
    let counter = AtomicCount::new(2);

    assert_eq!(counter.add(3), 5);
    assert_eq!(counter.get(), 5);
}

#[test]
fn test_add_zero_returns_current_value() {
    let counter = AtomicCount::new(2);

    assert_eq!(counter.add(0), 2);
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_try_add_success() {
    let counter = AtomicCount::new(2);

    assert_eq!(counter.try_add(3), Some(5));
    assert_eq!(counter.get(), 5);
}

#[test]
fn test_try_add_overflow_keeps_value() {
    let counter = AtomicCount::new(1);

    assert_eq!(counter.try_add(usize::MAX), None);
    assert_eq!(counter.get(), 1);
}

#[test]
#[should_panic(expected = "atomic counter overflow")]
fn test_add_overflow_panics() {
    let counter = AtomicCount::new(usize::MAX);
    counter.inc();
}

#[test]
fn test_dec_returns_new_value() {
    let counter = AtomicCount::new(2);

    assert_eq!(counter.dec(), 1);
    assert_eq!(counter.dec(), 0);
    assert!(counter.is_zero());
}

#[test]
#[should_panic(expected = "atomic counter underflow")]
fn test_dec_underflow_panics() {
    let counter = AtomicCount::zero();
    counter.dec();
}

#[test]
fn test_try_dec_success() {
    let counter = AtomicCount::new(1);

    assert_eq!(counter.try_dec(), Some(0));
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_try_dec_underflow_keeps_value() {
    let counter = AtomicCount::zero();

    assert_eq!(counter.try_dec(), None);
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_sub_returns_new_value() {
    let counter = AtomicCount::new(5);

    assert_eq!(counter.sub(3), 2);
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_sub_zero_returns_current_value() {
    let counter = AtomicCount::new(5);

    assert_eq!(counter.sub(0), 5);
    assert_eq!(counter.get(), 5);
}

#[test]
#[should_panic(expected = "atomic counter underflow")]
fn test_sub_underflow_panics() {
    let counter = AtomicCount::new(2);
    counter.sub(3);
}

#[test]
fn test_try_sub_success() {
    let counter = AtomicCount::new(5);

    assert_eq!(counter.try_sub(3), Some(2));
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_try_sub_underflow_keeps_value() {
    let counter = AtomicCount::new(2);

    assert_eq!(counter.try_sub(3), None);
    assert_eq!(counter.get(), 2);
}

#[test]
fn test_debug() {
    let counter = AtomicCount::new(42);

    assert_eq!(format!("{counter:?}"), "AtomicCount { value: 42 }");
}

#[test]
fn test_display() {
    let counter = AtomicCount::new(42);

    assert_eq!(format!("{counter}"), "42");
}

#[test]
fn test_concurrent_inc() {
    const THREAD_COUNT: usize = 8;
    const ITERATIONS: usize = 1_000;

    let counter = Arc::new(AtomicCount::zero());
    let mut handles = Vec::with_capacity(THREAD_COUNT);

    for _ in 0..THREAD_COUNT {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS {
                counter.inc();
            }
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("counter increment thread should not panic");
    }

    assert_eq!(counter.get(), THREAD_COUNT * ITERATIONS);
}

#[test]
fn test_concurrent_dec_to_zero() {
    const THREAD_COUNT: usize = 8;
    const ITERATIONS: usize = 1_000;

    let counter = Arc::new(AtomicCount::new(THREAD_COUNT * ITERATIONS));
    let mut handles = Vec::with_capacity(THREAD_COUNT);

    for _ in 0..THREAD_COUNT {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ITERATIONS {
                counter.dec();
            }
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("counter decrement thread should not panic");
    }

    assert_eq!(counter.get(), 0);
    assert!(counter.is_zero());
}
