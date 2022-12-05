mod parser;
use parser::{assignment_pairs, AssignmentPairs};

pub fn process_part1(file_content: String) -> u32 {
    file_content.lines().fold(0_u32, |mut total, pairs| {
        let (_, pairs) = assignment_pairs(pairs).unwrap();
        if check_within(pairs) {
            total += 1;
        }
        total
    })
}

pub fn process_part2(file_content: String) -> u32 {
    file_content.lines().fold(0_u32, |mut total, pairs| {
        let (_, pairs) = assignment_pairs(pairs).unwrap();
        if check_overlap(pairs) {
            total += 1;
        }
        total
    })
}

pub fn open_read_file(file_path: String) -> String {
    std::fs::read_to_string(file_path).expect("Unable to read file (file does not exist)")
}

fn check_within(range_pairs: AssignmentPairs) -> bool {
    let first = range_pairs.0;
    let second = range_pairs.1;

    let first_within_second = second.0 <= first.0 && second.1 >= first.1;
    let second_within_first = first.0 <= second.0 && first.1 >= second.1;

    first_within_second || second_within_first
}

fn check_overlap(range_pairs: AssignmentPairs) -> bool {
    let first = range_pairs.0;
    let second = range_pairs.1;

    (first.0 >= second.0 && first.0 <= second.1)
        || (first.1 >= second.0 && first.1 <= second.1)
        || (second.0 >= first.0 && second.0 <= first.1)
        || (second.1 >= first.0 && second.1 <= first.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data<'a>() -> &'a str {
        "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
    }

    #[test]
    fn part1_works() {
        let result = process_part1(get_data().to_string());
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() {
        let result = process_part2(get_data().to_string());
        assert_eq!(result, 4);
    }
}
