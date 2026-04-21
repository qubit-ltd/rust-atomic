/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use std::sync::Arc;

use qubit_atomic::{
    ArcAtomicCount,
    AtomicCount,
};

#[test]
fn test_arc_atomic_count_shared_owner() {
    let counter = ArcAtomicCount::zero();
    let shared = counter.clone();

    shared.inc();

    assert_eq!(counter.get(), 1);
    assert_eq!(counter.strong_count(), 2);
}

#[test]
fn test_arc_atomic_count_constructors_and_arc_access() {
    let from_value = ArcAtomicCount::from(3);
    assert_eq!(from_value.get(), 3);

    let from_count = ArcAtomicCount::from_count(AtomicCount::new(5));
    assert_eq!(from_count.get(), 5);

    let raw = Arc::new(AtomicCount::new(7));
    let wrapped = ArcAtomicCount::from_arc(Arc::clone(&raw));
    assert!(Arc::ptr_eq(wrapped.as_arc(), &raw));
    assert_eq!(wrapped.strong_count(), 2);

    let unwrapped = wrapped.into_arc();
    assert!(Arc::ptr_eq(&unwrapped, &raw));
}

#[test]
fn test_arc_atomic_count_default_debug_display() {
    let counter = ArcAtomicCount::default();
    counter.add(42);

    assert_eq!(format!("{counter}"), "42");
    assert_eq!(
        format!("{counter:?}"),
        "ArcAtomicCount { value: 42, strong_count: 1 }",
    );
}
