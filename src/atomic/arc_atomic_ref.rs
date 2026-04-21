/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Atomic Reference Wrapper
//!
//! Provides [`ArcAtomicRef<T>`], a convenience wrapper around
//! `Arc<AtomicRef<T>>`.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use super::atomic_ref::AtomicRef;

/// Shared-owner wrapper around [`AtomicRef<T>`].
///
/// This type is a convenience newtype for `Arc<AtomicRef<T>>`. Cloning an
/// [`ArcAtomicRef`] clones the shared owner handle, so all clones operate on
/// the same atomic reference container.
///
/// This is different from [`AtomicRef::clone`], which creates a new independent
/// atomic container that initially points to the same value.
///
/// # Example
///
/// ```rust
/// use qubit_atomic::ArcAtomicRef;
/// use std::sync::Arc;
///
/// let config = ArcAtomicRef::from_value(10);
/// let shared = config.clone();
///
/// shared.store(Arc::new(20));
/// assert_eq!(*config.load(), 20);
/// assert_eq!(config.strong_count(), 2);
/// ```
pub struct ArcAtomicRef<T> {
    /// Shared owner of the underlying atomic reference container.
    inner: Arc<AtomicRef<T>>,
}

impl<T> ArcAtomicRef<T> {
    /// Creates a new shared atomic reference from an [`Arc<T>`].
    ///
    /// # Parameters
    ///
    /// * `value` - The initial shared value stored in the atomic reference.
    ///
    /// # Returns
    ///
    /// A shared atomic reference wrapper initialized to `value`.
    #[inline]
    pub fn new(value: Arc<T>) -> Self {
        Self::from_atomic_ref(AtomicRef::new(value))
    }

    /// Creates a new shared atomic reference from an owned value.
    ///
    /// # Parameters
    ///
    /// * `value` - The owned value to store as the initial reference.
    ///
    /// # Returns
    ///
    /// A shared atomic reference wrapper initialized to `Arc::new(value)`.
    #[inline]
    pub fn from_value(value: T) -> Self {
        Self::from_atomic_ref(AtomicRef::from_value(value))
    }

    /// Wraps an existing [`AtomicRef<T>`] in an [`Arc`].
    ///
    /// # Parameters
    ///
    /// * `atomic_ref` - The atomic reference container to share.
    ///
    /// # Returns
    ///
    /// A shared atomic reference wrapper owning `atomic_ref`.
    #[inline]
    pub fn from_atomic_ref(atomic_ref: AtomicRef<T>) -> Self {
        Self {
            inner: Arc::new(atomic_ref),
        }
    }

    /// Wraps an existing shared atomic reference container.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared atomic reference container to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    pub fn from_arc(inner: Arc<AtomicRef<T>>) -> Self {
        Self { inner }
    }

    /// Returns the underlying [`Arc`] without cloning it.
    ///
    /// # Returns
    ///
    /// A shared reference to the underlying `Arc<AtomicRef<T>>`.
    #[inline]
    pub fn as_arc(&self) -> &Arc<AtomicRef<T>> {
        &self.inner
    }

    /// Consumes this wrapper and returns the underlying [`Arc`].
    ///
    /// # Returns
    ///
    /// The underlying `Arc<AtomicRef<T>>`.
    #[inline]
    pub fn into_arc(self) -> Arc<AtomicRef<T>> {
        self.inner
    }

    /// Returns the number of strong [`Arc`] owners.
    ///
    /// # Returns
    ///
    /// The current strong reference count of the shared atomic reference
    /// container.
    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}

impl<T> Clone for ArcAtomicRef<T> {
    /// Clones the shared owner handle.
    ///
    /// # Returns
    ///
    /// A new wrapper pointing to the same underlying atomic reference
    /// container.
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Deref for ArcAtomicRef<T> {
    type Target = AtomicRef<T>;

    /// Dereferences to the underlying [`AtomicRef<T>`].
    ///
    /// # Returns
    ///
    /// A shared reference to the atomic reference container.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<T> From<AtomicRef<T>> for ArcAtomicRef<T> {
    /// Converts an atomic reference container into a shared wrapper.
    ///
    /// # Parameters
    ///
    /// * `atomic_ref` - The atomic reference container to share.
    ///
    /// # Returns
    ///
    /// A shared atomic reference wrapper owning `atomic_ref`.
    #[inline]
    fn from(atomic_ref: AtomicRef<T>) -> Self {
        Self::from_atomic_ref(atomic_ref)
    }
}

impl<T> From<Arc<AtomicRef<T>>> for ArcAtomicRef<T> {
    /// Converts an existing shared atomic reference container into its wrapper.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared atomic reference container to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    fn from(inner: Arc<AtomicRef<T>>) -> Self {
        Self::from_arc(inner)
    }
}

impl<T: fmt::Debug> fmt::Debug for ArcAtomicRef<T> {
    /// Formats the currently loaded reference and sharing state for debugging.
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
        f.debug_struct("ArcAtomicRef")
            .field("value", &self.load())
            .field("strong_count", &self.strong_count())
            .finish()
    }
}

impl<T: fmt::Display> fmt::Display for ArcAtomicRef<T> {
    /// Formats the currently loaded reference with display formatting.
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
