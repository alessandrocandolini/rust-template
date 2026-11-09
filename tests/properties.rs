use proptest::prelude::*;

proptest! {
    // Generate u32 inputs so their sum always fits in the function's u64 result.
    #[test]
    fn addition_is_commutative(left in any::<u32>(), right in any::<u32>()) {
        let left = u64::from(left);
        let right = u64::from(right);
        prop_assert_eq!({{crate_name}}::add(left, right), {{crate_name}}::add(right, left));
    }

    #[test]
    fn adding_zero_preserves_the_number(value in any::<u64>()) {
        prop_assert_eq!({{crate_name}}::add(value, 0), value);
    }
}
