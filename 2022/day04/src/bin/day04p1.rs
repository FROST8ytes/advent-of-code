use colored::Colorize;
use day04::{open_read_file, process_part1};

fn main() {
    let Some(file_path) = std::env::args().nth(1) else {
        eprintln!("File path must be passed as an argument.");
        std::process::exit(1);
    };

    let file_content = open_read_file(file_path);
    let result = process_part1(file_content);
    println!(
        "{} {} assignment pairs fully contain one range in its other range.",
        "Part 1:".italic(),
        result.to_string().green().bold()
    );
}
