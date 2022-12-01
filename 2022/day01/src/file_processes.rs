use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn process_file(file_path: String) -> io::Result<Vec<Vec<u32>>> {
    let mut result = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut elf: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let calories = line?;
        if calories.len() != 0 {
            elf.push(calories.parse().unwrap());
        } else {
            result.push(elf.clone());
            elf.clear();
        }
    }

    Ok(result)
}
