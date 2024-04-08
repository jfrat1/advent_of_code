

pub struct PuzzleSolver {
    lines: Vec<PuzzleLine>,
}

impl PuzzleSolver {
    pub fn new(puzzle_data: &String) -> PuzzleSolver {
        let lines: Vec<PuzzleLine> = puzzle_data
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| PuzzleLine::new(s))
            .collect();

        PuzzleSolver { lines: lines }
    }

    pub fn sum_of_card_points(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| line.points())
            .sum()
    }
}

struct PuzzleLine {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl PuzzleLine {
    pub fn new(line: &str) -> PuzzleLine {
        PuzzleLine {
            winning_numbers: Self::winning_numbers_from_line(line),
            card_numbers:Self::card_numbers_from_line(line),
        }
    }

    fn winning_numbers_from_line(line: &str) -> Vec<u32> {
        let winning_numbers_str = Self::winning_numbers_str_from_line(line);
        winning_numbers_str
            .split(" ")
            .filter_map(|num| {
                if num.is_empty() {
                    None
                } else {
                    Some(
                        num.parse::<u32>().expect(format!("Should be able to parse {} to u32", num).as_str())
                    )
                }
            })
            .collect()
    }

    fn winning_numbers_str_from_line(line: &str) -> &str {
        let first_match = line.split_once(":");

        if let Some(first) = first_match {
            let after_card_number = first.1;
            let second_match = after_card_number.split_once("|");
            if let Some(second) = second_match {
                return second.0.trim()
            }
        }

        ""
    }

    fn card_numbers_from_line(line: &str) -> Vec<u32> {
        let card_numbers_str = Self::card_numbers_str_from_line(line);
        card_numbers_str
            .split(" ")
            .filter_map(|num| {
                if num.is_empty() {
                    None
                } else {
                    Some(
                        num.parse::<u32>().expect(format!("Should be able to parse {} to u32", num).as_str())
                    )
                }
            })
            .collect()
    }

    fn card_numbers_str_from_line(line: &str) -> &str {
        let first_match = line.split_once(":");

        if let Some(first) = first_match {
            let after_card_number = first.1;
            let second_match = after_card_number.split_once("|");
            if let Some(second) = second_match {
                return second.1.trim()
            }
        }

        ""
    }

    fn points(&self) -> u32 {
        let winning_count = self.winning_number_count();
        if winning_count == 0 {
            return 0
        }

        (2 as u32).pow(winning_count - 1)
    }

    fn winning_number_count(&self) -> u32 {
        let mut count: u32 = 0;

        self.card_numbers.iter().for_each(|num| {
            if self.winning_numbers.contains(num) {
                count += 1;
            }
        });

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod puzzle_line {
        use super::*;

        fn puzzle_data_line() -> &'static str {
            "Card 1: 41 48 83  86 17 | 83 86  6 31 17  9 48 53"
        }

        #[test]
        fn test_get_winning_numbers_str() {
            let line = puzzle_data_line();
            let winning_numbers_str = PuzzleLine::winning_numbers_str_from_line(line);
            assert_eq!(winning_numbers_str, "41 48 83  86 17");
        }

        #[test]
        fn test_get_card_numbers_str() {
            let line = puzzle_data_line();
            let winning_numbers_str = PuzzleLine::card_numbers_str_from_line(line);
            assert_eq!(winning_numbers_str, "83 86  6 31 17  9 48 53");
        }

        #[test]
        fn test_winning_numbers_from_line() {
            let line = puzzle_data_line();
            let winning_numbers = PuzzleLine::winning_numbers_from_line(line);
            assert_eq!(winning_numbers, vec![41, 48, 83, 86, 17]);
        }

        #[test]
        fn test_card_numbers_from_line() {
            let line = puzzle_data_line();
            let winning_numbers = PuzzleLine::card_numbers_from_line(line);
            assert_eq!(winning_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        }

        #[test]
        fn test_winning_number_count() {
            let line_data = puzzle_data_line();
            let line = PuzzleLine::new(line_data);
            let count = line.winning_number_count();
            assert_eq!(count, 4)
        }

        #[test]
        fn test_points() {
            let line_data = puzzle_data_line();
            let line = PuzzleLine::new(line_data);
            let points = line.points();
            assert_eq!(points, 8)
        }
    }

    mod puzzle_solver {
        use super::*;

        fn puzzle_data() -> String {
            String::from(
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
        }

        #[test]
        fn test_sum_of_card_points() {
            let data = puzzle_data();
            let solver = PuzzleSolver::new(&data);
            let points = solver.sum_of_card_points();
            assert_eq!(points, 13);
        }
    }
}