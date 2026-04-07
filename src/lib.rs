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
//! This crate provides easy-to-use atomic types with reasonable default memory
//! orderings, similar to Java's `java.util.concurrent.atomic` package.
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
//! - Boolean atomic type: `AtomicBool`
//! - Integer atomic types: `AtomicI8`, `AtomicU8`, `AtomicI16`, `AtomicU16`,
//!   `AtomicI32`, `AtomicU32`, `AtomicI64`, `AtomicU64`, `AtomicIsize`,
//!   `AtomicUsize`
//! - Floating-point atomic types: `AtomicF32`, `AtomicF64`
//! - Reference atomic type: `AtomicRef<T>`
//!
//! ## Example
//!
//! ```rust
//! use qubit_atomic::{AtomicI32, Atomic, AtomicNumber};
//! use std::sync::Arc;
//! use std::thread;
//!
//! // Basic usage
//! let counter = AtomicI32::new(0);
//! counter.fetch_inc();
//! assert_eq!(counter.load(), 1);
//!
//! // Concurrent usage
//! let counter = Arc::new(AtomicI32::new(0));
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

pub mod atomic;

// Re-export all atomic types and traits
pub use atomic::{
    Atomic,
    AtomicBool,
    AtomicF32,
    AtomicF64,
    AtomicI16,
    AtomicI32,
    AtomicI64,
    AtomicI8,
    AtomicIsize,
    AtomicNumber,
    AtomicRef,
    AtomicU16,
    AtomicU32,
    AtomicU64,
    AtomicU8,
    AtomicUsize,
};
