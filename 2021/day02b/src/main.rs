use std::env::args;
use std::io::{self, prelude::*};

mod position;
use position::*;

mod file_ops;
use file_ops::*;

fn main() -> io::Result<()> {
    let Some(path_to_file) = args().nth(1) else {
        eprintln!("Path to file is required to be passed as an argument.");
        std::process::exit(1);
    };

    let mut ship = Position::new();

    let reader = setup_input_file(path_to_file)?;

    for line in reader.lines() {
        let (command, value) = parse_line(line?);
        ship.run(command, value);
    }

    println!("Final position: {}", ship.calculate_final_position());

    Ok(())
}

