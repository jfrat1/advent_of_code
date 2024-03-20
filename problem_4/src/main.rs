use std::fs::read_to_string;

mod solver;

fn main() {
    let puzzle_data = read_to_string("./data.txt").unwrap();

    let sum_of_card_values = sum_of_puzzle_card_points(&puzzle_data);
    println!("Puzzle 1 Solution: {}", sum_of_card_values);
    // First answer: 26426 - Correct!



}

fn sum_of_puzzle_card_points(puzzle_data: &String) -> u32 {
    let solver = solver::PuzzleSolver::new(puzzle_data);
    solver.sum_of_card_points()
}


#[cfg(test)]
mod test {
    use super::*;

    fn provided_check_data() -> String {
        String::from(
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
    }

    #[test]
    fn test_provided_input_puzzle_1() {
        let data = provided_check_data();
        let actual = sum_of_puzzle_card_points(&data);
        assert_eq!(actual, 13);
    }
}