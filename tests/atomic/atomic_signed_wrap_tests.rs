/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use qubit_atomic::Atomic;

macro_rules! test_signed_fetch_div_min_by_negative_one_wraps {
    ($value_type:ty, $test_name:ident) => {
        #[test]
        fn $test_name() {
            let atomic = Atomic::<$value_type>::new(<$value_type>::MIN);
            let old = atomic.fetch_div(-1);
            assert_eq!(old, <$value_type>::MIN);
            assert_eq!(atomic.load(), <$value_type>::MIN);
        }
    };
}

test_signed_fetch_div_min_by_negative_one_wraps!(i8, test_i8_fetch_div_min_by_negative_one_wraps);
test_signed_fetch_div_min_by_negative_one_wraps!(i16, test_i16_fetch_div_min_by_negative_one_wraps);
test_signed_fetch_div_min_by_negative_one_wraps!(i32, test_i32_fetch_div_min_by_negative_one_wraps);
test_signed_fetch_div_min_by_negative_one_wraps!(i64, test_i64_fetch_div_min_by_negative_one_wraps);
test_signed_fetch_div_min_by_negative_one_wraps!(
    i128,
    test_i128_fetch_div_min_by_negative_one_wraps
);
test_signed_fetch_div_min_by_negative_one_wraps!(
    isize,
    test_isize_fetch_div_min_by_negative_one_wraps
);
