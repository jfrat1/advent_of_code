use super::*;

use crate::test_utils;

#[test]
fn test_schem_line_character_idxs() {
    let line = SchematicLine {
        line: String::from("467..114.."),
    };
    assert!(line.inidices_of_symbol_characters().is_none());

    let line = SchematicLine {
        line: String::from("...*......"),
    };
    assert_eq!(line.inidices_of_symbol_characters(), Some(vec![3]));

    let line = SchematicLine {
        line: String::from(".....+.58."),
    };
    assert_eq!(line.inidices_of_symbol_characters(), Some(vec![5]));

    let line = SchematicLine {
        line: String::from("...$.*...."),
    };
    assert_eq!(line.inidices_of_symbol_characters(), Some(vec![3, 5]));

    let line = SchematicLine {
        line: String::from(".........*"),
    };
    assert_eq!(line.inidices_of_symbol_characters(), Some(vec![9]));

    let line = SchematicLine {
        line: String::from("*........."),
    };
    assert_eq!(line.inidices_of_symbol_characters(), Some(vec![0]));
}

#[test]
fn test_schem_line_part_numbers_and_idx_ranges() {
    let line = SchematicLine {
        line: String::from("467..114.."),
    };
    assert_eq!(
        line.part_numbers_and_idx_ranges(),
        Some(HashMap::from([
            (String::from("467"), (0..=2)),
            (String::from("114"), 5..=7)
        ])),
    );

    let line = SchematicLine {
        line: String::from("...*......"),
    };
    assert!(line.part_numbers_and_idx_ranges().is_none());

    let line = SchematicLine {
        line: String::from(".....+.58."),
    };
    assert_eq!(
        line.part_numbers_and_idx_ranges(),
        Some(HashMap::from([(String::from("58"), 7..=8)])),
    );

    let line = SchematicLine {
        line: String::from("...$.*...."),
    };
    assert!(line.part_numbers_and_idx_ranges().is_none());

    // part number at end of line
    let line = SchematicLine {
        line: String::from("...$.*..73"),
    };
    assert_eq!(
        line.part_numbers_and_idx_ranges(),
        Some(HashMap::from([(String::from("73"), 8..=8)])),
    )
}

#[test]
fn test_schem_line_part_numbers_from_line_single_line_two_matches() {
    let line = SchematicLine {
        line: String::from("467*.*114..."),
    };

    let expect: Vec<String> = vec![String::from("467"), String::from("114")];
    assert!(test_utils::is_vec_set_of_strings_equal(
        line.part_numbers_from_line(None, None),
        expect
    ));
}

#[test]
fn test_schem_line_part_numbers_from_line_single_line_part_num_at_end_of_line() {
    let line = SchematicLine {
        line: String::from("467*....*114"),
    };

    let expect: Vec<String> = vec![String::from("467"), String::from("114")];
    assert!(test_utils::is_vec_set_of_strings_equal(
        line.part_numbers_from_line(None, None),
        expect
    ));
}

#[test]
fn test_schem_line_part_numbers_from_line_with_next_line() {
    let line = SchematicLine {
        line: String::from("467..114.."),
    };

    let next_line: SchematicLine = SchematicLine {
        line: String::from("...*......"),
    };
    let expect: Vec<String> = vec![String::from("467")];
    assert!(test_utils::is_vec_set_of_strings_equal(
        line.part_numbers_from_line(None, Some(&next_line)),
        expect,
    ));
}

#[test]
fn test_schem_line_part_numbers_from_line_with_prev_and_next_lines() {
    let line = SchematicLine {
        line: String::from("..35..633."),
    };
    let prev_line: SchematicLine = SchematicLine {
        line: String::from("...*......"),
    };
    let next_line: SchematicLine = SchematicLine {
        line: String::from("......#..."),
    };
    let expect: Vec<String> = vec![String::from("35"), String::from("633")];

    assert!(test_utils::is_vec_set_of_strings_equal(
        line.part_numbers_from_line(Some(&prev_line), Some(&next_line)),
        expect
    ));
}

#[test]
fn test_schem_line_part_numbers_from_line_with_symbol_in_primary_line() {
    let line = SchematicLine {
        line: String::from("617*...22."),
    };
    let next_line: SchematicLine = SchematicLine {
        line: String::from(".......58."),
    };
    let expect: Vec<String> = vec![String::from("617")];
    assert!(test_utils::is_vec_set_of_strings_equal(
        line.part_numbers_from_line(None, Some(&next_line)),
        expect,
    ));
}
