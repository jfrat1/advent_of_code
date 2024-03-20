

// FIXME - make private. this is for a test of the raw data
pub mod line_parse;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SchematicLine {
    pub line: String,
}

impl From<String> for SchematicLine {
    fn from(line: String) -> Self {
        SchematicLine { line }
    }
}

impl SchematicLine {
    #[cfg(test)]
    pub fn new() -> Self {
        SchematicLine::default()
    }

    pub fn symbol_character_bitfield(&self) -> Vec<bool> {
        self.line
        .chars()
        .map(line_parse::char_is_not_numeric_or_dot)
        .collect()
    }

    pub fn numeric_char_groups_and_idx_ranges(&self) -> Vec<line_parse::NumericStringAndRange> {
        line_parse::ranges_of_continuous_numeric_characters_in(&self.line)
    }

    pub fn star_character_idxs(&self) -> Vec<usize> {
        let idxs_and_chars = self.line
            .chars()
            .enumerate();

        let star_character_idxs: Vec<usize> = idxs_and_chars
            .filter_map(|(index, char)| {
                (char == '*').then_some(index)
            })
            .collect();

        star_character_idxs
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod symbol_character_bitfield {
        use std::iter;
        use super::*;

        #[test]
        fn test_no_symbols_in_string() {
            let line = SchematicLine {
                line: String::from("..........")
            };
            assert_eq!(
                line.symbol_character_bitfield(),
                iter::repeat(false).take(10).collect::<Vec<bool>>(),
            )
        }

        #[test]
        fn test_one_symbol_in_string() {
            let line = SchematicLine {
                line: String::from("..*.......")
            };
            let expect: Vec<bool> = vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0]
                .iter()
                .map(|b| *b != 0)
                .collect();
            assert_eq!(
                line.symbol_character_bitfield(),
                expect,
            )
        }

        #[test]
        fn test_multiple_symbols_in_string() {
            let line = SchematicLine {
                line: String::from("..*..&...%")
            };
            let expect: Vec<bool> = vec![0, 0, 1, 0, 0, 1, 0, 0, 0, 1]
                .iter()
                .map(|b| *b != 0)
                .collect();
            assert_eq!(
                line.symbol_character_bitfield(),
                expect,
            )
        }
    }

    mod numeric_char_groups_and_idx_ranges {
        use super::*;

        #[test]
        fn test_two_numeric_goups_in_string() {
            let line = SchematicLine {
                line: String::from("467..114.."),
            };
            assert_eq!(
                line.numeric_char_groups_and_idx_ranges(),
                Vec::from([
                    line_parse::NumericStringAndRange{numeric_string: String::from("467"), range: 0..=2},
                    line_parse::NumericStringAndRange{numeric_string: String::from("114"), range: 5..=7},
                ]),
            );
        }

        #[test]
        fn test_no_numeric_groups_in_string() {
            let line = SchematicLine {
                line: String::from("...*......"),
            };
            assert_eq!(line.numeric_char_groups_and_idx_ranges(), Vec::new());
        }

        #[test]
        fn test_one_numeric_group_with_symbol_in_string() {
            let line = SchematicLine {
                line: String::from(".....+.58."),
            };
            assert_eq!(
                line.numeric_char_groups_and_idx_ranges(),
                Vec::from([
                    line_parse::NumericStringAndRange{numeric_string:String::from("58"), range: 7..=8},
                ]),
            );
        }

        #[test]
        fn test_one_numeric_group_with_multiple_symbols_in_string() {
            // part number at end of line
            let line = SchematicLine {
                line: String::from("...$...*73"),
            };
            assert_eq!(
                line.numeric_char_groups_and_idx_ranges(),
                Vec::from([
                    line_parse::NumericStringAndRange{numeric_string:String::from("73"), range: 8..=8},
                ]),
            )
        }
    }

    mod star_character_idxs {
        use super::*;

        #[test]
        fn test_no_stars_in_line() {
            let line = SchematicLine {
                line: String::from("..........")
            };
            assert_eq!(
                line.star_character_idxs(),
                Vec::new(),
            )
        }

        #[test]
        fn test_one_star_in_line() {
            let line = SchematicLine {
                line: String::from("...*......")
            };
            assert_eq!(
                line.star_character_idxs(),
                vec![3],
            )
        }

        #[test]
        fn test_multiple_stars_in_line() {
            let line = SchematicLine {
                line: String::from("*..*...*..")
            };
            assert_eq!(
                line.star_character_idxs(),
                vec![0, 3, 7],
            )
        }

        #[test]
        fn test_multiple_stars_with_numbers_mixed_in() {
            let line = SchematicLine {
                line: String::from("12..*..45*..")
            };
            assert_eq!(
                line.star_character_idxs(),
                vec![4, 9],
            )
        }

        #[test]
        fn test_multiple_stars_with_other_symbols_mixed_in() {
            let line = SchematicLine {
                line: String::from("&...*..%*..")
            };
            assert_eq!(
                line.star_character_idxs(),
                vec![4, 8],
            )
        }
    }

}