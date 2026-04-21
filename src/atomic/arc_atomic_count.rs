/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Atomic Count Wrapper
//!
//! Provides [`ArcAtomicCount`], a convenience wrapper around
//! `Arc<AtomicCount>`.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use super::atomic_count::AtomicCount;

/// Shared-owner wrapper around [`AtomicCount`].
///
/// This type is a convenience newtype for `Arc<AtomicCount>`. Cloning an
/// [`ArcAtomicCount`] clones the shared owner handle, so all clones operate on
/// the same underlying non-negative counter.
///
/// # Example
///
/// ```rust
/// use qubit_atomic::ArcAtomicCount;
///
/// let counter = ArcAtomicCount::zero();
/// let shared = counter.clone();
///
/// shared.inc();
/// assert_eq!(counter.get(), 1);
/// assert_eq!(counter.strong_count(), 2);
/// ```
pub struct ArcAtomicCount {
    /// Shared owner of the underlying atomic counter.
    inner: Arc<AtomicCount>,
}

impl ArcAtomicCount {
    /// Creates a new shared non-negative atomic counter.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A shared counter wrapper initialized to `value`.
    #[inline]
    pub fn new(value: usize) -> Self {
        Self::from_count(AtomicCount::new(value))
    }

    /// Creates a new shared counter initialized to zero.
    ///
    /// # Returns
    ///
    /// A shared counter wrapper whose current value is zero.
    #[inline]
    pub fn zero() -> Self {
        Self::new(0)
    }

    /// Wraps an existing [`AtomicCount`] in an [`Arc`].
    ///
    /// # Parameters
    ///
    /// * `counter` - The counter container to share.
    ///
    /// # Returns
    ///
    /// A shared counter wrapper owning `counter`.
    #[inline]
    pub fn from_count(counter: AtomicCount) -> Self {
        Self {
            inner: Arc::new(counter),
        }
    }

    /// Wraps an existing shared atomic counter.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared atomic counter to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    pub fn from_arc(inner: Arc<AtomicCount>) -> Self {
        Self { inner }
    }

    /// Returns the underlying [`Arc`] without cloning it.
    ///
    /// # Returns
    ///
    /// A shared reference to the underlying `Arc<AtomicCount>`.
    #[inline]
    pub fn as_arc(&self) -> &Arc<AtomicCount> {
        &self.inner
    }

    /// Consumes this wrapper and returns the underlying [`Arc`].
    ///
    /// # Returns
    ///
    /// The underlying `Arc<AtomicCount>`.
    #[inline]
    pub fn into_arc(self) -> Arc<AtomicCount> {
        self.inner
    }

    /// Returns the number of strong [`Arc`] owners.
    ///
    /// # Returns
    ///
    /// The current strong reference count of the shared counter.
    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}

impl Clone for ArcAtomicCount {
    /// Clones the shared owner handle.
    ///
    /// # Returns
    ///
    /// A new wrapper pointing to the same underlying atomic counter.
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Default for ArcAtomicCount {
    /// Creates a zero-valued shared atomic counter.
    ///
    /// # Returns
    ///
    /// A shared counter wrapper whose current value is zero.
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl Deref for ArcAtomicCount {
    type Target = AtomicCount;

    /// Dereferences to the underlying [`AtomicCount`].
    ///
    /// # Returns
    ///
    /// A shared reference to the atomic counter.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl From<usize> for ArcAtomicCount {
    /// Converts an initial counter value into a shared atomic counter.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A shared counter wrapper initialized to `value`.
    #[inline]
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<AtomicCount> for ArcAtomicCount {
    /// Converts an atomic counter into a shared atomic counter wrapper.
    ///
    /// # Parameters
    ///
    /// * `counter` - The counter container to share.
    ///
    /// # Returns
    ///
    /// A shared counter wrapper owning `counter`.
    #[inline]
    fn from(counter: AtomicCount) -> Self {
        Self::from_count(counter)
    }
}

impl From<Arc<AtomicCount>> for ArcAtomicCount {
    /// Converts an existing shared atomic counter into its wrapper.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared atomic counter to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    fn from(inner: Arc<AtomicCount>) -> Self {
        Self::from_arc(inner)
    }
}

impl fmt::Debug for ArcAtomicCount {
    /// Formats the current counter value and sharing state for debugging.
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
        f.debug_struct("ArcAtomicCount")
            .field("value", &self.get())
            .field("strong_count", &self.strong_count())
            .finish()
    }
}

impl fmt::Display for ArcAtomicCount {
    /// Formats the current counter value with decimal display formatting.
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
        write!(f, "{}", self.get())
    }
}
