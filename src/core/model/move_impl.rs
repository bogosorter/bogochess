use crate::core::model::{Move, MoveType, Castling};

// Moves are represented as 32-bit integers whose fields are
// - from (6 bits)
// - to (6 bits)
// - type (2 bits)
// - piece (3 bits)
// - captured (3 bits)
// - promoted (3 bits)
// - previous en-passant (4 bits)
// - previous castling right (4 bits)
impl Move {
    pub fn new(from: usize, to: usize, t: MoveType, piece: usize, captured: Option<usize>, promoted: Option<usize>, previous_en_passant: Option<usize>, previous_castling: Castling) -> Move {
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

        Move(bits)
    }

    pub fn origin(&self) -> usize {
        (self.0 & 0x3F) as usize
    }

    pub fn destination(&self) -> usize {
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
