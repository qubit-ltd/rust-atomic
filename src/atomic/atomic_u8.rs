/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic 8-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 8-bit unsigned integer type with sensible
//! default memory orderings.
//!
//! # Author
//!
//! Haixing Hu

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicU8,
    std::sync::atomic::AtomicU8,
    u8,
    "8-bit unsigned integer"
);
