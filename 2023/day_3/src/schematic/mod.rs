use std::vec::Vec;

mod extractors;
mod line;
mod number_container;
mod range_idx_compare;

use line::SchematicLine;

#[derive(Debug, Eq, PartialEq)]
pub enum ExtractorType {
    GearRatio,
    PartNumber,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Schematic {
    lines: Vec<SchematicLine>,
    extractor_type: ExtractorType,
}

impl From<&String> for Schematic {
    fn from(data: &String) -> Self {
        let lines: Vec<SchematicLine> = data
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| SchematicLine::from(s.to_owned()))
            .collect();

        Schematic {
            lines: lines,
            extractor_type: ExtractorType::GearRatio,
        }
    }
}

impl Schematic {
    pub fn new(data: &String, extractor_type: ExtractorType) -> Schematic {
        let lines: Vec<SchematicLine> = data
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| SchematicLine::from(s.to_owned()))
            .collect();

        Schematic { lines: lines, extractor_type: extractor_type }
    }

    fn next_line(&self, line_idx: usize) -> Option<&line::SchematicLine> {
        let num_lines = self.lines.len();
        let next_line: Option<&line::SchematicLine> = if line_idx == num_lines - 1 {
            None
        } else {
            Some(&self.lines[line_idx + 1])
        };
        next_line
    }

    fn previous_line(&self, line_idx: usize) -> Option<&line::SchematicLine> {
        let prev_line: Option<&line::SchematicLine> = if line_idx == 0 {
            None
        } else {
            Some(&self.lines[line_idx - 1])
        };
        prev_line
    }

    pub fn sum_of_extracted_items(&self) -> u32 {
        self
            .items_by_line()
            .iter()
            .map(|gear_ratios_in_line| {
                gear_ratios_in_line.sum()
            })
            .sum()
    }


    fn items_by_line(&self) -> Vec<number_container::LineNumberContainer> {
        let mut items_by_line: Vec<number_container::LineNumberContainer> = Vec::new();

        for (line_idx, line) in self.lines.iter().enumerate() {
            let extractor = self.get_line_extractor(line, line_idx);
            items_by_line.push(extractor.items_in_line());
        }

        items_by_line
    }

    fn get_line_extractor<'a>(&'a self, line: &'a line::SchematicLine, line_idx: usize) -> Box<dyn extractors::ItemExtractor + 'a> {
        match self.extractor_type {
            ExtractorType::GearRatio => Box::new(extractors::gear_ratio_extractor::LineGearRatioExtractor::new(
                line,
                self.previous_line(line_idx),
                self.next_line(line_idx),
            )),
            ExtractorType::PartNumber => Box::new(extractors::part_number_extractor::LinePartNumberExtractor::new(
                line,
                self.previous_line(line_idx),
                self.next_line(line_idx),
            )),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::schematic::ExtractorType;

    use super::{line::SchematicLine, Schematic};

    fn test_data() -> String {
        String::from(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        )
    }


    #[test]
    fn test_schematic_from_string() {
        let data = test_data();

        let schematic = Schematic::new(&data, ExtractorType::PartNumber);
        assert_eq!(
            schematic,
            Schematic {
                lines: vec![
                    SchematicLine {
                        line: String::from("467..114..")
                    },
                    SchematicLine {
                        line: String::from("...*......")
                    },
                    SchematicLine {
                        line: String::from("..35..633.")
                    },
                    SchematicLine {
                        line: String::from("......#...")
                    },
                    SchematicLine {
                        line: String::from("617*......")
                    },
                    SchematicLine {
                        line: String::from(".....+.58.")
                    },
                    SchematicLine {
                        line: String::from("..592.....")
                    },
                    SchematicLine {
                        line: String::from("......755.")
                    },
                    SchematicLine {
                        line: String::from("...$.*....")
                    },
                    SchematicLine {
                        line: String::from(".664.598..")
                    },
                ],
                extractor_type: ExtractorType::PartNumber,
            }
        )
    }

    #[test]
    fn test_schematic_sum_of_part_numbers_with_duplicates() {
        let schematic = Schematic {
            lines: vec![
                SchematicLine { line: String::from("467..114..") },
                SchematicLine { line: String::from("...*......") },
                SchematicLine { line: String::from("..35..35*.") },
                SchematicLine { line: String::from("......#...") },
                SchematicLine { line: String::from("617*......") },
                SchematicLine { line: String::from("617*...*50") },
            ],
            extractor_type: ExtractorType::PartNumber,
        };

        assert_eq!(schematic.sum_of_extracted_items(), 1821);
    }

    #[test]
    fn test_schematic_sum_of_part_numbers_in_test_data() {
        let data = test_data();
        let schematic = Schematic::new(&data, ExtractorType::PartNumber);

        assert_eq!(schematic.sum_of_extracted_items(), 4361);
    }

    #[test]
    fn test_schematic_sum_of_gear_ratios_in_test_data() {
        let data = test_data();
        let schematic = Schematic::new(&data, ExtractorType::GearRatio);

        assert_eq!(schematic.sum_of_extracted_items(), 467835);
    }
}
