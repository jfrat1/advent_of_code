use std::fs::read_to_string;

mod schematic;

#[cfg(test)]
mod test_utils;


fn sum_of_puzzle_part_numbers(puzzle_data: &String) -> u32 {
    let schematic = schematic::Schematic::new(
        puzzle_data,
        schematic::ExtractorType::PartNumber
    );

    schematic.sum_of_extracted_items()
}

fn sum_of_puzzle_gear_ratios(puzzle_data: &String) -> u32 {
    let schematic = schematic::Schematic::new(
        puzzle_data,
        schematic::ExtractorType::GearRatio
    );

    schematic.sum_of_extracted_items()
}

fn main() {
    let puzzle_data = read_to_string("./data.txt").unwrap();

    let sum_of_part_numbers = sum_of_puzzle_part_numbers(&puzzle_data);
    println!("Puzzle 1 Solution: {}", sum_of_part_numbers);

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

    // Fourth shot: 527364
    // Correct
    // Found a bug where we weren't actually getting duplicated values from a single line

    let sum_of_gear_ratios = sum_of_puzzle_gear_ratios(&puzzle_data);
    println!("Puzzle 2 Solution: {}", sum_of_gear_ratios);
    // First shot: 79026871
    // That's correct!!  Here's to some solid code structure that was easily
    // extended to add a gear ratio extractor.

}

#[cfg(test)]
mod test {
    use super::*;

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
        let schematic = schematic::Schematic::new(
            &data,
            schematic::ExtractorType::PartNumber,
        );

        let expected_sum: u32 = 4361;
        assert_eq!(schematic.sum_of_extracted_items(), expected_sum);
    }

    #[test]
    fn test_does_any_line_in_puzzle_data_have_repeating_number() {

        let puzzle_data = read_to_string("./data.txt").unwrap();

        for line in puzzle_data.lines() {
            let mut nums_in_line: Vec<String> = Vec::new();
            let mut part_number: String = String::new();

            for (idx, character) in line.chars().enumerate() {
                if character.is_numeric() {
                    part_number += &character.to_string();
                }

                let is_last_character = idx == (line.len() - 1);
                if !character.is_numeric() || is_last_character {
                    if !part_number.is_empty() {
                        nums_in_line.push(part_number.clone());

                        // reinitialize the static variable
                        part_number = String::new();
                    }
                }
            }

            for part_number in &nums_in_line {
                let part_num_count_in_line: i32 = nums_in_line
                    .iter()
                    .map(|other_num| if *part_number == *other_num {1} else {0})
                    .sum();

                if part_num_count_in_line > 1 {
                    println!("found {} counts of {} in line:\n {}", part_num_count_in_line, part_number, line);
                }
            }
        }
    }
}
