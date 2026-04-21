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

use qubit_atomic::{ArcAtomic, Atomic};

#[test]
fn test_arc_atomic_shared_owner() {
    let counter = ArcAtomic::new(0usize);
    let shared = counter.clone();

    let handle = thread::spawn(move || {
        shared.fetch_inc();
    });
    handle
        .join()
        .expect("shared atomic increment thread should not panic");

    assert_eq!(counter.load(), 1);
    assert_eq!(counter.strong_count(), 1);
}

#[test]
fn test_arc_atomic_constructors_and_arc_access() {
    let from_value = ArcAtomic::<i32>::from(3);
    assert_eq!(from_value.load(), 3);

    let from_atomic = ArcAtomic::from_atomic(Atomic::<i32>::new(5));
    assert_eq!(from_atomic.load(), 5);

    let raw = Arc::new(Atomic::<i32>::new(7));
    let wrapped = ArcAtomic::from_arc(Arc::clone(&raw));
    assert!(Arc::ptr_eq(wrapped.as_arc(), &raw));
    assert_eq!(wrapped.strong_count(), 2);

    let unwrapped = wrapped.into_arc();
    assert!(Arc::ptr_eq(&unwrapped, &raw));
}

#[test]
fn test_arc_atomic_from_trait_conversions() {
    let from_atomic: ArcAtomic<i32> = Atomic::<i32>::new(9).into();
    assert_eq!(from_atomic.load(), 9);

    let raw = Arc::new(Atomic::<i32>::new(11));
    let from_arc: ArcAtomic<i32> = Arc::clone(&raw).into();
    assert!(Arc::ptr_eq(from_arc.as_arc(), &raw));
    assert_eq!(from_arc.load(), 11);
}

#[test]
fn test_arc_atomic_debug_display() {
    let atomic = ArcAtomic::new(42i32);

    assert_eq!(format!("{atomic}"), "42");
    assert_eq!(
        format!("{atomic:?}"),
        "ArcAtomic { value: 42, strong_count: 1 }",
    );
}
