use crate::core::model::Color;
use std::ops::Not;

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
