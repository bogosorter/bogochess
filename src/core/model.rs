mod state_impl;
mod position_impl;
mod square_impl;
mod piece_impl;
mod color_impl;


pub struct State {
    pub position: Option<Position>,
    pub zobrist: ZobristValues,
    pub tt: Box<TranspositionTable>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Position {
    pub board: Board,
    pub ended: bool,
    pub current_player: Color,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub halfmoves: u32
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Move {
    pub t: MoveType,
    pub origin: Square,
    pub destination: Square,
    pub captured: Option<Piece>,
    pub promotion: Option<PieceType>,
    pub previous_castling: Castling,
    pub previous_en_passant: Option<Square>
}

// Position not only represents valid chess positions but also offsets, which
// may include negative values. That is explains why the type of each of the
// coordinates is i32 and why there is an is_valid function.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Square {
    pub row: i32,
    pub column: i32
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Piece {
    pub t: PieceType,
    pub color: Color
}

pub type Board = [[Option<Piece>; 8]; 8];

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

bitflags::bitflags! {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Castling: u8 {
        const WhiteQueen = 0b0001;
        const WhiteKing = 0b0010;
        const BlackQueen = 0b0100;
        const BlackKing = 0b1000;
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MoveType {
    Normal,
    EnPassant,
    Castle,
    TwoSquare
}

#[derive(Debug)]
pub struct ZobristValues {
    pub piece_square: [[[[u64; 2]; 6]; 8]; 8],
    pub current_player: [u64; 2],
    pub castling: [u64; 16],
    pub en_passant: [u64; 8]
}

pub const TT_SIZE: usize = 1024 * 1024;
pub const TRANSPOSITION_MASK: usize = TT_SIZE - 1;
pub type TranspositionTable = [Option<TranspositionEntry>; TT_SIZE];

#[derive(Debug)]
pub struct TranspositionEntry {
    pub hash: u64,
    pub depth: u32,
    pub value: f32,
    pub best_move: Option<Move>,
    pub t: TranspositionType
}

#[derive(PartialEq, Eq, Debug)]
pub enum TranspositionType {
    Exact,
    UpperBound,
    LowerBound
}
