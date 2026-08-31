use crate::core::model::{BitBoard, Square};

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

impl BitBoard {
    pub fn empty(&self) -> bool {
        *self == BitBoard(0)
    }

    pub fn row(n: usize) -> BitBoard {
        BitBoard(0xFF << (n * 8))
    }

    pub fn col(n: usize) -> BitBoard {
        BitBoard(0x0101010101010101 << n)
    }

    pub fn shift_row_forward(&self, amount: usize) -> BitBoard {
        BitBoard(self.0 << (amount * 8))
    }

    pub fn shift_row_backward(&self, amount: usize) -> BitBoard {
        BitBoard(self.0 >> (amount * 8))
    }

    pub fn shift_column_forward(&self, amount: i8) -> BitBoard {
        BitBoard(self.0 << amount)
    }

    pub fn shift_column_backward(&self, amount: i8) -> BitBoard {
        BitBoard(self.0 >> amount)
    }
}

impl Iterator for BitBoard {
    type Item = Square;
    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            return None;
        }

        let position = Square(self.0.trailing_zeros() as usize);
        self.0 &= self.0 - 1;
        Some(position)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl Not for BitBoard {
    type Output = BitBoard;
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}
