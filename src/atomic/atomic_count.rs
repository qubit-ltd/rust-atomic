/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Count
//!
//! Provides a non-negative atomic counter for values whose transitions are
//! used as synchronization signals.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::sync::atomic::{AtomicUsize as StdAtomicUsize, Ordering};

/// A non-negative atomic counter with synchronization-oriented operations.
///
/// Use this type when the counter value is part of a concurrent state machine,
/// such as active task counts, in-flight request counts, resource usage counts,
/// or shutdown and termination checks. Its operations return the value after
/// the update, which makes zero-transition logic straightforward.
///
/// For pure metrics, statistics, event counters, or ID generation, prefer the
/// regular atomic integer types such as [`Atomic<usize>`](crate::Atomic).
/// Those types keep arithmetic operations lightweight for pure counting.
///
/// This counter never wraps. Incrementing past [`usize::MAX`] panics, and
/// decrementing below zero panics. Use [`try_add`](Self::try_add),
/// [`try_dec`](Self::try_dec), or [`try_sub`](Self::try_sub) when overflow or
/// underflow is a normal business outcome.
///
/// # Example
///
/// ```rust
/// use qubit_atomic::AtomicCount;
///
/// let active_tasks = AtomicCount::zero();
///
/// active_tasks.inc();
/// assert!(!active_tasks.is_zero());
///
/// if active_tasks.dec() == 0 {
///     // The last active task finished; notify termination waiters here.
/// }
/// ```
///
/// # Author
///
/// Haixing Hu
#[repr(transparent)]
pub struct AtomicCount {
    /// Standard-library atomic storage for the non-negative counter value.
    inner: StdAtomicUsize,
}

impl AtomicCount {
    /// Creates a new non-negative atomic counter.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A counter initialized to `value`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(3);
    /// assert_eq!(counter.get(), 3);
    /// ```
    #[inline]
    pub const fn new(value: usize) -> Self {
        Self {
            inner: StdAtomicUsize::new(value),
        }
    }

    /// Creates a new counter initialized to zero.
    ///
    /// # Returns
    ///
    /// A counter whose current value is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::zero();
    /// assert!(counter.is_zero());
    /// ```
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0)
    }

    /// Gets the current counter value.
    ///
    /// # Returns
    ///
    /// The current counter value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(7);
    /// assert_eq!(counter.get(), 7);
    /// ```
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.load(Ordering::Acquire)
    }

    /// Returns whether the current counter value is zero.
    ///
    /// # Returns
    ///
    /// `true` if the current value is zero, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::zero();
    /// assert!(counter.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.get() == 0
    }

    /// Returns whether the current counter value is greater than zero.
    ///
    /// # Returns
    ///
    /// `true` if the current value is greater than zero, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(1);
    /// assert!(counter.is_positive());
    /// ```
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.get() > 0
    }

    /// Increments the counter by one and returns the new value.
    ///
    /// # Returns
    ///
    /// The counter value after the increment.
    ///
    /// # Panics
    ///
    /// Panics if the increment would overflow [`usize::MAX`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::zero();
    /// assert_eq!(counter.inc(), 1);
    /// ```
    #[inline]
    pub fn inc(&self) -> usize {
        self.add(1)
    }

    /// Adds `delta` to the counter and returns the new value.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to add.
    ///
    /// # Returns
    ///
    /// The counter value after the addition.
    ///
    /// # Panics
    ///
    /// Panics if the addition would overflow [`usize::MAX`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(2);
    /// assert_eq!(counter.add(3), 5);
    /// ```
    #[inline]
    pub fn add(&self, delta: usize) -> usize {
        self.try_add(delta).expect("atomic counter overflow")
    }

    /// Tries to add `delta` to the counter.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to add.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` if the addition succeeds, or `None` if it would
    /// overflow [`usize::MAX`]. On `None`, the counter is left unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(2);
    /// assert_eq!(counter.try_add(3), Some(5));
    /// ```
    #[inline]
    pub fn try_add(&self, delta: usize) -> Option<usize> {
        self.try_update(|current| current.checked_add(delta))
    }

    /// Decrements the counter by one and returns the new value.
    ///
    /// This method is useful for detecting the transition to zero:
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(1);
    /// if counter.dec() == 0 {
    ///     // This call consumed the final counted item.
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// The counter value after the decrement.
    ///
    /// # Panics
    ///
    /// Panics if the current value is zero.
    #[inline]
    pub fn dec(&self) -> usize {
        self.try_dec().expect("atomic counter underflow")
    }

    /// Tries to decrement the counter by one.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` if the decrement succeeds, or `None` if the current
    /// value is zero. On `None`, the counter is left unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(1);
    /// assert_eq!(counter.try_dec(), Some(0));
    /// assert_eq!(counter.try_dec(), None);
    /// ```
    #[inline]
    pub fn try_dec(&self) -> Option<usize> {
        self.try_sub(1)
    }

    /// Subtracts `delta` from the counter and returns the new value.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to subtract.
    ///
    /// # Returns
    ///
    /// The counter value after the subtraction.
    ///
    /// # Panics
    ///
    /// Panics if the subtraction would make the counter negative.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(5);
    /// assert_eq!(counter.sub(2), 3);
    /// ```
    #[inline]
    pub fn sub(&self, delta: usize) -> usize {
        self.try_sub(delta).expect("atomic counter underflow")
    }

    /// Tries to subtract `delta` from the counter.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to subtract.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` if the subtraction succeeds, or `None` if it would
    /// make the counter negative. On `None`, the counter is left unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicCount;
    ///
    /// let counter = AtomicCount::new(3);
    /// assert_eq!(counter.try_sub(2), Some(1));
    /// assert_eq!(counter.try_sub(2), None);
    /// ```
    #[inline]
    pub fn try_sub(&self, delta: usize) -> Option<usize> {
        self.try_update(|current| current.checked_sub(delta))
    }

    /// Applies a checked update with synchronization semantics.
    ///
    /// # Parameters
    ///
    /// * `update` - A function that maps the current value to the next value,
    ///   or returns `None` to reject the update.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` if the update succeeds, or `None` if `update`
    /// rejects the current value. A rejected update leaves the counter
    /// unchanged.
    #[inline]
    fn try_update<F>(&self, update: F) -> Option<usize>
    where
        F: Fn(usize) -> Option<usize>,
    {
        let mut current = self.get();
        loop {
            let next = update(current)?;
            match self.inner.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(next),
                Err(actual) => current = actual,
            }
        }
    }
}

impl Default for AtomicCount {
    /// Creates a zero-valued atomic counter.
    ///
    /// # Returns
    ///
    /// A counter whose current value is zero.
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl From<usize> for AtomicCount {
    /// Converts an initial counter value into an [`AtomicCount`].
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A counter initialized to `value`.
    #[inline]
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl fmt::Debug for AtomicCount {
    /// Formats the current counter value for debugging.
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
        f.debug_struct("AtomicCount")
            .field("value", &self.get())
            .finish()
    }
}

impl fmt::Display for AtomicCount {
    /// Formats the current counter value with decimal display formatting.
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
        write!(f, "{}", self.get())
    }
}
