/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use loom::thread;

/// Performs one increment with the same weak-CAS retry pattern used by this
/// crate's `fetch_update`/`compare_set_weak` loops.
fn increment_once(counter: &AtomicUsize) {
    let mut current = counter.load(Ordering::Acquire);
    loop {
        let new = current + 1;
        match counter.compare_exchange_weak(current, new, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return,
            Err(actual) => current = actual,
        }
    }
}

#[test]
fn test_loom_release_acquire_visibility() {
    loom::model(|| {
        let data = Arc::new(AtomicUsize::new(0));
        let ready = Arc::new(AtomicBool::new(false));

        let data_writer = data.clone();
        let ready_writer = ready.clone();
        let writer = thread::spawn(move || {
            // Publish data first, then publish the ready flag.
            data_writer.store(42, Ordering::Relaxed);
            ready_writer.store(true, Ordering::Release);
        });

        let data_reader = data.clone();
        let ready_reader = ready.clone();
        let reader = thread::spawn(move || {
            while !ready_reader.load(Ordering::Acquire) {
                thread::yield_now();
            }
            assert_eq!(data_reader.load(Ordering::Relaxed), 42);
        });

        writer.join().unwrap();
        reader.join().unwrap();
    });
}

#[test]
fn test_loom_weak_cas_retry_linearizable() {
    loom::model(|| {
        let counter = Arc::new(AtomicUsize::new(0));

        let c1 = counter.clone();
        let t1 = thread::spawn(move || {
            increment_once(&c1);
        });

        let c2 = counter.clone();
        let t2 = thread::spawn(move || {
            increment_once(&c2);
        });

        t1.join().unwrap();
        t2.join().unwrap();
        assert_eq!(counter.load(Ordering::Acquire), 2);
    });
}
