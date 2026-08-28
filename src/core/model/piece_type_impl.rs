use crate::core::model::{BitBoard, PieceType};
use std::ops::{Index, IndexMut};

impl Index<PieceType> for [BitBoard; 6] {
    type Output = BitBoard;
    fn index(&self, p: PieceType) -> &BitBoard {
        &self[p as usize]
    }
}

impl IndexMut<PieceType> for [BitBoard; 6] {
    fn index_mut(&mut self, p: PieceType) -> &mut BitBoard {
        &mut self[p as usize]
    }
}
