use crate::core::model::*;

impl Position {
    // This function generates pseudo-legal moves (where checks are allowed).
    // Pseudo-legal moves are used because ensuring that there is no check
    // requires another step of move generation to see if the king can be
    // captured, and that makes the process ~20x slower. Since the next move
    // will be a king capture anyway, which will give the current move a bad
    // evaluation, we can discard the check.
    pub fn moves(&mut self) -> Vec<Move> {
        if self.ended {
            return Vec::new();
        }

        let mut moves = Vec::with_capacity(220);
        let mut player_pieces = self.board.colors[self.current_player as usize];


        moves
    }

    pub fn in_check(&mut self) -> bool {
        false
    }

    pub fn apply(&mut self, m: &Move) {

    }

    pub fn undo(&mut self, m: &Move) {

    }
}
