use std::{collections::HashMap, ops::RangeInclusive};

mod line_parse;
mod vector_index_cmp;

#[derive(Debug, Eq, PartialEq)]
pub struct SchematicLine {
    pub line: String,
}

impl From<String> for SchematicLine {
    fn from(line: String) -> Self {
        return SchematicLine { line: line };
    }
}

impl SchematicLine {
    pub fn part_numbers_from_line(
        &self,
        prev_line: Option<&Self>,
        next_line: Option<&Self>,
    ) -> Vec<String> {
        let mut part_numbers: Vec<String> = Vec::new();
        if let Some(part_numbers_and_ranges) = self.part_numbers_and_idx_ranges() {
            for (part_number, range) in part_numbers_and_ranges {
                // TODO - remove this duplication of code

                // check this line
                if let Some(this_line_symbol_idxs) = self.inidices_of_symbol_characters() {
                    for idx in this_line_symbol_idxs {
                        if vector_index_cmp::is_index_in_or_adjacent_to_range(idx, &range)
                            && !part_numbers.contains(&part_number)
                        {
                            part_numbers.push(part_number.clone());
                        }
                    }
                }

                // check previous line
                if let Some(prev) = &prev_line {
                    if let Some(prev_symbox_idxs) = prev.inidices_of_symbol_characters() {
                        for idx in prev_symbox_idxs {
                            if vector_index_cmp::is_index_in_or_adjacent_to_range(idx, &range)
                                && !part_numbers.contains(&part_number)
                            {
                                part_numbers.push(part_number.clone());
                            }
                        }
                    }
                }

                // check next line
                if let Some(next) = &next_line {
                    if let Some(next_symbox_idxs) = next.inidices_of_symbol_characters() {
                        for idx in next_symbox_idxs {
                            if vector_index_cmp::is_index_in_or_adjacent_to_range(idx, &range)
                                && !part_numbers.contains(&part_number)
                            {
                                part_numbers.push(part_number.clone());
                            }
                        }
                    }
                }
            }
        }

        return part_numbers;
    }

    pub fn part_numbers_and_idx_ranges(&self) -> Option<HashMap<String, RangeInclusive<usize>>> {
        return line_parse::ranges_of_continuous_numeric_characters_in_line(&self.line);
    }

    pub fn inidices_of_symbol_characters(&self) -> Option<Vec<usize>> {
        return line_parse::indices_of_non_dot_symbol_characters_in_line(&self.line);
    }
}

#[cfg(test)]
mod tests;
