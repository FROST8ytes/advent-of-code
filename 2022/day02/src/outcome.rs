#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

pub fn determine_outcome(outcome: &str) -> Outcome {
    match outcome {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Undefined data in input file."),

    }
}