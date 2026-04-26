/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Generic Atomic Wrapper
//!
//! Provides the public [`Atomic<T>`] wrapper for supported primitive values.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;

use super::atomic_integer_value::AtomicIntegerValue;
use super::atomic_number_ops::AtomicNumberOps;
use super::atomic_ops::AtomicOps;
use super::atomic_value::AtomicValue;

/// A high-level atomic wrapper for supported primitive value types.
///
/// This type is the main entry point for primitive atomic values. It hides
/// explicit memory-ordering parameters behind crate-defined defaults while
/// still exposing the raw backend through [`inner`](Self::inner) for advanced
/// use cases.
///
/// Supported value types are:
///
/// - `bool`
/// - `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`
/// - `usize`, `isize`
/// - `f32`, `f64`
///
/// The `i128` and `u128` specializations use `portable-atomic` internally
/// because the corresponding standard-library atomic types are not yet stable.
///
/// # Specialization API
///
/// Rustdoc documents [`Atomic<T>`] as one generic type rather than generating a
/// separate page for each concrete `T`. The following table summarizes the
/// methods available for each specialization:
///
/// | Specialization | Additional methods |
/// | --- | --- |
/// | `Atomic<bool>` | `fetch_set`, `fetch_clear`, `fetch_not`, `fetch_and`, `fetch_or`, `fetch_xor`, `set_if_false`, `set_if_true` |
/// | `Atomic<i8>`, `Atomic<i16>`, `Atomic<i32>`, `Atomic<i64>`, `Atomic<i128>`, `Atomic<isize>` | `fetch_add`, `fetch_sub`, `fetch_mul`, `fetch_div`, `fetch_inc`, `fetch_dec`, `fetch_and`, `fetch_or`, `fetch_xor`, `fetch_not`, `fetch_accumulate`, `fetch_max`, `fetch_min` |
/// | `Atomic<u8>`, `Atomic<u16>`, `Atomic<u32>`, `Atomic<u64>`, `Atomic<u128>`, `Atomic<usize>` | `fetch_add`, `fetch_sub`, `fetch_mul`, `fetch_div`, `fetch_inc`, `fetch_dec`, `fetch_and`, `fetch_or`, `fetch_xor`, `fetch_not`, `fetch_accumulate`, `fetch_max`, `fetch_min` |
/// | `Atomic<f32>`, `Atomic<f64>` | `fetch_add`, `fetch_sub`, `fetch_mul`, `fetch_div` |
///
/// All supported specializations also provide [`new`](Self::new),
/// [`load`](Self::load), [`store`](Self::store), [`swap`](Self::swap),
/// [`compare_set`](Self::compare_set),
/// [`compare_set_weak`](Self::compare_set_weak),
/// [`compare_and_exchange`](Self::compare_and_exchange),
/// [`compare_and_exchange_weak`](Self::compare_and_exchange_weak),
/// [`fetch_update`](Self::fetch_update), [`try_update`](Self::try_update),
/// and [`inner`](Self::inner).
///
/// Integer arithmetic operations intentionally follow Rust atomic integer
/// semantics and wrap on overflow and underflow. Use [`crate::AtomicCount`] or
/// [`crate::AtomicSignedCount`] when overflow or underflow must be rejected
/// instead of wrapping.
///
/// Floating-point compare-and-set/exchange operations compare raw
/// [`to_bits`](f32::to_bits) representations, not [`PartialEq`]. This means
/// distinct bit patterns that compare equal, such as `0.0` and `-0.0`, do not
/// match for CAS, and NaN payloads must match exactly. Prefer
/// [`compare_set`](Self::compare_set) or [`compare_set_weak`](Self::compare_set_weak)
/// when the caller needs an explicit success indicator for `f32` or `f64`.
///
/// # Example
///
/// ```rust
/// use qubit_atomic::Atomic;
///
/// let counter = Atomic::new(0);
/// counter.fetch_inc();
/// assert_eq!(counter.load(), 1);
///
/// let flag = Atomic::new(false);
/// assert_eq!(flag.fetch_set(), false);
/// assert!(flag.load());
/// ```
///
/// When the value type is ambiguous (for example integer literals), specify
/// `T` explicitly with a [turbofish] on the constructor, or by annotating the
/// binding:
///
/// ```rust
/// use qubit_atomic::Atomic;
///
/// let wide: Atomic<u64> = Atomic::new(0);
/// assert_eq!(wide.load(), 0u64);
///
/// let narrow = Atomic::<i16>::new(0);
/// assert_eq!(narrow.load(), 0i16);
/// ```
///
/// [turbofish]: https://doc.rust-lang.org/book/appendix-02-operators.html#the-turbofish
///
/// # Author
///
/// Haixing Hu
#[doc(alias = "AtomicBool")]
#[doc(alias = "AtomicI8")]
#[doc(alias = "AtomicU8")]
#[doc(alias = "AtomicI16")]
#[doc(alias = "AtomicU16")]
#[doc(alias = "AtomicI32")]
#[doc(alias = "AtomicU32")]
#[doc(alias = "AtomicI64")]
#[doc(alias = "AtomicU64")]
#[doc(alias = "AtomicI128")]
#[doc(alias = "AtomicU128")]
#[doc(alias = "AtomicIsize")]
#[doc(alias = "AtomicUsize")]
#[doc(alias = "AtomicF32")]
#[doc(alias = "AtomicF64")]
#[repr(transparent)]
pub struct Atomic<T>
where
    T: AtomicValue,
{
    /// Primitive backend that performs the concrete atomic operations for `T`.
    primitive: T::Primitive,
}

impl<T> Atomic<T>
where
    T: AtomicValue,
{
    /// Creates a new atomic value.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value.
    ///
    /// # Returns
    ///
    /// An atomic wrapper initialized to `value`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(42);
    /// assert_eq!(atomic.load(), 42);
    /// ```
    ///
    /// To pick a concrete `T` when inference would be ambiguous (for example
    /// `0` as `u64` vs `i32`), write `Atomic::<T>::new(...)` (turbofish) or add
    /// a type annotation on the binding (for example `let x: Atomic<u64> = ...`):
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let a = Atomic::<u64>::new(0);
    /// assert_eq!(a.load(), 0u64);
    ///
    /// let b: Atomic<isize> = Atomic::new(0);
    /// assert_eq!(b.load(), 0isize);
    /// ```
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            primitive: T::new_primitive(value),
        }
    }

    /// Loads the current value.
    ///
    /// Uses `Acquire` ordering by default.
    ///
    /// # Returns
    ///
    /// The current value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(7);
    /// assert_eq!(atomic.load(), 7);
    /// ```
    #[inline]
    pub fn load(&self) -> T {
        AtomicOps::load(&self.primitive)
    }

    /// Stores a new value.
    ///
    /// Uses `Release` ordering by default.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to store.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// atomic.store(2);
    /// assert_eq!(atomic.load(), 2);
    /// ```
    #[inline]
    pub fn store(&self, value: T) {
        AtomicOps::store(&self.primitive, value);
    }

    /// Swaps the current value with `value`.
    ///
    /// Unlike [`store`](Self::store), this returns the value that was in the
    /// atomic immediately before the swap, in the same atomic step. Use
    /// [`store`](Self::store) when you do not need the previous value.
    ///
    /// Uses `AcqRel` ordering by default.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to store.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let a = Atomic::new(100);
    /// a.store(200);
    /// // store has no return value; you only see the new value via load().
    /// assert_eq!(a.load(), 200);
    ///
    /// let b = Atomic::new(100);
    /// // swap returns the old value and installs the new one atomically.
    /// assert_eq!(b.swap(200), 100);
    /// assert_eq!(b.load(), 200);
    /// ```
    #[inline]
    pub fn swap(&self, value: T) -> T {
        AtomicOps::swap(&self.primitive, value)
    }

    /// Compares the current value with `current` and stores `new` on match.
    ///
    /// Uses `AcqRel` ordering on success and `Acquire` ordering on failure.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails. In that case, `new` is not stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// assert!(atomic.compare_set(1, 2).is_ok());
    /// assert_eq!(atomic.load(), 2);
    /// assert_eq!(atomic.compare_set(1, 3), Err(2));
    /// ```
    #[inline]
    pub fn compare_set(&self, current: T, new: T) -> Result<(), T> {
        AtomicOps::compare_set(&self.primitive, current, new)
    }

    /// Weak version of [`compare_set`](Self::compare_set).
    ///
    /// This operation may fail spuriously and is intended for retry loops.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// `Ok(())` when the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(actual)` with the observed value when the comparison
    /// fails, including possible spurious failures. In that case, `new` is not
    /// stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// loop {
    ///     match atomic.compare_set_weak(1, 2) {
    ///         Ok(()) => break,
    ///         Err(actual) => assert_eq!(actual, 1),
    ///     }
    /// }
    /// assert_eq!(atomic.load(), 2);
    /// ```
    #[inline]
    pub fn compare_set_weak(&self, current: T, new: T) -> Result<(), T> {
        AtomicOps::compare_set_weak(&self.primitive, current, new)
    }

    /// Compares and exchanges the value, returning the value seen before the
    /// operation.
    ///
    /// If the return value equals `current`, the exchange succeeded.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// The value observed before the operation completed. If the returned
    /// value equals `current`, the exchange succeeded; otherwise it is the
    /// actual value that prevented the exchange.
    ///
    /// For `Atomic<f32>` and `Atomic<f64>`, CAS compares raw IEEE-754 bit
    /// patterns rather than [`PartialEq`]. A returned floating-point value
    /// comparing equal to `current` is therefore not always enough to prove
    /// success; use [`compare_set`](Self::compare_set) for an explicit
    /// `Ok`/`Err`, or compare [`to_bits`](f32::to_bits) values yourself.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(5);
    /// assert_eq!(atomic.compare_and_exchange(5, 10), 5);
    /// assert_eq!(atomic.load(), 10);
    /// assert_eq!(atomic.compare_and_exchange(5, 0), 10);
    /// ```
    #[inline]
    pub fn compare_and_exchange(&self, current: T, new: T) -> T {
        AtomicOps::compare_exchange(&self.primitive, current, new)
    }

    /// Weak version of [`compare_and_exchange`](Self::compare_and_exchange).
    ///
    /// This operation may fail spuriously and is intended for retry loops.
    ///
    /// # Parameters
    ///
    /// * `current` - The expected current value.
    /// * `new` - The replacement value to store when the comparison matches.
    ///
    /// # Returns
    ///
    /// The value observed before the operation completed. Because this
    /// operation may fail spuriously, a returned value equal to `current` does
    /// not by itself prove that `new` was stored; use
    /// [`compare_set_weak`](Self::compare_set_weak) when the caller needs an
    /// explicit success indicator.
    ///
    /// For `Atomic<f32>` and `Atomic<f64>`, the same caveat applies to raw-bit
    /// equality: `0.0` and `-0.0` compare equal by [`PartialEq`] but are
    /// different CAS values. Use [`compare_set_weak`](Self::compare_set_weak)
    /// or compare [`to_bits`](f32::to_bits) values when distinguishing success
    /// from failure matters.
    ///
    /// # Example
    ///
    /// Weak CAS may fail spuriously; retry until [`load`](Self::load) shows the
    /// expected outcome (or use [`compare_set_weak`](Self::compare_set_weak)
    /// which reports success explicitly).
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(5);
    /// loop {
    ///     let _ = atomic.compare_and_exchange_weak(5, 10);
    ///     if atomic.load() == 10 {
    ///         break;
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn compare_and_exchange_weak(&self, current: T, new: T) -> T {
        AtomicOps::compare_exchange_weak(&self.primitive, current, new)
    }

    /// Updates the value with a function and returns the previous value.
    ///
    /// The update uses a CAS loop until it succeeds. The closure may be called
    /// more than once under contention.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that maps the current value to the next value.
    ///
    /// # Returns
    ///
    /// The value before the successful update.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.fetch_update(|x| x * 2), 3);
    /// assert_eq!(atomic.load(), 6);
    /// ```
    #[inline]
    pub fn fetch_update<F>(&self, f: F) -> T
    where
        F: Fn(T) -> T,
    {
        AtomicOps::fetch_update(&self.primitive, f)
    }

    /// Conditionally updates the value with a function.
    ///
    /// The update uses a CAS loop until it succeeds or the closure rejects the
    /// observed current value by returning `None`. The closure may be called
    /// more than once under contention.
    ///
    /// # Parameters
    ///
    /// * `f` - A function that maps the current value to `Some(next)` to update
    ///   the atomic, or `None` to leave it unchanged.
    ///
    /// # Returns
    ///
    /// `Some(old_value)` with the value before the successful update, or `None`
    /// when `f` rejects the observed current value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.try_update(|x| (x % 2 == 1).then_some(x + 1)), Some(3));
    /// assert_eq!(atomic.load(), 4);
    /// assert_eq!(atomic.try_update(|x| (x % 2 == 1).then_some(x + 1)), None);
    /// assert_eq!(atomic.load(), 4);
    /// ```
    #[inline]
    pub fn try_update<F>(&self, f: F) -> Option<T>
    where
        F: Fn(T) -> Option<T>,
    {
        AtomicOps::try_update(&self.primitive, f)
    }

    /// Returns the raw backend atomic value.
    ///
    /// Use this method only when the default orderings are not appropriate
    /// and the caller needs direct access to the backend atomic storage.
    ///
    /// # Returns
    ///
    /// A shared reference to the raw backend atomic value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    /// use std::sync::atomic::Ordering;
    ///
    /// let atomic = Atomic::<i32>::new(0);
    /// assert_eq!(atomic.inner().load(Ordering::Relaxed), 0);
    /// ```
    #[inline]
    pub fn inner(&self) -> &T::Inner {
        T::inner(&self.primitive)
    }
}

impl<T> Atomic<T>
where
    T: AtomicValue,
    T::Primitive: AtomicNumberOps<Value = T>,
{
    /// Adds `delta` to the value and returns the previous value.
    ///
    /// Integer atomics use relaxed ordering for this operation. Floating-point
    /// atomics use a CAS loop. Integer addition wraps on overflow and
    /// underflow.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to add.
    ///
    /// # Returns
    ///
    /// The value before the addition.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_add(3), 10);
    /// assert_eq!(atomic.load(), 13);
    /// ```
    #[inline]
    pub fn fetch_add(&self, delta: T) -> T {
        AtomicNumberOps::fetch_add(&self.primitive, delta)
    }

    /// Subtracts `delta` from the value and returns the previous value.
    ///
    /// Integer atomics use relaxed ordering for this operation. Floating-point
    /// atomics use a CAS loop. Integer subtraction wraps on overflow and
    /// underflow.
    ///
    /// # Parameters
    ///
    /// * `delta` - The value to subtract.
    ///
    /// # Returns
    ///
    /// The value before the subtraction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_sub(3), 10);
    /// assert_eq!(atomic.load(), 7);
    /// ```
    #[inline]
    pub fn fetch_sub(&self, delta: T) -> T {
        AtomicNumberOps::fetch_sub(&self.primitive, delta)
    }

    /// Multiplies the value by `factor` and returns the previous value.
    ///
    /// This operation uses a CAS loop. Integer multiplication wraps on
    /// overflow and underflow.
    ///
    /// # Parameters
    ///
    /// * `factor` - The value to multiply by.
    ///
    /// # Returns
    ///
    /// The value before the multiplication.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.fetch_mul(4), 3);
    /// assert_eq!(atomic.load(), 12);
    /// ```
    #[inline]
    pub fn fetch_mul(&self, factor: T) -> T {
        AtomicNumberOps::fetch_mul(&self.primitive, factor)
    }

    /// Divides the value by `divisor` and returns the previous value.
    ///
    /// This operation uses a CAS loop. Integer division uses wrapping
    /// semantics; for signed integers, `MIN / -1` wraps to `MIN`.
    ///
    /// # Parameters
    ///
    /// * `divisor` - The value to divide by.
    ///
    /// # Returns
    ///
    /// The value before the division.
    ///
    /// # Panics
    ///
    /// For integer specializations, panics if `divisor` is zero. Floating-point
    /// specializations follow IEEE-754 division semantics and do not panic
    /// solely because `divisor` is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(20);
    /// assert_eq!(atomic.fetch_div(4), 20);
    /// assert_eq!(atomic.load(), 5);
    /// ```
    #[inline]
    pub fn fetch_div(&self, divisor: T) -> T {
        AtomicNumberOps::fetch_div(&self.primitive, divisor)
    }
}

impl<T> Atomic<T>
where
    T: AtomicIntegerValue,
{
    /// Increments the value by one and returns the previous value.
    ///
    /// # Returns
    ///
    /// The value before the increment.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(0);
    /// assert_eq!(atomic.fetch_inc(), 0);
    /// assert_eq!(atomic.load(), 1);
    /// ```
    #[inline]
    pub fn fetch_inc(&self) -> T {
        T::fetch_inc(&self.primitive)
    }

    /// Decrements the value by one and returns the previous value.
    ///
    /// # Returns
    ///
    /// The value before the decrement.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(1);
    /// assert_eq!(atomic.fetch_dec(), 1);
    /// assert_eq!(atomic.load(), 0);
    /// ```
    #[inline]
    pub fn fetch_dec(&self) -> T {
        T::fetch_dec(&self.primitive)
    }

    /// Applies bitwise AND and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The mask to apply.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<u8>::new(0b1111);
    /// assert_eq!(atomic.fetch_and(0b1010), 0b1111);
    /// assert_eq!(atomic.load(), 0b1010);
    /// ```
    #[inline]
    pub fn fetch_and(&self, value: T) -> T {
        T::fetch_and(&self.primitive, value)
    }

    /// Applies bitwise OR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The mask to apply.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<u8>::new(0b1000);
    /// assert_eq!(atomic.fetch_or(0b0011), 0b1000);
    /// assert_eq!(atomic.load(), 0b1011);
    /// ```
    #[inline]
    pub fn fetch_or(&self, value: T) -> T {
        T::fetch_or(&self.primitive, value)
    }

    /// Applies bitwise XOR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The mask to apply.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<u8>::new(0b1111);
    /// assert_eq!(atomic.fetch_xor(0b1010), 0b1111);
    /// assert_eq!(atomic.load(), 0b0101);
    /// ```
    #[inline]
    pub fn fetch_xor(&self, value: T) -> T {
        T::fetch_xor(&self.primitive, value)
    }

    /// Flips all bits and returns the previous value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<i32>::new(0);
    /// assert_eq!(atomic.fetch_not(), 0);
    /// assert_eq!(atomic.load(), !0);
    /// ```
    #[inline]
    pub fn fetch_not(&self) -> T {
        T::fetch_not(&self.primitive)
    }

    /// Updates the value by accumulating it with `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - The right-hand input to the accumulator.
    /// * `f` - A function that combines the current value and `value`.
    ///
    /// # Returns
    ///
    /// The value before the successful update.
    ///
    /// The closure may be called more than once when concurrent updates cause
    /// CAS retries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_accumulate(5, |a, b| a + b), 10);
    /// assert_eq!(atomic.load(), 15);
    /// ```
    #[inline]
    pub fn fetch_accumulate<F>(&self, value: T, f: F) -> T
    where
        F: Fn(T, T) -> T,
    {
        T::fetch_accumulate(&self.primitive, value, f)
    }

    /// Replaces the value with the maximum of the current value and `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to compare with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(3);
    /// assert_eq!(atomic.fetch_max(10), 3);
    /// assert_eq!(atomic.load(), 10);
    /// ```
    #[inline]
    pub fn fetch_max(&self, value: T) -> T {
        T::fetch_max(&self.primitive, value)
    }

    /// Replaces the value with the minimum of the current value and `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to compare with the current value.
    ///
    /// # Returns
    ///
    /// The value before the operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(10);
    /// assert_eq!(atomic.fetch_min(3), 10);
    /// assert_eq!(atomic.load(), 3);
    /// ```
    #[inline]
    pub fn fetch_min(&self, value: T) -> T {
        T::fetch_min(&self.primitive, value)
    }
}

impl Atomic<bool> {
    /// Stores `true` and returns the previous value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert_eq!(flag.fetch_set(), false);
    /// assert!(flag.load());
    /// ```
    #[inline]
    pub fn fetch_set(&self) -> bool {
        self.primitive.fetch_set()
    }

    /// Stores `false` and returns the previous value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert_eq!(flag.fetch_clear(), true);
    /// assert!(!flag.load());
    /// ```
    #[inline]
    pub fn fetch_clear(&self) -> bool {
        self.primitive.fetch_clear()
    }

    /// Negates the value and returns the previous value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert_eq!(flag.fetch_not(), false);
    /// assert!(flag.load());
    /// ```
    #[inline]
    pub fn fetch_not(&self) -> bool {
        self.primitive.fetch_not()
    }

    /// Applies logical AND and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to combine with the current value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert_eq!(flag.fetch_and(false), true);
    /// assert!(!flag.load());
    /// ```
    #[inline]
    pub fn fetch_and(&self, value: bool) -> bool {
        self.primitive.fetch_and(value)
    }

    /// Applies logical OR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to combine with the current value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert_eq!(flag.fetch_or(true), false);
    /// assert!(flag.load());
    /// ```
    #[inline]
    pub fn fetch_or(&self, value: bool) -> bool {
        self.primitive.fetch_or(value)
    }

    /// Applies logical XOR and returns the previous value.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to combine with the current value.
    ///
    /// # Returns
    ///
    /// The previous value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert_eq!(flag.fetch_xor(true), true);
    /// assert!(!flag.load());
    /// ```
    #[inline]
    pub fn fetch_xor(&self, value: bool) -> bool {
        self.primitive.fetch_xor(value)
    }

    /// Stores `new` only when the current value is `false`.
    ///
    /// # Parameters
    ///
    /// * `new` - The replacement value.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(true)` if the observed current value was already `true`.
    /// In that case, `new` is not stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(false);
    /// assert!(flag.set_if_false(true).is_ok());
    /// assert!(flag.load());
    /// assert!(flag.set_if_false(false).is_err());
    /// ```
    #[inline]
    pub fn set_if_false(&self, new: bool) -> Result<(), bool> {
        self.primitive.set_if_false(new)
    }

    /// Stores `new` only when the current value is `true`.
    ///
    /// # Parameters
    ///
    /// * `new` - The replacement value.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was replaced.
    ///
    /// # Errors
    ///
    /// Returns `Err(false)` if the observed current value was already `false`.
    /// In that case, `new` is not stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let flag = Atomic::new(true);
    /// assert!(flag.set_if_true(false).is_ok());
    /// assert!(!flag.load());
    /// assert!(flag.set_if_true(true).is_err());
    /// ```
    #[inline]
    pub fn set_if_true(&self, new: bool) -> Result<(), bool> {
        self.primitive.set_if_true(new)
    }
}

impl<T> Default for Atomic<T>
where
    T: AtomicValue + Default,
{
    /// Creates an atomic with [`Default::default`] as the initial value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::<i32>::default();
    /// assert_eq!(atomic.load(), 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for Atomic<T>
where
    T: AtomicValue,
{
    /// Converts `value` into an [`Atomic`] via [`Atomic::new`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::from(42i32);
    /// assert_eq!(atomic.load(), 42);
    /// ```
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> fmt::Debug for Atomic<T>
where
    T: AtomicValue + fmt::Debug,
{
    /// Formats the loaded value as `Atomic { value: ... }`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(7);
    /// assert!(format!("{:?}", atomic).contains("7"));
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Atomic")
            .field("value", &self.load())
            .finish()
    }
}

impl<T> fmt::Display for Atomic<T>
where
    T: AtomicValue + fmt::Display,
{
    /// Formats the loaded value using its [`Display`](fmt::Display) implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use qubit_atomic::Atomic;
    ///
    /// let atomic = Atomic::new(42);
    /// assert_eq!(format!("{}", atomic), "42");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
