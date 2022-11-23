pub struct Position {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Position {
            horizontal: 0,
            vertical: 0,
            aim: 0,
        }
    }

    fn up(&mut self, value: i32) {
        self.aim -= value;
    }

    fn down(&mut self, value: i32) {
        self.aim += value;
    }

    fn forward(&mut self, value: i32) {
        self.horizontal += value;
        self.vertical += value * self.aim;
    }

    pub fn run(&mut self, command: String, value: i32) {
        match command.as_str() {
            "forward" => self.forward(value),
            "up" => self.up(value),
            "down" => self.down(value),
            _ => {
                eprintln!("Command not valid.");
                std::process::exit(2);
            },
        }
    }

    pub fn calculate_final_position(&self) -> i32 {
        self.horizontal * self.vertical
    }
}
