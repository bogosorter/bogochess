use crate::core::model::{GameState};

pub fn perft(game_state: &mut GameState, depth: u32) -> u32 {
    let result = perft_aux(game_state, depth, 0);
    println!("nodes searched: {}", result);
    result
}

fn perft_aux(game_state: &mut GameState, depth: u32, current_depth: u32) -> u32 {
    if current_depth == depth {
        return 1;
    }

    let mut result = 0;
    for m in game_state.legal_moves() {
        game_state.apply(&m);
        let count = perft_aux(game_state, depth, current_depth + 1);
        result += count;
        game_state.undo(&m);

        if current_depth == 0 {
            println!("{}: {}", m, count);
        }
    }

    result
}
