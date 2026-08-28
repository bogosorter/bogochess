use crate::core::model::*;

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
    pub fn new(from: Square, to: Square, t: MoveType, piece: PieceType, captured: Option<PieceType>, promoted: Option<PieceType>, previous_en_passant: Option<Square>, previous_castling: Castling) -> Move {
        let mut bits = from.0 as u32 | (to.0 as u32) << 6 | (t as u32) << 12 | (piece as u32) << 14 | (previous_castling.bits() as u32) << 27;

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
            bits |= (ep.0 as u32) << 23;
        } else {
            bits |= 0xF << 23;
        }

        Move(bits)
    }

    pub fn origin(&self) -> Square {
        Square((self.0 & 0x3F) as usize)
    }

    pub fn destination(&self) -> Square {
        Square(((self.0 >> 6) & 0x3F) as usize)
    }

    pub fn t(&self) -> MoveType {
        let bits = ((self.0 >> 12) & 0x3) as usize;
        [MoveType::Normal, MoveType::EnPassant, MoveType::Castle, MoveType::TwoSquare][bits]
    }

    pub fn piece(&self) -> PieceType {
        let bits = ((self.0 >> 14) & 0x07) as usize;
        [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King][bits]
    }

    pub fn captured(&self) -> Option<PieceType> {
        let bits = ((self.0 >> 17) & 0x7) as usize;

        if bits != 0x7 {
            Some([PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King][bits])
        } else {
            None
        }
    }

    pub fn promoted(&self) -> Option<PieceType> {
        let bits = ((self.0 >> 20) & 0x7) as usize;

        if bits != 0x7 {
            Some([PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King][bits])
        } else {
            None
        }
    }

    pub fn en_passant(&self) -> Option<PieceType> {
        let bits = ((self.0 >> 23) & 0xF) as usize;

        if bits != 0xF {
            Some([PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King][bits & 0x7])
        } else {
            None
        }
    }

    pub fn previous_castling(&self) -> Castling {
        Castling::from_bits(((self.0 >> 27) & 0xF) as u8).unwrap()
    }
}
