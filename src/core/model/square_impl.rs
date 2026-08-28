use crate::core::model::{BitBoard, Square};
use std::ops::Index;

impl Square {
    pub fn new(s: usize) -> Square {
        Square(s)
    }

    pub fn bitboard(&self) -> BitBoard {
        BitBoard(1 << self.0)
    }

    pub fn shift(&self, s: i8) -> Square {
        Square((self.0 as i8 + s) as usize)
    }

    pub fn row(&self) -> usize {
        self.0 / 8
    }

    pub fn col(&self) -> usize {
        self.0 % 8
    }
}

impl Index<Square> for [u64; 64] {
    type Output = u64;
    fn index(&self, index: Square) -> &u64 {
        &self[index.0]
    }
}
