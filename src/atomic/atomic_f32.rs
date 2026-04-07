/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic 32-bit Floating Point
//!
//! Provides an easy-to-use atomic 32-bit floating point type with sensible
//! default memory orderings. Implemented using bit conversion with AtomicU32.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use crate::atomic::traits::Atomic;
use crate::atomic::traits::AtomicNumber;

/// Atomic 32-bit floating point number.
///
/// Provides easy-to-use atomic operations with automatic memory ordering
/// selection. Implemented using `AtomicU32` with bit conversion.
///
/// # Memory Ordering Strategy
///
/// This type uses the same memory ordering strategy as atomic integers:
///
/// - **Read operations** (`load`): Use `Acquire` ordering to ensure
///   visibility of prior writes from other threads.
///
/// - **Write operations** (`store`): Use `Release` ordering to ensure
///   visibility of prior writes to other threads.
///
/// - **Read-Modify-Write operations** (`swap`, `compare_set`): Use
///   `AcqRel` ordering for full synchronization.
///
/// - **CAS-based arithmetic** (`fetch_add`, `fetch_sub`, etc.): Use
///   `AcqRel` on success and `Acquire` on failure within the CAS loop.
///   The loop ensures eventual consistency.
///
/// # Implementation Details
///
/// Since hardware doesn't provide native atomic floating-point operations,
/// this type is implemented using `AtomicU32` with `f32::to_bits()` and
/// `f32::from_bits()` conversions. This preserves bit patterns exactly,
/// including special values like NaN and infinity.
///
/// # Features
///
/// - Automatic memory ordering selection
/// - Arithmetic operations via CAS loops
/// - Zero-cost abstraction with inline methods
/// - Access to underlying type via `inner()` for advanced use cases
///
/// # Limitations
///
/// - Arithmetic operations use CAS loops (slower than integer operations)
/// - NaN values may cause unexpected behavior in CAS operations
/// - No max/min operations (complex floating point semantics)
///
/// # Example
///
/// ```rust
/// use qubit_atomic::AtomicF32;
/// use std::sync::Arc;
/// use std::thread;
///
/// let sum = Arc::new(AtomicF32::new(0.0));
/// let mut handles = vec![];
///
/// for _ in 0..10 {
///     let sum = sum.clone();
///     let handle = thread::spawn(move || {
///         for _ in 0..100 {
///             sum.add(0.1);
///         }
///     });
///     handles.push(handle);
/// }
///
/// for handle in handles {
///     handle.join().unwrap();
/// }
///
/// // Note: Due to floating point precision, result may not be exactly 100.0
/// let result = sum.load();
/// assert!((result - 100.0).abs() < 0.01);
/// ```
///
/// # Author
///
/// Haixing Hu
#[repr(transparent)]
pub struct AtomicF32 {
    inner: AtomicU32,
}

impl AtomicF32 {
    /// Creates a new atomic floating point number.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(3.14);
    /// assert_eq!(atomic.load(), 3.14);
    /// ```
    #[inline]
    pub fn new(value: f32) -> Self {
        Self {
            inner: AtomicU32::new(value.to_bits()),
        }
    }

    /// Gets the current value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Acquire` ordering on the underlying `AtomicU32`. This ensures
    /// that all writes from other threads that happened before a `Release`
    /// store are visible after this load.
    ///
    /// # Returns
    ///
    /// The current value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(3.14);
    /// assert_eq!(atomic.load(), 3.14);
    /// ```
    #[inline]
    pub fn load(&self) -> f32 {
        f32::from_bits(self.inner.load(Ordering::Acquire))
    }

    /// Sets a new value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `Release` ordering on the underlying `AtomicU32`. This ensures
    /// that all prior writes in this thread are visible to other threads
    /// that perform an `Acquire` load.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(0.0);
    /// atomic.store(3.14);
    /// assert_eq!(atomic.load(), 3.14);
    /// ```
    #[inline]
    pub fn store(&self, value: f32) {
        self.inner.store(value.to_bits(), Ordering::Release);
    }

    /// Swaps the current value with a new value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Uses `AcqRel` ordering on the underlying `AtomicU32`. This provides
    /// full synchronization for this read-modify-write operation.
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
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(1.0);
    /// let old = atomic.swap(2.0);
    /// assert_eq!(old, 1.0);
    /// assert_eq!(atomic.load(), 2.0);
    /// ```
    #[inline]
    pub fn swap(&self, value: f32) -> f32 {
        f32::from_bits(self.inner.swap(value.to_bits(), Ordering::AcqRel))
    }

    /// Compares and sets the value atomically.
    ///
    /// If the current value equals `current`, sets it to `new` and returns
    /// `Ok(())`. Otherwise, returns `Err(actual)` where `actual` is the
    /// current value.
    ///
    /// # Memory Ordering
    ///
    /// - **Success**: Uses `AcqRel` ordering on the underlying `AtomicU32`
    ///   to ensure full synchronization when the exchange succeeds.
    /// - **Failure**: Uses `Acquire` ordering to observe the actual value
    ///   written by another thread.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(actual)` on failure.
    ///
    /// # Warning
    ///
    /// Due to NaN != NaN, CAS operations with NaN values may behave
    /// unexpectedly. Avoid using NaN in atomic floating point operations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(1.0);
    /// assert!(atomic.compare_set(1.0, 2.0).is_ok());
    /// assert_eq!(atomic.load(), 2.0);
    /// ```
    #[inline]
    pub fn compare_set(&self, current: f32, new: f32) -> Result<(), f32> {
        self.inner
            .compare_exchange(
                current.to_bits(),
                new.to_bits(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map(|_| ())
            .map_err(f32::from_bits)
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
    /// `Ok(())` on success, or `Err(actual)` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(1.0);
    /// let mut current = atomic.load();
    /// loop {
    ///     match atomic.compare_set_weak(current, current + 1.0) {
    ///         Ok(_) => break,
    ///         Err(actual) => current = actual,
    ///     }
    /// }
    /// assert_eq!(atomic.load(), 2.0);
    /// ```
    #[inline]
    pub fn compare_set_weak(&self, current: f32, new: f32) -> Result<(), f32> {
        self.inner
            .compare_exchange_weak(
                current.to_bits(),
                new.to_bits(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map(|_| ())
            .map_err(f32::from_bits)
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
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(1.0);
    /// let prev = atomic.compare_and_exchange(1.0, 2.0);
    /// assert_eq!(prev, 1.0);
    /// assert_eq!(atomic.load(), 2.0);
    /// ```
    #[inline]
    pub fn compare_and_exchange(&self, current: f32, new: f32) -> f32 {
        match self.inner.compare_exchange(
            current.to_bits(),
            new.to_bits(),
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(prev_bits) => f32::from_bits(prev_bits),
            Err(actual_bits) => f32::from_bits(actual_bits),
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
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(1.0);
    /// let mut current = atomic.load();
    /// loop {
    ///     let prev = atomic.compare_and_exchange_weak(current, current + 1.0);
    ///     if prev == current {
    ///         break;
    ///     }
    ///     current = prev;
    /// }
    /// assert_eq!(atomic.load(), 2.0);
    /// ```
    #[inline]
    pub fn compare_and_exchange_weak(&self, current: f32, new: f32) -> f32 {
        match self.inner.compare_exchange_weak(
            current.to_bits(),
            new.to_bits(),
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(prev_bits) => f32::from_bits(prev_bits),
            Err(actual_bits) => f32::from_bits(actual_bits),
        }
    }

    /// Atomically adds a value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Performance
    ///
    /// May be slow in high-contention scenarios due to the CAS loop.
    /// Consider using atomic integers if performance is critical.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to add.
    ///
    /// # Returns
    ///
    /// The old value before adding.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(10.0);
    /// let old = atomic.fetch_add(5.5);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 15.5);
    /// ```
    #[inline]
    pub fn fetch_add(&self, delta: f32) -> f32 {
        let mut current = self.load();
        loop {
            let new = current + delta;
            match self.compare_set_weak(current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Atomically subtracts a value, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to subtract.
    ///
    /// # Returns
    ///
    /// The old value before subtracting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(10.0);
    /// let old = atomic.fetch_sub(3.5);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 6.5);
    /// ```
    #[inline]
    pub fn fetch_sub(&self, delta: f32) -> f32 {
        let mut current = self.load();
        loop {
            let new = current - delta;
            match self.compare_set_weak(current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Atomically multiplies by a factor, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Parameters
    ///
    /// * `factor` - The factor to multiply by.
    ///
    /// # Returns
    ///
    /// The old value before multiplying.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(10.0);
    /// let old = atomic.fetch_mul(2.5);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 25.0);
    /// ```
    #[inline]
    pub fn fetch_mul(&self, factor: f32) -> f32 {
        let mut current = self.load();
        loop {
            let new = current * factor;
            match self.compare_set_weak(current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Atomically divides by a divisor, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
    ///
    /// # Parameters
    ///
    /// * `divisor` - The divisor to divide by.
    ///
    /// # Returns
    ///
    /// The old value before dividing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(10.0);
    /// let old = atomic.fetch_div(2.0);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 5.0);
    /// ```
    #[inline]
    pub fn fetch_div(&self, divisor: f32) -> f32 {
        let mut current = self.load();
        loop {
            let new = current / divisor;
            match self.compare_set_weak(current, new) {
                Ok(_) => return current,
                Err(actual) => current = actual,
            }
        }
    }

    /// Updates the value using a function, returning the old value.
    ///
    /// # Memory Ordering
    ///
    /// Internally uses a CAS loop with `compare_set_weak`, which uses
    /// `AcqRel` on success and `Acquire` on failure. The loop ensures
    /// eventual consistency even under high contention.
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
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    ///
    /// let atomic = AtomicF32::new(10.0);
    /// let old = atomic.fetch_update(|x| x * 2.0);
    /// assert_eq!(old, 10.0);
    /// assert_eq!(atomic.load(), 20.0);
    /// ```
    #[inline]
    pub fn fetch_update<F>(&self, f: F) -> f32
    where
        F: Fn(f32) -> f32,
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

    /// Gets a reference to the underlying standard library atomic type.
    ///
    /// This allows direct access to the standard library's atomic operations
    /// for advanced use cases that require fine-grained control over memory
    /// ordering.
    ///
    /// # Memory Ordering
    ///
    /// When using the returned reference, you have full control over memory
    /// ordering. Remember to use `f32::to_bits()` and `f32::from_bits()` for
    /// conversions.
    ///
    /// # Returns
    ///
    /// A reference to the underlying `std::sync::atomic::AtomicU32`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicF32;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = AtomicF32::new(0.0);
    /// atomic.inner().store(3.14_f32.to_bits(), Ordering::Relaxed);
    /// let bits = atomic.inner().load(Ordering::Relaxed);
    /// assert_eq!(f32::from_bits(bits), 3.14);
    /// ```
    #[inline]
    pub fn inner(&self) -> &AtomicU32 {
        &self.inner
    }
}

impl Atomic for AtomicF32 {
    type Value = f32;

    #[inline]
    fn load(&self) -> f32 {
        self.load()
    }

    #[inline]
    fn store(&self, value: f32) {
        self.store(value);
    }

    #[inline]
    fn swap(&self, value: f32) -> f32 {
        self.swap(value)
    }

    #[inline]
    fn compare_set(&self, current: f32, new: f32) -> Result<(), f32> {
        self.compare_set(current, new)
    }

    #[inline]
    fn compare_set_weak(&self, current: f32, new: f32) -> Result<(), f32> {
        self.compare_set_weak(current, new)
    }

    #[inline]
    fn compare_exchange(&self, current: f32, new: f32) -> f32 {
        self.compare_and_exchange(current, new)
    }

    #[inline]
    fn compare_exchange_weak(&self, current: f32, new: f32) -> f32 {
        self.compare_and_exchange_weak(current, new)
    }

    #[inline]
    fn fetch_update<F>(&self, f: F) -> f32
    where
        F: Fn(f32) -> f32,
    {
        self.fetch_update(f)
    }
}

impl AtomicNumber for AtomicF32 {
    #[inline]
    fn fetch_add(&self, delta: f32) -> f32 {
        self.fetch_add(delta)
    }

    #[inline]
    fn fetch_sub(&self, delta: f32) -> f32 {
        self.fetch_sub(delta)
    }

    #[inline]
    fn fetch_mul(&self, factor: f32) -> f32 {
        self.fetch_mul(factor)
    }

    #[inline]
    fn fetch_div(&self, divisor: f32) -> f32 {
        self.fetch_div(divisor)
    }
}

unsafe impl Send for AtomicF32 {}
unsafe impl Sync for AtomicF32 {}

impl Default for AtomicF32 {
    #[inline]
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl From<f32> for AtomicF32 {
    #[inline]
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl fmt::Debug for AtomicF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AtomicF32")
            .field("value", &self.load())
            .finish()
    }
}

impl fmt::Display for AtomicF32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
