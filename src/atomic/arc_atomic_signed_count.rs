/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Shared Atomic Signed Count Wrapper
//!
//! Provides [`ArcAtomicSignedCount`], a convenience wrapper around
//! `Arc<AtomicSignedCount>`.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use super::atomic_signed_count::AtomicSignedCount;

/// Shared-owner wrapper around [`AtomicSignedCount`].
///
/// This type is a convenience newtype for `Arc<AtomicSignedCount>`. Cloning an
/// [`ArcAtomicSignedCount`] clones the shared owner handle, so all clones
/// operate on the same underlying signed counter.
///
/// # Example
///
/// ```rust
/// use qubit_atomic::ArcAtomicSignedCount;
///
/// let counter = ArcAtomicSignedCount::zero();
/// let shared = counter.clone();
///
/// shared.sub(3);
/// assert_eq!(counter.get(), -3);
/// assert_eq!(counter.strong_count(), 2);
/// ```
pub struct ArcAtomicSignedCount {
    /// Shared owner of the underlying signed atomic counter.
    inner: Arc<AtomicSignedCount>,
}

impl ArcAtomicSignedCount {
    /// Creates a new shared signed atomic counter.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A shared signed counter wrapper initialized to `value`.
    #[inline]
    pub fn new(value: isize) -> Self {
        Self::from_count(AtomicSignedCount::new(value))
    }

    /// Creates a new shared signed counter initialized to zero.
    ///
    /// # Returns
    ///
    /// A shared signed counter wrapper whose current value is zero.
    #[inline]
    pub fn zero() -> Self {
        Self::new(0)
    }

    /// Wraps an existing [`AtomicSignedCount`] in an [`Arc`].
    ///
    /// # Parameters
    ///
    /// * `counter` - The signed counter container to share.
    ///
    /// # Returns
    ///
    /// A shared signed counter wrapper owning `counter`.
    #[inline]
    pub fn from_count(counter: AtomicSignedCount) -> Self {
        Self {
            inner: Arc::new(counter),
        }
    }

    /// Wraps an existing shared signed atomic counter.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared signed atomic counter to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    pub fn from_arc(inner: Arc<AtomicSignedCount>) -> Self {
        Self { inner }
    }

    /// Returns the underlying [`Arc`] without cloning it.
    ///
    /// # Returns
    ///
    /// A shared reference to the underlying `Arc<AtomicSignedCount>`.
    #[inline]
    pub fn as_arc(&self) -> &Arc<AtomicSignedCount> {
        &self.inner
    }

    /// Consumes this wrapper and returns the underlying [`Arc`].
    ///
    /// # Returns
    ///
    /// The underlying `Arc<AtomicSignedCount>`.
    #[inline]
    pub fn into_arc(self) -> Arc<AtomicSignedCount> {
        self.inner
    }

    /// Returns the number of strong [`Arc`] owners.
    ///
    /// # Returns
    ///
    /// The current strong reference count of the shared signed counter.
    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }
}

impl Clone for ArcAtomicSignedCount {
    /// Clones the shared owner handle.
    ///
    /// # Returns
    ///
    /// A new wrapper pointing to the same underlying signed atomic counter.
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Default for ArcAtomicSignedCount {
    /// Creates a zero-valued shared signed atomic counter.
    ///
    /// # Returns
    ///
    /// A shared signed counter wrapper whose current value is zero.
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl Deref for ArcAtomicSignedCount {
    type Target = AtomicSignedCount;

    /// Dereferences to the underlying [`AtomicSignedCount`].
    ///
    /// # Returns
    ///
    /// A shared reference to the signed atomic counter.
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl From<isize> for ArcAtomicSignedCount {
    /// Converts an initial counter value into a shared signed atomic counter.
    ///
    /// # Parameters
    ///
    /// * `value` - The initial counter value.
    ///
    /// # Returns
    ///
    /// A shared signed counter wrapper initialized to `value`.
    #[inline]
    fn from(value: isize) -> Self {
        Self::new(value)
    }
}

impl From<AtomicSignedCount> for ArcAtomicSignedCount {
    /// Converts a signed atomic counter into a shared signed counter wrapper.
    ///
    /// # Parameters
    ///
    /// * `counter` - The signed counter container to share.
    ///
    /// # Returns
    ///
    /// A shared signed counter wrapper owning `counter`.
    #[inline]
    fn from(counter: AtomicSignedCount) -> Self {
        Self::from_count(counter)
    }
}

impl From<Arc<AtomicSignedCount>> for ArcAtomicSignedCount {
    /// Converts an existing shared signed atomic counter into its wrapper.
    ///
    /// # Parameters
    ///
    /// * `inner` - The shared signed atomic counter to wrap.
    ///
    /// # Returns
    ///
    /// A wrapper around `inner`.
    #[inline]
    fn from(inner: Arc<AtomicSignedCount>) -> Self {
        Self::from_arc(inner)
    }
}

impl fmt::Debug for ArcAtomicSignedCount {
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
        f.debug_struct("ArcAtomicSignedCount")
            .field("value", &self.get())
            .field("strong_count", &self.strong_count())
            .finish()
    }
}

impl fmt::Display for ArcAtomicSignedCount {
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
