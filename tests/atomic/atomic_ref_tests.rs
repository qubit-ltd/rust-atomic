/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use qubit_atomic::atomic::{
    Atomic,
    AtomicRef,
};
use std::sync::Arc;
use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};
use std::thread;

#[derive(Debug, Clone, PartialEq)]
struct TestData {
    value: i32,
    name: String,
}

#[derive(Debug)]
struct DropTracked {
    drops: Arc<AtomicUsize>,
}

impl Drop for DropTracked {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn test_new() {
    let data = Arc::new(TestData {
        value: 42,
        name: "test".to_string(),
    });
    let atomic = AtomicRef::new(data.clone());
    assert_eq!(atomic.load().value, 42);
    assert_eq!(atomic.load().name, "test");
}

#[test]
fn test_get_set() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1);

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });
    atomic.store(data2);

    let current = atomic.load();
    assert_eq!(current.value, 100);
    assert_eq!(current.name, "second");
}

#[test]
fn test_swap() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });
    let old = atomic.swap(data2);

    assert_eq!(old.value, 42);
    assert_eq!(old.name, "first");
    assert_eq!(atomic.load().value, 100);
}

#[test]
fn test_compare_and_set_success() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    let current = atomic.load();
    assert!(atomic.compare_set(&current, data2).is_ok());
    assert_eq!(atomic.load().value, 100);
}

#[test]
fn test_compare_and_set_failure() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    let wrong_ref = Arc::new(TestData {
        value: 999,
        name: "wrong".to_string(),
    });

    match atomic.compare_set(&wrong_ref, data2) {
        Ok(_) => panic!("Should fail"),
        Err(actual) => {
            assert_eq!(actual.value, 42);
            assert_eq!(actual.name, "first");
        }
    }
    assert_eq!(atomic.load().value, 42);
}

#[test]
fn test_compare_and_exchange() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    let current = atomic.load();
    let prev = atomic.compare_exchange(current.clone(), data2);
    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(atomic.load().value, 100);
}

#[test]
fn test_get_and_update() {
    let data = Arc::new(TestData {
        value: 42,
        name: "test".to_string(),
    });
    let atomic = AtomicRef::new(data);

    let old = atomic.fetch_update(|current| {
        Arc::new(TestData {
            value: current.value * 2,
            name: format!("{}_updated", current.name),
        })
    });

    assert_eq!(old.value, 42);
    assert_eq!(old.name, "test");
    assert_eq!(atomic.load().value, 84);
    assert_eq!(atomic.load().name, "test_updated");
}

#[test]
fn test_concurrent_updates() {
    let data = Arc::new(TestData {
        value: 0,
        name: "counter".to_string(),
    });
    let atomic = Arc::new(AtomicRef::new(data));
    let mut handles = vec![];

    for i in 0..10 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            atomic.fetch_update(|current| {
                Arc::new(TestData {
                    value: current.value + 1,
                    name: format!("thread_{}", i),
                })
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(atomic.load().value, 10);
}

#[test]
fn test_concurrent_cas() {
    let data = Arc::new(0);
    let atomic = Arc::new(AtomicRef::new(data));
    let success_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let atomic = atomic.clone();
        let success_count = success_count.clone();
        let handle = thread::spawn(move || {
            let mut current = atomic.load();
            loop {
                let new = Arc::new(*current + 1);
                match atomic.compare_set_weak(&current, new) {
                    Ok(_) => {
                        success_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        break;
                    }
                    Err(actual) => current = actual,
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(*atomic.load(), 10);
    assert_eq!(success_count.load(std::sync::atomic::Ordering::Relaxed), 10);
}

#[test]
fn test_clone() {
    let data = Arc::new(TestData {
        value: 42,
        name: "test".to_string(),
    });
    let atomic1 = AtomicRef::new(data);
    let atomic2 = atomic1.clone();

    assert_eq!(atomic1.load().value, 42);
    assert_eq!(atomic2.load().value, 42);

    // Update atomic1
    atomic1.store(Arc::new(TestData {
        value: 100,
        name: "updated".to_string(),
    }));

    // atomic2 should still have the old value
    assert_eq!(atomic1.load().value, 100);
    assert_eq!(atomic2.load().value, 42);
}

#[test]
fn test_trait_atomic() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(42));
        assert_eq!(*atomic.load(), 42);
        let old = atomic.swap(Arc::new(100));
        assert_eq!(*old, 42);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_set_weak() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(10));
        let current = atomic.load();
        assert!(atomic.compare_set_weak(current, Arc::new(20)).is_ok());
        assert_eq!(*atomic.load(), 20);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_exchange() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(10));
        let current = atomic.load();
        let prev = atomic.compare_exchange(current.clone(), Arc::new(20));
        assert!(Arc::ptr_eq(&prev, &current) || *prev == 10);
        assert_eq!(*atomic.load(), 20);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_exchange_failure() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(10));
        let wrong = Arc::new(999);
        let prev = atomic.compare_exchange(wrong, Arc::new(20));
        assert_eq!(*prev, 10);
        assert_eq!(*atomic.load(), 10);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_exchange_weak() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(10));
        let current = atomic.load();
        let prev = atomic.compare_exchange_weak(current.clone(), Arc::new(20));
        assert!(Arc::ptr_eq(&prev, &current) || *prev == 10);
        assert_eq!(*atomic.load(), 20);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_compare_exchange_weak_failure() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(10));
        let wrong = Arc::new(999);
        let prev = atomic.compare_exchange_weak(wrong, Arc::new(20));
        assert_eq!(*prev, 10);
        assert_eq!(*atomic.load(), 10);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_trait_atomic_fetch_update() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(10));
        let old = atomic.fetch_update(|x| Arc::new(*x * 2));
        assert_eq!(*old, 10);
        assert_eq!(*atomic.load(), 20);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_debug_display() {
    let data = Arc::new(42);
    let atomic = AtomicRef::new(data);
    let debug_str = format!("{:?}", atomic);
    assert!(debug_str.contains("42"));
    let display_str = format!("{}", atomic);
    assert_eq!(display_str, "42");
}

#[test]
fn test_arc_reference_counting() {
    let data = Arc::new(TestData {
        value: 42,
        name: "test".to_string(),
    });

    // Initial ref count: 1
    assert_eq!(Arc::strong_count(&data), 1);

    let atomic = AtomicRef::new(data.clone());
    // Ref count: 2 (original + atomic)
    assert_eq!(Arc::strong_count(&data), 2);

    let retrieved = atomic.load();
    // Ref count: 3 (original + atomic + retrieved)
    assert_eq!(Arc::strong_count(&data), 3);

    drop(retrieved);
    // Ref count: 2 (original + atomic)
    assert_eq!(Arc::strong_count(&data), 2);

    drop(atomic);
    // Ref count: 1 (original only)
    assert_eq!(Arc::strong_count(&data), 1);
}

#[test]
fn test_compare_set_success_no_arc_leak() {
    let drops = Arc::new(AtomicUsize::new(0));
    let initial = Arc::new(DropTracked {
        drops: drops.clone(),
    });
    let atomic = AtomicRef::new(initial.clone());

    const ITERATIONS: usize = 100;
    for _ in 0..ITERATIONS {
        let current = atomic.load();
        let new_value = Arc::new(DropTracked {
            drops: drops.clone(),
        });
        assert!(atomic.compare_set(&current, new_value).is_ok());
    }

    drop(atomic);
    drop(initial);
    assert_eq!(drops.load(Ordering::Relaxed), ITERATIONS + 1);
}

#[test]
fn test_compare_set_weak_success_no_arc_leak() {
    let drops = Arc::new(AtomicUsize::new(0));
    let initial = Arc::new(DropTracked {
        drops: drops.clone(),
    });
    let atomic = AtomicRef::new(initial.clone());

    const ITERATIONS: usize = 100;
    for _ in 0..ITERATIONS {
        let current = atomic.load();
        let new_value = Arc::new(DropTracked {
            drops: drops.clone(),
        });
        assert!(atomic.compare_set_weak(&current, new_value).is_ok());
    }

    drop(atomic);
    drop(initial);
    assert_eq!(drops.load(Ordering::Relaxed), ITERATIONS + 1);
}

#[test]
fn test_compare_and_set_weak_success() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    let current = atomic.load();
    assert!(atomic.compare_set_weak(&current, data2).is_ok());
    assert_eq!(atomic.load().value, 100);
}

#[test]
fn test_compare_and_set_weak_failure() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    let wrong_ref = Arc::new(TestData {
        value: 999,
        name: "wrong".to_string(),
    });

    match atomic.compare_set_weak(&wrong_ref, data2) {
        Ok(_) => panic!("Should fail"),
        Err(actual) => {
            assert_eq!(actual.value, 42);
            assert_eq!(actual.name, "first");
        }
    }
}

#[test]
fn test_compare_and_exchange_weak() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    let current = atomic.load();
    let prev = atomic.compare_and_exchange_weak(&current, data2);
    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(atomic.load().value, 100);
}

#[test]
fn test_inner() {
    let data = Arc::new(TestData {
        value: 42,
        name: "test".to_string(),
    });
    let atomic = AtomicRef::new(data.clone());

    let snapshot = atomic.inner().load_full();
    assert_eq!(snapshot.value, 42);
    assert_eq!(snapshot.name, "test");

    let new_data = Arc::new(TestData {
        value: 100,
        name: "new".to_string(),
    });
    atomic.inner().store(new_data.clone());

    let retrieved = atomic.load();
    assert_eq!(retrieved.value, 100);
}

#[test]
fn test_new_with_primitive() {
    let atomic = AtomicRef::new(Arc::new(42));
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_new_with_string() {
    let atomic = AtomicRef::new(Arc::new("hello".to_string()));
    assert_eq!(*atomic.load(), "hello");
}

#[test]
fn test_swap_same_value() {
    let data = Arc::new(42);
    let atomic = AtomicRef::new(data.clone());

    let old = atomic.swap(data.clone());
    assert_eq!(*old, 42);
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_concurrent_get() {
    let data = Arc::new(TestData {
        value: 42,
        name: "shared".to_string(),
    });
    let atomic = Arc::new(AtomicRef::new(data));
    let mut handles = vec![];

    for _ in 0..10 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let value = atomic.load();
                assert_eq!(value.value, 42);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_concurrent_swap() {
    let data = Arc::new(0);
    let atomic = Arc::new(AtomicRef::new(data));
    let mut handles = vec![];

    for i in 0..10 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            atomic.swap(Arc::new(i));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final value should be one of 0-9
    let final_value = *atomic.load();
    assert!(final_value < 10);
}

#[test]
fn test_update_with_closure() {
    let data = Arc::new(10);
    let atomic = AtomicRef::new(data);

    let old = atomic.fetch_update(|current| Arc::new(**current * 2));
    assert_eq!(*old, 10);
    assert_eq!(*atomic.load(), 20);

    // fetch_update returns old value, not new
    let old2 = atomic.fetch_update(|current| Arc::new(**current + 5));
    assert_eq!(*old2, 20); // old value before update
    assert_eq!(*atomic.load(), 25); // new value after update
}

#[test]
fn test_compare_and_set_weak_in_loop() {
    let data = Arc::new(0);
    let atomic = AtomicRef::new(data);

    let mut current = atomic.load();
    for i in 0..10 {
        loop {
            let new_data = Arc::new(i + 1);
            match atomic.compare_set_weak(&current, new_data) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
        current = Arc::new(i + 1);
    }
    assert_eq!(*atomic.load(), 10);
}

#[test]
fn test_compare_and_exchange_weak_in_loop() {
    let data = Arc::new(0);
    let atomic = AtomicRef::new(data);

    let mut current = atomic.load();
    for i in 0..10 {
        loop {
            let new_data = Arc::new(i + 1);
            let prev = atomic.compare_and_exchange_weak(&current, new_data);
            if Arc::ptr_eq(&prev, &current) {
                break;
            }
            current = prev;
        }
        current = Arc::new(i + 1);
    }
    assert_eq!(*atomic.load(), 10);
}

#[test]
fn test_inner_compare_exchange() {
    let data = Arc::new(42);
    let atomic = AtomicRef::new(data.clone());

    let current = atomic.inner().load_full();
    let new_data = Arc::new(100);
    let prev = atomic.inner().compare_and_swap(&current, new_data.clone());
    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(*atomic.load(), 100);
}

#[test]
fn test_clone_independence() {
    let data = Arc::new(TestData {
        value: 42,
        name: "test".to_string(),
    });
    let atomic1 = AtomicRef::new(data);
    let atomic2 = atomic1.clone();

    assert_eq!(atomic1.load().value, 42);
    assert_eq!(atomic2.load().value, 42);

    atomic1.store(Arc::new(TestData {
        value: 100,
        name: "new".to_string(),
    }));

    // atomic2 should still have the old value
    assert_eq!(atomic2.load().value, 42);
}

#[test]
fn test_display_simple() {
    let atomic = AtomicRef::new(Arc::new(42));
    let display_str = format!("{}", atomic);
    assert_eq!(display_str, "42");
}

#[test]
fn test_debug_simple() {
    let atomic = AtomicRef::new(Arc::new(42));
    let debug_str = format!("{:?}", atomic);
    assert!(debug_str.contains("42"));
}

// TODO: Rewrite after UpdatableAtomic removal
// #[test]
// fn test_trait_updatable_atomic_comprehensive() {
//     // Test needs to be rewritten for fetch_update
// }

#[test]
fn test_trait_atomic_comprehensive() {
    fn test_atomic<T: Atomic<Value = Arc<i32>>>(atomic: &T) {
        atomic.store(Arc::new(5));
        assert_eq!(*atomic.load(), 5);

        let old = atomic.swap(Arc::new(10));
        assert_eq!(*old, 5);

        let current = atomic.load();
        assert!(atomic.compare_set(current.clone(), Arc::new(15)).is_ok());

        let current2 = atomic.load();
        let prev = atomic.compare_exchange(current2.clone(), Arc::new(20));
        assert!(Arc::ptr_eq(&prev, &current2) || *prev == 15);
    }

    let atomic = AtomicRef::new(Arc::new(0));
    test_atomic(&atomic);
}

#[test]
fn test_compare_and_set_same_value() {
    let data = Arc::new(42);
    let atomic = AtomicRef::new(data.clone());

    let current = atomic.load();
    assert!(atomic.compare_set(&current, data.clone()).is_ok());
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_compare_and_exchange_same_value() {
    let data = Arc::new(42);
    let atomic = AtomicRef::new(data.clone());

    let current = atomic.load();
    let prev = atomic.compare_exchange(current.clone(), data.clone());
    assert!(Arc::ptr_eq(&prev, &current));
}

#[test]
fn test_compare_and_set_failure_path() {
    let data1 = Arc::new(42);
    let data2 = Arc::new(100);
    let wrong = Arc::new(999);
    let atomic = AtomicRef::new(data1);

    match atomic.compare_set(&wrong, data2) {
        Ok(_) => panic!("Should have failed"),
        Err(actual) => assert_eq!(*actual, 42),
    }
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_compare_and_exchange_failure_path() {
    let data1 = Arc::new(42);
    let data2 = Arc::new(100);
    let wrong = Arc::new(999);
    let atomic = AtomicRef::new(data1);

    let prev = atomic.compare_exchange(wrong.clone(), data2);
    assert_eq!(*prev, 42);
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_compare_and_set_weak_failure_path() {
    let data1 = Arc::new(42);
    let data2 = Arc::new(100);
    let wrong = Arc::new(999);
    let atomic = AtomicRef::new(data1);

    match atomic.compare_set_weak(&wrong, data2) {
        Ok(_) => panic!("Should have failed"),
        Err(actual) => assert_eq!(*actual, 42),
    }
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_compare_and_exchange_weak_failure_path() {
    let data1 = Arc::new(42);
    let data2 = Arc::new(100);
    let wrong = Arc::new(999);
    let atomic = AtomicRef::new(data1);

    let prev = atomic.compare_and_exchange_weak(&wrong, data2);
    assert_eq!(*prev, 42);
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_inner_compare_exchange_failure() {
    let data = Arc::new(42);
    let atomic = AtomicRef::new(data.clone());

    let wrong = Arc::new(999);
    let new_data = Arc::new(100);
    let prev = atomic.inner().compare_and_swap(&wrong, new_data);
    assert_eq!(**prev, 42);
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_inner_compare_exchange_weak_failure() {
    let data = Arc::new(42);
    let atomic = AtomicRef::new(data.clone());

    let wrong = Arc::new(999);
    let new_data = Arc::new(100);
    let prev = atomic.inner().compare_and_swap(&wrong, new_data);
    assert_eq!(**prev, 42);
    assert_eq!(*atomic.load(), 42);
}

#[test]
fn test_compare_and_exchange_success_path() {
    let data1 = Arc::new(42);
    let atomic = AtomicRef::new(data1.clone());

    let current = atomic.load();
    let data2 = Arc::new(100);
    let prev = atomic.compare_exchange(current.clone(), data2);

    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(*atomic.load(), 100);
}

#[test]
fn test_compare_and_exchange_weak_success_path() {
    let data1 = Arc::new(42);
    let atomic = AtomicRef::new(data1.clone());

    let current = atomic.load();
    let data2 = Arc::new(100);
    let prev = atomic.compare_and_exchange_weak(&current, data2);

    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(*atomic.load(), 100);
}

#[test]
fn test_concurrent_get_and_update_high_contention() {
    let atomic = Arc::new(AtomicRef::new(Arc::new(0)));
    let mut handles = vec![];

    for _ in 0..20 {
        let atomic = atomic.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                atomic.fetch_update(|current| Arc::new(**current + 1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Result should be 20 * 10 = 200
    assert_eq!(*atomic.load(), 200);
}

#[test]
fn test_direct_compare_and_exchange_success() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let current = atomic.load();
    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    // Directly call compare_and_exchange method (parameter is reference)
    let prev = atomic.compare_and_exchange(&current, data2);
    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(atomic.load().value, 100);
    assert_eq!(atomic.load().name, "second");
}

#[test]
fn test_direct_compare_and_exchange_failure() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let wrong_ref = Arc::new(TestData {
        value: 999,
        name: "wrong".to_string(),
    });
    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    // Directly call compare_and_exchange method with wrong reference
    let prev = atomic.compare_and_exchange(&wrong_ref, data2);
    assert_eq!(prev.value, 42);
    assert_eq!(prev.name, "first");
    assert_eq!(atomic.load().value, 42);
}

#[test]
fn test_direct_compare_and_exchange_with_simple_type() {
    let data1 = Arc::new(42);
    let atomic = AtomicRef::new(data1.clone());

    let current = atomic.load();
    let data2 = Arc::new(100);

    // Directly call compare_and_exchange method
    let prev = atomic.compare_and_exchange(&current, data2);
    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(*atomic.load(), 100);
}

#[test]
fn test_direct_compare_and_exchange_weak_success() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let current = atomic.load();
    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    // Directly call compare_and_exchange_weak method (parameter is reference)
    let prev = atomic.compare_and_exchange_weak(&current, data2);
    assert!(Arc::ptr_eq(&prev, &current));
    assert_eq!(atomic.load().value, 100);
}

#[test]
fn test_direct_compare_and_exchange_weak_failure() {
    let data1 = Arc::new(TestData {
        value: 42,
        name: "first".to_string(),
    });
    let atomic = AtomicRef::new(data1.clone());

    let wrong_ref = Arc::new(TestData {
        value: 999,
        name: "wrong".to_string(),
    });
    let data2 = Arc::new(TestData {
        value: 100,
        name: "second".to_string(),
    });

    // Directly call compare_and_exchange_weak method with wrong reference
    let prev = atomic.compare_and_exchange_weak(&wrong_ref, data2);
    assert_eq!(prev.value, 42);
    assert_eq!(prev.name, "first");
    assert_eq!(atomic.load().value, 42);
}

#[test]
fn test_direct_compare_and_exchange_weak_in_loop() {
    let data = Arc::new(TestData {
        value: 0,
        name: "counter".to_string(),
    });
    let atomic = AtomicRef::new(data);

    // Use compare_and_exchange_weak in loop for updates
    let mut current = atomic.load();
    loop {
        let new_data = Arc::new(TestData {
            value: current.value + 1,
            name: "updated".to_string(),
        });
        let prev = atomic.compare_and_exchange_weak(&current, new_data);
        if Arc::ptr_eq(&prev, &current) {
            break;
        }
        current = prev;
    }

    assert_eq!(atomic.load().value, 1);
    assert_eq!(atomic.load().name, "updated");
}
