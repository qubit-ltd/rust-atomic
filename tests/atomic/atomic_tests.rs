/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use std::cell::Cell;
use std::sync::atomic::Ordering;

use qubit_atomic::Atomic;

#[test]
fn test_default_from_debug_display_impls() {
    let bool_default = Atomic::<bool>::default();
    assert!(!bool_default.load());
    let bool_from = Atomic::<bool>::from(true);
    assert_eq!(format!("{bool_from:?}"), "Atomic { value: true }");
    assert_eq!(format!("{bool_from}"), "true");

    let int_default = Atomic::<i32>::default();
    assert_eq!(int_default.load(), 0);
    let int_from = Atomic::<i32>::from(42);
    assert_eq!(format!("{int_from:?}"), "Atomic { value: 42 }");
    assert_eq!(format!("{int_from}"), "42");

    let float_default = Atomic::<f32>::default();
    assert_eq!(float_default.load(), 0.0);
    let float_from = Atomic::<f32>::from(1.5);
    assert_eq!(format!("{float_from:?}"), "Atomic { value: 1.5 }");
    assert_eq!(format!("{float_from}"), "1.5");

    let double_default = Atomic::<f64>::default();
    assert_eq!(double_default.load(), 0.0);
    let double_from = Atomic::<f64>::from(2.5);
    assert_eq!(format!("{double_from:?}"), "Atomic { value: 2.5 }");
    assert_eq!(format!("{double_from}"), "2.5");
}

#[test]
fn test_fetch_update_retry_paths() {
    let bool_atomic = Atomic::<bool>::new(false);
    let bool_raced = Cell::new(false);
    let bool_old = bool_atomic.fetch_update(|current| {
        if !bool_raced.replace(true) {
            bool_atomic.store(!current);
        }
        !current
    });
    assert!(bool_old);
    assert!(!bool_atomic.load());

    let int_atomic = Atomic::<i32>::new(1);
    let int_raced = Cell::new(false);
    let int_old = int_atomic.fetch_update(|current| {
        if !int_raced.replace(true) {
            int_atomic.store(current + 10);
        }
        current * 2
    });
    assert_eq!(int_old, 11);
    assert_eq!(int_atomic.load(), 22);

    let float_atomic = Atomic::<f32>::new(1.0);
    let float_raced = Cell::new(false);
    let float_old = float_atomic.fetch_update(|current| {
        if !float_raced.replace(true) {
            float_atomic.store(current + 10.0);
        }
        current * 2.0
    });
    assert_eq!(float_old, 11.0);
    assert_eq!(float_atomic.load(), 22.0);

    let double_atomic = Atomic::<f64>::new(1.0);
    let double_raced = Cell::new(false);
    let double_old = double_atomic.fetch_update(|current| {
        if !double_raced.replace(true) {
            double_atomic.store(current + 10.0);
        }
        current * 2.0
    });
    assert_eq!(double_old, 11.0);
    assert_eq!(double_atomic.load(), 22.0);
}

#[test]
fn test_try_update_success_and_reject_paths() {
    let bool_atomic = Atomic::<bool>::new(false);
    assert_eq!(
        bool_atomic.try_update(|current| (!current).then_some(true)),
        Some(false),
    );
    assert!(bool_atomic.load());
    assert_eq!(
        bool_atomic.try_update(|current| (!current).then_some(true)),
        None,
    );
    assert!(bool_atomic.load());

    let int_atomic = Atomic::<i32>::new(3);
    assert_eq!(
        int_atomic.try_update(|current| (current % 2 == 1).then_some(current + 1)),
        Some(3),
    );
    assert_eq!(int_atomic.load(), 4);
    assert_eq!(
        int_atomic.try_update(|current| (current % 2 == 1).then_some(current + 1)),
        None,
    );
    assert_eq!(int_atomic.load(), 4);

    let float_atomic = Atomic::<f32>::new(1.5);
    assert_eq!(
        float_atomic.try_update(|current| (current > 0.0).then_some(current * 2.0)),
        Some(1.5),
    );
    assert_eq!(float_atomic.load(), 3.0);
    assert_eq!(
        float_atomic.try_update(|current| (current < 0.0).then_some(current * 2.0)),
        None,
    );
    assert_eq!(float_atomic.load(), 3.0);

    let double_atomic = Atomic::<f64>::new(1.5);
    assert_eq!(
        double_atomic.try_update(|current| (current > 0.0).then_some(current * 2.0)),
        Some(1.5),
    );
    assert_eq!(double_atomic.load(), 3.0);
    assert_eq!(
        double_atomic.try_update(|current| (current < 0.0).then_some(current * 2.0)),
        None,
    );
    assert_eq!(double_atomic.load(), 3.0);
}

#[test]
fn test_try_update_retry_paths() {
    let bool_atomic = Atomic::<bool>::new(false);
    let bool_raced = Cell::new(false);
    let bool_old = bool_atomic.try_update(|current| {
        if !bool_raced.replace(true) {
            bool_atomic.store(!current);
        }
        Some(!current)
    });
    assert_eq!(bool_old, Some(true));
    assert!(!bool_atomic.load());

    let int_atomic = Atomic::<i32>::new(1);
    let int_raced = Cell::new(false);
    let int_old = int_atomic.try_update(|current| {
        if !int_raced.replace(true) {
            int_atomic.store(current + 10);
        }
        Some(current * 2)
    });
    assert_eq!(int_old, Some(11));
    assert_eq!(int_atomic.load(), 22);

    let float_atomic = Atomic::<f32>::new(1.0);
    let float_raced = Cell::new(false);
    let float_old = float_atomic.try_update(|current| {
        if !float_raced.replace(true) {
            float_atomic.store(current + 10.0);
        }
        Some(current * 2.0)
    });
    assert_eq!(float_old, Some(11.0));
    assert_eq!(float_atomic.load(), 22.0);

    let double_atomic = Atomic::<f64>::new(1.0);
    let double_raced = Cell::new(false);
    let double_old = double_atomic.try_update(|current| {
        if !double_raced.replace(true) {
            double_atomic.store(current + 10.0);
        }
        Some(current * 2.0)
    });
    assert_eq!(double_old, Some(11.0));
    assert_eq!(double_atomic.load(), 22.0);
}

#[test]
fn test_f32_compare_exchange_uses_raw_bits() {
    let atomic = Atomic::<f32>::new(-0.0);

    assert_ne!(0.0f32.to_bits(), (-0.0f32).to_bits());
    assert!(atomic.compare_set(0.0, 1.0).is_err());

    let prev = atomic.compare_and_exchange(0.0, 1.0);
    assert_eq!(prev, 0.0);
    assert_eq!(prev.to_bits(), (-0.0f32).to_bits());
    assert_eq!(atomic.load().to_bits(), (-0.0f32).to_bits());
}

#[test]
fn test_f64_compare_exchange_uses_raw_bits() {
    let atomic = Atomic::<f64>::new(-0.0);

    assert_ne!(0.0f64.to_bits(), (-0.0f64).to_bits());
    assert!(atomic.compare_set(0.0, 1.0).is_err());

    let prev = atomic.compare_and_exchange(0.0, 1.0);
    assert_eq!(prev, 0.0);
    assert_eq!(prev.to_bits(), (-0.0f64).to_bits());
    assert_eq!(atomic.load().to_bits(), (-0.0f64).to_bits());
}

#[test]
fn test_inner_backends() {
    let flag = Atomic::<bool>::new(false);
    assert!(!flag.inner().load(Ordering::Acquire));

    let number = Atomic::<i32>::new(42);
    assert_eq!(number.inner().load(Ordering::Acquire), 42);
}
