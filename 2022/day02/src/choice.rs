use crate::outcome::Outcome;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn determine_choices_p1(choice: &str) -> Choice {
    match choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Undefined data from file."),
    }
}

pub fn determine_choices_p2(choice: &str) -> Choice {
    match choice {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Undefined data from file."),
    }
}

pub fn determine_choice_for_desired_outcome(guide: &(Choice, Outcome)) -> Choice {
    match guide.1 {
        Outcome::Draw => guide.0,
        Outcome::Win => {
            match guide.0 {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            }
        },
        Outcome::Lose => {
            match guide.0 {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            }
        }
    }
}