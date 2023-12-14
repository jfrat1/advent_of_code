use std::fs::read_to_string;
use std::vec::Vec;

mod schematic_line;

#[derive(Debug, Eq, PartialEq)]
struct Schematic {
    lines: Vec<schematic_line::SchematicLine>,
}

impl From<String> for Schematic {
    fn from(data: String) -> Self {
        let lines: Vec<schematic_line::SchematicLine> = data
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| schematic_line::SchematicLine::from(s.to_owned()))
            .collect();

        return Schematic { lines: lines };
    }
}

impl Schematic {
    fn part_numbers_by_line(&self) -> Vec<Vec<String>> {
        let mut part_numbers_by_line: Vec<Vec<String>> = Vec::new();
        let num_lines = self.lines.len();
        for (line_idx, line) in self.lines.iter().enumerate() {
            let prev_line: Option<&schematic_line::SchematicLine> = if line_idx == 0 {
                None
            } else {
                Some(&self.lines[line_idx - 1])
            };
            let next_line: Option<&schematic_line::SchematicLine> = if line_idx == num_lines - 1 {
                None
            } else {
                Some(&self.lines[line_idx + 1])
            };
            let line_part_numbers = line.part_numbers_from_line(prev_line, next_line);
            part_numbers_by_line.push(line_part_numbers);
        }

        return part_numbers_by_line;
    }
}

fn solve_puzzle_1(puzzle_data: String) -> u32 {
    let schematic = Schematic::from(puzzle_data);

    let schematic_part_numbers_sum: u32 = schematic
        .part_numbers_by_line()
        .iter()
        .map(|parts_in_line| {
            parts_in_line
                .iter()
                .map(|s| s.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sum();

    return schematic_part_numbers_sum;
}

fn main() {
    let puzzle_data = read_to_string("./data.txt").unwrap();

    let puzzle_1_solution = solve_puzzle_1(puzzle_data);
    println!("Puzzle 1 Solution: {}", puzzle_1_solution)

    // First shot : 327115
    // That's not the right answer; your answer is too low.

    // Second shot: 523914
    // This time I allowed for multiple lines to define the same part number
    // The first answer would throw out any duplicate part numbers
    // That's not the right answer; your answer is too low.

    // Third shot: 525799
    // I tried to allow for a single line to define the same part number, but
    // that didn't change my result
    // I found a bug where a part number at the end of a line wouldn't get picked up
    // That's not the right answer; your answer is too low.
    // Frick
}

#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_schematic_from_string() {
        let data = String::from(
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
        );

        let schematic = Schematic::from(data);
        assert_eq!(
            schematic,
            Schematic {
                lines: vec![
                    schematic_line::SchematicLine {
                        line: String::from("467..114..")
                    },
                    schematic_line::SchematicLine {
                        line: String::from("...*......")
                    },
                    schematic_line::SchematicLine {
                        line: String::from("..35..633.")
                    },
                    schematic_line::SchematicLine {
                        line: String::from("......#...")
                    },
                    schematic_line::SchematicLine {
                        line: String::from("617*......")
                    },
                    schematic_line::SchematicLine {
                        line: String::from(".....+.58.")
                    },
                    schematic_line::SchematicLine {
                        line: String::from("..592.....")
                    },
                    schematic_line::SchematicLine {
                        line: String::from("......755.")
                    },
                    schematic_line::SchematicLine {
                        line: String::from("...$.*....")
                    },
                    schematic_line::SchematicLine {
                        line: String::from(".664.598..")
                    },
                ]
            }
        )
    }

    #[test]
    fn test_schematic_part_numbers_by_line() {
        let schematic = Schematic {
            lines: vec![
                schematic_line::SchematicLine {
                    line: String::from("467..114.."),
                },
                schematic_line::SchematicLine {
                    line: String::from("...*......"),
                },
                schematic_line::SchematicLine {
                    line: String::from("..35..633."),
                },
                schematic_line::SchematicLine {
                    line: String::from("......#..."),
                },
                schematic_line::SchematicLine {
                    line: String::from("617*......"),
                },
                schematic_line::SchematicLine {
                    line: String::from("617*...*50"),
                },
            ],
        };

        // vector order issues - need a better comparator
        let expected_part_numbers_by_line = vec![
            vec![String::from("467")],
            Vec::new(),
            vec![String::from("35"), String::from("633")],
            Vec::new(),
            vec![String::from("617")],
            vec![String::from("617"), String::from("50")],
        ];

        // Iterate through the lines of actual/expect and compare their returned values
        for (actual, expect) in schematic
            .part_numbers_by_line()
            .iter()
            .zip(expected_part_numbers_by_line)
        {
            let actual_owned = actual.to_owned();
            assert!(test_utils::is_vec_set_of_strings_equal(
                actual_owned,
                expect
            ))
        }
    }

    #[test]
    fn test_provided_input_puzzle_1() {
        let data = String::from(
            "467..114..
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
        );
        let schematic = Schematic::from(data);

        // I had trouble getting this to work in a simple way due to vector ordering
        let expected_part_numbers: Vec<Vec<String>> = vec![
            vec![String::from("467")],
            Vec::new(),
            vec![String::from("35"), String::from("633")],
            Vec::new(),
            vec![String::from("617")],
            Vec::new(),
            vec![String::from("592")],
            vec![String::from("755")],
            Vec::new(),
            vec![String::from("664"), String::from("598")],
        ];
        let expected_sum: u32 = 4361;

        let schematic_part_numbers = schematic.part_numbers_by_line();

        // Iterate through the lines of actual/expect and compare their returned values
        for (actual, expect) in schematic_part_numbers.iter().zip(expected_part_numbers) {
            let actual_owned = actual.to_owned();

            assert!(test_utils::is_vec_set_of_strings_equal(
                actual_owned,
                expect
            ))
        }

        let schematic_part_numbers_sum: u32 = schematic_part_numbers
            .iter()
            .map(|parts_in_line| {
                parts_in_line
                    .iter()
                    .map(|s| s.parse::<u32>().unwrap())
                    .sum::<u32>()
            })
            .sum();
        assert_eq!(schematic_part_numbers_sum, expected_sum);
    }
}
