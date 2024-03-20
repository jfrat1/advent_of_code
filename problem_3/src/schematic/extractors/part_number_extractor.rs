use super::ItemExtractor;

use super::SchematicLine;
use super::super::line::line_parse::NumericStringAndRange;
use super::super::number_container::LineNumberContainer;
use super::super::range_idx_compare::any_bits_in_or_adjacent_to_range;

pub struct LinePartNumberExtractor<'a> {
    inner: PartNumberExtractorInner<'a>,
}


impl<'a> ItemExtractor for LinePartNumberExtractor<'a> {
    fn items_in_line(&self) -> LineNumberContainer {
        self.inner.part_numbers_in_line()
    }
}

impl LinePartNumberExtractor<'_> {
    pub fn new<'a>(
        line: &'a SchematicLine,
        next_line: Option<&'a SchematicLine>,
        prev_line: Option<&'a SchematicLine>,
    ) -> LinePartNumberExtractor<'a> {
        let part_numbers_line = line;
        let mut symbols_lines = vec![line];
        if let Some(prev) = prev_line {
            symbols_lines.push(prev);
        }
        if let Some(next) = next_line {
            symbols_lines.push(next);
        }
        LinePartNumberExtractor {
            inner: PartNumberExtractorInner {
                part_number_line: part_numbers_line,
                symbol_lines: symbols_lines,
            }
        }
    }

    fn part_numbers_in_line(&self) -> LineNumberContainer {
        self.inner.part_numbers_in_line()
    }
}

struct PartNumberExtractorInner<'a> {
    part_number_line: &'a SchematicLine,
    symbol_lines: Vec<&'a SchematicLine>,
}

impl PartNumberExtractorInner<'_> {
    pub fn part_numbers_in_line(&self) -> LineNumberContainer {
        let mut part_numbers = LineNumberContainer::new();

        if self.symbol_lines.is_empty() {
            return part_numbers;
        }

        let part_number_candidates = self.part_number_line.numeric_char_groups_and_idx_ranges();
        for candidate in part_number_candidates {
            if let Some(part_number) = self.part_number_from_candidate(candidate) {
                part_numbers.push_allow_duplicates(part_number)
            }
        }

        part_numbers
    }

    fn symbol_character_bitfield(&self) -> Vec<bool> {
        if self.symbol_lines.is_empty() {
            return Vec::new()
        }

        let first_line = self.symbol_lines[0];
        let other_lines = &self.symbol_lines[1..];

        let mut symbol_character_bitfield = first_line.symbol_character_bitfield();
        for line in other_lines {
            symbol_character_bitfield = bitwise_or(
                symbol_character_bitfield,
                 line.symbol_character_bitfield(),
            );
        }

        symbol_character_bitfield
    }

    fn part_number_from_candidate(&self, candidate: NumericStringAndRange) -> Option<u32> {
        let mut part_number = None;

        if any_bits_in_or_adjacent_to_range(&candidate.range, self.symbol_character_bitfield()) {
            part_number = Some(parse_part_number(candidate.numeric_string));
        }

        part_number
    }


}


fn bitwise_or(vec_1: Vec<bool>, vec_2: Vec<bool>) -> Vec<bool> {
    vec_1.iter().zip(vec_2).map(|(v1, v2)| v1 | v2).collect()
}

fn parse_part_number(part_number: String) -> u32 {
    part_number.parse::<u32>().unwrap_or_else(
        |_| panic!("String {} cannot be parsed to a part number.", part_number)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_or_with_overlap() {
        assert_eq!(
            bitwise_or(
                vec![0, 0, 1, 0, 0, 0, 0, 0].iter().map(|b| *b != 0).collect(),
                vec![0, 0, 1, 0, 0, 1, 0, 0].iter().map(|b| *b != 0).collect(),
            ),
            vec![0, 0, 1, 0, 0, 1, 0, 0].iter().map(|b| *b != 0).collect::<Vec<bool>>(),
        )
    }

    #[test]
    fn test_bitwise_or_no_overlap() {
        assert_eq!(
            bitwise_or(
                vec![0, 0, 1, 0, 0, 0, 0, 1].iter().map(|b| *b != 0).collect(),
                vec![0, 0, 0, 0, 1, 1, 0, 0].iter().map(|b| *b != 0).collect(),
            ),
            vec![0, 0, 1, 0, 1, 1, 0, 1].iter().map(|b| *b != 0).collect::<Vec<bool>>(),
        )
    }


    mod part_number_extractor {
        use super::{LineNumberContainer, LinePartNumberExtractor, SchematicLine};

        #[test]
        fn test_part_nums_in_line_duplicate_entries_in_one_line() {
            let prev_line = SchematicLine{ line: String::from(".......&....*..")};
            let this_line = SchematicLine{ line: String::from(".12...234...12.")};
            let next_line = SchematicLine{ line: String::from(".*.............")};

            let extractor = LinePartNumberExtractor::new(
                &this_line,
                Some(&prev_line),
                Some(&next_line),
            );
            assert_eq!(
                extractor.part_numbers_in_line(),
                LineNumberContainer::from(vec![12, 234, 12]),
            );
        }
    }

    mod part_number_extractor_inner {
        use super::*;

        mod part_numbers_in_line {
            use super::*;

            #[test]
            fn test_no_symbols_lines() {
                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456..")),
                    symbol_lines: Vec::new(),
                };
                assert_eq!(extractor.part_numbers_in_line(), LineNumberContainer::new());
            }

            #[test]
            fn test_one_symbols_line_no_matches() {
                let symbol_line = SchematicLine::from(String::from(".........."));
                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456")),
                    symbol_lines: vec![&symbol_line],
                };
                assert_eq!(extractor.part_numbers_in_line(), LineNumberContainer::new());
            }

            #[test]
            fn test_one_symbols_line_with_single_match() {
                let symbol_line = SchematicLine::from(String::from(".*........"));
                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456")),
                    symbol_lines: vec![&symbol_line],
                };
                assert_eq!(
                    extractor.part_numbers_in_line(),
                    LineNumberContainer::from(vec![123]),
                );
            }

            #[test]
            fn test_one_symbols_line_with_muiltiple_matches() {
                let symbol_line = SchematicLine::from(String::from(".*....&..."));
                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456")),
                    symbol_lines: vec![&symbol_line],
                };
                assert_eq!(
                    extractor.part_numbers_in_line(),
                    LineNumberContainer::from(vec![123, 456]),
                );
            }

            #[test]
            fn test_mutiple_symbols_lines_no_matches() {
                let symbol_line_1 = SchematicLine::from(String::from(".........."));
                let symbol_line_2 = SchematicLine::from(String::from(".........."));

                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456")),
                    symbol_lines: vec![
                        &symbol_line_1,
                        &symbol_line_2,
                    ],
                };
                assert_eq!(
                    extractor.part_numbers_in_line(),
                    LineNumberContainer::new(),
                );
            }

            #[test]
            fn test_mutiple_symbols_lines_with_matches() {
                let symbol_line_1 = SchematicLine::from(String::from(".........."));
                let symbol_line_2 = SchematicLine::from(String::from(".%........"));

                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456")),
                    symbol_lines: vec![
                        &symbol_line_1,
                        &symbol_line_2,
                    ],
                };
                assert_eq!(
                    extractor.part_numbers_in_line(),
                    LineNumberContainer::from(vec![123]),
                );
            }

            #[test]
            fn test_mutiple_symbols_lines_with_multiple_matches() {
                let symbol_line_1 = SchematicLine::from(String::from(".........@"));
                let symbol_line_2 = SchematicLine::from(String::from("&........."));

                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456")),
                    symbol_lines: vec![
                        &symbol_line_1,
                        &symbol_line_2,
                    ],
                };
                assert_eq!(
                    extractor.part_numbers_in_line(),
                    LineNumberContainer::from(vec![123, 456]),
                );
            }

            #[test]
            fn test_mutiple_symbols_lines_with_duplicated_matches() {
                let symbol_line_1 = SchematicLine::from(String::from(".........@....#"));
                let symbol_line_2 = SchematicLine::from(String::from("&.............."));

                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456..123")),
                    symbol_lines: vec![
                        &symbol_line_1,
                        &symbol_line_2,
                    ],
                };
                assert_eq!(
                    extractor.part_numbers_in_line(),
                    LineNumberContainer::from(vec![123, 456, 123]),
                );
            }
        }

        mod symbol_character_bitfield {
            use crate::test_utils::ints_to_bools;

            use super::PartNumberExtractorInner;
            use super::SchematicLine;

            #[test]
            fn test_no_symbols_lines() {
                let extractor = PartNumberExtractorInner {
                    part_number_line: &SchematicLine::new(),
                    symbol_lines: Vec::new(),
                };
                assert_eq!(extractor.symbol_character_bitfield(), Vec::new())
            }

            #[test]
            fn test_one_symbol_line_no_symbols() {
                let symbol_line = SchematicLine::from(String::from(".........."));
                let extractor = PartNumberExtractorInner {
                    part_number_line: &SchematicLine::new(),
                    symbol_lines: vec![&symbol_line],
                };
                assert_eq!(
                    extractor.symbol_character_bitfield(),
                    ints_to_bools(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                )
            }

            #[test]
            fn test_multiple_symbol_lines_no_symbols() {
                let symbol_line_1 = SchematicLine::from(String::from(".........."));
                let symbol_line_2 = SchematicLine::from(String::from(".........."));
                let symbol_line_3 = SchematicLine::from(String::from(".........."));

                let extractor = PartNumberExtractorInner {
                    part_number_line: &SchematicLine::new(),
                    symbol_lines: vec![
                        &symbol_line_1,
                        &symbol_line_2,
                        &symbol_line_3,
                    ],
                };
                assert_eq!(
                    extractor.symbol_character_bitfield(),
                    ints_to_bools(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                )
            }

            #[test]
            fn test_one_symbol_line_with_symbols() {
                let symbol_line = SchematicLine::from(String::from(".*.....&./"));
                let extractor = PartNumberExtractorInner {
                    part_number_line: &SchematicLine::new(),
                    symbol_lines: vec![
                        &symbol_line,
                    ],
                };
                assert_eq!(
                    extractor.symbol_character_bitfield(),
                    ints_to_bools(vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 1]),
                )
            }

            #[test]
            fn test_multiple_symbol_lines_with_non_overlapping_symbols() {
                let symbol_line_1 = SchematicLine::from(String::from(".*........"));
                let symbol_line_2 = SchematicLine::from(String::from(".........#"));
                let symbol_line_3 = SchematicLine::from(String::from(".......%.."));

                let extractor = PartNumberExtractorInner {
                    part_number_line: &SchematicLine::new(),
                    symbol_lines: vec![
                        &symbol_line_1,
                        &symbol_line_2,
                        &symbol_line_3,
                    ],
                };

                assert_eq!(
                    extractor.symbol_character_bitfield(),
                    ints_to_bools(vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 1]),
                )
            }
        }

        mod part_number_from_candidate {
            use super::*;

            #[test]
            fn test_candidate_is_not_part_num() {
                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456..")),
                    symbol_lines: Vec::new(),
                };

                let candidate = NumericStringAndRange {
                    numeric_string: String::from("123"),
                    range: 1..=3,
                };
                assert_eq!(
                    extractor.part_number_from_candidate(candidate),
                    None,
                )
            }

            #[test]
            fn test_candidate_is_part_num() {
                let symbol_line = SchematicLine::from(String::from(".*........"));
                let extractor = PartNumberExtractorInner{
                    part_number_line: &SchematicLine::from(String::from(".123...456..")),
                    symbol_lines: vec![&symbol_line],
                };

                let candidate = NumericStringAndRange {
                    numeric_string: String::from("123"),
                    range: 1..=3,
                };

                assert_eq!(
                    extractor.part_number_from_candidate(candidate),
                    Some(123),
                )
            }
        }
    }
}