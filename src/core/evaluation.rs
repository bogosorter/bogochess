use crate::core::model::{GameState, Square, Move, Color, PieceType};
use std::cmp::Ordering;

impl GameState {
    pub fn value(&self) -> f32 {
        // All scores are calculated from white's perspective, and inverted if needed

        // We can cut the calculation short if one of the kings is missing. This
        // is possible because search uses pseudo-legal moves
        let kings = self.board.pieces[PieceType::King];
        if (self.board.colors[self.current_player] & kings).empty() {
            return -1.0;
        }
        if (self.board.colors[!self.current_player] & kings).empty() {
            return 1.0;
        }

        let white = self.board.colors[Color::White];
        let black = self.board.colors[Color::Black];
        let mut material: f32 = 0.0;

        let mut material_score = 0.0;
        let mut development_score: f32 = 0.0;
        let mut endgame_score: f32 = 0.0;

        let types = [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King];
        for t in types {
            for position in self.board.pieces[t] {
                let s = t.value();
                material += s;

                if (white & position.bitboard()).nonempty() {
                    // For historical reasons, the bonus tables consider row 0
                    // to be the top row. Since we'll have to invert the row of
                    // black or white pieces either way, I left it that way.
                    let inverted = Square::from(7 - position.row(), position.column());
                    material_score += s;
                    development_score += BONUS_TABLES[t][0][inverted];
                    endgame_score += BONUS_TABLES[t][1][inverted];
                } else {
                    material_score -= s;
                    development_score -= BONUS_TABLES[t][0][position];
                    endgame_score -= BONUS_TABLES[t][1][position];
                }
            }
        }

        let phase = material / 78.0;
        let mut score = material_score + development_score * phase + endgame_score * (1.0 - phase);

        // We give a bonus to the bishop pair
        let bishops = self.board.pieces[PieceType::Bishop];
        if (white & bishops).count() >= 2 {
            score += 0.5;
        }
        if (black & bishops).count() >= 2 {
            score -= 0.5;
        }

        // We penalize the rook pair
        let rooks = self.board.pieces[PieceType::Rook];
        if (white & rooks).count() >= 2 {
            score -= 0.3;
        }
        if (black & rooks).count() >= 2 {
            score += 0.3;
        }

        if self.current_player == Color::Black {
            score = -score;
        }

        // We divide by 100 to ensure that checkmate (whose absolute value is 1)
        // is always preferred when possible
        score / 100.0
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

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.promoted() != other.promoted() {
            return Some(other.promoted().cmp(&self.promoted()));
        }

        if self.captured() != other.captured() {
            return Some(other.captured().cmp(&self.captured()));
        }

        None
    }
}

const BONUS_TABLES: [[[f32; 64]; 2]; 6]  = [
    [
        [ // Pawns
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.15, 0.15, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.0,
            0.05, -0.05, -0.1, 0.0, 0.0, -0.1, -0.05, 0.05,
            0.05, 0.1, 0.1, -0.1, -0.1, 0.1, 0.1, 0.05,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ], [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4,
            0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3,
            0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2,
            0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ]
    ], [
        [ // Knights
            -0.5, -0.4, -0.3, -0.3, -0.3, -0.3, -0.4, -0.5,
            -0.4, -0.2, 0.0, 0.0, 0.0, 0.0, -0.2, -0.4,
            -0.3, 0.0, 0.1, 0.15, 0.15, 0.1, 0.0, -0.3,
            -0.3, 0.0, 0.15, 0.2, 0.2, 0.15, 0.0, -0.3,
            -0.3, 0.0, 0.15, 0.2, 0.2, 0.15, 0.0, -0.3,
            -0.3, 0.0, 0.1, 0.15, 0.15, 0.1, 0.0, -0.3,
            -0.4, -0.2, 0.0, 0.0, 0.0, 0.0, -0.2, -0.4,
            -0.5, -0.4, -0.3, -0.3, -0.3, -0.3, -0.4, -0.5
        ], [
            -0.5, -0.4, -0.3, -0.3, -0.3, -0.3, -0.4, -0.5,
            -0.4, -0.2, 0.0, 0.0, 0.0, 0.0, -0.2, -0.4,
            -0.3, 0.0, 0.1, 0.15, 0.15, 0.1, 0.0, -0.3,
            -0.3, 0.0, 0.15, 0.2, 0.2, 0.15, 0.0, -0.3,
            -0.3, 0.0, 0.15, 0.2, 0.2, 0.15, 0.0, -0.3,
            -0.3, 0.0, 0.1, 0.15, 0.15, 0.1, 0.0, -0.3,
            -0.4, -0.2, 0.0, 0.0, 0.0, 0.0, -0.2, -0.4,
            -0.5, -0.4, -0.3, -0.3, -0.3, -0.3, -0.4, -0.5
        ]
    ], [ // Bishops
        [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ], [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ]
    ], [
        [ // Rooks
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.05, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ], [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.05, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ]
    ], [
        [ // Queens
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ], [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
        ]
    ], [
        [ // Kings
            -0.3, -0.4, -0.4, -0.5, -0.5, -0.4, -0.4, -0.3,
            -0.3, -0.4, -0.4, -0.5, -0.5, -0.4, -0.4, -0.3,
            -0.3, -0.4, -0.4, -0.5, -0.5, -0.4, -0.4, -0.3,
            -0.3, -0.4, -0.4, -0.5, -0.5, -0.4, -0.4, -0.3,
            -0.2, -0.3, -0.3, -0.4, -0.4, -0.3, -0.3, -0.2,
            -0.1, -0.2, -0.2, -0.2, -0.2, -0.2, -0.2, -0.1,
            0.2, 0.2, 0.0, 0.0, 0.0, 0.0, 0.2, 0.2,
            0.2, 0.3, 0.1, 0.0, 0.0, 0.1, 0.3, 0.2
        ], [
            -0.5, -0.4, -0.3, -0.2, -0.2, -0.3, -0.4, -0.5,
            -0.3, -0.2, -0.1, 0.0, 0.0, -0.1, -0.2, -0.3,
            -0.3, -0.1, 0.2, 0.3, 0.3, 0.2, -0.1, -0.3,
            -0.3, -0.1, 0.3, 0.4, 0.4, 0.3, -0.1, -0.3,
            -0.3, -0.1, 0.3, 0.4, 0.4, 0.3, -0.1, -0.3,
            -0.3, -0.1, 0.2, 0.3, 0.3, 0.2, -0.1, -0.3,
            -0.3, -0.2, 0.0, 0.0, 0.0, 0.0, -0.2, -0.3,
            -0.5, -0.4, -0.3, -0.3, -0.3, -0.3, -0.3, -0.05
        ]
    ]
];
