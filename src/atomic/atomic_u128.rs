/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic 128-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 128-bit unsigned integer type with sensible
//! default memory orderings.
//!
//! # Author
//!
//! Haixing Hu

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicU128,
    portable_atomic::AtomicU128,
    u128,
    "128-bit unsigned integer"
);
