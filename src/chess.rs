use std::collections::HashMap;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Not;

pub struct GameState {
    pub board: Board,
    pub active_color: Color,
    pub castlings: Vec<Piece>,
    pub en_passant: Option<Position>,
    pub halfmoves: u32
}

impl GameState {
    pub fn moves(&mut self) -> Vec<Move> {
        let pieces: Vec<(Position, Piece)> = self.board.iter().filter(|piece| piece.1.color == self.active_color).map(|(pos, piece)| (*pos, *piece)).collect();
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
                    moves.push(Move {
                        t: MoveType::Normal,
                        origin: position,
                        destination: new_position,
                        captured: None,
                        promotion: None,
                        previous_castlings: self.castlings.clone()
                    });

                    // Two-square move is allowed
                    if piece.color == Color::White && position.row == 6 || piece.color == Color::Black && position.row == 1 {
                        let new_position = new_position + offset;
                        if !self.board.contains_key(&new_position) {
                            moves.push(Move {
                                t: MoveType::Normal,
                                origin: position,
                                destination: new_position,
                                captured: None,
                                promotion: None,
                                previous_castlings: self.castlings.clone()
                            });
                        }
                    }
                }

                // Capture
                let offsets = vec![Position::new(direction, -1), Position::new(direction, 1)];
                for offset in offsets {
                    let new_position = position + offset;
                    if let Some(&captured) = self.board.get(&new_position) && captured.color != self.active_color {
                        moves.push(Move {
                            t: MoveType::Normal,
                            origin: position,
                            destination: new_position,
                            captured: Some(captured),
                            promotion: None,
                            previous_castlings: self.castlings.clone()
                        });
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

                // TODO: No castling if in check
                // TODO: No castling if any of the squares the king moves through is under attack

                // White castling queen-side
                if self.active_color == Color::White && self.castlings.contains(&Piece::new(PieceType::Queen, Color::White)) {
                    if !self.board.contains_key(&Position::new(7, 1)) && !self.board.contains_key(&Position::new(7, 2)) && !self.board.contains_key(&Position::new(7, 3)) {
                        moves.push(Move {
                            t: MoveType::Castle,
                            origin: position,
                            destination: Position::new(7, 2),
                            captured: None,
                            promotion: None,
                            previous_castlings: self.castlings.clone()
                        });
                    }
                }
                // White castling king-side
                if self.active_color == Color::White && self.castlings.contains(&Piece::new(PieceType::King, Color::White)) {
                    if !self.board.contains_key(&Position::new(7, 5)) && !self.board.contains_key(&Position::new(7, 6)) {
                        moves.push(Move {
                            t: MoveType::Castle,
                            origin: position,
                            destination: Position::new(7, 6),
                            captured: None,
                            promotion: None,
                            previous_castlings: self.castlings.clone()
                        });
                    }
                }
                // Black castling queen-side
                if self.active_color == Color::Black && self.castlings.contains(&Piece::new(PieceType::Queen, Color::Black)) {
                    if !self.board.contains_key(&Position::new(0, 1)) && !self.board.contains_key(&Position::new(0, 2)) && !self.board.contains_key(&Position::new(0, 3)) {
                        moves.push(Move {
                            t: MoveType::Castle,
                            origin: position,
                            destination: Position::new(0, 2),
                            captured: None,
                            promotion: None,
                            previous_castlings: self.castlings.clone()
                        });
                    }
                }
                // Black castling king-side
                if self.active_color == Color::Black && self.castlings.contains(&Piece::new(PieceType::King, Color::Black)) {
                    if !self.board.contains_key(&Position::new(0, 5)) && !self.board.contains_key(&Position::new(0, 6)) {
                        moves.push(Move {
                            t: MoveType::Castle,
                            origin: position,
                            destination: Position::new(0, 6),
                            captured: None,
                            promotion: None,
                            previous_castlings: self.castlings.clone()
                        });
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
                let in_check = self.in_check();
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
                self.active_color != other_piece.color
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
            previous_castlings: self.castlings.clone()
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
                    previous_castlings: self.castlings.clone()
                });
                current += offset;
            }

            // Another move is still possible, if the piece blocking the movement can be captured
            if let Some(&captured) = self.board.get(&current) && captured.color != self.active_color {
                moves.push(Move {
                    t: MoveType::Normal,
                    origin: position,
                    destination: current,
                    captured: Some(captured),
                    promotion: None,
                    previous_castlings: self.castlings.clone()
                });
            }

            moves
        }).flatten().collect()
    }

    pub fn in_check(&mut self) -> bool {
        let pieces: Vec<(Position, Piece)> = self.board.iter().filter(|piece| piece.1.color == self.active_color).map(|(pos, piece)| (*pos, *piece)).collect();
        let next_moves: Vec<Move> = pieces.iter().map(|piece| self.piece_move(piece.0, piece.1, true)).flatten().collect();
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

        // If this is a castle, move the rook too
        if m.t == MoveType::Castle {
            if self.active_color == Color::White {
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

        self.active_color = !self.active_color;
    }

    pub fn undo(&mut self, m: &Move) {
        let piece = self.board.remove(&m.destination).expect("position should have a piece");
        self.board.insert(m.origin, piece);

        if let Some(captured) = m.captured {
            self.board.insert(m.destination, captured);
        }

        self.active_color = !self.active_color;

        // If this is a castle, move the rook too
        if m.t == MoveType::Castle {
            if self.active_color == Color::White {
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

        self.castlings = m.previous_castlings.clone();
    }
}

// Position not only represents valid chess positions but also offsets, which
// may include negative values. That is explains why the type of each of the
// coordinates is i32 and why there is an is_valid function

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub row: i32,
    pub column: i32
}

impl Position {
    pub fn new(row: i32, column: i32) -> Position {
        Position {row, column}
    }

    pub fn is_valid(&self) -> bool {
        self.row >= 0 && self.row < 8 && self.column >= 0 && self.column < 8
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, other: Position) -> Position {
        Position {
            row: self.row + other.row,
            column: self.column + other.column
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        self.row += other.row;
        self.column += other.column;
    }
}

pub type Board = HashMap<Position, Piece>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub t: PieceType,
    pub color: Color
}

impl Piece {
    pub fn new(t: PieceType, color: Color) -> Piece {
        Piece {
            t,
            color
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

pub struct Castling {
    pub t: CastlingType,
    pub color: Color
}

pub enum CastlingType {
    KingSide,
    QueenSide
}

pub struct Move {
    pub t: MoveType,
    pub origin: Position,
    pub destination: Position,
    pub captured: Option<Piece>,
    pub promotion: Option<PieceType>,
    pub previous_castlings: Vec<Piece>
}

#[derive(PartialEq, Eq)]
pub enum MoveType {
    Normal,
    EnPassant,
    Castle
}
