/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Reference
//!
//! Provides an easy-to-use atomic reference type with sensible default memory
//! orderings. Uses `Arc<T>` for thread-safe reference counting.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::atomic::traits::Atomic;

/// Atomic reference type.
///
/// Provides easy-to-use atomic operations on references with automatic memory
/// ordering selection. Uses `Arc<T>` for thread-safe reference counting.
///
/// # Memory Ordering Strategy
///
/// This type uses the same memory ordering strategy as other atomic types:
///
/// - **Read operations** (`load`): Use `Acquire` ordering to ensure that
///   all writes from other threads that happened before a `Release` store
///   are visible after this load.
///
/// - **Write operations** (`store`): Use `Release` ordering to ensure that
///   all prior writes in this thread are visible to other threads that
///   perform an `Acquire` load.
///
/// - **Read-Modify-Write operations** (`swap`, `compare_set`): Use
///   `AcqRel` ordering to combine both `Acquire` and `Release` semantics.
///
/// - **CAS failure**: Use `Acquire` ordering on failure to observe the
///   actual value written by another thread.
///
/// # Implementation Details
///
/// This type stores an `Arc<T>` as a raw pointer in `AtomicPtr<T>`. All
/// operations properly manage reference counts to prevent memory leaks or
/// use-after-free errors.
///
/// # Features
///
/// - Automatic memory ordering selection
/// - Thread-safe reference counting via `Arc`
/// - Functional update operations
/// - Zero-cost abstraction with inline methods
///
/// # Example
///
/// ```rust
/// use qubit_atomic::AtomicRef;
/// use std::sync::Arc;
///
/// #[derive(Debug, Clone)]
/// struct Config {
///     timeout: u64,
///     max_retries: u32,
/// }
///
/// let config = Arc::new(Config {
///     timeout: 1000,
///     max_retries: 3,
/// });
///
/// let atomic_config = AtomicRef::new(config);
///
/// // Update configuration
/// let new_config = Arc::new(Config {
///     timeout: 2000,
///     max_retries: 5,
/// });
///
/// let old_config = atomic_config.swap(new_config);
/// assert_eq!(old_config.timeout, 1000);
/// assert_eq!(atomic_config.load().timeout, 2000);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct AtomicRef<T> {
    inner: AtomicPtr<T>,
}

impl<T> AtomicRef<T> {
    /// Creates a new atomic reference.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial reference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let data = Arc::new(42);
    /// let atomic = AtomicRef::new(data);
    /// assert_eq!(*atomic.load(), 42);
    /// ```
    #[inline]
    pub fn new(value: Arc<T>) -> Self {
        let ptr = Arc::into_raw(value) as *mut T;
        Self {
            inner: AtomicPtr::new(ptr),
        }
    }

    /// Gets the current reference.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Acquire` ordering. This ensures that all writes from other
    /// threads that happened before a `Release` store are visible after
    /// this load.
    ///
    /// # Returns
    ///
    /// A cloned `Arc` pointing to the current value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(42));
    /// let value = atomic.load();
    /// assert_eq!(*value, 42);
    /// ```
    #[inline]
    pub fn load(&self) -> Arc<T> {
        let ptr = self.inner.load(Ordering::Acquire);
        unsafe {
            // Increment reference count but don't drop the original pointer
            let arc = Arc::from_raw(ptr);
            let cloned = arc.clone();
            let _ = Arc::into_raw(arc); // Prevent dropping
            cloned
        }
    }

    /// Sets a new reference.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Release` ordering. This ensures that all prior writes in this
    /// thread are visible to other threads that perform an `Acquire` load.
    ///
    /// # Parameters
    ///
    /// * `value` - The new reference to set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(42));
    /// atomic.store(Arc::new(100));
    /// assert_eq!(*atomic.load(), 100);
    /// ```
    #[inline]
    pub fn store(&self, value: Arc<T>) {
        let new_ptr = Arc::into_raw(value) as *mut T;
        let old_ptr = self.inner.swap(new_ptr, Ordering::AcqRel);
        unsafe {
            if !old_ptr.is_null() {
                // Drop the old value
                Arc::from_raw(old_ptr);
            }
        }
    }

    /// Swaps the current reference with a new reference, returning the old
    /// reference.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering. This provides full synchronization for this
    /// read-modify-write operation.
    ///
    /// # Parameters
    ///
    /// * `value` - The new reference to swap in.
    ///
    /// # Returns
    ///
    /// The old reference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let old = atomic.swap(Arc::new(20));
    /// assert_eq!(*old, 10);
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[inline]
    pub fn swap(&self, value: Arc<T>) -> Arc<T> {
        let new_ptr = Arc::into_raw(value) as *mut T;
        let old_ptr = self.inner.swap(new_ptr, Ordering::AcqRel);
        unsafe { Arc::from_raw(old_ptr) }
    }

    /// Compares and sets the reference atomically.
    ///
    /// If the current reference equals `current` (by pointer equality), sets
    /// it to `new` and returns `Ok(())`. Otherwise, returns `Err(actual)`
    /// where `actual` is the current reference.
    ///
    /// # Memory Ordering
    ///
    /// - **Success**: Uses `AcqRel` ordering to ensure full synchronization
    ///   when the exchange succeeds.
    /// - **Failure**: Uses `Acquire` ordering to observe the actual value
    ///   written by another thread.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(actual)` on failure.
    ///
    /// # Note
    ///
    /// Comparison uses pointer equality (`Arc::ptr_eq`), not value equality.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let current = atomic.load();
    ///
    /// assert!(atomic.compare_set(&current, Arc::new(20)).is_ok());
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[inline]
    pub fn compare_set(&self, current: &Arc<T>, new: Arc<T>) -> Result<(), Arc<T>> {
        let current_ptr = Arc::as_ptr(current) as *mut T;
        let new_ptr = Arc::into_raw(new) as *mut T;

        match self
            .inner
            .compare_exchange(current_ptr, new_ptr, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => Ok(()),
            Err(actual_ptr) => unsafe {
                // CAS failed, need to restore the new Arc and return actual
                let _new_arc = Arc::from_raw(new_ptr);
                let actual_arc = Arc::from_raw(actual_ptr);
                let cloned = actual_arc.clone();
                let _ = Arc::into_raw(actual_arc); // Prevent dropping
                Err(cloned)
            },
        }
    }

    /// Weak version of compare-and-set.
    ///
    /// May spuriously fail even when the comparison succeeds. Should be used
    /// in a loop.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(actual)` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let mut current = atomic.load();
    /// loop {
    ///     match atomic.compare_set_weak(&current, Arc::new(20)) {
    ///         Ok(_) => break,
    ///         Err(actual) => current = actual,
    ///     }
    /// }
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[inline]
    pub fn compare_set_weak(&self, current: &Arc<T>, new: Arc<T>) -> Result<(), Arc<T>> {
        let current_ptr = Arc::as_ptr(current) as *mut T;
        let new_ptr = Arc::into_raw(new) as *mut T;

        match self.inner.compare_exchange_weak(
            current_ptr,
            new_ptr,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => Ok(()),
            Err(actual_ptr) => unsafe {
                // CAS failed, need to restore the new Arc and return actual
                let _new_arc = Arc::from_raw(new_ptr);
                let actual_arc = Arc::from_raw(actual_ptr);
                let cloned = actual_arc.clone();
                let _ = Arc::into_raw(actual_arc); // Prevent dropping
                Err(cloned)
            },
        }
    }

    /// Compares and exchanges the reference atomically, returning the
    /// previous reference.
    ///
    /// If the current reference equals `current` (by pointer equality), sets
    /// it to `new` and returns the old reference. Otherwise, returns the
    /// actual current reference.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// The reference before the operation.
    ///
    /// # Note
    ///
    /// Comparison uses pointer equality (`Arc::ptr_eq`), not value equality.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let current = atomic.load();
    ///
    /// let prev = atomic.compare_and_exchange(&current, Arc::new(20));
    /// assert!(Arc::ptr_eq(&prev, &current));
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[inline]
    pub fn compare_and_exchange(&self, current: &Arc<T>, new: Arc<T>) -> Arc<T> {
        let current_ptr = Arc::as_ptr(current) as *mut T;
        let new_ptr = Arc::into_raw(new) as *mut T;

        match self
            .inner
            .compare_exchange(current_ptr, new_ptr, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(prev_ptr) => unsafe { Arc::from_raw(prev_ptr) },
            Err(actual_ptr) => unsafe {
                // CAS failed, need to restore the new Arc and return actual
                let _ = Arc::from_raw(new_ptr);
                let actual_arc = Arc::from_raw(actual_ptr);
                let cloned = actual_arc.clone();
                let _ = Arc::into_raw(actual_arc); // Prevent dropping
                cloned
            },
        }
    }

    /// Weak version of compare-and-exchange.
    ///
    /// May spuriously fail even when the comparison succeeds. Should be used
    /// in a loop.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// The reference before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let mut current = atomic.load();
    /// loop {
    ///     let prev =
    ///         atomic.compare_and_exchange_weak(&current, Arc::new(20));
    ///     if Arc::ptr_eq(&prev, &current) {
    ///         break;
    ///     }
    ///     current = prev;
    /// }
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[inline]
    pub fn compare_and_exchange_weak(&self, current: &Arc<T>, new: Arc<T>) -> Arc<T> {
        let current_ptr = Arc::as_ptr(current) as *mut T;
        let new_ptr = Arc::into_raw(new) as *mut T;

        match self.inner.compare_exchange_weak(
            current_ptr,
            new_ptr,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(prev_ptr) => unsafe { Arc::from_raw(prev_ptr) },
            Err(actual_ptr) => unsafe {
                // CAS failed, need to restore the new Arc and return actual
                let _ = Arc::from_raw(new_ptr);
                let actual_arc = Arc::from_raw(actual_ptr);
                let cloned = actual_arc.clone();
                let _ = Arc::into_raw(actual_arc); // Prevent dropping
                cloned
            },
        }
    }

    /// Updates the reference using a function, returning the old reference.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current reference and returns the
    ///   new reference.
    ///
    /// # Returns
    ///
    /// The old reference before the update.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let old = atomic.fetch_update(|x| Arc::new(*x * 2));
    /// assert_eq!(*old, 10);
    /// assert_eq!(*atomic.load(), 20);
    /// ```
    #[inline]
    pub fn fetch_update<F>(&self, f: F) -> Arc<T>
    where
        F: Fn(&Arc<T>) -> Arc<T>,
    {
        let mut current = self.load();
        loop {
            let new = f(&current);
            match self.compare_set_weak(&current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Gets a reference to the underlying standard library atomic type.
    ///
    /// This allows direct access to the standard library's atomic operations
    /// for advanced use cases that require fine-grained control over memory
    /// ordering.
    ///
    /// # Memory Ordering
    ///
    /// When using the returned reference, you have full control over memory
    /// ordering. Choose the appropriate ordering based on your specific
    /// synchronization requirements.
    ///
    /// # Returns
    ///
    /// A reference to the underlying `std::sync::atomic::AtomicPtr<T>`.
    ///
    /// # Warning
    ///
    /// Direct manipulation of the underlying pointer requires careful
    /// management of Arc reference counts to avoid memory leaks or
    /// use-after-free bugs.
    #[inline]
    pub fn inner(&self) -> &AtomicPtr<T> {
        &self.inner
    }
}

impl<T> Atomic for AtomicRef<T> {
    type Value = Arc<T>;

    #[inline]
    fn load(&self) -> Arc<T> {
        self.load()
    }

    #[inline]
    fn store(&self, value: Arc<T>) {
        self.store(value);
    }

    #[inline]
    fn swap(&self, value: Arc<T>) -> Arc<T> {
        self.swap(value)
    }

    #[inline]
    fn compare_set(&self, current: Arc<T>, new: Arc<T>) -> Result<(), Arc<T>> {
        self.compare_set(&current, new)
    }

    #[inline]
    fn compare_set_weak(&self, current: Arc<T>, new: Arc<T>) -> Result<(), Arc<T>> {
        self.compare_set_weak(&current, new)
    }

    #[inline]
    fn compare_exchange(&self, current: Arc<T>, new: Arc<T>) -> Arc<T> {
        match self.compare_set(&current, new.clone()) {
            Ok(_) => current,
            Err(_) => self.load(),
        }
    }

    #[inline]
    fn compare_exchange_weak(&self, current: Arc<T>, new: Arc<T>) -> Arc<T> {
        match self.compare_set_weak(&current, new.clone()) {
            Ok(_) => current,
            Err(_) => self.load(),
        }
    }

    #[inline]
    fn fetch_update<F>(&self, f: F) -> Arc<T>
    where
        F: Fn(Arc<T>) -> Arc<T>,
    {
        self.fetch_update(|x| f(x.clone()))
    }
}

impl<T> Clone for AtomicRef<T> {
    /// Clones the atomic reference.
    ///
    /// Creates a new `AtomicRef` that initially points to the same value as
    /// the original, but subsequent atomic operations are independent.
    fn clone(&self) -> Self {
        Self::new(self.load())
    }
}

impl<T> Drop for AtomicRef<T> {
    fn drop(&mut self) {
        let ptr = self.inner.load(Ordering::Acquire);
        unsafe {
            if !ptr.is_null() {
                Arc::from_raw(ptr);
            }
        }
    }
}

unsafe impl<T: Send + Sync> Send for AtomicRef<T> {}
unsafe impl<T: Send + Sync> Sync for AtomicRef<T> {}

impl<T: fmt::Debug> fmt::Debug for AtomicRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AtomicRef")
            .field("value", &self.load())
            .finish()
    }
}

impl<T: fmt::Display> fmt::Display for AtomicRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
