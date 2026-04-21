/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # qubit-atomic
//!
//! User-friendly atomic operations wrapper providing JDK-like atomic API.
//!
//! This crate provides an easy-to-use generic atomic wrapper with reasonable
//! default memory orderings, similar to Java's
//! `java.util.concurrent.atomic` package.
//!
//! ## Design Goals
//!
//! - **Ease of Use**: Hides memory ordering complexity with reasonable defaults
//! - **Completeness**: Provides high-level operations similar to JDK atomic
//! - **Safety**: Guarantees memory safety and thread safety
//! - **Performance**: Zero-cost abstraction with no additional overhead
//! - **Flexibility**: Exposes underlying types via `inner()` for advanced users
//!
//! ## Features
//!
//! - Primitive atomic values: `Atomic<bool>`, `Atomic<i32>`,
//!   `Atomic<u64>`, `Atomic<usize>`, `Atomic<f32>`, and other supported
//!   primitive types
//! - Counter atomic types: `AtomicCount`, `AtomicSignedCount`
//! - Reference atomic type: `AtomicRef<T>`
//! - Shared-owner wrappers: `ArcAtomic<T>`, `ArcAtomicRef<T>`,
//!   `ArcAtomicCount`, and `ArcAtomicSignedCount`
//!
//! ## Example
//!
//! ```rust
//! use qubit_atomic::Atomic;
//! use std::sync::Arc;
//! use std::thread;
//!
//! // Basic usage
//! let counter = Atomic::new(0);
//! counter.fetch_inc();
//! assert_eq!(counter.load(), 1);
//!
//! // Concurrent usage
//! let counter = Arc::new(Atomic::new(0));
//! let mut handles = vec![];
//!
//! for _ in 0..10 {
//!     let counter = counter.clone();
//!     let handle = thread::spawn(move || {
//!         for _ in 0..100 {
//!             counter.fetch_inc();
//!         }
//!     });
//!     handles.push(handle);
//! }
//!
//! for handle in handles {
//!     handle.join().unwrap();
//! }
//!
//! assert_eq!(counter.load(), 1000);
//! ```
//!
//! ## Author
//!
//! Haixing Hu

#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Atomic value types and reference/counting helpers.
pub mod atomic;

// Re-export the public atomic API.
pub use atomic::{
    ArcAtomic, ArcAtomicCount, ArcAtomicRef, ArcAtomicSignedCount, Atomic, AtomicCount, AtomicRef,
    AtomicSignedCount,
};
