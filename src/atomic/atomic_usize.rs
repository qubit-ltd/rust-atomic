/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Pointer-Sized Unsigned Integer
//!
//! Provides an easy-to-use atomic pointer-sized unsigned integer type with
//! sensible default memory orderings.
//!
//! # Author
//!
//! Haixing Hu

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicUsize,
    std::sync::atomic::AtomicUsize,
    usize,
    "pointer-sized unsigned integer"
);
