use crate::core::model::Square;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Square {
    pub fn new(row: i32, column: i32) -> Square {
        Square {row, column}
    }

    pub fn index(&self) -> usize {
        (self.row * 8 + self.column) as usize
    }

    pub fn is_valid(&self) -> bool {
        self.row >= 0 && self.row < 8 && self.column >= 0 && self.column < 8
    }
}

impl Add for Square {
    type Output = Self;
    fn add(self, other: Square) -> Square {
        Square {
            row: self.row + other.row,
            column: self.column + other.column
        }
    }
}

impl AddAssign for Square {
    fn add_assign(&mut self, other: Square) {
        self.row += other.row;
        self.column += other.column;
    }
}

impl Sub for Square {
    type Output = Self;
    fn sub(self, other: Square) -> Square {
        Square {
            row: self.row - other.row,
            column: self.column - other.column
        }
    }
}

impl SubAssign for Square {
    fn sub_assign(&mut self, other: Square) {
        self.row -= other.row;
        self.column -= other.column;
    }
}
