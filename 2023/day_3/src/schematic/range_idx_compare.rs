use std::iter;
use std::ops::RangeInclusive;

pub fn is_index_in_or_adjacent_to_range(index: usize, range: &RangeInclusive<usize>) -> bool {
    let range_plus_adjacent_idxs = range_with_adjacent_indices(range);
    range_plus_adjacent_idxs.contains(&index)
}

pub fn any_bits_in_or_adjacent_to_range(range: &RangeInclusive<usize>, bitfield: Vec<bool>) -> bool {
    let range_plus_adjacent_idxs = range_with_adjacent_indices(range);
    let mut range_bits: Vec<bool> = iter::repeat(false).take(*range_plus_adjacent_idxs.end()).collect();

    range_plus_adjacent_idxs.into_iter().for_each(|i| range_bits.insert(i, true));
    range_bits.iter().zip(bitfield).any(|(r, b)| r & b)
}

fn range_with_adjacent_indices(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    let extended_range_start = usize_subtract_min_zero(range.start(), &1);
    let extended_range_end = range.end() + 1;
    extended_range_start..=extended_range_end
}

fn usize_subtract_min_zero(minuend: &usize, subtractend: &usize) -> usize {
    if subtractend > minuend {
        0
    } else {
        minuend - subtractend
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::ints_to_bools;

    #[test]
    fn test_any_bits_in_or_adjacent_to_range_true_in_range() {
        let range = 2..=4;
        let bitfield = ints_to_bools(vec![0, 0, 0, 1, 0, 0]);
        assert!(any_bits_in_or_adjacent_to_range(&range, bitfield));
    }

    #[test]
    fn test_any_bits_in_or_adjacent_to_range_true_adjacent_to_range() {
        let range = 2..=4;
        let bitfield = ints_to_bools(vec![0, 1, 0, 0, 0, 0]);
        assert!(any_bits_in_or_adjacent_to_range(&range, bitfield));
    }

    #[test]
    fn test_any_bits_in_or_adjacent_to_range_false_out_of_range() {
        let range = 2..=4;
        let bitfield = ints_to_bools(vec![1, 0, 0, 0, 0, 0]);
        assert!(!any_bits_in_or_adjacent_to_range(&range, bitfield));
    }

    #[test]
    fn test_subtract_usize_min_zero_no_limit() {
        assert_eq!(usize_subtract_min_zero(&2, &1), 1);
    }

    #[test]
    fn test_subtract_usize_min_zero_with_limit() {
        assert_eq!(usize_subtract_min_zero(&2, &3), 0);
    }

    #[test]
    fn test_subtract_usize_min_zero_with_limit_minuend_zero() {
        assert_eq!(usize_subtract_min_zero(&0, &1), 0);
    }

    #[test]
    fn test_is_index_in_or_adjacent_to_range() {
        // idx inside range
        assert!(is_index_in_or_adjacent_to_range(3, &(2..=4)));

        // idx adjacent to range, left
        assert!(is_index_in_or_adjacent_to_range(3, &(4..=6)));

        // idx adjacent to range, right
        assert!(is_index_in_or_adjacent_to_range(3, &(0..=2)));

        // idx outside of range
        assert!(!is_index_in_or_adjacent_to_range(6, &(2..=4)));

        // Check for underflow below zero when calculating the range
        // with adjacent indices
        assert!(is_index_in_or_adjacent_to_range(0, &(0..=4)));

        // Check for underflow below zero when calculating the range
        // with adjacent indices
        assert!(is_index_in_or_adjacent_to_range(5, &(0..=4)));
    }
}
