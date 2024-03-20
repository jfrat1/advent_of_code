use super::ItemExtractor;

use super::SchematicLine;
use super::super::line::line_parse::NumericStringAndRange;
use super::super::number_container::LineNumberContainer;
use super::super::range_idx_compare;

pub struct LineGearRatioExtractor<'a> {
    inner: GearRatioExtractorInner<'a>,
}

impl<'a> ItemExtractor for LineGearRatioExtractor<'a> {
    fn items_in_line(&self) -> LineNumberContainer {
        self.inner.gear_ratios_in_line()
    }
}

impl LineGearRatioExtractor<'_> {
    pub fn new<'a>(
        line: &'a SchematicLine,
        prev_line: Option<&'a SchematicLine>,
        next_line: Option<&'a SchematicLine>,
    ) -> LineGearRatioExtractor<'a> {
        let part_numbers_line = line;
        let mut symbols_lines = vec![line];
        if let Some(prev) = prev_line {
            symbols_lines.push(prev);
        }
        if let Some(next) = next_line {
            symbols_lines.push(next);
        }
        LineGearRatioExtractor {
            inner: GearRatioExtractorInner {
                gear_line: part_numbers_line,
                part_number_lines: symbols_lines,
            }
        }
    }
}

struct GearRatioExtractorInner<'a> {
    gear_line: &'a SchematicLine,
    part_number_lines: Vec<&'a SchematicLine>,
}

impl GearRatioExtractorInner<'_> {
    fn gear_ratios_in_line(&self) -> LineNumberContainer {
        let mut gear_ratios =  LineNumberContainer::new();

        let gear_indices = self.gear_line.star_character_idxs();

        for gear_idx in gear_indices {
            if let Some(gear_ratio) = self.gear_ratio_for_gear_index(gear_idx) {
                gear_ratios.push_allow_duplicates(gear_ratio);
            }
        }

        gear_ratios
    }

    fn part_number_candidates(&self) -> Vec<NumericStringAndRange> {
        let mut part_number_candidates: Vec<NumericStringAndRange> = Vec::new();
        for &line in self.part_number_lines.iter() {
            part_number_candidates.extend(line.numeric_char_groups_and_idx_ranges());
        }
        part_number_candidates
    }

    fn gear_ratio_for_gear_index(&self, gear_index: usize) -> Option<u32> {
        let mut gear_ratio_candidate = GearRatioCandidate::new();

        for part_number in &self.part_number_candidates() {
            if range_idx_compare::is_index_in_or_adjacent_to_range(gear_index, &part_number.range) {
                gear_ratio_candidate.push_part_number(part_number.numeric_string.parse::<u32>().expect(""));
            }
        }

        gear_ratio_candidate.gear_ratio()
    }
}

struct GearRatioCandidate {
    adjacent_part_numbers: Vec<u32>,
}

impl GearRatioCandidate {
    fn new() -> GearRatioCandidate {
        GearRatioCandidate { adjacent_part_numbers: Vec::new() }
    }

    fn push_part_number(&mut self, part_number: u32) {
        self.adjacent_part_numbers.push(part_number);
    }

    fn gear_ratio(&mut self) -> Option<u32> {
        if self.adjacent_part_numbers.len() != 2 {
            return None
        }

        let mut gear_ratio: u32 = 1;
        self.adjacent_part_numbers.iter().for_each(|part_num| gear_ratio *= part_num);

        Some(gear_ratio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod gear_ratio_extractor {
        use super::*;

        mod constructor {
            use super::*;

            #[test]
            fn test_no_prev_line_no_next_line() {
                let line = SchematicLine::from(String::from(".........."));
                let extractor = LineGearRatioExtractor::new(
                    &line,
                    None,
                    None,
                );

                assert_eq!(extractor.inner.gear_line, &line);
                assert!(extractor.inner.part_number_lines.len() == 1);
                assert_eq!(
                    *extractor.inner.part_number_lines.get(0).expect(""),
                    &line
                )
            }

            #[test]
            fn test_no_prev_line() {
                let line = SchematicLine::from(String::from(".........."));
                let next_line = SchematicLine::from(String::from("foo"));

                let extractor = LineGearRatioExtractor::new(
                    &line,
                    None,
                    Some(&next_line),
                );

                assert_eq!(extractor.inner.gear_line, &line);
                assert!(extractor.inner.part_number_lines.len() == 2);
                assert_eq!(
                    *extractor.inner.part_number_lines.get(0).expect(""),
                    &line
                );
                assert_eq!(
                    *extractor.inner.part_number_lines.get(1).expect(""),
                    &next_line
                );
            }

            #[test]
            fn test_no_next_line() {
                let line = SchematicLine::from(String::from(".........."));
                let prev_line = SchematicLine::from(String::from("foo"));
                let extractor = LineGearRatioExtractor::new(
                    &line,
                    Some(&prev_line),
                    None,
                );

                assert_eq!(extractor.inner.gear_line, &line);
                assert!(extractor.inner.part_number_lines.len() == 2);
                assert_eq!(
                    *extractor.inner.part_number_lines.get(0).expect(""),
                    &line
                );
                assert_eq!(
                    *extractor.inner.part_number_lines.get(1).expect(""),
                    &prev_line
                );
            }

            #[test]
            fn test_all_lines() {
                let line = SchematicLine::from(String::from(".........."));
                let next_line = SchematicLine::from(String::from("foo"));
                let prev_line = SchematicLine::from(String::from("bar"));
                let extractor = LineGearRatioExtractor::new(
                    &line,
                    Some(&prev_line),
                    Some(&next_line),
                );

                assert_eq!(extractor.inner.gear_line, &line);
                assert!(extractor.inner.part_number_lines.len() == 3);
                assert_eq!(
                    *extractor.inner.part_number_lines.get(0).expect(""),
                    &line
                );
                assert_eq!(
                    *extractor.inner.part_number_lines.get(1).expect(""),
                    &prev_line
                );
                assert_eq!(
                    *extractor.inner.part_number_lines.get(2).expect(""),
                    &next_line
                );
            }
        }


    }

    mod gear_ratio_extractor_inner {
        use super::*;

        mod gear_ratios_in_line {
            use super::*;

            #[test]
            fn test_single_line_no_stars() {
                let line = SchematicLine::from(String::from(".........."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &line,
                    part_number_lines: vec![&line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::new())
            }

            #[test]
            fn test_single_line_no_part_numbers() {
                let line = SchematicLine::from(String::from(".....*...."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &line,
                    part_number_lines: vec![&line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::new())
            }

            #[test]
            fn test_single_line_no_gear_ratios_single_part_number_match() {
                let line = SchematicLine::from(String::from("...43*...."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &line,
                    part_number_lines: vec![&line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::new())
            }

            #[test]
            fn test_single_line_one_gear_ratio_match() {
                let line = SchematicLine::from(String::from("...43*12.."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &line,
                    part_number_lines: vec![&line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::from(vec![516]))
            }

            #[test]
            fn test_single_line_multiple_gear_ratio_matches() {
                let line = SchematicLine::from(String::from("...43*12..123*4.."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &line,
                    part_number_lines: vec![&line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::from(vec![516, 492]))
            }

            #[test]
            fn test_two_lines_one_gear_ratio_match() {
                let prev_line = SchematicLine::from(String::from("...43.."));
                let gear_line = SchematicLine::from(String::from("....*12.."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &gear_line,
                    part_number_lines: vec![&gear_line, &prev_line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::from(vec![516]))
            }

            #[test]
            fn test_two_lines_three_parts_on_gear_gets_rejected() {
                let prev_line = SchematicLine::from(String::from("...43...."));
                let gear_line = SchematicLine::from(String::from("..22*12.."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &gear_line,
                    part_number_lines: vec![&gear_line, &prev_line],
                };
                assert!(extractor.gear_ratios_in_line().is_empty());
            }

            #[test]
            fn test_three_lines_one_gear_ratio_match() {
                let prev_line = SchematicLine::from(String::from("...43...."));
                let gear_line = SchematicLine::from(String::from("....*...."));
                let next_line = SchematicLine::from(String::from(".....12.."));
                let extractor = GearRatioExtractorInner {
                    gear_line: &gear_line,
                    part_number_lines: vec![&gear_line, &prev_line, &next_line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::from(vec![516]))
            }

            #[test]
            fn test_three_lines_multiple_gear_ratio_matces() {
                let prev_line = SchematicLine::from(String::from(".43...."));
                let gear_line = SchematicLine::from(String::from("..*...6*"));
                let next_line = SchematicLine::from(String::from("...12...22"));
                let extractor = GearRatioExtractorInner {
                    gear_line: &gear_line,
                    part_number_lines: vec![&gear_line, &prev_line, &next_line],
                };
                assert_eq!(extractor.gear_ratios_in_line(), LineNumberContainer::from(vec![516, 132]))
            }

        }

    }

    mod gear_ratio_candidate {
        use super::*;

        #[test]
        fn test_push_part_number() {
            let mut candidate = GearRatioCandidate::new();
            candidate.push_part_number(123);

            assert_eq!(candidate.adjacent_part_numbers, vec![123]);
        }

        #[test]
        fn test_push_two_part_numbers() {
            let mut candidate = GearRatioCandidate::new();
            candidate.push_part_number(123);
            candidate.push_part_number(456);

            assert_eq!(candidate.adjacent_part_numbers, vec![123, 456]);
        }

        #[test]
        fn test_gear_ratio_with_one_part_number_returns_none() {
            let mut candidate = GearRatioCandidate::new();
            candidate.push_part_number(123);

            assert_eq!(candidate.gear_ratio(), None);
        }

        #[test]
        fn test_gear_ratio_with_three_part_numbers_returns_none() {
            let mut candidate = GearRatioCandidate::new();
            candidate.push_part_number(123);
            candidate.push_part_number(456);
            candidate.push_part_number(789);

            assert_eq!(candidate.gear_ratio(), None);
        }

        #[test]
        fn test_gear_ratio_with_two_part_numbers_returns_ratio() {
            let mut candidate = GearRatioCandidate::new();
            candidate.push_part_number(123);
            candidate.push_part_number(456);

            assert_eq!(candidate.gear_ratio(), Some(56_088));
        }
    }
}