use std::ops::RangeInclusive;


pub fn is_index_in_or_adjacent_to_range(index: usize, range: &RangeInclusive<usize>) -> bool {
    // protect for underflow
    let range_start: &usize = range.start();
    let range_end: &usize = range.end();

    let range_start_with_adjacent_idx: usize = if *range_start == 0 {*range_start} else {*range_start - 1};
    let range_end_with_adjacent_idx: usize = *range_end + 1;

    let range_plus_adjacent_idxs: RangeInclusive<usize> = range_start_with_adjacent_idx..=range_end_with_adjacent_idx;
    return range_plus_adjacent_idxs.contains(&index);
}


#[cfg(test)]
mod tests {
    use super::is_index_in_or_adjacent_to_range;

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