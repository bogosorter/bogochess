use crate::core::model::BitBoard;

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

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
