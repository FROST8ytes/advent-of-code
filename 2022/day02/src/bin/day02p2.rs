use day02::*;

use std::env::args;

fn main() {
    let Some(file_path) = args().nth(1) else {
        eprintln!("File path must be passed as an argument.");
        std::process::exit(1);
    };

    let file_content = read_file(file_path);
    let result = process_part2(file_content);
    println!("Part 1: Total score is {result}.");
}