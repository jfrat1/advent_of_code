pub fn is_vec_set_of_strings_equal(vec_1: Vec<String>, vec_2: Vec<String>) -> bool {
    if vec_1.len() != vec_2.len() {
        return false;
    }

    let mut vec_2_owned = vec_2.to_owned();

    let mut matches: usize = 0;
    for vec_1_str in vec_1.iter() {
        if let Some(vec_2_pos) = vec_2_owned
            .iter()
            .position(|vec_2_str| *vec_2_str == *vec_1_str)
        {
            vec_2_owned.remove(vec_2_pos);
            matches += 1;
        }
    }
    return matches == vec_1.len();
}

#[cfg(test)]
mod tests {
    use super::is_vec_set_of_strings_equal;

    #[test]
    fn test_is_vec_set_equal_1_element_true() {
        assert!(is_vec_set_of_strings_equal(
            vec![String::from("foo")],
            vec![String::from("foo")],
        ))
    }

    #[test]
    fn test_is_vec_set_equal_1_element_false() {
        assert!(!is_vec_set_of_strings_equal(
            vec![String::from("foo")],
            vec![String::from("bar")],
        ))
    }

    #[test]
    fn test_is_vec_set_equal_2_elements_true() {
        assert!(is_vec_set_of_strings_equal(
            vec![String::from("foo"), String::from("bar")],
            vec![String::from("foo"), String::from("bar")],
        ))
    }

    #[test]
    fn test_is_vec_set_equal_2_elements_false() {
        assert!(!is_vec_set_of_strings_equal(
            vec![String::from("foo"), String::from("bar")],
            vec![String::from("foo"), String::from("doo")],
        ))
    }

    #[test]
    fn test_is_vec_set_equal_repeating_elements_true() {
        assert!(is_vec_set_of_strings_equal(
            vec![
                String::from("foo"),
                String::from("foo"),
                String::from("bar")
            ],
            vec![
                String::from("foo"),
                String::from("foo"),
                String::from("bar")
            ],
        ))
    }

    #[test]
    fn test_is_vec_set_equal_repeating_elements_different_order_true() {
        assert!(is_vec_set_of_strings_equal(
            vec![
                String::from("foo"),
                String::from("foo"),
                String::from("bar")
            ],
            vec![
                String::from("foo"),
                String::from("bar"),
                String::from("foo")
            ],
        ))
    }
}
