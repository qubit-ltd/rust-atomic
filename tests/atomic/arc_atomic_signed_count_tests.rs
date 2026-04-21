/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use std::sync::Arc;

use qubit_atomic::{ArcAtomicSignedCount, AtomicSignedCount};

#[test]
fn test_arc_atomic_signed_count_shared_owner() {
    let counter = ArcAtomicSignedCount::zero();
    let shared = counter.clone();

    shared.sub(3);

    assert_eq!(counter.get(), -3);
    assert_eq!(counter.strong_count(), 2);
}

#[test]
fn test_arc_atomic_signed_count_constructors_and_arc_access() {
    let from_value = ArcAtomicSignedCount::from(-3);
    assert_eq!(from_value.get(), -3);

    let from_count = ArcAtomicSignedCount::from_count(AtomicSignedCount::new(-5));
    assert_eq!(from_count.get(), -5);

    let raw = Arc::new(AtomicSignedCount::new(-7));
    let wrapped = ArcAtomicSignedCount::from_arc(Arc::clone(&raw));
    assert!(Arc::ptr_eq(wrapped.as_arc(), &raw));
    assert_eq!(wrapped.strong_count(), 2);

    let unwrapped = wrapped.into_arc();
    assert!(Arc::ptr_eq(&unwrapped, &raw));
}

#[test]
fn test_arc_atomic_signed_count_from_trait_conversions() {
    let from_count: ArcAtomicSignedCount = AtomicSignedCount::new(-9).into();
    assert_eq!(from_count.get(), -9);

    let raw = Arc::new(AtomicSignedCount::new(-11));
    let from_arc: ArcAtomicSignedCount = Arc::clone(&raw).into();
    assert!(Arc::ptr_eq(from_arc.as_arc(), &raw));
    assert_eq!(from_arc.get(), -11);
}

#[test]
fn test_arc_atomic_signed_count_default_debug_display() {
    let counter = ArcAtomicSignedCount::default();
    counter.sub(42);

    assert_eq!(format!("{counter}"), "-42");
    assert_eq!(
        format!("{counter:?}"),
        "ArcAtomicSignedCount { value: -42, strong_count: 1 }",
    );
}
