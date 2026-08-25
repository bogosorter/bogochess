use crate::core::model::*;

impl State {
    pub fn moves(&mut self) -> Vec<Move> {
        let pieces: Vec<(Position, Piece)> = self.board.iter().filter(|piece| piece.1.color == self.current_player).map(|(pos, piece)| (*pos, *piece)).collect();
        pieces.iter().map(|piece| self.piece_move(piece.0, piece.1, false)).flatten().collect()
    }

    pub fn piece_move(&mut self, position: Position, piece: Piece, in_check_test: bool) -> Vec<Move> {
        let moves = match piece.t {
            PieceType::Pawn => {
                let direction = if piece.color == Color::White { -1 } else { 1 };
                let offset = Position::new(direction, 0);
                let new_position = position + offset;

                let mut moves = Vec::new();

                // Forward and double-square move
                if !self.board.contains_key(&new_position) {
                    // Promotion
                    if piece.color == Color::White && new_position.row == 0 || piece.color == Color::Black && new_position.row == 7 {
                        for promoted_type in [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
                            moves.push(Move {
                                t: MoveType::Normal,
                                origin: position,
                                destination: new_position,
                                captured: None,
                                promotion: Some(promoted_type),
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    } else {
                        moves.push(Move {
                            t: MoveType::Normal,
                            origin: position,
                            destination: new_position,
                            captured: None,
                            promotion: None,
                            previous_castlings: self.castlings.clone(),
                            previous_en_passant: self.en_passant
                        });
                    }

                    // Two-square move is allowed
                    if piece.color == Color::White && position.row == 6 || piece.color == Color::Black && position.row == 1 {
                        let new_position = new_position + offset;
                        if !self.board.contains_key(&new_position) {
                            moves.push(Move {
                                t: MoveType::TwoSquare,
                                origin: position,
                                destination: new_position,
                                captured: None,
                                promotion: None,
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }

                // Capture
                let offsets = vec![Position::new(direction, -1), Position::new(direction, 1)];
                for offset in offsets {
                    let new_position = position + offset;
                    if let Some(&captured) = self.board.get(&new_position) && captured.color != self.current_player {
                        // Promotion
                        if piece.color == Color::White && new_position.row == 0 || piece.color == Color::Black && new_position.row == 7 {
                            for promoted_type in [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
                                moves.push(Move {
                                    t: MoveType::Normal,
                                    origin: position,
                                    destination: new_position,
                                    captured: Some(captured),
                                    promotion: Some(promoted_type),
                                    previous_castlings: self.castlings.clone(),
                                    previous_en_passant: self.en_passant
                                });
                            }
                        } else {
                            moves.push(Move {
                                t: MoveType::Normal,
                                origin: position,
                                destination: new_position,
                                captured: Some(captured),
                                promotion: None,
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }

                    // En-passant
                    else {
                        if let Some(en_passant) = self.en_passant && new_position == en_passant {
                            moves.push(Move {
                                t: MoveType::EnPassant,
                                origin: position,
                                destination: new_position,
                                captured: None,
                                promotion: None,
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }

                moves
            },

            PieceType::Knight => {
                let offsets = vec![
                    Position::new(2, -1),
                    Position::new(2, 1),
                    Position::new(-2, -1),
                    Position::new(-2, 1),
                    Position::new(1, -2),
                    Position::new(1, 2),
                    Position::new(-1, -2),
                    Position::new(-1, 2)
                ];

                self.offsets_move(position, offsets)
            },

            PieceType::Bishop => {
                let slides = vec![
                    Position::new(1, -1),
                    Position::new(1, 1),
                    Position::new(-1, -1),
                    Position::new(-1, 1)
                ];

                self.sliding_move(position, slides)
            },

            PieceType::Rook => {
                let slides = vec![
                    Position::new(1, 0),
                    Position::new(-1, 0),
                    Position::new(0, 1),
                    Position::new(0, -1)
                ];

                self.sliding_move(position, slides)
            },

            PieceType::Queen => {
                let slides = vec![
                    Position::new(1, -1),
                    Position::new(1, 0),
                    Position::new(1, 1),
                    Position::new(0, -1),
                    Position::new(0, 1),
                    Position::new(-1, -1),
                    Position::new(-1, 0),
                    Position::new(-1, 1),
                ];

                self.sliding_move(position, slides)
            },

            PieceType::King => {
                let offsets = vec![
                    Position::new(1, -1),
                    Position::new(1, 0),
                    Position::new(1, 1),
                    Position::new(0, -1),
                    Position::new(0, 1),
                    Position::new(-1, -1),
                    Position::new(-1, 0),
                    Position::new(-1, 1),
                ];

                let mut moves = self.offsets_move(position, offsets);

                // White castling queen-side
                if self.current_player == Color::White && self.castlings.contains(&Piece::new(PieceType::Queen, Color::White)) {
                    if !self.board.contains_key(&Position::new(7, 1)) && !self.board.contains_key(&Position::new(7, 2)) && !self.board.contains_key(&Position::new(7, 3)) {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let intermediate_position = Position::new(7, 3);
                        let king = self.board.get(&position).expect("king should be on the square").clone();
                        self.board.insert(intermediate_position, king);
                        let in_check = self.in_check();
                        self.board.remove(&intermediate_position);

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: position,
                                destination: Position::new(7, 2),
                                captured: None,
                                promotion: None,
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
                // White castling king-side
                if self.current_player == Color::White && self.castlings.contains(&Piece::new(PieceType::King, Color::White)) {
                    if !self.board.contains_key(&Position::new(7, 5)) && !self.board.contains_key(&Position::new(7, 6)) {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let intermediate_position = Position::new(7, 5);
                        let king = self.board.get(&position).expect("king should be on the square").clone();
                        self.board.insert(intermediate_position, king);
                        let in_check = self.in_check();
                        self.board.remove(&intermediate_position);

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: position,
                                destination: Position::new(7, 6),
                                captured: None,
                                promotion: None,
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
                // Black castling queen-side
                if self.current_player == Color::Black && self.castlings.contains(&Piece::new(PieceType::Queen, Color::Black)) {
                    if !self.board.contains_key(&Position::new(0, 1)) && !self.board.contains_key(&Position::new(0, 2)) && !self.board.contains_key(&Position::new(0, 3)) {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let intermediate_position = Position::new(0, 3);
                        let king = self.board.get(&position).expect("king should be on the square").clone();
                        self.board.insert(intermediate_position, king);
                        let in_check = self.in_check();
                        self.board.remove(&intermediate_position);

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: position,
                                destination: Position::new(0, 2),
                                captured: None,
                                promotion: None,
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }
                // Black castling king-side
                if self.current_player == Color::Black && self.castlings.contains(&Piece::new(PieceType::King, Color::Black)) {
                    if !self.board.contains_key(&Position::new(0, 5)) && !self.board.contains_key(&Position::new(0, 6)) {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let intermediate_position = Position::new(0, 5);
                        let king = self.board.get(&position).expect("king should be on the square").clone();
                        self.board.insert(intermediate_position, king);
                        let in_check = self.in_check();
                        self.board.remove(&intermediate_position);

                        if !in_check {
                            moves.push(Move {
                                t: MoveType::Castle,
                                origin: position,
                                destination: Position::new(0, 6),
                                captured: None,
                                promotion: None,
                                previous_castlings: self.castlings.clone(),
                                previous_en_passant: self.en_passant
                            });
                        }
                    }
                }

                moves
            }
        };

        // If any of the subsequent moves may lead to a king capture, we are
        // placing the king in chess, which is not allowed. To prevent an
        // infinite recursion, (since the moves needed inside the in_check call
        // are also calling in_check, we do not perform that check in the
        // recursive case).
        if !in_check_test {
            moves.into_iter().filter(|m| {
                self.apply(m);
                self.current_player = !self.current_player;
                let in_check = self.in_check();
                self.current_player = !self.current_player;
                self.undo(m);
                !in_check
            }).collect()
        } else {
            moves
        }
    }

    pub fn offsets_move(&self, position: Position, offsets: Vec<Position>) -> Vec<Move> {
        let destinations = offsets.iter().map(|offset| position + *offset).filter(|destination| {
            if !destination.is_valid() {
                return false;
            }

            if let Some(other_piece) = self.board.get(destination) {
                self.current_player != other_piece.color
            } else {
                true
            }
        });

        destinations.map(|destination| Move {
            t: MoveType::Normal,
            origin: position,
            destination,
            captured: self.board.get(&destination).copied(),
            promotion: None,
            previous_castlings: self.castlings.clone(),
            previous_en_passant: self.en_passant
        }).collect()
    }

    pub fn sliding_move(&self, position: Position, slides: Vec<Position>) -> Vec<Move> {
        slides.iter().map(|&slide| {
            let mut moves = Vec::new();
            let offset = slide;

            let mut current = position + offset;
            while current.is_valid() && !self.board.contains_key(&current) {
                moves.push(Move {
                    t: MoveType::Normal,
                    origin: position,
                    destination: current,
                    captured: None,
                    promotion: None,
                    previous_castlings: self.castlings.clone(),
                    previous_en_passant: self.en_passant
                });
                current += offset;
            }

            // Another move is still possible, if the piece blocking the movement can be captured
            if let Some(&captured) = self.board.get(&current) && captured.color != self.current_player {
                moves.push(Move {
                    t: MoveType::Normal,
                    origin: position,
                    destination: current,
                    captured: Some(captured),
                    promotion: None,
                    previous_castlings: self.castlings.clone(),
                    previous_en_passant: self.en_passant
                });
            }

            moves
        }).flatten().collect()
    }

    pub fn in_check(&mut self) -> bool {
        self.current_player = !self.current_player;
        let pieces: Vec<(Position, Piece)> = self.board.iter().filter(|piece| piece.1.color == self.current_player).map(|(pos, piece)| (*pos, *piece)).collect();
        let next_moves: Vec<Move> = pieces.iter().map(|piece| self.piece_move(piece.0, piece.1, true)).flatten().collect();
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
        let piece = self.board.remove(&m.origin).expect("position should have a piece");
        self.board.insert(m.destination, piece);
        self.en_passant = None;

        // If this is a castle, move the rook too
        if m.t == MoveType::Castle {
            if self.current_player == Color::White {
                // White queen-side
                if m.destination == Position::new(7, 2) {
                    let rook = self.board.remove(&Position::new(7, 0)).expect("position should have a rook");
                    self.board.insert(Position::new(7, 3), rook);
                }
                // White king-side
                else {
                    let rook = self.board.remove(&Position::new(7, 7)).expect("position should have a rook");
                    self.board.insert(Position::new(7, 5), rook);
                }
                self.castlings.retain(|piece| piece.color != Color::White);
            } else {
                // Black queen-side
                if m.destination == Position::new(0, 2) {
                    let rook = self.board.remove(&Position::new(0, 0)).expect("position should have a rook");
                    self.board.insert(Position::new(0, 3), rook);
                }
                // Black king-side
                else {
                    let rook = self.board.remove(&Position::new(0, 7)).expect("position should have a rook");
                    self.board.insert(Position::new(0, 5), rook);
                }
                self.castlings.retain(|piece| piece.color != Color::Black);
            }
        }
        // The en-passant square has to be set if this is a two-square move
        else if m.t == MoveType::TwoSquare {
            self.en_passant = Some(Position::new((m.origin.row + m.destination.row) / 2, m.origin.column));
        }
        else if m.t == MoveType::EnPassant {
            self.board.remove(&Position::new(m.origin.row, m.destination.column));
        }
        // Remove castling abilities if king or rook are moving
        else {
            // White king movement
            if m.origin == Position::new(7, 4) {
                self.castlings.retain(|piece| piece.color != Color::White);
            }
            // Black king movement
            else if m.origin == Position::new(0, 4) {
                self.castlings.retain(|piece| piece.color != Color::Black);
            }
            // White queen-side rook movement
            else if m.origin == Position::new(7, 0) {
                self.castlings.retain(|piece| piece.color != Color::White || piece.t != PieceType::Queen);
            }
            // White king-side rook movement
            else if m.origin == Position::new(7, 7) {
                self.castlings.retain(|piece| piece.color != Color::White || piece.t != PieceType::King);
            }
            // Black queen-side rook movement
            else if m.origin == Position::new(0, 0) {
                self.castlings.retain(|piece| piece.color != Color::Black || piece.t != PieceType::Queen);
            }
            // Black king-side rook movement
            else if m.origin == Position::new(0, 7) {
                self.castlings.retain(|piece| piece.color != Color::Black || piece.t != PieceType::King);
            }
        }

        if let Some(promoted_type) = m.promotion {
            self.board.insert(m.destination, Piece::new(promoted_type, piece.color));
        }

        // We have to prevent future castlings if a rook is captured
        if let Some(captured) = m.captured && captured.t == PieceType::Rook {
            if m.destination == Position::new(7, 0) {
                self.castlings.retain(|piece| piece.color != Color::White || piece.t != PieceType::Queen);
            }
            if m.destination == Position::new(7, 7) {
                self.castlings.retain(|piece| piece.color != Color::White || piece.t != PieceType::King);
            }
            if m.destination == Position::new(0, 0) {
                self.castlings.retain(|piece| piece.color != Color::Black || piece.t != PieceType::Queen);
            }
            if m.destination == Position::new(0, 7) {
                self.castlings.retain(|piece| piece.color != Color::Black || piece.t != PieceType::King);
            }
        }

        self.current_player = !self.current_player;
    }

    pub fn undo(&mut self, m: &Move) {
        let piece = self.board.remove(&m.destination).expect("position should have a piece");
        self.board.insert(m.origin, piece);

        if let Some(captured) = m.captured {
            self.board.insert(m.destination, captured);
        }

        self.current_player = !self.current_player;

        // If this is a castle, move the rook too
        if m.t == MoveType::Castle {
            if self.current_player == Color::White {
                // White queen-side
                if m.destination == Position::new(7, 2) {
                    let rook = self.board.remove(&Position::new(7, 3)).expect("position should have a rook");
                    self.board.insert(Position::new(7, 0), rook);
                }
                // White king-side
                else {
                    let rook = self.board.remove(&Position::new(7, 5)).expect("position should have a rook");
                    self.board.insert(Position::new(7, 7), rook);
                }
            } else {
                // Black queen-side
                if m.destination == Position::new(0, 2) {
                    let rook = self.board.remove(&Position::new(0, 3)).expect("position should have a rook");
                    self.board.insert(Position::new(0, 0), rook);
                }
                // Black king-side
                else {
                    let rook = self.board.remove(&Position::new(0, 5)).expect("position should have a rook");
                    self.board.insert(Position::new(0, 7), rook);
                }
            }
        }
        // The en-passant square has to be unset if this is a two-square move
        else if m.t == MoveType::EnPassant {
            self.board.insert(Position::new(m.origin.row, m.destination.column), Piece::new(PieceType::Pawn, !piece.color));
        }

        if !m.promotion.is_none() {
            self.board.insert(m.origin, Piece::new(PieceType::Pawn, piece.color));
        }

        self.castlings = m.previous_castlings.clone();
        self.en_passant = m.previous_en_passant;
    }
}
