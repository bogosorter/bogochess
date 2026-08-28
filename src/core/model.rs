use crate::core::search::transposition::TranspositionTable;

mod engine_state_impl;
mod board_impl;
mod bit_board_impl;
mod move_impl;
mod square_impl;
mod piece_type_impl;
mod color_impl;

pub struct EngineState {
    pub game_state: Option<GameState>,
    pub tt: TranspositionTable
}

#[derive(PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub current_player: Color,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub halfmoves: u8,
    pub ended: bool
}

#[derive(PartialEq, Eq)]
pub struct Board {
    pub pieces: [BitBoard; 6],
    pub colors: [BitBoard; 2]
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct BitBoard(u64);

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move(u32);

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Square(usize);

#[repr(usize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[repr(usize)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    White,
    Black
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub struct Castling: u8 {
        const WhiteQueen = 0b0001;
        const WhiteKing = 0b0010;
        const BlackQueen = 0b0100;
        const BlackKing = 0b1000;
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum MoveType {
    Normal,
    EnPassant,
    Castle,
    TwoSquare
}
