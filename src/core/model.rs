use std::collections::HashMap;

mod position_impl;
mod piece_impl;
mod color_impl;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct State {
    pub board: Board,
    pub current_player: Color,
    pub castlings: Vec<Piece>,
    pub en_passant: Option<Position>,
    pub halfmoves: u32
}

#[derive(PartialEq, Eq, Debug)]
pub struct Move {
    pub t: MoveType,
    pub origin: Position,
    pub destination: Position,
    pub captured: Option<Piece>,
    pub promotion: Option<PieceType>,
    pub previous_castlings: Vec<Piece>,
    pub previous_en_passant: Option<Position>
}

// Position not only represents valid chess positions but also offsets, which
// may include negative values. That is explains why the type of each of the
// coordinates is i32 and why there is an is_valid function.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    pub row: i32,
    pub column: i32
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Piece {
    pub t: PieceType,
    pub color: Color
}

pub type Board = HashMap<Position, Piece>;

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
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

#[derive(PartialEq, Eq, Debug)]
pub enum MoveType {
    Normal,
    EnPassant,
    Castle,
    TwoSquare
}
