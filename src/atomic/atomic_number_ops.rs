/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Internal Atomic Number Operations Trait
//!
//! Defines the internal trait used by [`crate::Atomic<T>`] to delegate numeric
//! operations to concrete backend implementations.
//!
//! # Author
//!
//! Haixing Hu

use super::atomic_ops::AtomicOps;

/// Internal trait for backend atomic numeric types that support arithmetic operations.
///
/// Provides common arithmetic operations (add, subtract, multiply, divide)
/// for both integer and floating-point atomic types. This trait unifies
/// the arithmetic interface across all numeric atomic types.
///
/// Integer implementations follow Rust atomic integer semantics and wrap on
/// overflow and underflow.
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
pub trait AtomicNumberOps: AtomicOps {
    /// Adds a delta to the value, returning the old value.
    ///
    /// For integers, uses `Relaxed` ordering by default and wraps on overflow
    /// or underflow.
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
    /// For integers, uses `Relaxed` ordering by default and wraps on overflow
    /// or underflow.
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
    /// Uses `AcqRel` ordering by default. Implemented via CAS loop. Integer
    /// multiplication wraps on overflow and underflow.
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
    /// Uses `AcqRel` ordering by default. Implemented via CAS loop. Integer
    /// division uses wrapping semantics; signed `MIN / -1` wraps to `MIN`.
    ///
    /// # Parameters
    ///
    /// * `divisor` - The value to divide by.
    ///
    /// # Returns
    ///
    /// The old value before dividing.
    ///
    /// # Panics
    ///
    /// Integer implementations panic if `divisor` is zero. Floating-point
    /// implementations follow IEEE-754 division semantics.
    fn fetch_div(&self, divisor: Self::Value) -> Self::Value;
}
