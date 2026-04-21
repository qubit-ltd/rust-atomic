/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Atomic Pointer-Sized Signed Integer
//!
//! Provides an easy-to-use atomic pointer-sized signed integer type with
//! sensible default memory orderings.
//!
//! # Author
//!
//! Haixing Hu

use std::sync::atomic::Ordering;

impl_atomic_number!(
    AtomicIsize,
    std::sync::atomic::AtomicIsize,
    isize,
    "pointer-sized signed integer"
);
