/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Integer Value Marker
//!
//! Defines the hidden marker trait for integer values supported by
//! integer-only [`crate::Atomic<T>`] operations.
//!
//! # Author
//!
//! Haixing Hu

use super::{
    atomic_i8, atomic_i16, atomic_i32, atomic_i64, atomic_i128, atomic_isize, atomic_u8,
    atomic_u16, atomic_u32, atomic_u64, atomic_u128, atomic_usize, atomic_value::AtomicValue,
    sealed,
};

/// Marker trait for integer values supported by integer-only operations.
///
/// This trait is sealed and hidden from the public documentation. It provides
/// the integer-only operations used by [`crate::Atomic<T>`] when `T` is one of
/// the supported signed or unsigned integer types.
#[doc(hidden)]
pub trait AtomicIntegerValue: AtomicValue {
    /// Increments the atomic integer and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    ///
    /// # Returns
    ///
    /// The value before the increment.
    fn fetch_inc(primitive: &Self::Primitive) -> Self;

    /// Decrements the atomic integer and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    ///
    /// # Returns
    ///
    /// The value before the decrement.
    fn fetch_dec(primitive: &Self::Primitive) -> Self;

    /// Applies bitwise AND and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    /// * `value` - The mask to AND with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    fn fetch_and(primitive: &Self::Primitive, value: Self) -> Self;

    /// Applies bitwise OR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    /// * `value` - The mask to OR with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    fn fetch_or(primitive: &Self::Primitive, value: Self) -> Self;

    /// Applies bitwise XOR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    /// * `value` - The mask to XOR with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    fn fetch_xor(primitive: &Self::Primitive, value: Self) -> Self;

    /// Flips all bits and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    fn fetch_not(primitive: &Self::Primitive) -> Self;

    /// Accumulates a value and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    /// * `value` - The right-hand input passed to the accumulator.
    /// * `f` - A function that combines the current value and `value`.
    ///
    /// # Returns
    ///
    /// The value before the successful update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    fn fetch_accumulate<F>(primitive: &Self::Primitive, value: Self, f: F) -> Self
    where
        F: Fn(Self, Self) -> Self;

    /// Replaces with the maximum value and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    /// * `value` - The value to compare with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    fn fetch_max(primitive: &Self::Primitive, value: Self) -> Self;

    /// Replaces with the minimum value and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to update.
    /// * `value` - The value to compare with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    fn fetch_min(primitive: &Self::Primitive, value: Self) -> Self;
}

impl_atomic_integer_value!(u8, atomic_u8::AtomicU8, std::sync::atomic::AtomicU8);
impl_atomic_integer_value!(i8, atomic_i8::AtomicI8, std::sync::atomic::AtomicI8);
impl_atomic_integer_value!(u16, atomic_u16::AtomicU16, std::sync::atomic::AtomicU16);
impl_atomic_integer_value!(i16, atomic_i16::AtomicI16, std::sync::atomic::AtomicI16);
impl_atomic_integer_value!(u32, atomic_u32::AtomicU32, std::sync::atomic::AtomicU32);
impl_atomic_integer_value!(i32, atomic_i32::AtomicI32, std::sync::atomic::AtomicI32);
impl_atomic_integer_value!(u64, atomic_u64::AtomicU64, std::sync::atomic::AtomicU64);
impl_atomic_integer_value!(i64, atomic_i64::AtomicI64, std::sync::atomic::AtomicI64);
impl_atomic_integer_value!(u128, atomic_u128::AtomicU128, portable_atomic::AtomicU128);
impl_atomic_integer_value!(i128, atomic_i128::AtomicI128, portable_atomic::AtomicI128);
impl_atomic_integer_value!(
    usize,
    atomic_usize::AtomicUsize,
    std::sync::atomic::AtomicUsize
);
impl_atomic_integer_value!(
    isize,
    atomic_isize::AtomicIsize,
    std::sync::atomic::AtomicIsize
);
