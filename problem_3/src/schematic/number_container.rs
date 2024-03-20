
#[derive(Debug, Default, Eq, PartialEq)]
pub struct LineNumberContainer {
    numbers: Vec<u32>,
}

impl From<Vec<u32>> for LineNumberContainer {
    fn from(numbers: Vec<u32>) -> LineNumberContainer {
        LineNumberContainer{numbers}
    }
}

impl LineNumberContainer {
    pub fn new() -> Self {
        LineNumberContainer::default()
    }
}

impl LineNumberContainer {
    pub fn sum(&self) -> u32 {
        self
            .numbers
            .iter()
            .sum()
    }

    pub fn push_allow_duplicates(&mut self, number: u32) {
        self.numbers.push(number);
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.numbers.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod line_part_numbers {
        use super::LineNumberContainer;

        #[test]
        fn test_new() {
            assert_eq!(LineNumberContainer::new(), LineNumberContainer{numbers: Vec::new()});
        }

        #[test]
        fn test_push_one_part_number() {
            let mut part_nums = LineNumberContainer::new();
            part_nums.push_allow_duplicates(123);
            assert_eq!(part_nums, LineNumberContainer{numbers: vec![123]});
        }

        #[test]
        fn test_push_two_part_numbers() {
            let mut part_nums = LineNumberContainer::new();
            part_nums.push_allow_duplicates(123);
            part_nums.push_allow_duplicates(456);
            assert_eq!(part_nums, LineNumberContainer{numbers: vec![123, 456]});
        }

        #[test]
        fn test_push_part_number_that_already_exists() {
            let mut part_nums = LineNumberContainer{
                numbers: vec![123]
            };

            part_nums.push_allow_duplicates(123);
            assert_eq!(part_nums, LineNumberContainer{numbers: vec![123, 123]})
        }


        #[test]
        fn test_sum() {
            let mut part_nums = LineNumberContainer::new();
            part_nums.push_allow_duplicates(123);
            part_nums.push_allow_duplicates(456);

            assert_eq!(part_nums.sum(), 579)
        }
    }


}
