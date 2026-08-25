use crate::core::model::*;

impl State {
    pub fn moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] && piece.color == self.current_player {
                    moves.extend(self.piece_move(Position::new(row as i32, col as i32), piece, false));
                }
            }
        }

        moves
    }

    pub fn piece_move(&mut self, position: Position, piece: Piece, in_check_test: bool) -> Vec<Move> {
        let moves = match piece.t {
            PieceType::Pawn => {
                let direction = if piece.color == Color::White { -1 } else { 1 };
                let offset = Position::new(direction, 0);
                let new_position = position + offset;

                let mut moves = Vec::new();

                // Forward and double-square move
                if self.board[new_position.row as usize][new_position.column as usize].is_none() {
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
                        if self.board[new_position.row as usize][new_position.column as usize].is_none() {
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
                    if new_position.is_valid() && let Some(captured) = self.board[new_position.row as usize][new_position.column as usize] && captured.color != self.current_player {
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
                    if self.board[7][1].is_none() && self.board[7][2].is_none() && self.board[7][3].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[position.row as usize][position.column as usize].expect("king should be on the square");
                        self.board[7][3] = Some(king);
                        let in_check = self.in_check();
                        self.board[7][3] = None;

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
                    if self.board[7][5].is_none() && self.board[7][6].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[position.row as usize][position.column as usize].expect("king should be on the square");
                        self.board[7][5] = Some(king);
                        let in_check = self.in_check();
                        self.board[7][5] = None;

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
                    if self.board[0][1].is_none() && self.board[0][2].is_none() && self.board[0][3].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[position.row as usize][position.column as usize].expect("king should be on the square");
                        self.board[0][3] = Some(king);
                        let in_check = self.in_check();
                        self.board[0][3] = None;

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
                    if self.board[0][5].is_none() && self.board[0][6].is_none() {
                        // To prevent castling while the passing square is in
                        // check, we insert the king into the intermediate
                        // square and check if it is in check. The king is not
                        // removed from the original position to also ensure
                        // that the king itself is not in check when the move
                        // initiates.

                        let king = self.board[position.row as usize][position.column as usize].expect("king should be on the square");
                        self.board[0][5] = Some(king);
                        let in_check = self.in_check();
                        self.board[0][5] = None;

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

            if let Some(other_piece) = self.board[destination.row as usize][destination.column as usize] {
                self.current_player != other_piece.color
            } else {
                true
            }
        });

        destinations.map(|destination| Move {
            t: MoveType::Normal,
            origin: position,
            destination,
            captured: self.board[destination.row as usize][destination.column as usize],
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
            while current.is_valid() && self.board[current.row as usize][current.column as usize].is_none() {
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

            if current.is_valid() {
                // Another move is still possible, if the piece blocking the movement can be captured
                if let Some(captured) = self.board[current.row as usize][current.column as usize] && captured.color != self.current_player {
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
            }

            moves
        }).flatten().collect()
    }

    pub fn in_check(&mut self) -> bool {
        self.current_player = !self.current_player;

        let mut next_moves = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] && piece.color == self.current_player {
                    next_moves.extend(self.piece_move(Position::new(row as i32, col as i32), piece, true));
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
                if m.destination == Position::new(7, 2) {
                    let rook = self.board[7][0].take().expect("position should have a rook");
                    self.board[7][3] = Some(rook);
                }
                // White king-side
                else {
                    let rook = self.board[7][7].take().expect("position should have a rook");
                    self.board[7][5] = Some(rook);
                }
                self.castlings.retain(|piece| piece.color != Color::White);
            } else {
                // Black queen-side
                if m.destination == Position::new(0, 2) {
                    let rook = self.board[0][0].take().expect("position should have a rook");
                    self.board[0][3] = Some(rook);
                }
                // Black king-side
                else {
                    let rook = self.board[0][7].take().expect("position should have a rook");
                    self.board[0][5] = Some(rook);
                }
                self.castlings.retain(|piece| piece.color != Color::Black);
            }
        }
        // The en-passant square has to be set if this is a two-square move
        else if m.t == MoveType::TwoSquare {
            self.en_passant = Some(Position::new((m.origin.row + m.destination.row) / 2, m.origin.column));
        }
        else if m.t == MoveType::EnPassant {
            self.board[m.origin.row as usize][m.destination.column as usize] = None;
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
            self.board[m.destination.row as usize][m.destination.column as usize] = Some(Piece::new(promoted_type, piece.color));
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
        let piece = self.board[m.destination.row as usize][m.destination.column as usize].take().expect("position should have a piece");
        self.board[m.origin.row as usize][m.origin.column as usize] = Some(piece);

        if let Some(captured) = m.captured {
            self.board[m.destination.row as usize][m.destination.column as usize] = Some(captured);
        }

        self.current_player = !self.current_player;

        // If this is a castle, move the rook too
        if m.t == MoveType::Castle {
            if self.current_player == Color::White {
                // White queen-side
                if m.destination == Position::new(7, 2) {
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
                if m.destination == Position::new(0, 2) {
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

        self.castlings = m.previous_castlings.clone();
        self.en_passant = m.previous_en_passant;
    }
}
