/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Signed Count
//!
//! Provides an atomic counter for values that may legitimately become
//! negative.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::sync::atomic::{
    AtomicIsize as StdAtomicIsize,
    Ordering,
};

/// A signed atomic counter with synchronization-oriented operations.
///
/// Use this type when the counter models a delta, balance, backlog, offset, or
/// other quantity that may legitimately cross zero. Examples include producer
/// minus consumer deltas, permit debt, retry backlog changes, or accumulated
/// scheduling offsets.
///
/// For counters that must never be negative, prefer
/// [`AtomicCount`](crate::AtomicCount). For pure metrics or statistics,
/// prefer the regular atomic integer types such as
/// [`Atomic<isize>`](crate::Atomic).
///
/// This counter never wraps. Operations that would overflow the signed range
/// panic. Use [`try_add`](Self::try_add) or [`try_sub`](Self::try_sub) when
/// overflow is a normal business outcome.
///
/// # Example
///
/// ```rust
/// use qubit_atomic::AtomicSignedCount;
///
/// let backlog_delta = AtomicSignedCount::zero();
///
/// assert_eq!(backlog_delta.add(5), 5);
/// assert_eq!(backlog_delta.sub(8), -3);
/// assert!(backlog_delta.is_negative());
/// ```
///
/// # Author
///
/// Haixing Hu
#[repr(transparent)]
pub struct AtomicSignedCount {
    /// Standard-library atomic storage for the signed counter value.
    inner: StdAtomicIsize,
}

impl AtomicSignedCount {
    /// Creates a new signed atomic counter.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A signed counter initialized to `value`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(-3);
    /// assert_eq!(counter.get(), -3);
    /// ```
    #[inline]
    pub const fn new(value: isize) -> Self {
        Self {
            inner: StdAtomicIsize::new(value),
        }
    }

    /// Creates a new signed counter initialized to zero.
    ///
    /// # Returns
    ///
    /// A signed counter whose current value is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::zero();
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
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(-7);
    /// assert_eq!(counter.get(), -7);
    /// ```
    #[inline]
    pub fn get(&self) -> isize {
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
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::zero();
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
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(1);
    /// assert!(counter.is_positive());
    /// ```
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.get() > 0
    }

    /// Returns whether the current counter value is less than zero.
    ///
    /// # Returns
    ///
    /// `true` if the current value is less than zero, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(-1);
    /// assert!(counter.is_negative());
    /// ```
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.get() < 0
    }

    /// Increments the counter by one and returns the new value.
    ///
    /// # Returns
    ///
    /// The counter value after the increment.
    ///
    /// # Panics
    ///
    /// Panics if the increment would overflow [`isize::MAX`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::zero();
    /// assert_eq!(counter.inc(), 1);
    /// ```
    #[inline]
    pub fn inc(&self) -> isize {
        self.add(1)
    }

    /// Decrements the counter by one and returns the new value.
    ///
    /// # Returns
    ///
    /// The counter value after the decrement.
    ///
    /// # Panics
    ///
    /// Panics if the decrement would underflow [`isize::MIN`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::zero();
    /// assert_eq!(counter.dec(), -1);
    /// ```
    #[inline]
    pub fn dec(&self) -> isize {
        self.sub(1)
    }

    /// Adds `delta` to the counter and returns the new value.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to add. It may be negative.
    ///
    /// # Returns
    ///
    /// The counter value after the addition.
    ///
    /// # Panics
    ///
    /// Panics if the addition would overflow or underflow the signed range.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(2);
    /// assert_eq!(counter.add(-5), -3);
    /// ```
    #[inline]
    pub fn add(&self, delta: isize) -> isize {
        self.try_add(delta).expect("atomic signed counter overflow")
    }

    /// Tries to add `delta` to the counter.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to add. It may be negative.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` if the addition succeeds, or `None` if it would
    /// overflow or underflow the signed range. On `None`, the counter is left
    /// unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(-2);
    /// assert_eq!(counter.try_add(5), Some(3));
    /// ```
    #[inline]
    pub fn try_add(&self, delta: isize) -> Option<isize> {
        self.try_update(|current| current.checked_add(delta))
    }

    /// Subtracts `delta` from the counter and returns the new value.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to subtract. It may be negative.
    ///
    /// # Returns
    ///
    /// The counter value after the subtraction.
    ///
    /// # Panics
    ///
    /// Panics if the subtraction would overflow or underflow the signed range.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(2);
    /// assert_eq!(counter.sub(5), -3);
    /// ```
    #[inline]
    pub fn sub(&self, delta: isize) -> isize {
        self.try_sub(delta).expect("atomic signed counter overflow")
    }

    /// Tries to subtract `delta` from the counter.
    ///
    /// # Parameters
    ///
    /// * `delta` - The amount to subtract. It may be negative.
    ///
    /// # Returns
    ///
    /// `Some(new_value)` if the subtraction succeeds, or `None` if it would
    /// overflow or underflow the signed range. On `None`, the counter is left
    /// unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::AtomicSignedCount;
    ///
    /// let counter = AtomicSignedCount::new(2);
    /// assert_eq!(counter.try_sub(5), Some(-3));
    /// ```
    #[inline]
    pub fn try_sub(&self, delta: isize) -> Option<isize> {
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
    fn try_update<F>(&self, update: F) -> Option<isize>
    where
        F: Fn(isize) -> Option<isize>,
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

impl Default for AtomicSignedCount {
    /// Creates a zero-valued signed atomic counter.
    ///
    /// # Returns
    ///
    /// A signed counter whose current value is zero.
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl From<isize> for AtomicSignedCount {
    /// Converts an initial counter value into an [`AtomicSignedCount`].
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A signed counter initialized to `value`.
    #[inline]
    fn from(value: isize) -> Self {
        Self::new(value)
    }
}

impl fmt::Debug for AtomicSignedCount {
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
        f.debug_struct("AtomicSignedCount")
            .field("value", &self.get())
            .finish()
    }
}

impl fmt::Display for AtomicSignedCount {
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
