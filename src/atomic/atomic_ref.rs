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

use arc_swap::{
    ArcSwap,
    Guard,
};
use std::fmt;
use std::sync::Arc;

/// Atomic reference type.
///
/// Provides easy-to-use atomic operations on references with automatic memory
/// ordering selection. Uses `Arc<T>` for thread-safe reference counting.
///
/// # Implementation Details
///
/// This type is backed by `arc_swap::ArcSwap<T>`, which provides lock-free,
/// memory-safe atomic replacement and loading of `Arc<T>` values without
/// exposing raw-pointer lifetime hazards.
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
    /// Lock-free atomic storage for the current shared reference.
    inner: ArcSwap<T>,
}

impl<T> AtomicRef<T> {
    /// Creates a new atomic reference.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial reference.
    ///
    /// # Returns
    ///
    /// An atomic reference initialized to `value`.
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
        Self {
            inner: ArcSwap::from(value),
        }
    }

    /// Creates a new atomic reference from an owned value.
    ///
    /// This is a convenience constructor for callers that do not already have
    /// an [`Arc<T>`]. It wraps `value` in [`Arc::new`] and then delegates to
    /// [`new`](Self::new).
    ///
    /// # Parameters
    ///
    /// * `value` - The owned value to store as the initial reference.
    ///
    /// # Returns
    ///
    /// An atomic reference initialized to `Arc::new(value)`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    ///
    /// let atomic = AtomicRef::from_value(42);
    /// assert_eq!(*atomic.load(), 42);
    /// ```
    #[inline]
    pub fn from_value(value: T) -> Self {
        Self::new(Arc::new(value))
    }

    /// Gets the current reference.
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
        self.inner.load_full()
    }

    /// Gets the current reference as an `ArcSwap` guard.
    ///
    /// This is useful for short-lived reads because it avoids cloning the
    /// underlying [`Arc`] on the fast path. Use [`load`](Self::load) when the
    /// caller needs an owned [`Arc<T>`] that can be stored or moved freely.
    ///
    /// # Returns
    ///
    /// A guard pointing to the current `Arc`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(42));
    /// let guard = atomic.load_guard();
    /// assert_eq!(**guard, 42);
    /// ```
    #[inline]
    pub fn load_guard(&self) -> Guard<Arc<T>> {
        self.inner.load()
    }

    /// Sets a new reference.
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
        self.inner.store(value);
    }

    /// Swaps the current reference with a new reference, returning the old
    /// reference.
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
        self.inner.swap(value)
    }

    /// Compares and sets the reference atomically.
    ///
    /// If the current reference equals `current` (by pointer equality), sets
    /// it to `new` and returns `Ok(())`. Otherwise, returns `Err(actual)`
    /// where `actual` is the current reference.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the pointer comparison succeeds and `new` is stored.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed current reference when the
    /// pointer comparison fails. On failure, `new` is not installed.
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
        let prev = Guard::into_inner(self.inner.compare_and_swap(current, new));
        if Arc::ptr_eq(&prev, current) {
            Ok(())
        } else {
            Err(prev)
        }
    }

    /// Weak version of compare-and-set.
    ///
    /// This implementation currently delegates to the same lock-free CAS as
    /// `compare_set`, so it does not introduce extra spurious failures.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current reference.
    /// * `new` - The new reference to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the pointer comparison succeeds and `new` is stored.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed current reference when the
    /// pointer comparison fails. On failure, `new` is not installed. This
    /// implementation currently delegates to [`compare_set`](Self::compare_set)
    /// and does not add extra spurious failures.
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
        self.compare_set(current, new)
    }

    /// Compares and exchanges the reference atomically, returning the
    /// previous reference.
    ///
    /// If the current reference equals `current` (by pointer equality), sets
    /// it to `new` and returns the old reference. Otherwise, returns the
    /// actual current reference.
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
        Guard::into_inner(self.inner.compare_and_swap(current, new))
    }

    /// Weak version of compare-and-exchange.
    ///
    /// This implementation currently delegates to the same lock-free CAS as
    /// `compare_and_exchange`, so it does not introduce extra spurious
    /// failures.
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
        self.compare_and_exchange(current, new)
    }

    /// Updates the reference using a function, returning the old reference.
    ///
    /// Internally uses a CAS loop until the update succeeds.
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
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(10));
    /// let old = atomic.fetch_update(|x| Arc::new(**x * 2));
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

    /// Conditionally updates the reference using a function.
    ///
    /// Internally uses a pointer-based CAS loop until the update succeeds or
    /// the closure rejects the current reference by returning `None`.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current reference and returns the new
    ///   reference, or `None` to leave the current reference unchanged.
    ///
    /// # Returns
    ///
    /// `Some(old_reference)` when the update succeeds, or `None` when `f`
    /// rejects the observed current reference.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicRef;
    /// use std::sync::Arc;
    ///
    /// let atomic = AtomicRef::new(Arc::new(3));
    /// let old = atomic.try_update(|current| {
    ///     (**current % 2 == 1).then_some(Arc::new(**current + 1))
    /// });
    /// assert_eq!(*old.unwrap(), 3);
    /// assert_eq!(*atomic.load(), 4);
    /// assert!(atomic
    ///     .try_update(|current| {
    ///         (**current % 2 == 1).then_some(Arc::new(**current + 1))
    ///     })
    ///     .is_none());
    /// assert_eq!(*atomic.load(), 4);
    /// ```
    #[inline]
    pub fn try_update<F>(&self, f: F) -> Option<Arc<T>>
    where
        F: Fn(&Arc<T>) -> Option<Arc<T>>,
    {
        let mut current = self.load();
        loop {
            let new = f(&current)?;
            match self.compare_set_weak(&current, new) {
                Ok(_) => return Some(current),
                Err(actual) => current = actual,
            }
        }
    }

    /// Gets a reference to the underlying `ArcSwap`.
    ///
    /// This allows advanced users to access lower-level `ArcSwap` APIs for
    /// custom synchronization strategies.
    ///
    /// # Returns
    ///
    /// A reference to the underlying `arc_swap::ArcSwap<T>`.
    #[inline]
    pub fn inner(&self) -> &ArcSwap<T> {
        &self.inner
    }
}

impl<T> Clone for AtomicRef<T> {
    /// Clones the atomic reference.
    ///
    /// Creates a new `AtomicRef` that initially points to the same value as
    /// the original, but subsequent atomic operations are independent.
    ///
    /// # Returns
    ///
    /// A new atomic reference initialized with a clone of the currently loaded
    /// `Arc`.
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.load())
    }
}

impl<T: fmt::Debug> fmt::Debug for AtomicRef<T> {
    /// Formats the currently loaded reference for debugging.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter receiving the debug representation.
    ///
    /// # Returns
    ///
    /// A formatting result from the formatter.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AtomicRef")
            .field("value", &self.load())
            .finish()
    }
}

impl<T: fmt::Display> fmt::Display for AtomicRef<T> {
    /// Formats the currently loaded reference with display formatting.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter receiving the displayed value.
    ///
    /// # Returns
    ///
    /// A formatting result from the formatter.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
