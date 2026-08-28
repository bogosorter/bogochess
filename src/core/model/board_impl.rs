use crate::core::model::{Board, BitBoard};

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [BitBoard(0); 6],
            colors: [BitBoard(0); 2]
        }
    }
}
