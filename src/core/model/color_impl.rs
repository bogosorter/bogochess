use crate::core::model::{BitBoard, Color};
use std::ops::{Not, Index, IndexMut};

impl Not for Color {
    type Output = Self;

    fn not(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl Index<Color> for [BitBoard; 2] {
    type Output = BitBoard;
    fn index(&self, c: Color) -> &BitBoard {
        &self[c as usize]
    }
}

impl IndexMut<Color> for [BitBoard; 2] {
    fn index_mut(&mut self, c: Color) -> &mut BitBoard {
        &mut self[c as usize]
    }
}
