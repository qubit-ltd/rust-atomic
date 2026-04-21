/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use std::sync::Arc;

use qubit_atomic::{ArcAtomicRef, AtomicRef};

#[derive(Debug, Clone, PartialEq)]
struct TestData {
    value: i32,
    name: String,
}

#[test]
fn test_arc_atomic_ref_shared_owner() {
    let atomic = ArcAtomicRef::from_value(TestData {
        value: 42,
        name: "initial".to_string(),
    });
    let shared = atomic.clone();

    shared.store(Arc::new(TestData {
        value: 100,
        name: "updated".to_string(),
    }));

    assert_eq!(atomic.load().value, 100);
    assert_eq!(atomic.load().name, "updated");
    assert_eq!(atomic.strong_count(), 2);
}

#[test]
fn test_arc_atomic_ref_clone_differs_from_atomic_ref_clone() {
    let shared = ArcAtomicRef::from_value(TestData {
        value: 1,
        name: "one".to_string(),
    });
    let independent = AtomicRef::clone(&*shared);

    shared.store(Arc::new(TestData {
        value: 2,
        name: "two".to_string(),
    }));

    assert_eq!(shared.load().value, 2);
    assert_eq!(independent.load().value, 1);
}

#[test]
fn test_arc_atomic_ref_constructors_and_arc_access() {
    let from_arc_value = ArcAtomicRef::new(Arc::new(10));
    assert_eq!(*from_arc_value.load(), 10);

    let from_atomic_ref = ArcAtomicRef::from_atomic_ref(AtomicRef::from_value(20));
    assert_eq!(*from_atomic_ref.load(), 20);

    let raw = Arc::new(AtomicRef::from_value(30));
    let wrapped = ArcAtomicRef::from_arc(Arc::clone(&raw));
    assert!(Arc::ptr_eq(wrapped.as_arc(), &raw));
    assert_eq!(wrapped.strong_count(), 2);

    let unwrapped = wrapped.into_arc();
    assert!(Arc::ptr_eq(&unwrapped, &raw));
}

#[test]
fn test_arc_atomic_ref_from_trait_conversions() {
    let from_atomic_ref: ArcAtomicRef<i32> = AtomicRef::from_value(40).into();
    assert_eq!(*from_atomic_ref.load(), 40);

    let raw = Arc::new(AtomicRef::from_value(50));
    let from_arc: ArcAtomicRef<i32> = Arc::clone(&raw).into();
    assert!(Arc::ptr_eq(from_arc.as_arc(), &raw));
    assert_eq!(*from_arc.load(), 50);
}

#[test]
fn test_arc_atomic_ref_debug_display() {
    let atomic = ArcAtomicRef::from_value(42);

    assert_eq!(format!("{atomic}"), "42");
    assert_eq!(
        format!("{atomic:?}"),
        "ArcAtomicRef { value: 42, strong_count: 1 }",
    );
}
