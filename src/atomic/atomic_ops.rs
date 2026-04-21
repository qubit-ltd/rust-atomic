/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Internal Atomic Operations Trait
//!
//! Defines the internal trait used by [`crate::Atomic<T>`] to delegate common
//! operations to concrete backend implementations.
//!
//! # Author
//!
//! Haixing Hu

/// Internal common trait for all backend atomic types.
///
/// Provides basic atomic operations including load, store, swap,
/// compare-and-set, and functional updates.
///
/// # Author
///
/// Haixing Hu
pub trait AtomicOps {
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
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails. In that case, `new` is not stored.
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
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails, including possible spurious failures. In that case, `new` is not
    /// stored.
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
    /// The value observed before the operation completed. Because this
    /// operation may fail spuriously, a returned value equal to `current` does
    /// not by itself prove that `new` was stored; use
    /// [`compare_set_weak`](Self::compare_set_weak) when the caller needs an
    /// explicit success indicator.
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
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    fn fetch_update<F>(&self, f: F) -> Self::Value
    where
        F: Fn(Self::Value) -> Self::Value;
}
