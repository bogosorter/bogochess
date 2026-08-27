use crate::core::model::{State, ZobristValues};

use rand::{RngExt, SeedableRng};
use rand::rngs::StdRng;

impl State {
    pub fn new() -> State {
        let mut rng = StdRng::seed_from_u64(42);
        State {
            position: None,
            zobrist: ZobristValues {
                piece_square: std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| std::array::from_fn(|_| rng.random())))),
                current_player: std::array::from_fn(|_| rng.random()),
                castling: std::array::from_fn(|_| rng.random()),
                en_passant: std::array::from_fn(|_| rng.random())
            }
        }
    }
}
