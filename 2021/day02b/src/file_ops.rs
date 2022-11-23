use std::fs::File;
use std::io::{self, BufReader};

pub fn setup_input_file(path_to_file: String) -> io::Result<BufReader<File>> {
    let input_file = File::open(path_to_file)?;
    return Ok(BufReader::new(input_file));
}

pub fn parse_line(input_line: String) -> (String, i32) {
    let split_line: Vec<String> = input_line.split(" ").map(str::to_string).collect();
    let val = split_line.get(1).unwrap().parse::<i32>().unwrap();
    (split_line.first().unwrap().clone(), val)
}