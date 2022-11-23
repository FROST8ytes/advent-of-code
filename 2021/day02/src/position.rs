use std::ops::{Add, AddAssign};
use std::iter::Sum;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub horizontal: i32,
    pub vertical: i32,
}

impl Position {
    pub fn new(h: i32, v: i32) -> Self {
        Position {
            horizontal: h,
            vertical: v,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            horizontal: self.horizontal + rhs.horizontal,
            vertical: self.vertical + rhs.vertical,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.horizontal += rhs.horizontal;
        self.vertical += rhs.vertical;
    }
}

impl Sum for Position {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut total = Position::new(0, 0);
        for point in iter {
            total += point;
        }
        total
    }
}