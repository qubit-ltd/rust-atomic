/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Traits
//!
//! Defines common traits for atomic types, providing a unified interface
//! for atomic operations.
//!
//! # Author
//!
//! Haixing Hu

/// Common trait for all atomic types.
///
/// Provides basic atomic operations including load, store, swap,
/// compare-and-set, and functional updates.
///
/// # Author
///
/// Haixing Hu
pub trait Atomic {
    /// The value type stored in the atomic.
    type Value;

    /// Loads the current value.
    ///
    /// Uses `Acquire` ordering by default.
    ///
    /// # Returns
    ///
    /// The current value.
    fn load(&self) -> Self::Value;

    /// Stores a new value.
    ///
    /// Uses `Release` ordering by default.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to store.
    fn store(&self, value: Self::Value);

    /// Swaps the current value with a new value, returning the old
    /// value.
    ///
    /// Uses `AcqRel` ordering by default.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to swap in.
    ///
    /// # Returns
    ///
    /// The old value.
    fn swap(&self, value: Self::Value) -> Self::Value;

    /// Compares and sets the value atomically.
    ///
    /// If the current value equals `current`, sets it to `new` and
    /// returns `Ok(())`. Otherwise, returns `Err(actual)` where
    /// `actual` is the current value.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on
    /// failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(actual)` on failure where
    /// `actual` is the real current value.
    fn compare_set(&self, current: Self::Value, new: Self::Value) -> Result<(), Self::Value>;

    /// Weak version of compare-and-set.
    ///
    /// May spuriously fail even when the comparison succeeds. Should
    /// be used in a loop.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on
    /// failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(actual)` on failure.
    fn compare_set_weak(&self, current: Self::Value, new: Self::Value) -> Result<(), Self::Value>;

    /// Compares and exchanges the value atomically, returning the
    /// previous value.
    ///
    /// If the current value equals `current`, sets it to `new` and
    /// returns the old value. Otherwise, returns the actual current
    /// value.
    ///
    /// This is similar to `compare_set` but always returns the actual
    /// value instead of a Result, which can be more convenient in CAS
    /// loops.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on
    /// failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// The value before the operation. If it equals `current`, the
    /// operation succeeded.
    fn compare_exchange(&self, current: Self::Value, new: Self::Value) -> Self::Value;

    /// Weak version of compare-and-exchange.
    ///
    /// May spuriously fail even when the comparison succeeds. Should
    /// be used in a loop.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on
    /// failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The new value to set if current matches.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    fn compare_exchange_weak(&self, current: Self::Value, new: Self::Value) -> Self::Value;

    /// Updates the value using a function, returning the old value.
    ///
    /// Internally uses a CAS loop until the update succeeds.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes the current value and returns
    ///   the new value.
    ///
    /// # Returns
    ///
    /// The old value before the update.
    fn fetch_update<F>(&self, f: F) -> Self::Value
    where
        F: Fn(Self::Value) -> Self::Value;
}

/// Trait for atomic numeric types that support arithmetic operations.
///
/// Provides common arithmetic operations (add, subtract, multiply, divide)
/// for both integer and floating-point atomic types. This trait unifies
/// the arithmetic interface across all numeric atomic types.
///
/// # Note
///
/// Integer types also provide `fetch_inc()` and `fetch_dec()` methods
/// as convenient shortcuts for incrementing/decrementing by 1, but these
/// are not part of this trait as they are integer-specific operations.
///
/// # Author
///
/// Haixing Hu
pub trait AtomicNumber: Atomic {
    /// Adds a delta to the value, returning the old value.
    ///
    /// For integers, uses `Relaxed` ordering by default.
    /// For floating-point types, uses `AcqRel` ordering (CAS loop).
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to add.
    ///
    /// # Returns
    ///
    /// The old value before adding.
    fn fetch_add(&self, delta: Self::Value) -> Self::Value;

    /// Subtracts a delta from the value, returning the old value.
    ///
    /// For integers, uses `Relaxed` ordering by default.
    /// For floating-point types, uses `AcqRel` ordering (CAS loop).
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to subtract.
    ///
    /// # Returns
    ///
    /// The old value before subtracting.
    fn fetch_sub(&self, delta: Self::Value) -> Self::Value;

    /// Multiplies the value by a factor, returning the old value.
    ///
    /// Uses `AcqRel` ordering by default. Implemented via CAS loop.
    ///
    /// # Parameters
    ///
    /// * `factor` - The value to multiply by.
    ///
    /// # Returns
    ///
    /// The old value before multiplying.
    fn fetch_mul(&self, factor: Self::Value) -> Self::Value;

    /// Divides the value by a divisor, returning the old value.
    ///
    /// Uses `AcqRel` ordering by default. Implemented via CAS loop.
    ///
    /// # Parameters
    ///
    /// * `divisor` - The value to divide by.
    ///
    /// # Returns
    ///
    /// The old value before dividing.
    fn fetch_div(&self, divisor: Self::Value) -> Self::Value;
}
