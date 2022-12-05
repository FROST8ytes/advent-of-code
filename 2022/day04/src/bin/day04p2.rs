use colored::Colorize;
use day04::{open_read_file, process_part2};

fn main() {
    let Some(file_path) = std::env::args().nth(1) else {
        eprintln!("File path must be passed as an argument.");
        std::process::exit(1);
    };

    let file_content = open_read_file(file_path);
    let result = process_part2(file_content);
    println!(
        "{} {} assignment pairs contain ranges that overlap.",
        "Part 2:".italic(),
        result.to_string().green().bold()
    );
}
