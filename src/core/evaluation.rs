use crate::core::model::{State, Move, PieceType, Color};
use std::cmp::Ordering;

impl State {
    pub fn value(&self) -> f32 {
        let mut score: f32 = 0.0;
        let mut current_king = false;
        let mut other_king = false;

        for row in 0..8 {
            for column in 0..8 {
                if let Some(piece) = self.board[row][column] {
                    let mut s = piece.t.value();

                    // Pawns that are further up the board are given a little bonus to
                    // encourage promotion
                    if piece.t == PieceType::Pawn {
                        if piece.color == Color::White {
                            s += (6 - row) as f32 * 0.1;
                        } else {
                            s += (row - 1) as f32 * 0.1;
                        }
                    } else if piece.t == PieceType::King {
                        if piece.color == self.current_player {
                            current_king = true;
                        } else {
                            other_king = true;
                        }
                    }

                    score += if piece.color == self.current_player { s } else { -s };
                }
            }
        }

        if !current_king {
            return -1.0;
        }

        if !other_king {
            return 1.0;
        }

        // We divide by 100 to ensure that checkmate (whose absolute value is 1)
        // is always preferred when possible
        score / 100.0
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.promotion != other.promotion {
            return Some(other.promotion.cmp(&self.promotion));
        }

        if self.captured != other.captured {
            return Some(other.captured.cmp(&self.captured));
        }

        None
    }
}

impl PieceType {
    pub fn value(&self) -> f32 {
        match self {
            PieceType::Pawn => 1.0,
            PieceType::Knight => 3.0,
            PieceType::Bishop => 3.0,
            PieceType::Rook => 5.0,
            PieceType::Queen => 9.0,
            PieceType::King => 0.0
        }
    }
}
