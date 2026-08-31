use crate::core::model::{GameState, Move};

use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;

pub struct TranspositionTable {
    size: u32,
    items: u32,
    mask: u64,
    zobrist: ZobristValues,
    depth_table: Box<[Option<TranspositionEntry>]>,
    fresh_table: Box<[Option<TranspositionEntry>]>
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
            depth_table: std::iter::repeat_with(|| None).take(size as usize).collect::<Vec<_>>().into_boxed_slice(),
            fresh_table: std::iter::repeat_with(|| None).take(size as usize).collect::<Vec<_>>().into_boxed_slice()
        }
    }

    pub fn get(&self, hash: u64, depth: u32) -> Option<&TranspositionEntry> {
        if let Some(entry) = self.depth_table[(hash & self.mask) as usize].as_ref() && entry.hash == hash && entry.depth >= depth {
           return Some(entry);
        }
        if let Some(entry) = self.fresh_table[(hash & self.mask) as usize].as_ref() && entry.hash == hash && entry.depth >= depth {
           return Some(entry);
        }
        None
    }

    pub fn insert(&mut self, hash: u64, depth: u32, value: f32, best_move: Option<Move>, t: TranspositionType) {
        let position = (hash & self.mask) as usize;

        if let Some(entry) = self.depth_table[position].as_ref() && entry.depth > depth {
            if self.fresh_table[position].is_none() {
                self.items += 1;
            }

            self.fresh_table[position] = Some(TranspositionEntry {
                hash,
                depth,
                value,
                best_move: best_move,
                t
            });
        } else {
            if self.depth_table[position].is_none() {
                self.items += 1;
            }

            self.depth_table[position] = Some(TranspositionEntry {
                hash,
                depth,
                value,
                best_move: best_move,
                t
            });
        }
    }

    pub fn hash(&self, game_state: &GameState) -> u64 {
        let mut result
            = self.zobrist.current_player[game_state.current_player as usize]
            ^ self.zobrist.castling[game_state.castling.bits() as usize];

        if let Some(square) = game_state.en_passant {
            result ^= self.zobrist.en_passant[square.column()]
        }

        //for row in 0..8 {
        //    for column in 0..8 {
        //        if let Some(piece) = position.board[row][column] {
        //            result ^= self.zobrist.piece_square[row][column][piece.t as usize][piece.color as usize];
        //        }
        //    }
        //}

        result
    }

    pub fn load(&self) -> f32 {
        self.items as f32 / self.size as f32 / 2.0
    }
}

#[derive(Debug)]
pub struct ZobristValues {
    pub piece_square: [[[[u64; 2]; 6]; 8]; 8],
    pub current_player: [u64; 2],
    pub castling: [u64; 16],
    pub en_passant: [u64; 8]
}

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
