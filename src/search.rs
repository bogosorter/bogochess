use crate::chess::GameState;
use crate::chess::Move;
use rand::seq::IteratorRandom;

pub fn best_move(state: &mut GameState) -> Option<Move> {
    return state.moves().into_iter().choose(&mut rand::rng());
}
