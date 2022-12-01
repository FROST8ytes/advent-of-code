mod file_processes;

use file_processes::*;

use std::env::args;
use std::io;

fn main() -> io::Result<()> {
    let Some(file_path) = args().nth(1) else {
        eprintln!("Input file path needed to be passed as an argument.");
        std::process::exit(1);
    };

    let processed_data = process_file(file_path)?;
    let mut max_total = u32::MIN;
    let mut max_elf= usize::MIN;

    // stupid patch for part 2
    let mut second_max_total = u32::MIN;
    let mut second_max_elf = usize::MIN;
    let mut third_max_total = u32::MIN;
    let mut third_max_elf = usize::MIN;

    for (elf_id, calories_carried) in processed_data.into_iter().enumerate() {
        let total: u32 = calories_carried.iter().sum();
        if total > max_total {
            third_max_total = second_max_total;
            third_max_elf = second_max_elf;
            second_max_total = max_total;
            second_max_elf = max_elf;
            max_elf = elf_id;
            max_total = total;
        } else if total > second_max_total {
            third_max_total = second_max_total;
            third_max_elf = second_max_elf;
            second_max_total = total;
            second_max_elf = elf_id;
        } else if total > third_max_total {
            third_max_total = total;
            third_max_elf = elf_id;
        }
    }

    println!("Part 1: Elf {max_elf} carries the most calories, which is {max_total}.");
    println!("Part 2: Total calories carried by elves {max_elf}, {second_max_elf}, and {third_max_elf} is {}.", max_total + second_max_total + third_max_total);
    Ok(())
}
