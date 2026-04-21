/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Macro for implementing hidden integer marker operations.

/// Implements hidden integer marker operations for a supported primitive type.
macro_rules! impl_atomic_integer_value {
    ($value_type:ty, $primitive_type:ty, $inner_type:ty) => {
        impl_atomic_value!($value_type, $primitive_type, $inner_type);

        impl AtomicIntegerValue for $value_type {
            #[inline]
            fn fetch_inc(primitive: &Self::Primitive) -> Self {
                primitive.fetch_inc()
            }

            #[inline]
            fn fetch_dec(primitive: &Self::Primitive) -> Self {
                primitive.fetch_dec()
            }

            #[inline]
            fn fetch_and(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_and(value)
            }

            #[inline]
            fn fetch_or(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_or(value)
            }

            #[inline]
            fn fetch_xor(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_xor(value)
            }

            #[inline]
            fn fetch_not(primitive: &Self::Primitive) -> Self {
                primitive.fetch_not()
            }

            #[inline]
            fn fetch_accumulate<F>(primitive: &Self::Primitive, value: Self, f: F) -> Self
            where
                F: Fn(Self, Self) -> Self,
            {
                primitive.fetch_accumulate(value, f)
            }

            #[inline]
            fn fetch_max(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_max(value)
            }

            #[inline]
            fn fetch_min(primitive: &Self::Primitive, value: Self) -> Self {
                primitive.fetch_min(value)
            }
        }
    };
}
