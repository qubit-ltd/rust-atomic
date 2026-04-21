/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Atomic value wrappers, counters, and reference helpers.

#[macro_use]
mod macros;

mod arc_atomic;
mod arc_atomic_count;
mod arc_atomic_ref;
mod arc_atomic_signed_count;
#[allow(clippy::module_inception)]
mod atomic;
mod atomic_bool;
mod atomic_count;
mod atomic_f32;
mod atomic_f64;
mod atomic_i128;
mod atomic_i16;
mod atomic_i32;
mod atomic_i64;
mod atomic_i8;
mod atomic_integer_value;
mod atomic_isize;
mod atomic_number_ops;
mod atomic_ops;
mod atomic_ref;
mod atomic_signed_count;
mod atomic_u128;
mod atomic_u16;
mod atomic_u32;
mod atomic_u64;
mod atomic_u8;
mod atomic_usize;
mod atomic_value;
mod sealed;

pub use arc_atomic::ArcAtomic;
pub use arc_atomic_count::ArcAtomicCount;
pub use arc_atomic_ref::ArcAtomicRef;
pub use arc_atomic_signed_count::ArcAtomicSignedCount;
pub use atomic::Atomic;
pub use atomic_count::AtomicCount;
pub use atomic_ref::AtomicRef;
pub use atomic_signed_count::AtomicSignedCount;
