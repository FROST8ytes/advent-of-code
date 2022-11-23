use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env::args;

mod position;
use position::Position;

fn main() -> io::Result<()> {
    let path_to_file = args().nth(1);
    if path_to_file == None {
        eprintln!("Path to input file is required to be passed as an argument.");
        return Ok(())
    }
    let input_file = File::open(path_to_file.unwrap())?;
    let reader = BufReader::new(input_file);

    let mut calculated_positions: Vec<Position> = Vec::new();

    for line in reader.lines() {
        let split_line: Vec<String> = line?.split(" ").map(str::to_string).collect();
        let command = split_line.first().unwrap().clone();
        let val = split_line.get(1).unwrap().parse::<i32>().unwrap();
        calculated_positions.push(determine_position(command, val));
    }

    let final_pos: Position = calculated_positions.into_iter().sum();
    let answer = final_pos.horizontal * final_pos.vertical;
    println!("{answer:?}");
    Ok(())
}

fn determine_position(command: String, value: i32) -> Position {
    let mut answer = Position::new(0, 0);

    match command.as_str() {
        "forward" => {
            answer.horizontal += value;
        },
        "down" => {
            answer.vertical += value;
        },
        "up" => {
            answer.vertical -= value;
        },
        _ => {
            panic!("Unknown command from input.");
        },
    }

    answer
}