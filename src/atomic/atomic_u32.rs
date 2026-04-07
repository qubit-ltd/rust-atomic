/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic 32-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 32-bit unsigned integer type with sensible
//! default memory orderings.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicU32,
    std::sync::atomic::AtomicU32,
    u32,
    "32-bit unsigned integer"
);
