mod choice;
mod outcome;

use choice::*;
use std::fs;
use crate::outcome::{determine_outcome, Outcome};

const WIN: u32 = 6;
const LOSE: u32 = 0;
const DRAW: u32 = 3;

pub fn read_file(file_path: String) -> String {
    fs::read_to_string(file_path).expect("Unable to open file.")
}

pub fn process_part1(file_content: String) -> u32 {
    file_content.split(determine_newline())
        .map(|round|{
            round.split(' ')
                .map(|choice| {
                    determine_choices_p1(choice)
                })
                .collect::<Vec<Choice>>()
        })
        .fold(Vec::<u32>::new(), |mut score, choice_in_round|{
            let total_score= calculate_score(choice_in_round);
            score.push(total_score);
            score
        })
        .iter()
        .sum::<u32>()
}

pub fn process_part2(file_content: String) -> u32 {
    let guide = file_content.split(determine_newline());
    let mut total = u32::MIN;
    let mut result: Vec<(Choice, Outcome)> = Vec::new();
    for data in guide {
        let data: Vec<&str> = data.split(' ').collect();
        let choice = determine_choices_p2(data.get(0).unwrap());
        let outcome = determine_outcome(data.get(1).unwrap());
        result.push((choice, outcome));
    }

    for (idx, parsed_guide) in result.iter().enumerate() {
        let desired_choice = determine_choice_for_desired_outcome(parsed_guide);
        total += (result.get(idx).unwrap().1 as u32) + (desired_choice as u32);
    }
    total
}

fn determine_newline<'a>() -> &'a str {
    if cfg!(windows) {
        return "\r\n";
    }
    "\n"
}

fn calculate_score(choice_in_round: Vec<Choice>) -> u32 {
    let opponent_choice = choice_in_round.get(0).unwrap().clone();
    let self_choice = choice_in_round.get(1).unwrap().clone();
    let mut total_score = self_choice as u32;

    if opponent_choice == Choice::Rock {
        if self_choice == Choice::Paper {
            total_score += WIN;
        } else if self_choice == Choice::Rock {
            total_score += DRAW;
        } else {
            total_score += LOSE;
        }
    } else if opponent_choice == Choice::Paper {
        if self_choice == Choice::Scissors {
            total_score += WIN;
        } else if self_choice == Choice::Paper {
            total_score += DRAW;
        } else {
            total_score += LOSE;
        }
    } else {
        if self_choice == Choice::Rock {
            total_score += WIN;
        } else if self_choice == Choice::Scissors {
            total_score += DRAW;
        } else {
            total_score += LOSE;
        }
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data<'a>() -> &'a str {
        if cfg!(windows) {
            return "A Y\r\nB X\r\nC Z";
        }
        "A Y\nB X\nC Z"
    }

    #[test]
    fn test_part1() {
        let data = get_data();

        assert_eq!(15, process_part1(data.to_string()));
    }

    #[test]
    fn test_part2() {
        let data = get_data();
        assert_eq!(12, process_part2(data.to_string()));
    }
}
