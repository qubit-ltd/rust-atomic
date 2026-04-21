/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Value Marker
//!
//! Defines the hidden marker trait for values supported by
//! [`crate::Atomic<T>`].
//!
//! # Author
//!
//! Haixing Hu

use super::{
    atomic_bool,
    atomic_f32,
    atomic_f64,
    atomic_ops::AtomicOps,
    sealed,
};

/// Marker trait for values supported by [`crate::Atomic<T>`].
///
/// This trait is sealed and hidden from the public documentation. Users should
/// treat the supported primitive list in [`crate::Atomic`] as the stable API
/// surface.
#[doc(hidden)]
pub trait AtomicValue: sealed::Sealed + Copy {
    /// Internal primitive wrapper type.
    type Primitive: AtomicOps<Value = Self>;

    /// Raw backend atomic type returned by [`crate::Atomic::inner`].
    type Inner;

    /// Creates the internal primitive wrapper.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value stored in the primitive wrapper.
    ///
    /// # Returns
    ///
    /// A primitive wrapper initialized to `value`.
    fn new_primitive(value: Self) -> Self::Primitive;

    /// Returns the raw backend atomic type from the primitive wrapper.
    ///
    /// # Parameters
    ///
    /// * `primitive` - The primitive wrapper to expose.
    ///
    /// # Returns
    ///
    /// A shared reference to the backend atomic storage.
    fn inner(primitive: &Self::Primitive) -> &Self::Inner;
}

impl_atomic_value!(bool, atomic_bool::AtomicBool, std::sync::atomic::AtomicBool);
impl_atomic_value!(f32, atomic_f32::AtomicF32, std::sync::atomic::AtomicU32);
impl_atomic_value!(f64, atomic_f64::AtomicF64, std::sync::atomic::AtomicU64);
