use std::collections::HashMap;

pub struct GameState {
    pub board: Board,
    pub active_color: Color,
    pub castlings: Vec<Piece>,
    pub en_passant: Option<Position>,
    pub halfmoves: u32
}

pub type Position = (u32, u32);
pub type Board = HashMap<Position, Piece>;

pub struct Piece {
    pub piece: PieceType,
    pub color: Color
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

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
