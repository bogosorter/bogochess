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
        self.pawn_captures(&mut moves);
        self.knight_movements(&mut moves);
        self.bishop_movements(&mut moves);
        self.rook_movements(&mut moves);
        self.queen_movements(&mut moves);
        self.king_movements(&mut moves);

        moves
    }

    fn pawn_pushes(&mut self, moves: &mut Vec<Move>) {
        let all = self.board.colors[0] | self.board.colors[1];
        let pawns = self.board.colors[self.current_player] & self.board.pieces[PieceType::Pawn];

        match self.current_player {
            Color::White => {
                // Single pushes
                let pushable = pawns & !all.shift_row_backward(1);
                for position in pushable {
                    moves.push(Move::new(position, position.shift(8), MoveType::Normal, PieceType::Pawn, None, None, self.en_passant, self.castling));
                }

                // Double pushes
                let double_pushable = pushable & BitBoard::row(1) & !all.shift_row_backward(2);
                for position in double_pushable {
                    moves.push(Move::new(position, position.shift(16), MoveType::TwoSquare, PieceType::Pawn, None, None, self.en_passant, self.castling));
                }
            },
            Color::Black => {
                // Single pushes
                let pushable = pawns & !all.shift_row_forward(1);
                for position in pushable {
                    moves.push(Move::new(position, position.shift(-8), MoveType::Normal, PieceType::Pawn, None, None, self.en_passant, self.castling));
                }

                // Double pushes
                let double_pushable = pushable & BitBoard::row(6) & !all.shift_row_forward(2);
                for position in double_pushable {
                    moves.push(Move::new(position, position.shift(-16), MoveType::TwoSquare, PieceType::Pawn, None, None, self.en_passant, self.castling));
                }
            }
        }
    }

    fn pawn_captures(&mut self, moves: &mut Vec<Move>) {
        let pawns = self.board.colors[self.current_player] & self.board.pieces[PieceType::Pawn];
        let theirs = self.board.colors[!self.current_player];

        match self.current_player {
            Color::White => {
                // Captures to the left
                let captures = pawns & (theirs.shift_row_backward(1).shift_column_forward(1) & !BitBoard::col(0));
                for position in captures {
                    let destination = position.shift(7);
                    moves.push(Move::new(position, destination, MoveType::Normal, PieceType::Pawn, self.piece_at(destination), None, self.en_passant, self.castling));
                }

                // Captures to the right
                let captures = pawns & (theirs.shift_row_backward(1).shift_column_backward(1) & !BitBoard::col(7));
                for position in captures {
                    let destination = position.shift(9);
                    moves.push(Move::new(position, destination, MoveType::Normal, PieceType::Pawn, self.piece_at(destination), None, self.en_passant, self.castling));
                }
            },
            Color::Black => {
                // Captures to the left
                let captures = pawns & (theirs.shift_row_forward(1).shift_column_forward(1) & !BitBoard::col(0));
                for position in captures {
                    let destination = position.shift(-9);
                    moves.push(Move::new(position, destination, MoveType::Normal, PieceType::Pawn, self.piece_at(destination), None, self.en_passant, self.castling));
                }

                // Captures to the right
                let captures = pawns & (theirs.shift_row_forward(1).shift_column_backward(1) & !BitBoard::col(7));
                for position in captures {
                    let destination = position.shift(-7);
                    moves.push(Move::new(position, destination, MoveType::Normal, PieceType::Pawn, self.piece_at(destination), None, self.en_passant, self.castling));
                }
            }
        }
    }

    fn knight_movements(&mut self, moves: &mut Vec<Move>) {
        self.offset_movements(moves, PieceType::Knight, &KNIGHT_TABLE);
    }

    fn bishop_movements(&mut self, moves: &mut Vec<Move>) {
        self.sliding_movements(moves, PieceType::Bishop, &[(-1, -1), (-1, 1), (1, -1), (1, 1)]);
    }

    fn rook_movements(&mut self, moves: &mut Vec<Move>) {
        self.sliding_movements(moves, PieceType::Rook, &[(-1, 0), (1, 0), (0, -1), (0, 1)]);
    }

    fn queen_movements(&mut self, moves: &mut Vec<Move>) {
        self.sliding_movements(moves, PieceType::Queen, &[(-1, -1), (-1, 1), (1, -1), (1, 1), (-1, 0), (1, 0), (0, -1), (0, 1)]);
    }

    fn king_movements(&mut self, moves: &mut Vec<Move>) {
        self.offset_movements(moves, PieceType::King, &KING_TABLE);
    }

    fn offset_movements(&mut self, moves: &mut Vec<Move>, t: PieceType, table: &[BitBoard; 64]) {
        let pieces = self.board.colors[self.current_player] & self.board.pieces[t];
        let all = self.board.colors[Color::White] | self.board.colors[Color::Black];

        for origin in pieces {
            let captures = table[origin] & self.board.colors[!self.current_player as usize];
            let non_captures = table[origin] & !all;

            for destination in captures {
                moves.push(Move::new(origin, destination, MoveType::Normal, t, self.piece_at(destination), None, self.en_passant, self.castling));
            }

            for destination in non_captures {
                moves.push(Move::new(origin, destination, MoveType::Normal, t, None, None, self.en_passant, self.castling));
            }
        }
    }

    fn sliding_movements(&mut self, moves: &mut Vec<Move>, t: PieceType, offsets: &[(i32, i32)]) {
        let pieces = self.board.colors[self.current_player] & self.board.pieces[t];
        let ours = self.board.colors[self.current_player];
        let theirs = self.board.colors[!self.current_player];

        for origin in pieces {
            for offset in offsets {
                let mut current_row;
                let mut current_column;
                let (row_offset, column_offset) = offset;
                (current_row, current_column) = (origin.row() as i32 + row_offset, origin.column() as i32 + column_offset);

                while Square::valid(current_row, current_column) {
                    let destination = Square::from(current_row as usize, current_column as usize);

                    if !(destination.bitboard() & ours).empty() {
                        break;
                    }

                    if !(destination.bitboard() & theirs).empty() {
                        moves.push(Move::new(origin, destination, MoveType::Normal, t, self.piece_at(destination), None, self.en_passant, self.castling));
                        break;
                    } else {
                        moves.push(Move::new(origin, destination, MoveType::Normal, t, None, None, self.en_passant, self.castling));
                    }

                    (current_row, current_column) = (current_row + row_offset, current_column + column_offset);
                }
            }
        }
    }

    pub fn apply(&mut self, m: &Move) {
        let from = m.origin();
        let to = m.destination();
        let piece = m.piece();

        // Update the captured piece. This has to be done before the moving
        // piece is updated - if done after, and if the types of the pieces
        // coincide, the removal of the piece would result in the removal of the
        // moving piece and not the captured piece.
        if let Some(captured) = m.captured() {
            self.board.colors[!self.current_player] &= !to.bitboard();
            self.board.pieces[captured] &= !to.bitboard();
        }

        // Update the piece that is moving
        self.board.colors[self.current_player] &= !from.bitboard();
        self.board.pieces[piece] &= !from.bitboard();
        self.board.colors[self.current_player] |= to.bitboard();
        self.board.pieces[piece] |= to.bitboard();

        self.current_player = !self.current_player;
    }

    pub fn undo(&mut self, m: &Move) {
        // The implementation is very similar to the implementation of apply,
        // but in reverse order. It is essentially undoing a stack of changes.

        let from = m.origin();
        let to = m.destination();
        let piece = m.piece();

        self.current_player = !self.current_player;

        // Update the piece that moved
        self.board.colors[self.current_player] &= !to.bitboard();
        self.board.pieces[piece] &= !to.bitboard();
        self.board.colors[self.current_player] |= from.bitboard();
        self.board.pieces[piece] |= from.bitboard();

        // Update the captured piece.
        if let Some(captured) = m.captured() {
            self.board.colors[!self.current_player] |= to.bitboard();
            self.board.pieces[captured] |= to.bitboard();
        }
    }

    pub fn in_check(&mut self) -> bool {
        false
    }

    fn piece_at(&self, position: Square) -> Option<PieceType> {
        let mask = position.bitboard();

        if !(self.board.pieces[PieceType::Pawn] & mask).empty() {
            return Some(PieceType::Pawn);
        }

        if !(self.board.pieces[PieceType::Knight] & mask).empty() {
            return Some(PieceType::Knight);
        }

        if !(self.board.pieces[PieceType::Bishop] & mask).empty() {
            return Some(PieceType::Bishop);
        }

        if !(self.board.pieces[PieceType::Rook] & mask).empty() {
            return Some(PieceType::Rook);
        }

        if !(self.board.pieces[PieceType::Queen] & mask).empty() {
            return Some(PieceType::Queen);
        }

        if !(self.board.pieces[PieceType::King] & mask).empty() {
            return Some(PieceType::King);
        }

        return None;
    }
}

const KNIGHT_TABLE: [BitBoard; 64] = offsets_table(&[(-2, -1), (-2, 1), (2, -1), (2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2)]);
const KING_TABLE: [BitBoard; 64] = offsets_table(&[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]);

const fn offsets_table(offsets: &[(i32, i32); 8]) -> [BitBoard; 64] {
    let mut table = [BitBoard(0); 64];

    let mut i = 0;
    while i < 64 {
        table[i] = offset_movements(&offsets, i);
        i += 1;
    }

    table
}

const fn offset_movements(offsets: &[(i32, i32); 8], position: usize) -> BitBoard {
    let row = (position / 8) as i32;
    let column = (position % 8) as i32;

    let mut result = 0;

    let mut i = 0;
    while i < 8 {
        let new_row = row + offsets[i].0;
        let new_column = column + offsets[i].1;
        i += 1;

        if !Square::valid(new_row, new_column) {
            continue;
        }

        let index = new_row * 8 + new_column;
        result |= 1 << index;
    }

    BitBoard(result)
}
