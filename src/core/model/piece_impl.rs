use crate::core::model::{Piece, PieceType, Color};

impl Piece {
    pub fn new(t: PieceType, color: Color) -> Piece {
        Piece { t, color }
    }
}
