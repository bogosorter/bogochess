use std::collections::HashMap;
use std::ops::Add;

pub struct GameState {
    pub board: Board,
    pub active_color: Color,
    pub castlings: Vec<Piece>,
    pub en_passant: Option<Position>,
    pub halfmoves: u32
}

impl GameState {
    pub fn moves(&self) -> Vec<Move> {
        let pieces = self.board.iter().filter(|piece| piece.1.color == self.active_color);
        pieces.map(|piece| self.piece_move(*piece.0, piece.1)).flatten().collect()
    }

    pub fn piece_move(&self, position: Position, piece: &Piece) -> Vec<Move> {
        match piece.t {
            PieceType::Pawn => {
                let direction = if piece.color == Color::White { -1 } else { 1 };
                let offset = Position::new(direction, 0);
                let new_position = position + offset;

                if !self.board.contains_key(&new_position) {
                    vec![Move {
                        t: MoveType::Normal,
                        origin: position,
                        destination: new_position,
                        captured: None,
                        promotion: None
                    }]
                } else {
                    Vec::new()
                }
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

                let destinations = offsets.iter().map(|offset| position + *offset).filter(|destination| {
                    if !destination.is_valid() {
                        return false;
                    }

                    if let Some(other_piece) = self.board.get(destination) {
                        piece.color != other_piece.color
                    } else {
                        true
                    }
                });

                destinations.map(|destination| Move {
                    t: MoveType::Normal,
                    origin: position,
                    destination,
                    captured: self.board.get(&destination).copied(),
                    promotion: None
                }).collect()
            },
            _ => Vec::new()
        }
    }

    pub fn apply(&mut self, m: &Move) {
        let piece = self.board.remove(&m.origin).expect("position should have a piece");
        self.board.insert(m.destination, piece);
    }

    pub fn undo(&mut self, m: &Move) {
        let piece = self.board.remove(&m.destination).expect("position should have a piece");
        self.board.insert(m.origin, piece);

        if let Some(captured) = m.captured {
            self.board.insert(m.destination, captured);
        }
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

pub type Board = HashMap<Position, Piece>;

#[derive(Clone, Copy)]
pub struct Piece {
    pub t: PieceType,
    pub color: Color
}

#[derive(Clone, Copy)]
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
    pub promotion: Option<PieceType>
}

pub enum MoveType {
    Normal,
    EnPassant,
    Castle
}
