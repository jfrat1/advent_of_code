use std::{collections::HashMap, ops::RangeInclusive};

pub mod line_compare;
mod line_parse;

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq)]
pub struct SchematicLine {
    pub line: String,
}

impl From<String> for SchematicLine {
    fn from(line: String) -> Self {
        SchematicLine { line }
    }
}

impl SchematicLine {
    // use this method to extract the 3 chunks of repeated code in
    // the next method
    fn part_numbers_with_symbols_adjacent_in_other_line(&self, other: &Self) -> Vec<String> {
        let mut part_numbers: Vec<String> = Vec::new();
        if let Some(part_numbers_and_ranges) = self.part_numbers_and_idx_ranges() {
            if let Some(other_line_symbol_idxs) = &other.inidices_of_symbol_characters() {
                for (part_number, range) in part_numbers_and_ranges {
                    for idx in other_line_symbol_idxs.iter() {
                        if line_compare::is_index_in_or_adjacent_to_range(*idx, &range) {
                            part_numbers.push(part_number.clone());
                        }
                    }
                }
            }
        }

        part_numbers
    }

    pub fn part_numbers_and_idx_ranges(&self) -> Option<HashMap<String, RangeInclusive<usize>>> {
        line_parse::ranges_of_continuous_numeric_characters_in_line(&self.line)
    }

    pub fn inidices_of_symbol_characters(&self) -> Option<Vec<usize>> {
        line_parse::indices_of_non_dot_symbol_characters_in_line(&self.line)
    }
}
