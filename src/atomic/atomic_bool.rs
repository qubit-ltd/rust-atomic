/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Boolean
//!
//! Provides an easy-to-use atomic boolean type with sensible default memory
//! orderings.
//!
//! # Author
//!
//! Haixing Hu

use std::sync::atomic::AtomicBool as StdAtomicBool;
use std::sync::atomic::Ordering;

use crate::atomic::atomic_ops::AtomicOps;

/// Atomic boolean type.
///
/// Provides easy-to-use atomic operations with automatic memory ordering
/// selection. All methods are thread-safe and can be shared across threads.
///
/// # Memory Ordering Strategy
///
/// This type uses carefully selected default memory orderings:
///
/// - **Read operations** (`load`): Use `Acquire` ordering to ensure that
///   all writes from other threads that happened before a `Release` store
///   are visible after this load.
///
/// - **Write operations** (`store`): Use `Release` ordering to ensure that
///   all prior writes in this thread are visible to other threads that
///   perform an `Acquire` load.
///
/// - **Read-Modify-Write operations** (`swap`, `compare_set`, `fetch_*`):
///   Use `AcqRel` ordering to combine both `Acquire` and `Release`
///   semantics, ensuring proper synchronization in both directions.
///
/// - **CAS failure**: Use `Acquire` ordering on failure to observe the
///   actual value written by another thread.
///
/// These orderings provide a balance between performance and correctness
/// for typical concurrent programming patterns.
///
/// # Features
///
/// - Automatic memory ordering selection
/// - Rich set of boolean-specific operations
/// - Zero-cost abstraction with inline methods
/// - Access to underlying type via `inner()` for advanced use cases
///
/// # Example
///
/// ```rust
/// use qubit_atomic::Atomic;
/// use std::sync::Arc;
/// use std::thread;
///
/// let flag = Arc::new(Atomic::<bool>::new(false));
/// let flag_clone = flag.clone();
///
/// let handle = thread::spawn(move || {
///     flag_clone.store(true);
/// });
///
/// handle.join().unwrap();
/// assert_eq!(flag.load(), true);
/// ```
///
/// # Author
///
/// Haixing Hu
#[repr(transparent)]
pub struct AtomicBool {
    /// Standard-library atomic boolean used as the storage backend.
    inner: StdAtomicBool,
}

impl AtomicBool {
    /// Creates a new atomic boolean.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value.
    ///
    /// # Returns
    ///
    /// An atomic boolean initialized to `value`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert_eq!(flag.load(), false);
    /// ```
    #[inline]
    pub const fn new(value: bool) -> Self {
        Self {
            inner: StdAtomicBool::new(value),
        }
    }

    /// Gets the current value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Acquire` ordering. This ensures that:
    /// - All writes from other threads that happened before a `Release`
    ///   store are visible after this load.
    /// - Forms a synchronizes-with relationship with `Release` stores.
    /// - Prevents reordering of subsequent reads/writes before this load.
    ///
    /// This is appropriate for reading shared state that may have been
    /// modified by other threads.
    ///
    /// # Returns
    ///
    /// The current value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(true);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn load(&self) -> bool {
        self.inner.load(Ordering::Acquire)
    }

    /// Sets a new value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Release` ordering. This ensures that:
    /// - All prior writes in this thread are visible to other threads that
    ///   perform an `Acquire` load.
    /// - Forms a synchronizes-with relationship with `Acquire` loads.
    /// - Prevents reordering of prior reads/writes after this store.
    ///
    /// This is appropriate for publishing shared state to other threads.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// flag.store(true);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn store(&self, value: bool) {
        self.inner.store(value, Ordering::Release);
    }

    /// Swaps the current value with a new value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering. This ensures that:
    /// - **Acquire**: All writes from other threads that happened before
    ///   their `Release` operations are visible after this operation.
    /// - **Release**: All prior writes in this thread are visible to other
    ///   threads that perform subsequent `Acquire` operations.
    ///
    /// This provides full synchronization for read-modify-write operations.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to swap in.
    ///
    /// # Returns
    ///
    /// The old value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// let old = flag.swap(true);
    /// assert_eq!(old, false);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn swap(&self, value: bool) -> bool {
        self.inner.swap(value, Ordering::AcqRel)
    }

    /// Compares and sets the value atomically.
    ///
    /// If the current value equals `current`, sets it to `new` and returns
    /// `Ok(())`. Otherwise, returns `Err(actual)` where `actual` is the
    /// current value.
    ///
    /// # Memory Ordering
    ///
    /// - **Success**: Uses `AcqRel` ordering to ensure full synchronization
    ///   when the exchange succeeds.
    /// - **Failure**: Uses `Acquire` ordering to observe the actual value
    ///   written by another thread.
    ///
    /// This pattern is essential for implementing lock-free algorithms where
    /// you need to retry based on the observed value.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails. In that case, `new` is not stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert!(flag.compare_set(false, true).is_ok());
    /// assert_eq!(flag.load(), true);
    ///
    /// // Fails because current value is true, not false
    /// assert!(flag.compare_set(false, false).is_err());
    /// ```
    #[inline]
    pub fn compare_set(&self, current: bool, new: bool) -> Result<(), bool> {
        self.inner
            .compare_exchange(current, new, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
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
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails, including possible spurious failures. In that case, `new` is not
    /// stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// let mut current = flag.load();
    /// loop {
    ///     match flag.compare_set_weak(current, true) {
    ///         Ok(_) => break,
    ///         Err(actual) => current = actual,
    ///     }
    /// }
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn compare_set_weak(&self, current: bool, new: bool) -> Result<(), bool> {
        self.inner
            .compare_exchange_weak(current, new, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
    }

    /// Compares and exchanges the value atomically, returning the previous
    /// value.
    ///
    /// If the current value equals `current`, sets it to `new` and returns
    /// the old value. Otherwise, returns the actual current value.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// The value observed before the operation completed. If the returned
    /// value equals `current`, the exchange succeeded; otherwise it is the
    /// actual value that prevented the exchange.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// let prev = flag.compare_and_exchange(false, true);
    /// assert_eq!(prev, false);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn compare_and_exchange(&self, current: bool, new: bool) -> bool {
        match self
            .inner
            .compare_exchange(current, new, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(prev) => prev,
            Err(actual) => actual,
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
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// The value observed before the operation completed. Because this
    /// operation may fail spuriously, a returned value equal to `current` does
    /// not by itself prove that `new` was stored; use
    /// [`compare_set_weak`](Self::compare_set_weak) when the caller needs an
    /// explicit success indicator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// let mut current = flag.load();
    /// loop {
    ///     let prev = flag.compare_and_exchange_weak(current, true);
    ///     if flag.load() {
    ///         break;
    ///     }
    ///     current = prev;
    /// }
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn compare_and_exchange_weak(&self, current: bool, new: bool) -> bool {
        match self
            .inner
            .compare_exchange_weak(current, new, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(prev) => prev,
            Err(actual) => actual,
        }
    }

    /// Atomically sets the value to `true`, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering (via `swap`). This ensures full
    /// synchronization with other threads for this read-modify-write
    /// operation.
    ///
    /// # Returns
    ///
    /// The old value before setting to `true`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// let old = flag.fetch_set();
    /// assert_eq!(old, false);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn fetch_set(&self) -> bool {
        self.swap(true)
    }

    /// Atomically sets the value to `false`, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering (via `swap`). This ensures full
    /// synchronization with other threads for this read-modify-write
    /// operation.
    ///
    /// # Returns
    ///
    /// The old value before setting to `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(true);
    /// let old = flag.fetch_clear();
    /// assert_eq!(old, true);
    /// assert_eq!(flag.load(), false);
    /// ```
    #[inline]
    pub fn fetch_clear(&self) -> bool {
        self.swap(false)
    }

    /// Atomically negates the value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering. This ensures full synchronization with other
    /// threads for this read-modify-write operation.
    ///
    /// # Returns
    ///
    /// The old value before negation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert_eq!(flag.fetch_not(), false);
    /// assert_eq!(flag.load(), true);
    /// assert_eq!(flag.fetch_not(), true);
    /// assert_eq!(flag.load(), false);
    /// ```
    #[inline]
    pub fn fetch_not(&self) -> bool {
        self.inner.fetch_xor(true, Ordering::AcqRel)
    }

    /// Atomically performs logical AND, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering. This ensures full synchronization with other
    /// threads for this read-modify-write operation, which is necessary
    /// because the operation depends on the current value.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to AND with.
    ///
    /// # Returns
    ///
    /// The old value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(true);
    /// assert_eq!(flag.fetch_and(false), true);
    /// assert_eq!(flag.load(), false);
    /// ```
    #[inline]
    pub fn fetch_and(&self, value: bool) -> bool {
        self.inner.fetch_and(value, Ordering::AcqRel)
    }

    /// Atomically performs logical OR, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering. This ensures full synchronization with other
    /// threads for this read-modify-write operation, which is necessary
    /// because the operation depends on the current value.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to OR with.
    ///
    /// # Returns
    ///
    /// The old value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert_eq!(flag.fetch_or(true), false);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn fetch_or(&self, value: bool) -> bool {
        self.inner.fetch_or(value, Ordering::AcqRel)
    }

    /// Atomically performs logical XOR, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering. This ensures full synchronization with other
    /// threads for this read-modify-write operation, which is necessary
    /// because the operation depends on the current value.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to XOR with.
    ///
    /// # Returns
    ///
    /// The old value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert_eq!(flag.fetch_xor(true), false);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn fetch_xor(&self, value: bool) -> bool {
        self.inner.fetch_xor(value, Ordering::AcqRel)
    }

    /// Conditionally sets the value if it is currently `false`.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `new` - The new value to set if current is `false`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was `false` and has been set to `new`.
    ///
    /// # Errors
    ///
    /// Returns `Err(true)` if the value was already `true`. In that case,
    /// `new` is not stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert!(flag.set_if_false(true).is_ok());
    /// assert_eq!(flag.load(), true);
    ///
    /// // Second attempt fails
    /// assert!(flag.set_if_false(true).is_err());
    /// ```
    #[inline]
    pub fn set_if_false(&self, new: bool) -> Result<(), bool> {
        self.compare_set(false, new)
    }

    /// Conditionally sets the value if it is currently `true`.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `new` - The new value to set if current is `true`.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was `true` and has been set to `new`.
    ///
    /// # Errors
    ///
    /// Returns `Err(false)` if the value was already `false`. In that case,
    /// `new` is not stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(true);
    /// assert!(flag.set_if_true(false).is_ok());
    /// assert_eq!(flag.load(), false);
    ///
    /// // Second attempt fails
    /// assert!(flag.set_if_true(false).is_err());
    /// ```
    #[inline]
    pub fn set_if_true(&self, new: bool) -> Result<(), bool> {
        self.compare_set(true, new)
    }

    /// Updates the value using a function, returning the old value.
    ///
    /// Internally uses a CAS loop until the update succeeds.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current value and returns the new
    ///   value.
    ///
    /// # Returns
    ///
    /// The old value before the update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert_eq!(flag.fetch_update(|current| !current), false);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn fetch_update<F>(&self, f: F) -> bool
    where
        F: Fn(bool) -> bool,
    {
        let mut current = self.load();
        loop {
            let new = f(current);
            match self.compare_set_weak(current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Conditionally updates the value using a function.
    ///
    /// Internally uses a CAS loop until the update succeeds or the closure
    /// rejects the current value by returning `None`.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current value and returns the new
    ///   value, or `None` to leave the value unchanged.
    ///
    /// # Returns
    ///
    /// `Some(old_value)` when the update succeeds, or `None` when `f` rejects
    /// the observed current value.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// assert_eq!(flag.try_update(|current| (!current).then_some(true)), Some(false));
    /// assert_eq!(flag.load(), true);
    /// assert_eq!(flag.try_update(|current| (!current).then_some(true)), None);
    /// assert_eq!(flag.load(), true);
    /// ```
    #[inline]
    pub fn try_update<F>(&self, f: F) -> Option<bool>
    where
        F: Fn(bool) -> Option<bool>,
    {
        let mut current = self.load();
        loop {
            let new = f(current)?;
            match self.compare_set_weak(current, new) {
                Ok(_) => return Some(current),
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
    /// A reference to the underlying `std::sync::atomic::AtomicBool`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    /// use std::sync::atomic::Ordering;
    ///
    /// let flag = Atomic::<bool>::new(false);
    /// flag.inner().store(true, Ordering::Relaxed);
    /// assert_eq!(flag.inner().load(Ordering::Relaxed), true);
    /// ```
    #[inline]
    pub fn inner(&self) -> &StdAtomicBool {
        &self.inner
    }
}

impl AtomicOps for AtomicBool {
    type Value = bool;

    #[inline]
    fn load(&self) -> bool {
        self.load()
    }

    #[inline]
    fn store(&self, value: bool) {
        self.store(value);
    }

    #[inline]
    fn swap(&self, value: bool) -> bool {
        self.swap(value)
    }

    #[inline]
    fn compare_set(&self, current: bool, new: bool) -> Result<(), bool> {
        self.compare_set(current, new)
    }

    #[inline]
    fn compare_set_weak(&self, current: bool, new: bool) -> Result<(), bool> {
        self.compare_set_weak(current, new)
    }

    #[inline]
    fn compare_exchange(&self, current: bool, new: bool) -> bool {
        self.compare_and_exchange(current, new)
    }

    #[inline]
    fn compare_exchange_weak(&self, current: bool, new: bool) -> bool {
        self.compare_and_exchange_weak(current, new)
    }

    #[inline]
    fn fetch_update<F>(&self, f: F) -> bool
    where
        F: Fn(bool) -> bool,
    {
        self.fetch_update(f)
    }

    #[inline]
    fn try_update<F>(&self, f: F) -> Option<bool>
    where
        F: Fn(bool) -> Option<bool>,
    {
        self.try_update(f)
    }
}
