use crate::core::search::transposition::TranspositionTable;

mod state_impl;
mod color_impl;


pub struct State {
    pub position: Option<Position>,
    pub tt: TranspositionTable
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Position {
    pub board: Board,
    pub current_player: Color,
    pub castling: Castling,
    pub en_passant: Option<usize>,
    pub halfmoves: u8,
    pub ended: bool
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    pub pieces: [u64; 6],
    pub colors: [u64; 2]
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [0; 6],
            colors: [0, 2]
        }
    }
}

// Moves are represented as 32-bit integers whose fields are
// - from (6 bits)
// - to (6 bits)
// - type (2 bits)
// - piece (3 bits)
// - captured (3 bits)
// - promoted (3 bits)
// - previous en-passant (4 bits)
// - previous castling right (4 bits)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move(u32);

impl Move {
    pub fn new(from: usize, to: usize, t: MoveType, piece: usize, captured: Option<usize>, promoted: Option<usize>, previous_en_passant: Option<usize>, previous_castling: Castling) {
        let mut bits = from as u32 | (to as u32) << 6 | (t as u32) << 12 | (piece as u32) << 3 | (previous_castling.bits() as u32) << 27;

        if let Some(c) = captured {
            bits |= (c as u32) << 17;
        } else {
            bits |= 0x7 << 17;
        }

        if let Some(p) = promoted {
            bits |= (p as u32) << 20;
        } else {
            bits |= 0x7 << 20;
        }

        if let Some(ep) = previous_en_passant {
            bits |= (ep as u32) << 23;
        } else {
            bits |= 0xF << 23;
        }

        Move(bits);
    }

    pub fn from(&self) -> usize {
        (self.0 & 0x3F) as usize
    }

    pub fn to(&self) -> usize {
        ((self.0 >> 6) & 0x3F) as usize
    }

    pub fn t(&self) -> MoveType {
        let bits = ((self.0 >> 12) & 0x3) as usize;
        [MoveType::Normal, MoveType::EnPassant, MoveType::Castle, MoveType::TwoSquare][bits]
    }

    pub fn piece(&self) -> usize {
        ((self.0 >> 14) & 0x07) as usize
    }

    pub fn captured(&self) -> Option<usize> {
        let bits = ((self.0 >> 17) & 0x7) as usize;

        if bits != 0x7 {
            Some(bits)
        } else {
            None
        }
    }

    pub fn promoted(&self) -> Option<usize> {
        let bits = ((self.0 >> 20) & 0x7) as usize;

        if bits != 0x7 {
            Some(bits)
        } else {
            None
        }
    }

    pub fn en_passant(&self) -> Option<usize> {
        let bits = ((self.0 >> 23) & 0xF) as usize;

        if bits != 0xF {
            Some(bits & 0x7)
        } else {
            None
        }
    }

    pub fn previous_castling(&self) -> Castling {
        Castling::from_bits(((self.0 >> 27) & 0xF) as u8).unwrap()
    }
}

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

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum MoveType {
    Normal,
    EnPassant,
    Castle,
    TwoSquare
}
