use crate::core::model::*;

impl GameState {
    // This function generates pseudo-legal moves (where checks are allowed).
    // Pseudo-legal moves are used because ensuring that there is no check
    // requires another step of move generation to see if the king can be
    // captured, and that makes the process ~20x slower. Since the next move
    // will be a king capture anyway, which will give the current move a bad
    // evaluation, we can discard the check.
    pub fn moves(&mut self) -> Vec<Move> {
        if self.ended {
            return Vec::new();
        }

        let mut moves = Vec::with_capacity(220);
        self.pawn_pushes(&mut moves);
        self.knight_movements(&mut moves);

        moves
    }

    pub fn pawn_pushes(&mut self, moves: &mut Vec<Move>) {
        let all = self.board.colors[0] & self.board.colors[1];

        match self.current_player {
            Color::White => {
                let pawns = self.board.colors[Color::White as usize] & self.board.pieces[PAWN];

                // Single pushes
                let pushable = pawns & !previous_row(all);
                for position in BitIterator(pushable) {
                    moves.push(Move::new(position, position + 8, MoveType::Normal, PAWN, None, None, self.en_passant, self.castling));
                }

                // Double pushes
                let double_pushable = pushable & !previous_row(previous_row(all));
                for position in BitIterator(double_pushable) {
                    moves.push(Move::new(position, position + 16, MoveType::TwoSquare, PAWN, None, None, self.en_passant, self.castling));
                }
            },
            Color::Black => {
                let pawns = self.board.colors[Color::Black as usize] & self.board.pieces[PAWN];

                // Single pushes
                let pushable = pawns & !next_row(all);
                for position in BitIterator(pushable) {
                    moves.push(Move::new(position, position - 8, MoveType::Normal, PAWN, None, None, self.en_passant, self.castling));
                }

                // Double pushes
                let double_pushable = pushable & !next_row(next_row(all));
                for position in BitIterator(double_pushable) {
                    moves.push(Move::new(position, position + 16, MoveType::TwoSquare, PAWN, None, None, self.en_passant, self.castling));
                }
            }
        }
    }

    pub fn knight_movements(&mut self, moves: &mut Vec<Move>) {
        let knights = self.board.colors[self.current_player as usize] & self.board.pieces[KNIGHT];
        let all = self.board.colors[0] | self.board.colors[1];

        for origin in BitIterator(knights) {
            let captures = KNIGHT_TABLE[origin] & self.board.colors[!self.current_player as usize];
            let non_captures = KNIGHT_TABLE[origin] & !all;

            for destination in BitIterator(captures) {
                moves.push(Move::new(origin, destination, MoveType::Normal, KNIGHT, Some(self.piece_at(destination)), None, self.en_passant, self.castling));
            }

            for destination in BitIterator(non_captures) {
                moves.push(Move::new(origin, destination, MoveType::Normal, KNIGHT, None, None, self.en_passant, self.castling));
            }
        }
    }

    pub fn apply(&mut self, m: &Move) {

    }

    pub fn undo(&mut self, m: &Move) {

    }

    pub fn in_check(&mut self) -> bool {
        false
    }

    fn piece_at(&self, position: usize) -> usize {
        let mask = 1 << position;
        for i in 0..6 {
            if (self.board.pieces[i] & mask) != 0 {
                return i;
            }
        }
        return 1;
    }
}

const KNIGHT_TABLE: [u64; 64] = knight_table();

const fn knight_table() -> [u64; 64] {
    let mut table = [0; 64];

    let mut i = 0;
    while i < 64 {
        table[i] = knight_movements(i);
        i += 1;
    }

    table
}

const fn knight_movements(position: usize) -> u64 {
    let row = (position / 8) as i32;
    let column = (position % 8) as i32;
    let offsets = [(-2, -1), (-2, 1), (2, -1), (2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2)];

    let mut result = 0;

    let mut i = 0;
    while i < 8 {
        let new_row = row + offsets[i].0;
        let new_column = column + offsets[i].1;
        i += 1;

        if new_row < 0 || 8 <= new_row {
            continue;
        }

        if new_column < 0 || 8 <= new_column {
            continue;
        }

        let index = new_row * 8 + new_column;
        result |= 1 << index;
    }

    result
}

struct BitIterator(u64);
impl Iterator for BitIterator {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.0 == 0 {
            return None;
        }

        let position = self.0.trailing_zeros() as usize;
        self.0 &= self.0 - 1;
        Some(position)
    }
}

#[inline]
pub fn previous_row(bitboard: u64) -> u64 {
    bitboard >> 8
}

#[inline]
pub fn next_row(bitboard: u64) -> u64 {
    bitboard << 8
}

#[inline]
pub fn previous_column(bitboard: u64) -> u64 {
    bitboard >> 1
}

#[inline]
pub fn next_column(bitboard: u64) -> u64 {
    bitboard << 1
}

#[inline]
pub fn row(n: usize) -> u64 {
    0xFF << n * 8
}

#[inline]
pub fn column(n: usize) -> u64 {
    0x0101010101010101 << n
}
