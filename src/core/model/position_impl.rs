use crate::core::model::{Position, ZobristValues};

impl Position {
    pub fn hash(&self, zobrist: &ZobristValues) -> u64 {
        let mut result
            = zobrist.current_player[self.current_player as usize]
            ^ zobrist.castling[self.castling.bits() as usize];

        if let Some(square) = self.en_passant {
            result ^= zobrist.en_passant[square.column as usize]
        }

        for row in 0..8 {
            for column in 0..8 {
                if let Some(piece) = self.board[row][column] {
                    result ^= zobrist.piece_square[row][column][piece.t as usize][piece.color as usize];
                }
            }
        }

        result
    }
}
