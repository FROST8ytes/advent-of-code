mod file_processes;
use file_processes::*;

use std::env::args;

fn main() {
    let Some(file_path) = args().nth(1) else {
        eprintln!("Input file path needed to be passed as an argument.");
        std::process::exit(1);
    };

    let part_1 = process_file_rustic_p1(file_path.clone());
    println!("Part 1: Max calories carried by one elf are {part_1}.");

    let part_2 = process_file_rustic_p2(file_path);
    println!("Part 2: Max calories carried by 3 elves are {part_2}.");
}

