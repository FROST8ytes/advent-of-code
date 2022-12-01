use std::fs;

pub fn process_file_rustic_p1(file_path: String) -> u32 {
    let content = read_file(file_path);

    process_lines(content).into_iter()
        .max()
        .expect("Unable to calculate max. Perhaps due to dirty data.")
}

pub fn process_file_rustic_p2(file_path: String) -> u32 {
    let content = read_file(file_path);
    let mut result = process_lines(content);
    result.sort();
    result[result.len()-3..].iter().sum()
}


fn read_file(file_path: String) -> String {
    fs::read_to_string(file_path).expect("Unable to read file. Check file path.")
}

fn define_newline<'a>() -> &'a str {
    if cfg!(windows) {
        return "\r\n";
    }
    "\n"
}

fn process_lines(content: String) -> Vec<u32> {
    content.split(define_newline())
        .map(|load| {
            load.lines()
                .map(|calories| calories.parse::<u32>().expect("Data is not u32."))
                .sum::<u32>()
        })
        .collect()
}