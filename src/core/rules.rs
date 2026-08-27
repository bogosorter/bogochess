use crate::core::model::*;

impl Position {
    // This function generates pseudo-legal moves (where checks are allowed).
    // Pseudo-legal moves are used because ensuring that there is no check
    // requires another step of move generation to see if the king can be
    // captured, and that makes the process ~20x slower. Since the next move
    // will be a king capture anyway, which will give the current move a bad
    // evaluation, we can discard the check.
    pub fn moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(220);

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] && piece.color == self.current_player {
                    self.piece_move(&mut moves, Square::new(row as i32, col as i32), piece);
                }
            }
        }

        moves
    }

    pub fn piece_move(&mut self, mut moves: &mut Vec<Move>, square: Square, piece: Piece) {
        match piece.t {
            PieceType::Pawn => {
                let direction = if piece.color == Color::White { -1 } else { 1 };
                let offset = Square::new(direction, 0);
                let new_position = square + offset;

                // Forward and double-square move
                if self.board[new_position.row as usize][new_position.column as usize].is_none() {
                    // Promotion
                    if piece.color == Color::White && new_position.row == 0 || piece.color == Color::Black && new_position.row == 7 {
                        for promoted_type in [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
                            moves.push(Move {
                                t: MoveType::Normal,
                                origin: square,
                                destination: new_position,
                                captured: None,
                                promotion: Some(promoted_type),
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    } else {
                        moves.push(Move {
                            t: MoveType::Normal,
                            origin: square,
                            destination: new_position,
                            captured: None,
                            promotion: None,
                            previous_castling: self.castling.clone(),
                            previous_en_passant: self.en_passant
                        });
                    }

                    // Two-square move is allowed
                    if piece.color == Color::White && square.row == 6 || piece.color == Color::Black && square.row == 1 {
                        let new_position = new_position + offset;
                        if self.board[new_position.row as usize][new_position.column as usize].is_none() {
                            moves.push(Move {
                                t: MoveType::TwoSquare,
                                origin: square,
                                destination: new_position,
                                captured: None,
                                promotion: None,
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }

                // Capture
                let offsets = vec![Square::new(direction, -1), Square::new(direction, 1)];
                for offset in offsets {
                    let new_position = square + offset;
                    if new_position.is_valid() && let Some(captured) = self.board[new_position.row as usize][new_position.column as usize] && captured.color != self.current_player {
                        // Promotion
                        if piece.color == Color::White && new_position.row == 0 || piece.color == Color::Black && new_position.row == 7 {
                            for promoted_type in [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
                                moves.push(Move {
                                    t: MoveType::Normal,
                                    origin: square,
                                    destination: new_position,
                                    captured: Some(captured),
                                    promotion: Some(promoted_type),
                                    previous_castling: self.castling.clone(),
                                    previous_en_passant: self.en_passant
                                });
                            }
                        } else {
                            moves.push(Move {
                                t: MoveType::Normal,
                                origin: square,
                                destination: new_position,
                                captured: Some(captured),
                                promotion: None,
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }

                    // En-passant
                    else {
                        if let Some(en_passant) = self.en_passant && new_position == en_passant {
                            moves.push(Move {
                                t: MoveType::EnPassant,
                                origin: square,
                                destination: new_position,
                                captured: None,
                                promotion: None,
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
            },

            PieceType::Knight => {
                let offsets = vec![
                    Square::new(2, -1),
                    Square::new(2, 1),
                    Square::new(-2, -1),
                    Square::new(-2, 1),
                    Square::new(1, -2),
                    Square::new(1, 2),
                    Square::new(-1, -2),
                    Square::new(-1, 2)
                ];

                self.offsets_move(&mut moves, square, offsets);
            },

            PieceType::Bishop => {
                let slides = vec![
                    Square::new(1, -1),
                    Square::new(1, 1),
                    Square::new(-1, -1),
                    Square::new(-1, 1)
                ];

                self.sliding_move(&mut moves, square, slides);
            },

            PieceType::Rook => {
                let slides = vec![
                    Square::new(1, 0),
                    Square::new(-1, 0),
                    Square::new(0, 1),
                    Square::new(0, -1)
                ];

                self.sliding_move(&mut moves, square, slides);
            },

            PieceType::Queen => {
                let slides = vec![
                    Square::new(1, -1),
                    Square::new(1, 0),
                    Square::new(1, 1),
                    Square::new(0, -1),
                    Square::new(0, 1),
                    Square::new(-1, -1),
                    Square::new(-1, 0),
                    Square::new(-1, 1),
                ];

                self.sliding_move(&mut moves, square, slides);
            },

            PieceType::King => {
                let offsets = vec![
                    Square::new(1, -1),
                    Square::new(1, 0),
                    Square::new(1, 1),
                    Square::new(0, -1),
                    Square::new(0, 1),
                    Square::new(-1, -1),
                    Square::new(-1, 0),
                    Square::new(-1, 1),
                ];

                self.offsets_move(&mut moves, square, offsets);

                // White castling queen-side
                if self.current_player == Color::White && self.castling.contains(Castling::WhiteQueen) {
                    if self.board[7][1].is_none() && self.board[7][2].is_none() && self.board[7][3].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[square.row as usize][square.column as usize].expect("king should be on the square");
                        self.board[7][3] = Some(king);
                        let in_check = self.in_check();
                        self.board[7][3] = None;

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: square,
                                destination: Square::new(7, 2),
                                captured: None,
                                promotion: None,
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
                // White castling king-side
                if self.current_player == Color::White && self.castling.contains(Castling::WhiteKing) {
                    if self.board[7][5].is_none() && self.board[7][6].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[square.row as usize][square.column as usize].expect("king should be on the square");
                        self.board[7][5] = Some(king);
                        let in_check = self.in_check();
                        self.board[7][5] = None;

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: square,
                                destination: Square::new(7, 6),
                                captured: None,
                                promotion: None,
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
                // Black castling queen-side
                if self.current_player == Color::Black && self.castling.contains(Castling::BlackQueen) {
                    if self.board[0][1].is_none() && self.board[0][2].is_none() && self.board[0][3].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[square.row as usize][square.column as usize].expect("king should be on the square");
                        self.board[0][3] = Some(king);
                        let in_check = self.in_check();
                        self.board[0][3] = None;

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: square,
                                destination: Square::new(0, 2),
                                captured: None,
                                promotion: None,
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
                // Black castling king-side
                if self.current_player == Color::Black && self.castling.contains(Castling::BlackKing) {
                    if self.board[0][5].is_none() && self.board[0][6].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[square.row as usize][square.column as usize].expect("king should be on the square");
                        self.board[0][5] = Some(king);
                        let in_check = self.in_check();
                        self.board[0][5] = None;

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: square,
                                destination: Square::new(0, 6),
                                captured: None,
                                promotion: None,
                                previous_castling: self.castling.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
            }
        };
    }

    pub fn offsets_move(&self, moves: &mut Vec<Move>, square: Square, offsets: Vec<Square>){
        let destinations = offsets.iter().map(|offset| square + *offset).filter(|destination| {
            if !destination.is_valid() {
                return false;
            }

            if let Some(other_piece) = self.board[destination.row as usize][destination.column as usize] {
                self.current_player != other_piece.color
            } else {
                true
            }
        });

        for destination in destinations {
            moves.push(Move {
                t: MoveType::Normal,
                origin: square,
                destination,
                captured: self.board[destination.row as usize][destination.column as usize],
                promotion: None,
                previous_castling: self.castling.clone(),
                previous_en_passant: self.en_passant
            });
        }
    }

    pub fn sliding_move(&self, moves: &mut Vec<Move>, square: Square, slides: Vec<Square>) {
        for slide in slides {
            let offset = slide;

            let mut current = square + offset;
            while current.is_valid() && self.board[current.row as usize][current.column as usize].is_none() {
                moves.push(Move {
                    t: MoveType::Normal,
                    origin: square,
                    destination: current,
                    captured: None,
                    promotion: None,
                    previous_castling: self.castling.clone(),
                    previous_en_passant: self.en_passant
                });
                current += offset;
            }

            if current.is_valid() {
                // Another move is still possible, if the piece blocking the movement can be captured
                if let Some(captured) = self.board[current.row as usize][current.column as usize] && captured.color != self.current_player {
                    moves.push(Move {
                        t: MoveType::Normal,
                        origin: square,
                        destination: current,
                        captured: Some(captured),
                        promotion: None,
                        previous_castling: self.castling.clone(),
                        previous_en_passant: self.en_passant
                    });
                }
            }
        };
    }

    pub fn in_check(&mut self) -> bool {
        self.current_player = !self.current_player;

        let mut next_moves = Vec::with_capacity(220);
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] && piece.color == self.current_player {
                    self.piece_move(&mut next_moves, Square::new(row as i32, col as i32), piece);
                }
            }
        }

        self.current_player = !self.current_player;

        next_moves.iter().any(|next_move| {
            if let Some(captured) = next_move.captured {
                captured.t == PieceType::King
            } else {
                false
            }
        })
    }

    pub fn apply(&mut self, m: &Move) {
        let piece = self.board[m.origin.row as usize][m.origin.column as usize].take().expect("position should have a piece");
        self.board[m.destination.row as usize][m.destination.column as usize] = Some(piece);
        self.en_passant = None;

        // If this is a castle, move the rook too
        if m.t == MoveType::Castle {
            if self.current_player == Color::White {
                // White queen-side
                if m.destination == Square::new(7, 2) {
                    let rook = self.board[7][0].take().expect("position should have a rook");
                    self.board[7][3] = Some(rook);
                }
                // White king-side
                else {
                    let rook = self.board[7][7].take().expect("position should have a rook");
                    self.board[7][5] = Some(rook);
                }
                self.castling.remove(Castling::WhiteQueen | Castling::WhiteKing);
            } else {
                // Black queen-side
                if m.destination == Square::new(0, 2) {
                    let rook = self.board[0][0].take().expect("position should have a rook");
                    self.board[0][3] = Some(rook);
                }
                // Black king-side
                else {
                    let rook = self.board[0][7].take().expect("position should have a rook");
                    self.board[0][5] = Some(rook);
                }
                self.castling.remove(Castling::BlackQueen | Castling::BlackKing);
            }
        }
        // The en-passant square has to be set if this is a two-square move
        else if m.t == MoveType::TwoSquare {
            self.en_passant = Some(Square::new((m.origin.row + m.destination.row) / 2, m.origin.column));
        }
        else if m.t == MoveType::EnPassant {
            self.board[m.origin.row as usize][m.destination.column as usize] = None;
        }
        // Remove castling abilities if king or rook are moving
        else {
            // White king movement
            if m.origin == Square::new(7, 4) {
                self.castling.remove(Castling::WhiteQueen | Castling::WhiteKing);
            }
            // Black king movement
            else if m.origin == Square::new(0, 4) {
                self.castling.remove(Castling::BlackQueen | Castling::BlackKing);
            }
            // White queen-side rook movement
            else if m.origin == Square::new(7, 0) {
                self.castling.remove(Castling::WhiteQueen);
            }
            // White king-side rook movement
            else if m.origin == Square::new(7, 7) {
                self.castling.remove(Castling::WhiteKing);
            }
            // Black queen-side rook movement
            else if m.origin == Square::new(0, 0) {
                self.castling.remove(Castling::BlackQueen);
            }
            // Black king-side rook movement
            else if m.origin == Square::new(0, 7) {
                self.castling.remove(Castling::BlackKing);
            }
        }

        if let Some(promoted_type) = m.promotion {
            self.board[m.destination.row as usize][m.destination.column as usize] = Some(Piece::new(promoted_type, piece.color));
        }

        // We have to prevent future castlings if a rook is captured
        if let Some(captured) = m.captured {
            if captured.t == PieceType::Rook {
                if m.destination == Square::new(7, 0) {
                    self.castling.remove(Castling::WhiteQueen);
                }
                if m.destination == Square::new(7, 7) {
                    self.castling.remove(Castling::WhiteKing);
                }
                if m.destination == Square::new(0, 0) {
                    self.castling.remove(Castling::BlackQueen);
                }
                if m.destination == Square::new(0, 7) {
                    self.castling.remove(Castling::BlackKing);
                }
            } else if captured.t == PieceType::King {
                self.ended = true;
            }
        }

        self.current_player = !self.current_player;
    }

    pub fn undo(&mut self, m: &Move) {
        let piece = self.board[m.destination.row as usize][m.destination.column as usize].take().expect("position should have a piece");
        self.board[m.origin.row as usize][m.origin.column as usize] = Some(piece);

        if let Some(captured) = m.captured {
            self.board[m.destination.row as usize][m.destination.column as usize] = Some(captured);

            if captured.t == PieceType::King {
                self.ended = false;
            }
        }

        self.current_player = !self.current_player;

        // If this is a castle, move the rook too
        if m.t == MoveType::Castle {
            if self.current_player == Color::White {
                // White queen-side
                if m.destination == Square::new(7, 2) {
                    let rook = self.board[7][3].take().expect("position should have a rook");
                    self.board[7][0] = Some(rook);
                }
                // White king-side
                else {
                    let rook = self.board[7][5].take().expect("position should have a rook");
                    self.board[7][7] = Some(rook);
                }
            } else {
                // Black queen-side
                if m.destination == Square::new(0, 2) {
                    let rook = self.board[0][3].take().expect("position should have a rook");
                    self.board[0][0] = Some(rook);
                }
                // Black king-side
                else {
                    let rook = self.board[0][5].take().expect("position should have a rook");
                    self.board[0][7] = Some(rook);
                }
            }
        }
        // The en-passant square has to be unset if this is a two-square move
        else if m.t == MoveType::EnPassant {
            self.board[m.origin.row as usize][m.destination.column as usize] = Some(Piece::new(PieceType::Pawn, !piece.color));
        }

        if !m.promotion.is_none() {
            self.board[m.origin.row as usize][m.origin.column as usize] = Some(Piece::new(PieceType::Pawn, piece.color));
        }

        self.castling = m.previous_castling.clone();
        self.en_passant = m.previous_en_passant;
    }
}
