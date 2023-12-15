use super::line_parse;
use std::collections::HashMap;
use std::ops::RangeInclusive;

pub fn indices_of_non_dot_symbol_characters_in_line(line: &str) -> Option<Vec<usize>> {
    let line_idxs_with_symbols: Vec<usize> = line
        .chars()
        .enumerate()
        .filter_map(|(index, char)| line_parse::is_char_a_non_dot_symbol(char).then_some(index))
        .collect();

    let are_symbols_in_line = !line_idxs_with_symbols.is_empty();
    are_symbols_in_line.then_some(line_idxs_with_symbols)
}

pub fn is_char_a_non_dot_symbol(character: char) -> bool {
    !(character.is_numeric() || character == '.')
}

pub fn ranges_of_continuous_numeric_characters_in_line(
    line: &String,
) -> Option<HashMap<String, RangeInclusive<usize>>> {
    let mut continuous_numbers_and_ranges = HashMap::new();

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
                continuous_numbers_and_ranges.insert(part_number.clone(), start..=idx - 1);

                // reinitialize the static variables
                start_idx = None;
                part_number = String::new();
            }
        }
    }

    let are_numeric_chars_in_line = !continuous_numbers_and_ranges.is_empty();
    are_numeric_chars_in_line.then_some(continuous_numbers_and_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_char_a_non_dot_symbol() {
        assert!(!is_char_a_non_dot_symbol('4'));
        assert!(!is_char_a_non_dot_symbol('.'));
        assert!(is_char_a_non_dot_symbol('*'));
        assert!(is_char_a_non_dot_symbol('$'));
    }
}
