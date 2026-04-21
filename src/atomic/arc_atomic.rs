/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Atomic Wrapper
//!
//! Provides [`ArcAtomic<T>`], a convenience wrapper around `Arc<Atomic<T>>`.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use super::atomic::Atomic;
use super::atomic_value::AtomicValue;

/// Shared-owner wrapper around [`Atomic<T>`].
///
/// This type is a convenience newtype for `Arc<Atomic<T>>`. Cloning an
/// [`ArcAtomic`] clones the shared owner handle, so all clones operate on the
/// same underlying atomic container.
///
/// Use [`Atomic<T>`] when ownership stays local, and use [`ArcAtomic<T>`] when
/// the same atomic value must be shared across threads or components.
///
/// # Example
///
/// ```rust
/// use qubit_atomic::ArcAtomic;
///
/// let counter = ArcAtomic::new(0usize);
/// let shared = counter.clone();
///
/// shared.fetch_inc();
/// assert_eq!(counter.load(), 1);
/// assert_eq!(counter.strong_count(), 2);
/// ```
pub struct ArcAtomic<T>
where
    T: AtomicValue,
{
    /// Shared owner of the underlying atomic container.
    inner: Arc<Atomic<T>>,
}

impl<T> ArcAtomic<T>
where
    T: AtomicValue,
{
    /// Creates a new shared atomic value.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value stored in the atomic container.
    ///
    /// # Returns
    ///
    /// A shared atomic wrapper initialized to `value`.
    #[inline]
    pub fn new(value: T) -> Self {
        Self::from_atomic(Atomic::new(value))
    }

    /// Wraps an existing [`Atomic<T>`] in an [`Arc`].
    ///
    /// # Parameters
    ///
    /// * `atomic` - The atomic container to share.
    ///
    /// # Returns
    ///
    /// A shared atomic wrapper owning `atomic`.
    #[inline]
    pub fn from_atomic(atomic: Atomic<T>) -> Self {
        Self {
            inner: Arc::new(atomic),
        }
    }

    /// Wraps an existing shared atomic container.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared atomic container to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    pub fn from_arc(inner: Arc<Atomic<T>>) -> Self {
        Self { inner }
    }

    /// Returns the underlying [`Arc`] without cloning it.
    ///
    /// # Returns
    ///
    /// A shared reference to the underlying `Arc<Atomic<T>>`.
    #[inline]
    pub fn as_arc(&self) -> &Arc<Atomic<T>> {
        &self.inner
    }

    /// Consumes this wrapper and returns the underlying [`Arc`].
    ///
    /// # Returns
    ///
    /// The underlying `Arc<Atomic<T>>`.
    #[inline]
    pub fn into_arc(self) -> Arc<Atomic<T>> {
        self.inner
    }

    /// Returns the number of strong [`Arc`] owners.
    ///
    /// # Returns
    ///
    /// The current strong reference count of the shared atomic container.
    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}

impl<T> Clone for ArcAtomic<T>
where
    T: AtomicValue,
{
    /// Clones the shared owner handle.
    ///
    /// # Returns
    ///
    /// A new wrapper pointing to the same underlying atomic container.
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Deref for ArcAtomic<T>
where
    T: AtomicValue,
{
    type Target = Atomic<T>;

    /// Dereferences to the underlying [`Atomic<T>`].
    ///
    /// # Returns
    ///
    /// A shared reference to the atomic container.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<T> From<T> for ArcAtomic<T>
where
    T: AtomicValue,
{
    /// Converts an initial value into a shared atomic wrapper.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial value stored in the atomic container.
    ///
    /// # Returns
    ///
    /// A shared atomic wrapper initialized to `value`.
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> From<Atomic<T>> for ArcAtomic<T>
where
    T: AtomicValue,
{
    /// Converts an atomic container into a shared atomic wrapper.
    ///
    /// # Parameters
    ///
    /// * `atomic` - The atomic container to share.
    ///
    /// # Returns
    ///
    /// A shared atomic wrapper owning `atomic`.
    #[inline]
    fn from(atomic: Atomic<T>) -> Self {
        Self::from_atomic(atomic)
    }
}

impl<T> From<Arc<Atomic<T>>> for ArcAtomic<T>
where
    T: AtomicValue,
{
    /// Converts an existing shared atomic container into its wrapper.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared atomic container to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    fn from(inner: Arc<Atomic<T>>) -> Self {
        Self::from_arc(inner)
    }
}

impl<T> fmt::Debug for ArcAtomic<T>
where
    T: AtomicValue + fmt::Debug,
{
    /// Formats the current value and sharing state for debugging.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter receiving the debug representation.
    ///
    /// # Returns
    ///
    /// A formatting result from the formatter.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArcAtomic")
            .field("value", &self.load())
            .field("strong_count", &self.strong_count())
            .finish()
    }
}

impl<T> fmt::Display for ArcAtomic<T>
where
    T: AtomicValue + fmt::Display,
{
    /// Formats the currently loaded value with display formatting.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter receiving the displayed value.
    ///
    /// # Returns
    ///
    /// A formatting result from the formatter.
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.load())
    }
}
