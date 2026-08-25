use crate::core::model::Position;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Position {
    pub fn new(row: i32, column: i32) -> Position {
        Position {row, column}
    }

    pub fn index(&self) -> usize {
        (self.row * 8 + self.column) as usize
    }

    pub fn is_valid(&self) -> bool {
        self.row >= 0 && self.row < 8 && self.column >= 0 && self.column < 8
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, other: Position) -> Position {
        Position {
            row: self.row + other.row,
            column: self.column + other.column
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        self.row += other.row;
        self.column += other.column;
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, other: Position) -> Position {
        Position {
            row: self.row - other.row,
            column: self.column - other.column
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, other: Position) {
        self.row -= other.row;
        self.column -= other.column;
    }
}
