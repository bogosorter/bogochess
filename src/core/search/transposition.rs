use crate::core::model::{Position, Move};

use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;

pub struct TranspositionTable {
    size: u32,
    items: u32,
    mask: u64,
    zobrist: ZobristValues,
    table: Box<[Option<TranspositionEntry>]>
}

impl TranspositionTable {
    pub fn new(size: u32) -> TranspositionTable {
        let mut rng = StdRng::seed_from_u64(42);

        TranspositionTable {
            size,
            items: 0,
            mask: size as u64 - 1,
            zobrist: ZobristValues {
                piece_square: std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| rng.random())))),
                current_player: std::array::from_fn(|_| rng.random()),
                castling: std::array::from_fn(|_| rng.random()),
                en_passant: std::array::from_fn(|_| rng.random())
            },
            table: std::iter::repeat_with(|| None).take(size as usize).collect::<Vec<_>>().into_boxed_slice()
        }
    }

    pub fn get(&self, hash: u64, depth: i32) -> Option<&TranspositionEntry> {
        if let Some(entry) = self.table[(hash & self.mask) as usize].as_ref() && entry.hash == hash && entry.depth >= depth {
            Some(entry)
        } else {
            None
        }
    }

    pub fn insert(&mut self, hash: u64, depth: i32, value: f32, best_move: &Option<Move>, t: TranspositionType) {
        let position = (hash & self.mask) as usize;

        if self.table[position].is_none() {
            self.items += 1;
        }

        self.table[position] = Some(TranspositionEntry {
            hash,
            depth: depth,
            value,
            best_move: best_move.clone(),
            t
        });
    }

    pub fn hash(&self, position: &Position) -> u64 {
        let mut result
            = self.zobrist.current_player[position.current_player as usize]
            ^ self.zobrist.castling[position.castling.bits() as usize];

        if let Some(square) = position.en_passant {
            result ^= self.zobrist.en_passant[square.column as usize]
        }

        for row in 0..8 {
            for column in 0..8 {
                if let Some(piece) = position.board[row][column] {
                    result ^= self.zobrist.piece_square[row][column][piece.t as usize][piece.color as usize];
                }
            }
        }

        result
    }

    pub fn load(&self) -> f32 {
        self.items as f32 / self.size as f32
    }
}

#[derive(Debug)]
pub struct ZobristValues {
    pub piece_square: [[[[u64; 2]; 6]; 8]; 8],
    pub current_player: [u64; 2],
    pub castling: [u64; 16],
    pub en_passant: [u64; 8]
}

#[derive(Debug)]
pub struct TranspositionEntry {
    pub hash: u64,
    pub depth: i32,
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
