/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic 16-bit Unsigned Integer
//!
//! Provides an easy-to-use atomic 16-bit unsigned integer type with sensible
//! default memory orderings.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicU16,
    std::sync::atomic::AtomicU16,
    u16,
    "16-bit unsigned integer"
);
