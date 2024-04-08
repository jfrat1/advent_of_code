
use std::ops::RangeInclusive;

#[derive(Debug, Eq, PartialEq)]
pub struct NumericStringAndRange {
    pub numeric_string: String,
    pub range: RangeInclusive<usize>,
}

pub fn char_is_not_numeric_or_dot(character: char) -> bool {
    char_is_not_numeric(character) && char_is_not_a_dot(character)
}

fn char_is_not_numeric(character: char) -> bool {
    !character.is_numeric()
}

fn char_is_not_a_dot(character: char) -> bool {
    character != '.'
}

pub fn ranges_of_continuous_numeric_characters_in(
    line: &String,
) -> Vec<NumericStringAndRange> {
    let mut continuous_numbers_and_ranges = Vec::new();

    let mut start_idx: Option<usize> = None;
    let mut part_number: String = String::new();

    let line_len = line.len();
    for (idx, character) in line.chars().enumerate() {
        if character.is_numeric() {
            part_number += &character.to_string();
            start_idx = start_idx.or(Some(idx));
        }

        let is_last_character = idx == (line_len - 1);
        if !character.is_numeric() || is_last_character {
            if let Some(start) = start_idx {
                continuous_numbers_and_ranges.push(NumericStringAndRange{
                    numeric_string: part_number.clone(),
                    range: start..=idx - 1
                });

                // reinitialize the static variables
                start_idx = None;
                part_number = String::new();
            }
        }
    }

    continuous_numbers_and_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_is_not_numeric_or_dot_false_for_number() {
        assert!(!char_is_not_numeric_or_dot('4'));
    }

    #[test]
    fn test_char_is_not_numeric_or_dot_false_for_dot() {
        assert!(!char_is_not_numeric_or_dot('.'));
    }

    #[test]
    fn test_char_is_not_numeric_or_dot_true_for_symbol() {
        assert!(char_is_not_numeric_or_dot('*'));
    }
}
