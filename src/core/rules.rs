use crate::core::model::*;

impl GameState {
    // Pseudo-legal moves are used because ensuring that there is no check
    // requires another step of move generation to see if the king can be
    // captured, and that makes the process ~20x slower. Since the next move
    // will be a king capture anyway, which will give the current move a bad
    // evaluation, we can discard the check.
    pub fn pseudo_legal_moves(&mut self) -> Vec<Move> {
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
        self.king_castling(&mut moves);

        moves
    }

    pub fn legal_moves(&mut self) -> Vec<Move> {
        self.pseudo_legal_moves().into_iter().filter(|m| {
            self.apply(m);
            self.current_player = !self.current_player;
            let valid = !self.in_check();
            self.current_player = !self.current_player;
            self.undo(m);
            valid
        }).collect()
    }

    pub fn in_check(&mut self) -> bool {
        let king = self.board.pieces[PieceType::King] & self.board.colors[self.current_player];
        self.attacked(king)
    }

    pub fn apply(&mut self, m: &Move) {
        let from = m.origin();
        let to = m.destination();
        let piece = m.piece();
        let t = m.t();

        // Update the captured piece. This has to be done before the moving
        // piece is updated - if done after, and if the types of the pieces
        // coincide, the removal of the piece would result in the removal of the
        // moving piece and not the captured piece.
        if let Some(captured) = m.captured() {
            self.board.colors[!self.current_player] &= !to.bitboard();
            self.board.pieces[captured] &= !to.bitboard();

            // Remove castling rights if the rook is captured. Note that we only
            // check for the position, and not for the piece type, to save a
            // couple operations: a piece being captured on that position will
            // always mean that that castling is not possible, even if the piece
            // being captured is not a rook.
            match to {
                Square(0) => self.castling.remove(Castling::WhiteQueen),
                Square(7) => self.castling.remove(Castling::WhiteKing),
                Square(56) => self.castling.remove(Castling::BlackQueen),
                Square(63) => self.castling.remove(Castling::BlackKing),
                _ => {}
            }
        }

        // Update the piece that is moving
        self.board.colors[self.current_player] &= !from.bitboard();
        self.board.pieces[piece] &= !from.bitboard();
        self.board.colors[self.current_player] |= to.bitboard();
        self.board.pieces[piece] |= to.bitboard();

        // Remove the pawn from its place when captured through en-passant
        if t == MoveType::EnPassant {
            let square = Square::from(from.row(), to.column());
            self.board.colors[!self.current_player] &= !square.bitboard();
            self.board.pieces[PieceType::Pawn] &= !square.bitboard();
        }

        // Update the en-passant square
        if t == MoveType::TwoSquare {
            if self.current_player == Color::White {
                self.en_passant = Some(m.origin().shift(8));
            } else {
                self.en_passant = Some(m.origin().shift(-8));
            }
        } else {
            self.en_passant = None;
        }

        // Also move the rook on castling
        if t == MoveType::Castle {
            match to {
                // White queen-side
                Square(2) => {
                    self.board.colors[self.current_player] &= !Square(0).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(0).bitboard();
                    self.board.colors[self.current_player] |= Square(3).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(3).bitboard();
                }
                // White king-side
                Square(6) => {
                    self.board.colors[self.current_player] &= !Square(7).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(7).bitboard();
                    self.board.colors[self.current_player] |= Square(5).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(5).bitboard();
                }
                // Black queen-side
                Square(58) => {
                    self.board.colors[self.current_player] &= !Square(56).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(56).bitboard();
                    self.board.colors[self.current_player] |= Square(59).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(59).bitboard();
                }
                // Black king-side
                Square(62) => {
                    self.board.colors[self.current_player] &= !Square(63).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(63).bitboard();
                    self.board.colors[self.current_player] |= Square(61).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(61).bitboard();
                }
                _ => panic!("illegal destination for a castle")
            }
        }

        // Remove castling rights when pieces move. Note that we only check for
        // the position, and not for the piece type, to save a couple
        // operations: a piece movig on that position will always mean that that
        // castling is not possible, even if the piece being moved is not a
        // rook.
        match from {
            // Rooks
            Square(0) => self.castling.remove(Castling::WhiteQueen),
            Square(7) => self.castling.remove(Castling::WhiteKing),
            Square(56) => self.castling.remove(Castling::BlackQueen),
            Square(63) => self.castling.remove(Castling::BlackKing),
            // Kings
            Square(4) => self.castling.remove(Castling::WhiteQueen | Castling::WhiteKing),
            Square(60) => self.castling.remove(Castling::BlackQueen | Castling::BlackKing),
            _ => {}
        }

        self.current_player = !self.current_player;
    }

    pub fn undo(&mut self, m: &Move) {
        // The implementation is very similar to the implementation of apply,
        // but in reverse order. It is essentially undoing a stack of changes.

        let from = m.origin();
        let to = m.destination();
        let piece = m.piece();
        let t = m.t();

        self.current_player = !self.current_player;

        // Restore castling
        self.castling = m.previous_castling();

        // Also move the rook on castling
        if t == MoveType::Castle {
            match to {
                // White queen-side
                Square(2) => {
                    self.board.colors[self.current_player] |= Square(0).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(0).bitboard();
                    self.board.colors[self.current_player] &= !Square(3).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(3).bitboard();
                }
                // White king-side
                Square(6) => {
                    self.board.colors[self.current_player] |= Square(7).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(7).bitboard();
                    self.board.colors[self.current_player] &= !Square(5).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(5).bitboard();
                }
                // Black queen-side
                Square(58) => {
                    self.board.colors[self.current_player] |= Square(56).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(56).bitboard();
                    self.board.colors[self.current_player] &= !Square(59).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(59).bitboard();
                }
                // Black king-side
                Square(62) => {
                    self.board.colors[self.current_player] |= Square(63).bitboard();
                    self.board.pieces[PieceType::Rook] |= Square(63).bitboard();
                    self.board.colors[self.current_player] &= !Square(61).bitboard();
                    self.board.pieces[PieceType::Rook] &= !Square(61).bitboard();
                }
                _ => panic!("illegal destination for a castle")
            }
        }

        // Restore en-passant
        self.en_passant = m.previous_en_passant();

        // Restore the pawn to its place when captured through en-passant
        if t == MoveType::EnPassant {
            let square = Square::from(from.row(), to.column());
            self.board.colors[!self.current_player] |= square.bitboard();
            self.board.pieces[PieceType::Pawn] |= square.bitboard();
        }

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

    fn pawn_pushes(&self, moves: &mut Vec<Move>) {
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

    fn pawn_captures(&self, moves: &mut Vec<Move>) {
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

                if let Some(en_passant) = self.en_passant {
                    let back_left = en_passant.bitboard().shift_row_backward(1).shift_column_forward(1) & !BitBoard::col(0);
                    let back_right = en_passant.bitboard().shift_row_backward(1).shift_column_backward(1) & !BitBoard::col(7);
                    let capturers = pawns & (back_left | back_right);

                    for capturer in capturers {
                        moves.push(Move::new(capturer, en_passant, MoveType::EnPassant, PieceType::Pawn, None, None, self.en_passant, self.castling));
                    }
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

                if let Some(en_passant) = self.en_passant {
                    let forward_left = en_passant.bitboard().shift_row_forward(1).shift_column_forward(1) & !BitBoard::col(0);
                    let forward_right = en_passant.bitboard().shift_row_forward(1).shift_column_backward(1) & !BitBoard::col(7);
                    let capturers = pawns & (forward_left | forward_right);

                    for capturer in capturers {
                        moves.push(Move::new(capturer, en_passant, MoveType::EnPassant, PieceType::Pawn, None, None, self.en_passant, self.castling));
                    }
                }
            }
        }
    }

    fn knight_movements(&self, moves: &mut Vec<Move>) {
        self.offset_movements(moves, PieceType::Knight, &KNIGHT_TABLE);
    }

    fn bishop_movements(&self, moves: &mut Vec<Move>) {
        self.sliding_movements(moves, PieceType::Bishop, &[(-1, -1), (-1, 1), (1, -1), (1, 1)]);
    }

    fn rook_movements(&self, moves: &mut Vec<Move>) {
        self.sliding_movements(moves, PieceType::Rook, &[(-1, 0), (1, 0), (0, -1), (0, 1)]);
    }

    fn queen_movements(&self, moves: &mut Vec<Move>) {
        self.sliding_movements(moves, PieceType::Queen, &[(-1, -1), (-1, 1), (1, -1), (1, 1), (-1, 0), (1, 0), (0, -1), (0, 1)]);
    }

    fn king_movements(&self, moves: &mut Vec<Move>) {
        self.offset_movements(moves, PieceType::King, &KING_TABLE);
    }

    fn king_castling(&mut self, moves: &mut Vec<Move>) {
        if self.current_player == Color::White {
            if self.castling.contains(Castling::WhiteQueen) {
                let required_empty = BitBoard(0x7 << 1);
                let not_in_check = BitBoard(0x7 << 2);
                let all = self.board.colors[0] | self.board.colors[1];

                if (required_empty & all).empty() && !self.attacked(not_in_check) {
                    moves.push(Move::new(Square(4), Square(2), MoveType::Castle, PieceType::King, None, None, self.en_passant, self.castling));
                }
            }

            if self.castling.contains(Castling::WhiteKing) {
                let required_empty = BitBoard(0x3 << 5);
                let not_in_check = BitBoard(0x7 << 4);
                let all = self.board.colors[0] | self.board.colors[1];

                if (required_empty & all).empty() && !self.attacked(not_in_check) {
                    moves.push(Move::new(Square(4), Square(6), MoveType::Castle, PieceType::King, None, None, self.en_passant, self.castling));
                }
            }
        } else {
            if self.castling.contains(Castling::BlackQueen) {
                let required_empty = BitBoard(0x7 << 57);
                let not_in_check = BitBoard(0x7 << 58);
                let all = self.board.colors[0] | self.board.colors[1];

                if (required_empty & all).empty() && !self.attacked(not_in_check) {
                    moves.push(Move::new(Square(60), Square(58), MoveType::Castle, PieceType::King, None, None, self.en_passant, self.castling));
                }
            }

            if self.castling.contains(Castling::BlackKing) {
                let required_empty = BitBoard(0x3 << 61);
                let not_in_check = BitBoard(0x7 << 60);
                let all = self.board.colors[0] | self.board.colors[1];

                if (required_empty & all).empty() && !self.attacked(not_in_check) {
                    moves.push(Move::new(Square(60), Square(62), MoveType::Castle, PieceType::King, None, None, self.en_passant, self.castling));
                }
            }
        }
    }

    fn offset_movements(&self, moves: &mut Vec<Move>, t: PieceType, table: &[BitBoard; 64]) {
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

    fn sliding_movements(&self, moves: &mut Vec<Move>, t: PieceType, offsets: &[(i32, i32)]) {
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

    fn attacked(&mut self, target: BitBoard) -> bool {
        self.current_player = !self.current_player;
        let result
             = self.pawn_attack(target)
            || self.knight_attack(target)
            || self.bishop_attack(target)
            || self.rook_attack(target)
            || self.queen_attack(target)
            || self.king_attack(target);
        self.current_player = !self.current_player;

        result
    }

    fn knight_attack(&self, target: BitBoard) -> bool {
        self.offset_attack(PieceType::Knight, &KNIGHT_TABLE, target)
    }

    fn bishop_attack(&self, target: BitBoard) -> bool {
        self.sliding_attack(PieceType::Bishop, &[(-1, -1), (-1, 1), (1, -1), (1, 1)], target)
    }

    fn rook_attack(&self, target: BitBoard) -> bool {
        self.sliding_attack(PieceType::Rook, &[(-1, 0), (1, 0), (0, -1), (0, 1)], target)
    }

    fn queen_attack(&self, target: BitBoard) -> bool {
        self.sliding_attack(PieceType::Queen, &[(-1, -1), (-1, 1), (1, -1), (1, 1), (-1, 0), (1, 0), (0, -1), (0, 1)], target)
    }

    fn king_attack(&self, target: BitBoard) -> bool {
        self.offset_attack(PieceType::King, &KING_TABLE, target)
    }

    fn pawn_attack(&self, target: BitBoard) -> bool {
        let pawns = self.board.colors[self.current_player] & self.board.pieces[PieceType::Pawn];

        match self.current_player {
            Color::White => {
                // Captures to the left
                let captures = pawns & (target.shift_row_backward(1).shift_column_forward(1) & !BitBoard::col(0));
                if !captures.empty() {
                    return true;
                }

                // Captures to the right
                let captures = pawns & (target.shift_row_backward(1).shift_column_backward(1) & !BitBoard::col(7));
                if !captures.empty() {
                    return true;
                }
            },
            Color::Black => {
                // Captures to the left
                let captures = pawns & (target.shift_row_forward(1).shift_column_forward(1) & !BitBoard::col(0));
                if !captures.empty() {
                    return true;
                }

                // Captures to the right
                let captures = pawns & (target.shift_row_forward(1).shift_column_backward(1) & !BitBoard::col(7));
                if !captures.empty() {
                    return true;
                }
            }
        }

        false
    }

    fn offset_attack(&self, t: PieceType, table: &[BitBoard; 64], target: BitBoard) -> bool {
        let pieces = self.board.colors[self.current_player] & self.board.pieces[t];

        for origin in pieces {
            let captures = table[origin] & target;
            if !captures.empty() {
                return true;
            }
        }

        false
    }

    fn sliding_attack(&self, t: PieceType, offsets: &[(i32, i32)], target: BitBoard) -> bool {
        let pieces = self.board.colors[self.current_player] & self.board.pieces[t];
        let all = self.board.colors[0] | self.board.colors[1];

        for origin in pieces {
            for offset in offsets {
                let mut current_row;
                let mut current_column;
                let (row_offset, column_offset) = offset;
                (current_row, current_column) = (origin.row() as i32 + row_offset, origin.column() as i32 + column_offset);

                while Square::valid(current_row, current_column) {
                    let destination = Square::from(current_row as usize, current_column as usize);

                    if !(destination.bitboard() & target).empty() {
                        return true;
                    }

                    if !(destination.bitboard() & all).empty() {
                        break;
                    }

                    (current_row, current_column) = (current_row + row_offset, current_column + column_offset);
                }
            }
        }

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
